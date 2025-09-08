use crate::db::{
    DbOperation,
    models::nucleic_acid::library::{Library, NewLibrary},
};

impl NewLibrary {
    fn validate_volume() {}
}

impl DbOperation<Library> for NewLibrary {
    fn execute(
        self,
        db_conn: &mut diesel::PgConnection,
    ) -> crate::result::ScamplersResult<Library> {
        todo!()
    }
}
