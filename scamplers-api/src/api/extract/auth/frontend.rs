use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{
    TypedHeader,
    headers::{self, authorization::Bearer},
};

use crate::{api::extract::auth, state::AppState};

pub struct Frontend;

impl FromRequestParts<AppState> for Frontend {
    type Rejection = auth::Error;

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
            return Err(auth::Error::invalid_frontend_token())?;
        };

        if frontend_auth.token() != frontend_token {
            return Err(auth::Error::invalid_frontend_token())?;
        }

        Ok(Self)
    }
}
