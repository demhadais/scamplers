use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::dataset::{NewChromiumDataset, NewChromiumDatasetCore};
use scamplers_schema::{chemistry, chip_loading, gems, suspension, suspension_pool};
use uuid::Uuid;

use crate::db::error::{Error, Result};

trait NewChromiumDatasetExt {
    fn cmdline(&self) -> &str;
    fn n_metrics_files(&self) -> usize;
    fn core(&self) -> &NewChromiumDatasetCore;
    async fn validate_chemistry(&self, db_conn: &mut AsyncPgConnection) -> Result<()>;
    async fn validate_n_samples(&self, db_conn: &mut AsyncPgConnection) -> Result<()>;
}

impl NewChromiumDatasetExt for NewChromiumDataset {
    fn cmdline(&self) -> &str {
        self.into()
    }

    fn n_metrics_files(&self) -> usize {
        use NewChromiumDataset::{
            CellrangerCount, CellrangerMulti, CellrangerVdj, CellrangerarcCount,
            CellrangeratacCount,
        };

        match self {
            CellrangerCount(_)
            | CellrangerVdj(_)
            | CellrangerarcCount(_)
            | CellrangeratacCount(_) => 1,
            CellrangerMulti(d) => d.metrics.len(),
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

    async fn validate_chemistry(&self, db_conn: &mut AsyncPgConnection) -> Result<()> {
        let gems_id = &self.core().gems_id;

        let expected_cmdline: Option<String> = gems::table
            .left_join(chemistry::table)
            .filter(gems::id.eq(gems_id))
            .select(chemistry::cmdline.nullable())
            .first(db_conn)
            .await?;

        let expected_cmdline = expected_cmdline.unwrap_or("cellranger atac".to_string());

        let cmdline = self.cmdline();

        if expected_cmdline != self.cmdline() {
            return Err(Error::Other {
                message: format!(
                    "expected 'cmdline' {expected_cmdline} for {gems_id} based on chemistry, \
                     found {cmdline}"
                ),
            });
        }

        Ok(())
    }

    async fn validate_n_samples(&self, db_conn: &mut AsyncPgConnection) -> Result<()> {
        let gems_id = self.core().gems_id;

        let samples: Vec<(Option<Uuid>, Option<Uuid>)> = chip_loading::table
            .filter(chip_loading::gems_id.eq(gems_id))
            .left_join(suspension::table)
            .left_join(suspension_pool::table)
            .select((suspension::id.nullable(), suspension_pool::id.nullable()))
            .load(db_conn)
            .await?;

        let err = |expected_n_metrics_files| {
            Err(Error::Other {
                message: format!(
                    "expected {expected_n_metrics_files} metrics files for dataset derived from \
                     gems {gems_id}"
                ),
            })
        };

        let Some(suspension_pool_id) = samples[0].1 else {
            if samples.len() != 1 {
                return err(1);
            }

            return Ok(());
        };

        let n_suspensions: i64 = suspension::table
            .filter(suspension::pooled_into_id.eq(suspension_pool_id))
            .count()
            .get_result(db_conn)
            .await?;

        if n_suspensions as usize != self.n_metrics_files() {
            return err(n_suspensions);
        }

        Ok(())
    }
}
