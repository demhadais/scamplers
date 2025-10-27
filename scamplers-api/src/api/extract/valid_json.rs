use axum::{Json, extract::FromRequest, http::StatusCode};
use serde::{Serialize, de::DeserializeOwned};

use crate::{api, state::AppState, validate::Validate};

#[derive(Default, Serialize)]
pub struct ValidJson<T>(pub T);

impl<T> FromRequest<AppState> for ValidJson<T>
where
    T: Validate + DeserializeOwned + Send + Sync + 'static,
{
    type Rejection = api::ErrorResponse;

    async fn from_request(
        req: axum::extract::Request,
        state: &AppState,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(data) = <Json<T> as FromRequest<AppState>>::from_request(req, state).await?;

        let db_conn = state.db_conn().await?;
        let mut db_conn = db_conn.lock().map_err(|e| api::ErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            public_error: api::error::Error::Other,
            internal_error: None,
        })?;
        data.validate(&mut db_conn)?;

        Ok(Self(data))
    }
}
