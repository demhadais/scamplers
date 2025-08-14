use axum::extract::{FromRequestParts, OptionalFromRequestParts};
use axum::http::StatusCode;
use serde::{Serialize, Serializer, de::DeserializeOwned};

use crate::{
    db::models::Jsonify,
    result::{MalformedRequestError, ScamplersErrorResponse},
};

#[derive(Default, Serialize)]
pub struct Base64JsonQuery<T: Jsonify>(#[serde(serialize_with = "serialize_base64_json")] pub T);

fn serialize_base64_json<Q: Jsonify, S: Serializer>(
    query: &Q,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&query.to_base64_json())
}

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
            return Ok(Self::default());
        };

        let extracted = Q::from_base64_json(raw).map_err(err)?;

        Ok(Self(extracted))
    }
}
