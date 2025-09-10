use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::str::FromStr;

use any_value::AnyValue;
#[cfg(not(target_arch = "wasm32"))]
use heck::ToSnekCase;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::{derive::gen_stub_pyclass_complex_enum, impl_stub_type};
#[cfg(not(target_arch = "wasm32"))]
use regex::Regex;
use scamplers_macros::{
    Jsonify, PyJsonify, base_model, db_insertion, db_json, db_query, db_selection,
};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

#[cfg(not(target_arch = "wasm32"))]
use crate::result::DatasetMetricsFileParseError;
use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        lab::LabSummary,
        specimen::SpecimenQuery,
        tenx_assay::{TenxAssay, TenxAssayQuery},
    },
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[cfg(feature = "python")]
macro_rules! impl_metrics_methods {
    ($metrics_struct:ident) => {
        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pymethods]
        impl $metrics_struct {
            #[new]
            #[pyo3(signature = (*, filename, raw_contents))]
            fn new(filename: String, raw_contents: ValidString) -> pyo3::PyResult<Self> {
                let mut metrics = Self {
                    filename,
                    raw_contents,
                    contents: Default::default(),
                };

                metrics
                    .parse()
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))?;

                Ok(metrics)
            }

            #[setter]
            fn set_filename(&mut self, filename: String) {
                self.filename = filename;
            }

            #[setter]
            fn set_raw_contents(&mut self, raw_contents: ValidString) -> pyo3::PyResult<()> {
                self.raw_contents = raw_contents;
                self.parse()
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{e}")))
            }
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
fn read_csv<S: AsRef<str>>(
    raw: &S,
) -> Result<Vec<HashMap<String, AnyValue>>, DatasetMetricsFileParseError> {
    let rdr = csv::Reader::from_reader(raw.as_ref().as_bytes());
    let records = rdr.into_deserialize();
    let records: csv::Result<Vec<HashMap<String, AnyValue>>> = records.collect();

    records.map_err(|err| DatasetMetricsFileParseError {
        message: format!("failed to parse 10x csv: {err}"),
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_tenx_record(csv: HashMap<String, AnyValue>) -> HashMap<String, AnyValue> {
    let number_regex = Regex::new(r"^(\d+)\s\(\d{1,3}\.\d+\)$").unwrap();
    let mut new_map = HashMap::with_capacity(csv.len());

    for (key, mut value) in csv {
        let key = key.to_snek_case();

        // if we were able to parse it as a non-string, return that
        if !value.is_string() {
            new_map.insert(key, value);
            continue;
        }

        // if not, convert it to a string and remove commas, percent symbol, and
        // quotation marks
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

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub struct SingleRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: HashMap<String, AnyValue>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SingleRowCsvMetricsFile {
    fn parse(&mut self) -> Result<(), DatasetMetricsFileParseError> {
        let mut csv = read_csv(&self.raw_contents)?;

        if csv.len() != 1 {
            return Err(DatasetMetricsFileParseError {
                message: "expected csv with exactly 1 row".to_string(),
            });
        }

        self.contents = parse_tenx_record(csv.remove(0));

        Ok(())
    }
}

#[cfg(feature = "python")]
impl_metrics_methods!(SingleRowCsvMetricsFile);

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub struct MultiRowCsvMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]+/[[:alnum:]]+summary\.csv$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: Vec<HashMap<String, AnyValue>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl MultiRowCsvMetricsFile {
    fn parse(&mut self) -> Result<(), DatasetMetricsFileParseError> {
        let csv = read_csv(&self.raw_contents)?;

        self.contents = csv.into_iter().map(parse_tenx_record).collect();

        Ok(())
    }
}

#[cfg(feature = "python")]
impl_metrics_methods!(MultiRowCsvMetricsFile);

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[serde(transparent)]
#[garde(transparent)]
#[cfg_attr(
    feature = "python",
    pyo3(sequence, name = "_MultiRowCsvMetricsFileGroup")
)]
pub struct MultiRowCsvMetricsFileGroup {
    #[garde(dive, length(min = 1))]
    inner: Vec<MultiRowCsvMetricsFile>,
}

impl MultiRowCsvMetricsFileGroup {
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn as_mut_slice(&mut self) -> &mut [MultiRowCsvMetricsFile] {
        self.inner.as_mut_slice()
    }
}

impl From<Vec<MultiRowCsvMetricsFile>> for MultiRowCsvMetricsFileGroup {
    fn from(inner: Vec<MultiRowCsvMetricsFile>) -> Self {
        Self { inner }
    }
}

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub struct JsonMetricsFile {
    #[garde(pattern(r#"^[0-9A-Za-z_-]*summary\.json$"#))]
    pub filename: String,
    #[garde(dive)]
    pub raw_contents: ValidString,
    #[serde(skip)]
    pub contents: AnyValue,
}

#[cfg(not(target_arch = "wasm32"))]
impl JsonMetricsFile {
    fn parse(&mut self) -> Result<(), DatasetMetricsFileParseError> {
        self.contents = serde_json::from_slice(self.raw_contents.as_bytes()).map_err(|err| {
            DatasetMetricsFileParseError {
                message: format!("failed to parse 10x json: {err}"),
            }
        })?;

        Ok(())
    }
}

#[cfg_attr(feature = "python", gen_stub_pyclass_complex_enum)]
#[db_json]
#[serde(tag = "format")]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.responses"))]
pub enum ParsedMetrics {
    #[serde(rename = "10x_single_row_csv")]
    TenxSingleRowCsv(SingleRowCsvMetricsFile),
    #[serde(rename = "10x_multi_row_csv_group")]
    TenxMultiRowCsvGroup(MultiRowCsvMetricsFileGroup),
    #[serde(rename = "10x_json")]
    TenxJson(JsonMetricsFile),
}

#[cfg(not(target_arch = "wasm32"))]
impl TryFrom<SingleRowCsvMetricsFile> for ParsedMetrics {
    type Error = DatasetMetricsFileParseError;

    fn try_from(mut metrics_file: SingleRowCsvMetricsFile) -> Result<Self, Self::Error> {
        metrics_file.parse()?;
        Ok(Self::TenxSingleRowCsv(metrics_file))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl TryFrom<MultiRowCsvMetricsFileGroup> for ParsedMetrics {
    type Error = DatasetMetricsFileParseError;

    fn try_from(mut metrics_files: MultiRowCsvMetricsFileGroup) -> Result<Self, Self::Error> {
        for file in &mut metrics_files.inner {
            file.parse()?;
        }

        Ok(Self::TenxMultiRowCsvGroup(metrics_files))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl TryFrom<JsonMetricsFile> for ParsedMetrics {
    type Error = DatasetMetricsFileParseError;

    fn try_from(mut metrics_file: JsonMetricsFile) -> Result<Self, Self::Error> {
        metrics_file.parse()?;

        Ok(Self::TenxJson(metrics_file))
    }
}

#[cfg(feature = "python")]
impl_metrics_methods!(JsonMetricsFile);

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_dataset))]
#[cfg_attr(feature = "python", pyo3(name = "_NewChromiumDatasetCommon"))]
pub struct NewChromiumDatasetCommon {
    #[garde(dive)]
    pub name: ValidString,
    pub lab_id: Uuid,
    #[garde(dive)]
    pub data_path: ValidString,
    pub delivered_at: OffsetDateTime,
    #[garde(length(min = 1))]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub library_ids: Vec<Uuid>,
    #[garde(custom(validate_html))]
    pub web_summary: String,
}

#[cfg(feature = "python")]
macro_rules! impl_dataset_constructor {
    ($dataset_struct:path, $metrics_struct:path) => {
        #[pyo3_stub_gen::derive::gen_stub_pymethods]
        #[pymethods]
        impl $dataset_struct {
            #[new]
            #[pyo3(signature = (*, name, lab_id, data_path, delivered_at, library_ids, web_summary, metrics))]
            fn new(
                name: ValidString,
                lab_id: Uuid,
                data_path: ValidString,
                delivered_at: OffsetDateTime,
                library_ids: Vec<Uuid>,
                web_summary: String,
                metrics: $metrics_struct,
            ) -> Self {
                Self {
                    inner: NewChromiumDatasetCommon {
                        name,
                        lab_id,
                        data_path,
                        delivered_at,
                        library_ids,
                        web_summary,
                    },
                    metrics: metrics.into(),
                }
            }
        }
    };
}

#[base_model]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, eq, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub struct NewCellrangerarcCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub inner: NewChromiumDatasetCommon,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(NewCellrangerarcCountDataset, SingleRowCsvMetricsFile);

#[base_model]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, eq, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub struct NewCellrangeratacCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub inner: NewChromiumDatasetCommon,
    #[garde(dive)]
    pub metrics: JsonMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(NewCellrangeratacCountDataset, JsonMetricsFile);

#[base_model]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, eq, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub struct NewCellrangerCountDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub inner: NewChromiumDatasetCommon,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(NewCellrangerCountDataset, SingleRowCsvMetricsFile);

#[base_model]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, eq, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub struct NewCellrangerMultiDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub inner: NewChromiumDatasetCommon,
    #[garde(dive)]
    pub metrics: MultiRowCsvMetricsFileGroup,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(NewCellrangerMultiDataset, Vec<MultiRowCsvMetricsFile>);

#[base_model]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, eq, module = "scamplepy.create")
)]
#[derive(Jsonify, PyJsonify)]
pub struct NewCellrangerVdjDataset {
    #[serde(flatten)]
    #[garde(dive)]
    pub inner: NewChromiumDatasetCommon,
    pub metrics: SingleRowCsvMetricsFile,
}
#[cfg(feature = "python")]
impl_dataset_constructor!(NewCellrangerVdjDataset, SingleRowCsvMetricsFile);

#[base_model]
#[serde(tag = "cmdline")]
#[derive(strum::IntoStaticStr)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
pub enum NewChromiumDataset {
    #[serde(rename = "cellranger-arc count")]
    #[strum(serialize = "cellranger-arc count")]
    CellrangerarcCount(NewCellrangerarcCountDataset),

    #[serde(rename = "cellranger-atac count")]
    #[strum(serialize = "cellranger-atac count")]
    CellrangeratacCount(NewCellrangeratacCountDataset),

    #[serde(rename = "cellranger count")]
    #[strum(serialize = "cellranger count")]
    CellrangerCount(NewCellrangerCountDataset),

    #[serde(rename = "cellranger multi")]
    #[strum(serialize = "cellranger multi")]
    CellrangerMulti(NewCellrangerMultiDataset),

    #[serde(rename = "cellranger vdj")]
    #[strum(serialize = "cellranger vdj")]
    CellrangerVdj(NewCellrangerVdjDataset),
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn validate_html(document: &str, _: &()) -> garde::Result {
    let result = scraper::Html::parse_document(document);
    if !result.errors.is_empty() {
        return Err(garde::Error::new("invalid HTML"));
    }

    Ok(())
}

#[cfg(feature = "python")]
impl_stub_type!(
    NewChromiumDataset = NewCellrangerarcCountDataset
        | NewCellrangeratacCountDataset
        | NewCellrangerCountDataset
        | NewCellrangerMultiDataset
        | NewCellrangerVdjDataset
);

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::chromium_dataset))]
pub struct ChromiumDataset {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub name: String,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub lab: LabSummary,
    pub data_path: String,
    pub delivered_at: OffsetDateTime,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub tenx_assay: TenxAssay,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub metrics: ParsedMetrics,
    pub web_summary: String,
}

