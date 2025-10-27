use std::sync::LazyLock;

use regex::Regex;
use scamplers_models::person;

use crate::validate::Validate;

// https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()
});

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "cause")]
#[error("{email} invalid: {message}")]
pub enum Error {
    Email { email: String, message: String },
}

impl Validate for person::Creation {
    fn validate(&self, _db_conn: &mut diesel::PgConnection) -> Result<(), super::Error> {
        if !EMAIL_REGEX.is_match(self.email.as_ref()) {
            return Err(Error::Email {
                email: self.email.to_string(),
                message: "invalid email".to_string(),
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::validate::people::EMAIL_REGEX;

    #[rstest]
    fn valid_email() {
        assert!(EMAIL_REGEX.is_match("peter.parker@spiderman.avengers"))
    }

    #[rstest]
    fn email_has_no_domain() {
        assert!(!EMAIL_REGEX.is_match("SpongeBob"))
    }

    #[rstest]
    fn email_contains_space() {
        assert!(!EMAIL_REGEX.is_match("Harry Potter"))
    }
}
