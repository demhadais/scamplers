use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{
    TypedHeader,
    headers::{self, authorization::Bearer},
};

use crate::{
    api::{self, extract::auth},
    state::AppState,
};

pub struct AuthenticatedUi;

impl FromRequestParts<AppState> for AuthenticatedUi {
    type Rejection = api::ErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let ui_auth_token = match state {
            AppState::Dev { .. } => {
                return Ok(Self);
            }
            AppState::Prod { ui_auth_token, .. } => ui_auth_token,
        };

        let Ok(received_token) = parts
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
        else {
            return Err(auth::Error::no_ui_auth_token())?;
        };

        if received_token.token() != ui_auth_token {
            return Err(auth::Error::invalid_ui_auth_token())?;
        }

        Ok(Self)
    }
}
