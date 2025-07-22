use scamplers_core::model::dataset::{DatasetSummary, NewDataset};

use crate::db::model::{WriteToDb, WriteToDbInternal};

mod chromium;

impl WriteToDb for NewDataset {
    type Returns = DatasetSummary;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> crate::db::error::Result<Self::Returns> {
        match self {
            Self::Chromium(ds) => ds.write_to_db(db_conn).await,
        }
    }
}
