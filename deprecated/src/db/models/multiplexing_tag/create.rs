use diesel::RunQueryDsl;
use scamplers_schema::multiplexing_tag;

use crate::db::{DbOperation, models::multiplexing_tag::NewMultiplexingTag};

impl DbOperation<()> for Vec<NewMultiplexingTag> {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        diesel::insert_into(multiplexing_tag::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        Ok(())
    }
}
