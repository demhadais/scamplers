use diesel_async::{AsyncPgConnection, RunQueryDsl};
use scamplers_core::model::chemistry::Chemistry;
use scamplers_schema::chemistry;

use crate::{db::model::WriteToDb, result::ScamplersResult};

impl WriteToDb for Vec<Chemistry> {
    type Returns = ();

    async fn write_to_db(self, db_conn: &mut AsyncPgConnection) -> ScamplersResult<Self::Returns> {
        diesel::insert_into(chemistry::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(db_conn)
            .await?;

        Ok(())
    }
}
