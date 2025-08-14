use diesel::RunQueryDsl;
use scamplers_schema::chemistry;

use crate::db::{DbOperation, models::chemistry::Chemistry};

impl DbOperation<()> for Vec<Chemistry> {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        diesel::insert_into(chemistry::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        Ok(())
    }
}
