use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn prefix(&self, prefix_length: usize) -> &str {
        let Self(key) = self;
        &key[..prefix_length]
    }

    pub fn is_same_hash(&self, other: &str) -> bool {
        let argon2 = Argon2::default();

        let Ok(parsed_hash) = PasswordHash::new(other) else {
            return false;
        };

        argon2
            .verify_password(self.as_str().as_bytes(), &parsed_hash)
            .is_ok()
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        let Self(inner) = self;

        inner
    }
}

impl From<&str> for ApiKey {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<ApiKey> for String {
    fn from(value: ApiKey) -> Self {
        value.0
    }
}

impl IntoResponse for ApiKey {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
