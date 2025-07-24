use scamplers_core::model::dataset::{DatasetSummary, NewDataset};

use crate::{
    db::model::{WriteToDb, WriteToDbInternal},
    result::ScamplersResult,
};

mod chromium;

impl WriteToDb for NewDataset {
    type Returns = DatasetSummary;

    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> ScamplersResult<Self::Returns> {
        match self {
            Self::Chromium(ds) => ds.write_to_db(db_conn).await,
        }
    }
}
