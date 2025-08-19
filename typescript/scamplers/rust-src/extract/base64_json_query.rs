use axum::{extract::FromRequestParts, http::StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    db::models::Jsonify,
    result::{MalformedRequestError, ScamplersErrorResponse},
};

#[derive(Default)]
pub struct Base64JsonQuery<T: Jsonify>(pub T);

impl<S, Q> FromRequestParts<S> for Base64JsonQuery<Q>
where
    Q: DeserializeOwned + Jsonify + Default,
    S: Sync,
{
    type Rejection = ScamplersErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        fn err(e: &anyhow::Error) -> ScamplersErrorResponse {
            ScamplersErrorResponse::builder()
                .status(StatusCode::BAD_REQUEST)
                .error(
                    MalformedRequestError::builder()
                        .message(format! {"failed to read query: {e}"})
                        .build(),
                )
                .build()
        }

        let Some(raw) = parts.uri.query() else {
            return Ok(Self::default());
        };

        let extracted = Q::from_base64_json(raw).map_err(|e| err(&e))?;

        Ok(Self(extracted))
    }
}
