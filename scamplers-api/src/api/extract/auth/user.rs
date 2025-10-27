use axum::{Json, extract::FromRequestParts, http::HeaderValue, response::IntoResponse};
use diesel::{PgConnection, prelude::*};
use scamplers_schema::api_keys;
use uuid::Uuid;

use crate::{
    api::{
        self,
        extract::auth::{self, api_key::ApiKey},
    },
    db,
    state::AppState,
};

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct AuthenticatedUser(pub Uuid);

impl AuthenticatedUser {
    fn fetch_by_api_key(api_key: &ApiKey, conn: &mut PgConnection) -> Result<Self, auth::Error> {
        tracing::debug!(
            api_key_prefix = api_key.prefix(),
            api_key_hash = api_key.hash()
        );

        let (user_id, hash): (Uuid, String) = api_keys::table
            .filter(api_keys::prefix.eq(api_key.prefix()))
            .select((api_keys::user_id, api_keys::hash))
            .first(conn)
            .optional()?
            .ok_or_else(|| auth::Error::invalid_api_key())?;

        tracing::debug!(user_id = user_id.to_string());

        if !api_key.is_same_hash(&hash) {
            return Err(auth::Error::invalid_api_key());
        }

        Ok(Self(user_id))
    }
}

const API_KEY_HEADER: &'static str = "X-API-Key";
const API_KEY_HEADER_LOWER: &'static str = "x-api-key";

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = api::ErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        app_state: &AppState,
    ) -> Result<Self, api::ErrorResponse> {
        if let AppState::Dev { user_id, .. } = app_state {
            return Ok(Self(*user_id));
        }

        let headers = &parts.headers;

        let Some(Ok(api_key)) = headers
            .get(API_KEY_HEADER)
            .or_else(|| headers.get(API_KEY_HEADER_LOWER))
            .map(HeaderValue::to_str)
        else {
            return Err(auth::Error::no_api_key())?;
        };

        let db_conn = app_state.db_conn().await?;
        let api_key = api_key.into();

        Ok(db_conn
            .interact(move |db_conn| AuthenticatedUser::fetch_by_api_key(&api_key, db_conn))
            .await??)
    }
}
