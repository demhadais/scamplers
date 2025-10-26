use axum::{Json, extract::FromRequestParts, response::IntoResponse};
use diesel::{PgConnection, prelude::*};
use scamplers_schema::api_keys;
use uuid::Uuid;

use crate::{
    api::extract::auth::{self, api_key::ApiKey},
    db,
    state::AppState,
};

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct User(pub Uuid);

impl User {
    fn fetch_by_api_key(api_key: &ApiKey, conn: &mut PgConnection) -> Result<Self, auth::Error> {
        let (user_id, hash): (Uuid, String) = api_keys::table
            .select((api_keys::user_id, api_keys::hash))
            .filter(api_keys::prefix.eq(api_key.prefix()))
            .first(conn)
            .optional()?
            .ok_or(auth::Error::invalid_api_key())?;

        if !api_key.is_same_hash(&hash) {
            return Err(auth::Error::invalid_api_key());
        }

        Ok(Self(user_id))
    }
}

impl FromRequestParts<AppState> for User {
    type Rejection = auth::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        app_state: &AppState,
    ) -> Result<Self, auth::Error> {
        if let AppState::Dev { user_id, .. } = app_state {
            return Ok(Self(*user_id));
        }

        let Some(Ok(api_key)) = parts
            .headers
            .get("X-API-Key")
            .map(|s| s.to_str().unwrap().parse())
        else {
            return Err(auth::Error::invalid_api_key());
        };

        let db_conn = app_state.db_conn().await?;

        Ok(db_conn
            .interact(move |db_conn| User::fetch_by_api_key(&api_key, db_conn))
            .await??)
    }
}
