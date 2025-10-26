use diesel::PgConnection;

mod initial_data;
mod institutions;
mod people;

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error(transparent)]
pub enum Error {
    CreatePerson(#[from] people::Error),
}

pub trait Validate {
    fn validate(&self, _db_conn: &mut PgConnection) -> Result<(), Error> {
        Ok(())
    }
}
