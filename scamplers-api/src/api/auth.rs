use std::{fmt::Debug, str::FromStr};

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use axum::{
    Json, RequestPartsExt,
    extract::{FromRequestParts, OptionalFromRequestParts},
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{self, authorization::Bearer},
};
use diesel::{
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    prelude::*,
    serialize::{ToSql, WriteTuple},
    sql_types::{self, Bool, Record, Text},
};
use rand::{
    Rng, SeedableRng, TryRngCore,
    distr::Alphanumeric,
    rngs::{OsRng, StdRng},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db, state::AppState};

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

    fn prefix(&self) -> &str {
        let Self(key) = self;
        &key[..KEY_PREFIX_LENGTH]
    }

    #[must_use]
    pub fn hash(&self) -> HashedApiKey {
        let Self(key) = self;

        let mut salt = [0u8; 16];
        OsRng.try_fill_bytes(&mut salt).unwrap();

        let salt = SaltString::encode_b64(&salt).unwrap();

        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(key.as_bytes(), &salt)
            .unwrap()
            .to_string();

        HashedApiKey {
            prefix: self.prefix().to_string(),
            hash,
        }
    }

    fn is_same_hash(&self, other: &HashedApiKey) -> bool {
        let argon2 = Argon2::default();

        let Ok(parsed_hash) = PasswordHash::new(&other.hash) else {
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
impl FromStr for ApiKey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
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

#[derive(AsExpression, Debug, FromSqlRow, Deserialize)]
#[diesel(sql_type = scamplers_schema::sql_types::HashedKey)]
pub struct HashedApiKey {
    prefix: String,
    hash: String,
}

impl ToSql<scamplers_schema::sql_types::HashedKey, Pg> for HashedApiKey {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let Self { prefix, hash } = self;

        WriteTuple::<(sql_types::Text, sql_types::Text)>::write_tuple(
            &(prefix, hash),
            &mut out.reborrow(),
        )
    }
}

impl FromSql<scamplers_schema::sql_types::HashedKey, Pg> for HashedApiKey {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let (prefix, hash) =
            FromSql::<Record<(sql_types::Text, sql_types::Text)>, Pg>::from_sql(bytes)?;

        Ok(Self { prefix, hash })
    }
}

#[derive(Clone, Copy)]
pub(super) struct User(pub(super) Uuid);

#[derive(Debug, thiserror::Error, serde::Serialize)]
#[error(transparent)]
pub enum Error {
    #[error("{message}")]
    Unauthorized {
        message: String,
    },
    Database(#[from] db::Error),
}

impl Error {
    fn invalid_api_key() -> Self {
        Self::Unauthorized {
            message: "invalid API key".to_string(),
        }
    }

    fn invalid_frontend_token() -> Self {
        Self::Unauthorized {
            message: "invalid frontend token".to_string(),
        }
    }
}

impl From<deadpool_diesel::InteractError> for Error {
    fn from(err: deadpool_diesel::InteractError) -> Self {
        Self::Database(err.into())
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::Database(err.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match &self {
            Self::Unauthorized { .. } => {
                (axum::http::StatusCode::UNAUTHORIZED, Json(self)).into_response()
            }
            Self::Database(..) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"status": 500, "error": {"message": "something went wrong"}}),
                ),
                ).into_response(),
        }
    }
}

impl User {
    fn fetch_by_api_key(api_key: &ApiKey, conn: &mut PgConnection) -> Result<Self, Error> {
        use scamplers_schema::people::dsl::{hashed_api_key, id, people};

        let filter_query = diesel::dsl::sql::<Bool>("(hashed_api_key).prefix = ")
            .bind::<Text, _>(api_key.prefix());

        let (user_id, found_api_key) = people
            .filter(filter_query)
            .select((id, hashed_api_key.assume_not_null()))
            .first(conn)
            .optional()?
            .ok_or(Error::invalid_api_key())?;

        if !api_key.is_same_hash(&found_api_key) {
            return Err(Error::invalid_api_key());
        }

        Ok(Self(user_id))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        app_state: &AppState,
    ) -> Result<Self, Error> {
        if let AppState::Dev { user_id, .. } = app_state {
            return Ok(Self(*user_id));
        }

        let Some(Ok(api_key)) = parts
            .headers
            .get("X-API-Key")
            .map(|s| s.to_str().unwrap().parse())
        else {
            return Err(Error::invalid_api_key());
        };

        let db_conn = app_state.db_conn().await?;

        Ok(db_conn
            .interact(move |db_conn| User::fetch_by_api_key(&api_key, db_conn))
            .await??)
    }
}

impl OptionalFromRequestParts<AppState> for User {
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(
            <User as FromRequestParts<_>>::from_request_parts(parts, state)
                .await
                .ok(),
        )
    }
}

pub struct Frontend;
impl FromRequestParts<AppState> for Frontend {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let frontend_token = match state {
            AppState::Dev { .. } => {
                return Ok(Self);
            }
            AppState::Prod { frontend_token, .. } => frontend_token,
        };

        let Ok(frontend_auth) = parts
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
        else {
            return Err(Error::invalid_frontend_token())?;
        };

        if frontend_auth.token() != frontend_token {
            return Err(Error::invalid_frontend_token())?;
        }

        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::ApiKey;

    #[rstest]
    fn api_key_prefix_matches_hash_prefix() {
        let api_key = ApiKey::new();
        let hashed = api_key.hash();

        assert_eq!(api_key.prefix(), hashed.prefix);
    }
}
