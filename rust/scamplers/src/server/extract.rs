use axum::{
    Json,
    extract::{FromRequest, OptionalFromRequest, OptionalFromRequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use garde::Validate;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    db::models::Jsonify,
    result::{MalformedRequestError, ScamplersErrorResponse},
};

pub trait RequestExtractorExt<T> {
    fn inner(self) -> T;
}

impl<T> RequestExtractorExt<T> for ValidJsonBody<T> {
    fn inner(self) -> T {
        self.0
    }
}

#[derive(Default)]
pub struct ValidJsonBody<T>(T);

impl<S, T> FromRequest<S> for ValidJsonBody<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
    T::Context: std::default::Default,
{
    type Rejection = ScamplersErrorResponse;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(data) = <Json<T> as FromRequest<S>>::from_request(req, state).await?;
        data.validate()?;

        Ok(Self(data))
    }
}

impl<S, T> OptionalFromRequest<S> for ValidJsonBody<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
    T::Context: std::default::Default,
{
    type Rejection = ScamplersErrorResponse;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Option<Self>, Self::Rejection> {
        let Some(Json(data)) =
            <Json<T> as OptionalFromRequest<S>>::from_request(req, state).await?
        else {
            return Ok(None);
        };

        data.validate()?;

        Ok(Some(Self(data)))
    }
}

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
