use std::{collections::HashMap, str::FromStr};

use any_value::AnyValue;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use regex::Regex;
use scamplers_core::{
    model::dataset::{
        DatasetSummary, NewChromiumDataset, NewChromiumDatasetCore, ParsedMetricsFile, chromium,
    },
    result::{DatasetCmdlineError, DatasetMetricsFileParseError, DatasetNMetricsFilesError},
};
use scamplers_schema::{chemistry, chip_loading, dataset, gems, suspension, suspension_pool};
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::model::WriteToDbInternal,
    result::{ScamplersError, ScamplersResult},
};

#[derive(Insertable)]
#[diesel(table_name = dataset, check_for_backend(Pg))]
struct NewParsedChromiumDataset {
    #[diesel(embed)]
    core: NewChromiumDatasetCore,
    metrics: ParsedMetricsFile,
}

trait ParseMetricsFile {
    fn parse(self) -> ScamplersResult<ParsedMetricsFile>;
}

fn read_csv(raw: &ValidString) -> ScamplersResult<Vec<HashMap<String, AnyValue>>> {
    let rdr = csv::Reader::from_reader(raw.as_bytes());
    let records = rdr.into_deserialize();
    let records: csv::Result<Vec<HashMap<String, AnyValue>>> = records.collect();

    records.map_err(|err| {
        ScamplersError::new_unprocessable_entity_error(DatasetMetricsFileParseError {
            message: format!("failed to parse 10x csv:\n{err}"),
        })
    })
}

fn parse_tenx_record(csv: HashMap<String, AnyValue>) -> HashMap<String, AnyValue> {
    let number_regex = Regex::new(r"^(\d+)\s\(\d{1,3}\.\d+\)$").unwrap();
    let mut new_map = HashMap::with_capacity(csv.len());

    for (key, mut value) in csv {
        let key = heck::AsSnakeCase(key).to_string();

        // if we were able to parse it as a non-string, return that
        if !value.is_string() {
            new_map.insert(key, value);
            continue;
        }

        // if not, convert it to a string and remove the comma
        let value_as_string = value.to_string();
        let formatted = value_as_string.replace([',', '%', '"'], "");

        let matches = number_regex.captures(&formatted);

        let extracted_number = match matches {
            Some(capture_group) => {
                let (_, [number]) = capture_group.extract();
                number
            }
            None => &formatted,
        };

        if let Ok(n) = serde_json::Number::from_str(extracted_number) {
            // if the original string had a '%' in it, we want to divide by 100
            if value_as_string.contains('%') && extracted_number == formatted {
                value = AnyValue::from(n.as_f64().unwrap() / 100.0);
            } else {
                value = AnyValue::from(n.as_f64().unwrap());
            }
        }

        new_map.insert(key, value);
    }

    new_map
}

impl ParseMetricsFile for chromium::SingleRowCsvMetricsFile {
    fn parse(mut self) -> ScamplersResult<ParsedMetricsFile> {
        let mut csv = read_csv(&self.raw_contents)?;

        if csv.len() != 1 {
            return Err(ScamplersError::new_unprocessable_entity_error(
                DatasetMetricsFileParseError {
                    message: "expected csv with exactly 1 row".to_string(),
                },
            ));
        }

        self.contents = parse_tenx_record(csv.remove(0));

        Ok(ParsedMetricsFile::TenxSingleRowCsv(self))
    }
}

impl ParseMetricsFile for chromium::MultiRowCsvMetricsFileGroup {
    fn parse(mut self) -> ScamplersResult<ParsedMetricsFile> {
        for file in self.as_mut_slice() {
            let csv = read_csv(&file.raw_contents)?;

            file.contents = csv.into_iter().map(parse_tenx_record).collect();
        }

        Ok(ParsedMetricsFile::TenxMultiRowCsvGroup(self))
    }
}

impl ParseMetricsFile for chromium::JsonMetricsFile {
    fn parse(mut self) -> ScamplersResult<ParsedMetricsFile> {
        self.contents = serde_json::from_slice(self.raw_contents.as_bytes()).map_err(|err| {
            ScamplersError::new_unprocessable_entity_error(DatasetMetricsFileParseError {
                message: format!("failed to parse 10x json:\n{err}"),
            })
        })?;

        Ok(ParsedMetricsFile::TenxJson(self))
    }
}

