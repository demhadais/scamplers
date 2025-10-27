use std::fmt::Debug;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::response::IntoResponse;
use rand::{
    Rng, SeedableRng, TryRngCore,
    distr::Alphanumeric,
    rngs::{OsRng, StdRng},
};
use serde::{Deserialize, Serialize};

const KEY_PREFIX_LENGTH: usize = 8;
const KEY_LENGTH: usize = 32;

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ApiKey(String);

impl ApiKey {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(&self) -> &str {
        let Self(key) = self;
        &key[..KEY_PREFIX_LENGTH]
    }

    #[must_use]
    pub fn hash(&self) -> String {
        let Self(key) = self;

        let mut salt = [0u8; 16];
        OsRng.try_fill_bytes(&mut salt).unwrap();

        let salt = SaltString::encode_b64(&salt).unwrap();

        let argon2 = Argon2::default();

        argon2
            .hash_password(key.as_bytes(), &salt)
            .unwrap()
            .to_string()
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

impl Default for ApiKey {
    fn default() -> Self {
        let mut rng = StdRng::from_os_rng();
        let key = (0..KEY_LENGTH)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();

        Self(key)
    }
}
impl Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.hash().fmt(f)
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
