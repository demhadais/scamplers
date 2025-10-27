use diesel::PgConnection;

mod initial_data;
mod institutions;
mod people;

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "cause")]
#[error(transparent)]
pub enum Error {
    PersonCreation(#[from] people::Error),
}

pub trait Validate {
    fn validate(&self, _db_conn: &mut PgConnection) -> Result<(), Error> {
        Ok(())
    }
}
