use diesel::RunQueryDsl;
use scamplers_schema::library_type_specification;

use crate::db::{DbOperation, models::library_type_specification::LibraryTypeSpecification};

impl DbOperation<()> for Vec<LibraryTypeSpecification> {
    fn execute(self, db_conn: &mut diesel::PgConnection) -> crate::result::ScamplersResult<()> {
        diesel::insert_into(library_type_specification::table)
            .values(self)
            .on_conflict_do_nothing()
            .execute(db_conn)?;

        Ok(())
    }
}
