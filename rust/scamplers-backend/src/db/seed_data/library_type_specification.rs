use crate::db::{self, model::WriteToDb};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use scamplers_core::model::library_type_specification::NewLibraryTypeSpecification;
use scamplers_schema::library_type_specification;

impl WriteToDb for Vec<NewLibraryTypeSpecification> {
    type Returns = ();
    async fn write_to_db(
        self,
        db_conn: &mut diesel_async::AsyncPgConnection,
    ) -> db::error::Result<Self::Returns> {
        diesel::insert_into(library_type_specification::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
