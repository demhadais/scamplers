use std::sync::LazyLock;

use diesel::PgConnection;
use regex::Regex;
use scamplers_models::{institution, person};

use crate::initial_data::InitialData;

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error(transparent)]
pub enum Error {
    CreatePerson(#[from] CreatePersonError),
}

pub trait Validate {
    fn validate(&self, _db_conn: &mut PgConnection) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for institution::Creation {}

// https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"/^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/").unwrap()
});

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error("{email} invalid: {message}")]
pub enum CreatePersonError {
    ValidateEmail { email: String, message: String },
}

impl Validate for person::Creation {
    fn validate(&self, _db_conn: &mut diesel::PgConnection) -> Result<(), Error> {
        if !EMAIL_REGEX.is_match(self.email.as_ref()) {
            return Err(CreatePersonError::ValidateEmail {
                email: self.email.to_string(),
                message: "invalid email".to_string(),
            })?;
        }

        Ok(())
    }
}

impl Validate for InitialData {
    fn validate(&self, db_conn: &mut diesel::PgConnection) -> Result<(), Error> {
        let Self {
            institution,
            app_admin,
        } = &self;

        institution.validate(db_conn)?;
        app_admin.validate(db_conn)?;
        // self.index_set_urls.validate(db_conn)?;

        Ok(())
    }
}
