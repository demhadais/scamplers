use anyhow::anyhow;
use axum::{extract::FromRequestParts, http::StatusCode};

use crate::{
    db::models::Jsonify,
    result::{MalformedRequestError, ScamplersErrorResponse},
};

#[derive(Debug, Default)]
pub struct Base64JsonQuery<T>(pub T);

impl<S, Q> FromRequestParts<S> for Base64JsonQuery<Q>
where
    Q: Jsonify + Default,
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

        let Some(raw_querystring) = parts.uri.query() else {
            return Ok(Self::default());
        };

        if raw_querystring.is_empty() {
            return Ok(Self::default());
        }

        let Some(("query", raw_value)) = raw_querystring.split_once('=') else {
            return Err(err(&anyhow!(
                "query string must be of the form 'query=<base64-encoded json>'"
            )));
        };

        let extracted = Q::from_base64_json(raw_value).map_err(|e| err(&e))?;

        Ok(Self(extracted))
    }
}

#[cfg(all(feature = "app", test))]
mod tests {
    use std::str::FromStr;

    use axum::{body::Body, extract::FromRequest, http::Uri};
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use scamplers_macros::Jsonify;
    use serde::{Deserialize, Serialize};

    use crate::extract::Base64JsonQuery;

    #[derive(Debug, Default, Deserialize, Serialize, Jsonify, PartialEq)]
    struct Data {
        field: String,
    }

    fn empty_request(query: &str) -> axum::http::Request<Body> {
        let mut request = axum::http::Request::new(axum::body::Body::empty());
        *request.uri_mut() = Uri::from_str(&format!("http://localhost{query}")).unwrap();

        request
    }

    #[rstest]
    #[case("?query=eyJmaWVsZCI6ImludGVyZXN0aW5nIGRhdGEifQ==", "interesting data")]
    #[case("", "")]
    #[case("?", "")]
    #[tokio::test]
    async fn correct_query(#[case] query: &str, #[case] expected_value: &str) {
        let request = empty_request(query);

        let Base64JsonQuery(deserialized) = Base64JsonQuery::<Data>::from_request(request, &())
            .await
            .unwrap();

        assert_eq!(
            deserialized,
            Data {
                field: expected_value.to_string()
            }
        );
    }

    #[rstest]
    #[case("?filter=eyJmaWVsZCI6ImludGVyZXN0aW5nIGRhdGEifQ==")]
    #[case("?query=eyJmaWVsZCI6ImludGVyZXN0aW5nIGRhdGEi")]
    #[tokio::test]
    async fn malformed_query(#[case] query: &str) {
        let request = empty_request(query);

        Base64JsonQuery::<Data>::from_request(request, &())
            .await
            .unwrap_err();
    }
}
