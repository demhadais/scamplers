use diesel::prelude::*;
use scamplers_schema::{
    cdna, chip_loading, chromium_dataset, gems, library, suspension, tenx_assay,
};
use uuid::Uuid;

use crate::{
    db::{
        DbOperation,
        models::{
            dataset::chromium::{
                ChromiumDataset, ChromiumDatasetId, NewCellrangerCountDataset,
                NewCellrangerMultiDataset, NewCellrangerVdjDataset, NewCellrangerarcCountDataset,
                NewCellrangeratacCountDataset, NewChromiumDataset, NewChromiumDatasetCommon,
                ParsedMetrics,
            },
            nucleic_acid::common::gems_to_assay,
        },
    },
    result::{
        ChromiumDatasetError, DatasetMetricsFileParseError, DatasetNMetricsFilesError,
        ScamplersResult,
    },
};

impl NewChromiumDataset {
    fn cmdline(&self) -> &str {
        self.into()
    }

    fn inner(&self) -> &NewChromiumDatasetCommon {
        match self {
            Self::CellrangeratacCount(NewCellrangeratacCountDataset { inner, .. })
            | NewChromiumDataset::CellrangerarcCount(NewCellrangerarcCountDataset {
                inner, ..
            })
            | NewChromiumDataset::CellrangerCount(NewCellrangerCountDataset { inner, .. })
            | NewChromiumDataset::CellrangerVdj(NewCellrangerVdjDataset { inner, .. })
            | NewChromiumDataset::CellrangerMulti(NewCellrangerMultiDataset { inner, .. }) => inner,
        }
    }

    fn gems_id_and_expected_cmdlines(
        &self,
        db_conn: &mut PgConnection,
    ) -> ScamplersResult<(Uuid, Vec<Option<String>>)> {
        let mut gems_ids_and_cmdlines: Vec<(Uuid, Vec<Option<String>>)> = library::table
            .inner_join(cdna::table.inner_join(gems_to_assay()))
            .filter(library::id.eq_any(&self.inner().library_ids))
            .select((gems::id, tenx_assay::cmdlines.assume_not_null()))
            .distinct()
            .load(db_conn)?;

        if gems_ids_and_cmdlines.len() != 1 {
            return Err(ChromiumDatasetError::builder()
                .message("all libraries in Chromium dataset must come from one GEMs")
                .build()
                .into());
        }

        Ok(gems_ids_and_cmdlines.remove(0))
    }

    fn validate_cmdline(&self, expected_cmdlines: &[Option<String>]) -> ScamplersResult<()> {
        let expected_cmdlines: Vec<_> = expected_cmdlines
            .iter()
            .filter_map(|o| o.as_ref().map(std::string::String::as_str))
            .collect();

        if !expected_cmdlines.contains(&self.cmdline()) {
            return Err(ChromiumDatasetError::builder()
                .message(format!(
                    "expected one of the following cmdlines: {expected_cmdlines:?}"
                ))
                .build()
                .into());
        }

        Ok(())
    }

    fn validate_n_metrics_files(
        &self,
        gems_id: Uuid,
        db_conn: &mut PgConnection,
    ) -> ScamplersResult<()> {
        let Self::CellrangerMulti(NewCellrangerMultiDataset { metrics, .. }) = self else {
            return Ok(());
        };

        let samples: Vec<(Option<Uuid>, Option<Uuid>)> = chip_loading::table
            .filter(chip_loading::gems_id.eq(gems_id))
            .select((
                chip_loading::suspension_id,
                chip_loading::suspension_pool_id,
            ))
            .load(db_conn)?;

        // We know that we won't have both suspensions and suspension pools loaded onto
        // the chip, so if any supsension ID is present, then we immediately know that
        // the length of the returned data is the same as the number of samples (this
        // would be the case for OCM)
        if samples.iter().any(|(susp, _)| susp.is_some()) && samples.len() != metrics.len() {
            return Err(DatasetNMetricsFilesError::builder()
                .expected_n_metrics_files(samples.len() as u64)
                .found_n_metrics_files(metrics.len() as u64)
                .build()
                .into());
        }

        let suspension_pool_ids = samples.into_iter().filter_map(|(_, p)| p);

        let n_suspensions: i64 = suspension::table
            .filter(suspension::pooled_into.eq_any(suspension_pool_ids))
            .count()
            .get_result(db_conn)?;

        if n_suspensions != metrics.len() as i64 {
            return Err(DatasetNMetricsFilesError::builder()
                .expected_n_metrics_files(n_suspensions as u64)
                .found_n_metrics_files(metrics.len() as u64)
                .build()
                .into());
        }

        Ok(())
    }
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg), table_name = chromium_dataset)]
struct GenericNewChromiumDataset {
    #[diesel(embed)]
    inner: NewChromiumDatasetCommon,
    metrics: ParsedMetrics,
}
impl TryFrom<NewChromiumDataset> for GenericNewChromiumDataset {
    type Error = DatasetMetricsFileParseError;

    fn try_from(ds: NewChromiumDataset) -> Result<Self, DatasetMetricsFileParseError> {
        let dataset = match ds {
            NewChromiumDataset::CellrangeratacCount(NewCellrangeratacCountDataset {
                inner,
                metrics,
            }) => GenericNewChromiumDataset {
                inner,
                metrics: metrics.try_into()?,
            },
            NewChromiumDataset::CellrangerarcCount(NewCellrangerarcCountDataset {
                inner,
                metrics,
            })
            | NewChromiumDataset::CellrangerCount(NewCellrangerCountDataset { inner, metrics })
            | NewChromiumDataset::CellrangerVdj(NewCellrangerVdjDataset { inner, metrics }) => {
                GenericNewChromiumDataset {
                    inner,
                    metrics: metrics.try_into()?,
                }
            }
            NewChromiumDataset::CellrangerMulti(NewCellrangerMultiDataset { inner, metrics }) => {
                GenericNewChromiumDataset {
                    inner,
                    metrics: metrics.try_into()?,
                }
            }
        };

        Ok(dataset)
    }
}

impl DbOperation<ChromiumDataset> for NewChromiumDataset {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<ChromiumDataset> {
        let (gems_id, expected_cmdlines) = self.gems_id_and_expected_cmdlines(db_conn)?;
        self.validate_cmdline(&expected_cmdlines)?;
        self.validate_n_metrics_files(gems_id, db_conn)?;

        let generic = GenericNewChromiumDataset::try_from(self)?;

        let id = diesel::insert_into(chromium_dataset::table)
            .values(generic)
            .returning(chromium_dataset::id)
            .get_result(db_conn)?;

        ChromiumDatasetId(id).execute(db_conn)
    }
}
