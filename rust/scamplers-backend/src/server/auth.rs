use std::{fmt::Debug, str::FromStr};

use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};
use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, OptionalFromRequestParts},
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
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use rand::{
    Rng, SeedableRng, TryRngCore,
    distr::Alphanumeric,
    rngs::{OsRng, StdRng},
};
use reqwest::StatusCode;
use scamplers_core::result::{
    PermissionDeniedError, ScamplersCoreError, ScamplersCoreErrorResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use valuable::Valuable;

use super::AppState;
use crate::result::{ScamplersError, ScamplersResult};

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

    /// # Panics
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

#[derive(AsExpression, Debug, FromSqlRow, Deserialize, Valuable)]
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

impl ScamplersError {
    fn unauthorized(message: &str) -> Self {
        ScamplersCoreErrorResponse::builder()
            .status(StatusCode::UNAUTHORIZED)
            .error(PermissionDeniedError::builder().message(message).build())
            .build()
            .into()
    }
}

#[derive(Clone, Copy, Valuable)]
pub(super) struct User(pub(super) Uuid);
impl User {
    async fn fetch_by_api_key(
        api_key: &ApiKey,
        conn: &mut AsyncPgConnection,
    ) -> ScamplersResult<Self> {
        use scamplers_schema::person::dsl::{hashed_api_key, id, person};

        let filter_query = diesel::dsl::sql::<Bool>("(hashed_api_key).prefix = ")
            .bind::<Text, _>(api_key.prefix());

        let (user_id, found_api_key) = person
            .filter(filter_query)
            .select((id, hashed_api_key.assume_not_null()))
            .first(conn)
            .await?;

        if !api_key.is_same_hash(&found_api_key) {
            return Err(ScamplersError::unauthorized("invalid API key"));
        }

        Ok(Self(user_id))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = ScamplersError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        app_state: &AppState,
    ) -> ScamplersResult<Self> {
        if let AppState::Dev { user_id, .. } = app_state {
            return Ok(Self(*user_id));
        }

        let Some(Ok(api_key)) = parts
            .headers
            .get("X-API-Key")
            .map(|s| s.to_str().unwrap().parse())
        else {
            return Err(ScamplersError::unauthorized("invalid API key"));
        };

        let mut db_conn = app_state.db_conn().await?;

        let user_result = User::fetch_by_api_key(&api_key, &mut db_conn).await;

        let Err(error) = user_result else {
            return user_result;
        };

        let error = match error.inner() {
            ScamplersCoreError::ResourceNotFound(_) => {
                ScamplersError::unauthorized("invalid API key")
            }
            _ => error,
        };

        Err(error)
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
    type Rejection = ScamplersError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AppState::Prod { config, .. } = state else {
            return Ok(Self);
        };

        let err = ScamplersError::unauthorized("invalid frontend token");

        let Ok(frontend_auth) = parts
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
        else {
            return Err(err);
        };

        if frontend_auth.token() != config.frontend_token() {
            return Err(err);
        }

        Ok(Self)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::ApiKey;
    #[test]
    fn api_key_prefix_matches_hash_prefix() {
        let api_key = ApiKey::new();
        let hashed = api_key.hash();

        assert_eq!(api_key.prefix(), hashed.prefix);
    }
}
