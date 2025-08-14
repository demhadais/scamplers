use axum::{Json, extract::FromRequest};
use garde::Validate;
use serde::{Serialize, de::DeserializeOwned};

use crate::result::ScamplersErrorResponse;

#[derive(Default, Serialize)]
pub struct ValidJsonBody<T>(pub T);

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