define_ordering_enum! { ChromiumDatasetOrderBy { DeliveredAt, Name }, default = DeliveredAt }

#[db_query]
pub struct ChromiumDatasetQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub lab_ids: Vec<Uuid>,
    pub delivered_before: Option<OffsetDateTime>,
    pub delivered_after: Option<OffsetDateTime>,
    pub tenx_assay: Option<TenxAssayQuery>,
    pub specimen: Option<SpecimenQuery>,
    #[builder(default)]
    pub order_by: DefaultVec<ChromiumDatasetOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

uuid_newtype!(ChromiumDatasetId);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use any_value::AnyValue;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::db::models::dataset::chromium::{MultiRowCsvMetricsFile, SingleRowCsvMetricsFile};

    #[rstest]
    fn parse_single_row_csv() {
        let single_row_csv = include_str!("chromium/test-data/single-row.csv");

        let mut metrics = SingleRowCsvMetricsFile {
            filename: String::default(),
            raw_contents: single_row_csv.into(),
            contents: HashMap::default(),
        };

        metrics.parse().unwrap();

        assert_eq!(
            metrics.contents["estimated_number_of_cells"],
            AnyValue::from(65_558.0)
        );
    }

    #[rstest]
    fn parse_cellranger_multi_csv() {
        let single_row_csv = include_str!("chromium/test-data/cellranger_multi.csv");

        let mut metrics = MultiRowCsvMetricsFile {
            filename: String::default(),
            raw_contents: single_row_csv.into(),
            contents: Vec::default(),
        };

        metrics.parse().unwrap();

        let row = &metrics.contents[0];
        assert_eq!(row["category"], AnyValue::from("Cells"));
        assert_eq!(row["library_type"], AnyValue::from("Gene Expression"));
        assert_eq!(row["metric_value"], AnyValue::from(1866.0));

        let row = &metrics.contents[12];
        assert_eq!(row["category"], AnyValue::from("Cells"));
        assert_eq!(
            row["metric_name"],
            AnyValue::from("Cells detected in other samples")
        );
        assert_eq!(row["metric_value"], AnyValue::from(13_640.0));

        let row = &metrics.contents[38];
        assert_eq!(row["metric_value"], AnyValue::from("sample1"));
    }
}