trait NewChromiumDatasetExt {
    fn cmdline(&self) -> &str;
    fn n_metrics_files(&self) -> u64;
    fn core(&self) -> &NewChromiumDatasetCore;
    async fn validate_chemistry(&self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<()>;
    async fn validate_n_samples(&self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<()>;
    fn parse(self) -> ScamplersResult<NewParsedChromiumDataset>;
}

impl NewChromiumDatasetExt for NewChromiumDataset {
    fn cmdline(&self) -> &str {
        self.into()
    }

    fn n_metrics_files(&self) -> u64 {
        use NewChromiumDataset::{
            CellrangerCount, CellrangerMulti, CellrangerVdj, CellrangerarcCount,
            CellrangeratacCount,
        };

        match self {
            CellrangerCount(_)
            | CellrangerVdj(_)
            | CellrangerarcCount(_)
            | CellrangeratacCount(_) => 1,
            CellrangerMulti(d) => d.metrics.len() as u64,
        }
    }

    fn core(&self) -> &NewChromiumDatasetCore {
        use NewChromiumDataset::{
            CellrangerCount, CellrangerMulti, CellrangerVdj, CellrangerarcCount,
            CellrangeratacCount,
        };

        match self {
            CellrangerCount(d) | CellrangerVdj(d) | CellrangerarcCount(d) => &d.core,
            CellrangerMulti(d) => &d.core,
            CellrangeratacCount(d) => &d.core,
        }
    }

    async fn validate_chemistry(&self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<()> {
        let gems_id = &self.core().gems_id;

        let (stored_chemistry, expected_cmdline): (Option<String>, Option<String>) = gems::table
            .left_join(chemistry::table)
            .filter(gems::id.eq(gems_id))
            .select((gems::chemistry, chemistry::cmdline.nullable()))
            .first(db_conn)
            .await?;

        let expected_cmdline = expected_cmdline.unwrap_or("cellranger atac".to_string());

        let cmdline = self.cmdline();

        if expected_cmdline != self.cmdline() {
            return Err(ScamplersError::new_unprocessable_entity_error(
                DatasetCmdlineError {
                    chemistry: stored_chemistry,
                    found_cmdline: cmdline.to_string(),
                    expected_cmdline,
                },
            ));
        }

        Ok(())
    }

    async fn validate_n_samples(&self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<()> {
        let gems_id = self.core().gems_id;

        let samples: Vec<(Option<Uuid>, Option<Uuid>)> = chip_loading::table
            .filter(chip_loading::gems_id.eq(gems_id))
            .left_join(suspension::table)
            .left_join(suspension_pool::table)
            .select((suspension::id.nullable(), suspension_pool::id.nullable()))
            .load(db_conn)
            .await?;

        let err = |expected_n_metrics_files| {
            ScamplersError::new_unprocessable_entity_error(DatasetNMetricsFilesError {
                found_n_metrics_files: self.n_metrics_files(),
                expected_n_metrics_files,
            })
        };

        let Some(suspension_pool_id) = samples[0].1 else {
            if samples.len() != 1 {
                return Err(err(1));
            }

            return Ok(());
        };

        let n_suspensions: i64 = suspension::table
            .filter(suspension::pooled_into_id.eq(suspension_pool_id))
            .count()
            .get_result(db_conn)
            .await?;
        let n_suspensions = n_suspensions as u64;

        if n_suspensions != self.n_metrics_files() {
            return Err(err(n_suspensions));
        }

        Ok(())
    }

    fn parse(self) -> ScamplersResult<NewParsedChromiumDataset> {
        let (core, parsed_metrics) = match self {
            Self::CellrangerarcCount(ds) | Self::CellrangerCount(ds) | Self::CellrangerVdj(ds) => {
                (ds.core, ds.metrics.parse())
            }
            Self::CellrangeratacCount(ds) => (ds.core, ds.metrics.parse()),
            Self::CellrangerMulti(ds) => (ds.core, ds.metrics.parse()),
        };

        Ok(NewParsedChromiumDataset {
            core,
            metrics: parsed_metrics?,
        })
    }
}

impl WriteToDbInternal for NewChromiumDataset {
    type Returns = DatasetSummary;

    async fn write_to_db(self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self::Returns> {
        self.validate_chemistry(db_conn).await?;
        self.validate_n_samples(db_conn).await?;

        let parsed = self.parse()?;

        Ok(diesel::insert_into(dataset::table)
            .values(parsed)
            .returning(DatasetSummary::as_returning())
            .get_result(db_conn)
            .await?)
    }
}
