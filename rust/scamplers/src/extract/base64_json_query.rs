use axum::extract::OptionalFromRequestParts;
use axum::http::StatusCode;
use serde::de::DeserializeOwned;

use crate::{
    db::models::Jsonify,
    result::{MalformedRequestError, ScamplersErrorResponse},
};

#[derive(Default)]
pub struct Base64JsonQuery<T>(T);

impl<S, Q> OptionalFromRequestParts<S> for Base64JsonQuery<Q>
where
    Q: DeserializeOwned + Jsonify,
    S: Sync,
{
    type Rejection = ScamplersErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        fn err(e: anyhow::Error) -> ScamplersErrorResponse {
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
            return Ok(None);
        };

        let extracted = Q::from_base64_json(raw).map_err(err)?;

        Ok(Some(Self(extracted)))
    }
}
