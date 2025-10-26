use axum::extract::{FromRequestParts, Path};
use serde::de::DeserializeOwned;

use crate::{db::models::Jsonify, extract::Base64JsonQuery, result::ScamplersErrorResponse};

pub struct RelativesQuery<Id, Query>(pub (Id, Query));

impl<S, Id, Query> FromRequestParts<S> for RelativesQuery<Id, Query>
where
    S: Send + Sync,
    Id: Send + DeserializeOwned,
    Query: Jsonify + Default,
{
    type Rejection = ScamplersErrorResponse;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Path(id) = <Path<Id> as FromRequestParts<S>>::from_request_parts(parts, state).await?;
        let Base64JsonQuery(query) =
            <Base64JsonQuery<Query> as FromRequestParts<S>>::from_request_parts(parts, state)
                .await?;

        Ok(Self((id, query)))
    }
}
