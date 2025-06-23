use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::dataset_metadata::{DatasetMetadata, NewDatasetMetadata};
use scamplers_schema::{
    dataset_metadata::{self, id as id_col},
    lab,
};
use uuid::Uuid;

use crate::db::{self, model::Write};

#[diesel::dsl::auto_type]
fn query_base() -> _ {
    dataset_metadata::table.inner_join(lab::table)
}

impl Write for NewDatasetMetadata {
    type Returns = DatasetMetadata;

    async fn write(self, db_conn: &mut AsyncPgConnection) -> db::error::Result<Self::Returns> {
        let id: Uuid = diesel::insert_into(dataset_metadata::table)
            .values(self)
            .returning(id_col)
            .get_result(db_conn)
            .await?;

        Ok(query_base()
            .filter(id_col.eq(id))
            .select(DatasetMetadata::as_select())
            .first(db_conn)
            .await?)
    }
}
