use axum::extract::{FromRequest, State};
use axum::http::StatusCode;

use crate::server::extract::RequestExtractorExt;
use crate::{
    db::{
        DbOperation,
        models::institution::{Institution, NewInstitution},
    },
    result::ScamplersErrorResponse,
    server::endpoints::ApiEndpoint,
};

pub mod endpoints;
pub mod extract;

macro_rules! router {
    ($(($request:ty, $response:ty));*) => {{
        use endpoints::ApiEndpoint;
        use axum::{http::Method, routing::*};

        let mut router = axum::Router::new();

        $(
            let method = <($request, $response)>::METHOD;
            let path = <($request, $response)>::PATH;

            router = match method {
                Method::GET => router.route(path, get(handle::<$request, $response>)),
                Method::POST => router.route(path, post(handle::<$request, $response>)),
                Method::PATCH => router.route(path, patch(handle::<$request, $response>)),
                Method::DELETE => router.route(path, delete(handle::<$request, $response>))
            };
        )*

        router
    }};
}

type ApiResponse<T> = Result<T, ScamplersErrorResponse>;

async fn handle<Request, Response>(
    State(state): State<AppState>,
    User(user_id): User,
    request: <(Request, Response) as ApiEndpoint>::RequestExtractor,
) -> ApiResponse<(
    StatusCode,
    <(Request, Response) as ApiEndpoint>::ResponseWrapper,
)>
where
    (Request, Response): ApiEndpoint,
    <(Request, Response) as ApiEndpoint>::RequestExtractor:
        FromRequest<AppState> + RequestExtractorExt<Request>,
    <(Request, Response) as ApiEndpoint>::ResponseWrapper: From<Response>,
    Request: DbOperation<Response>,
{
    let db_conn = state.db_conn();
    let request = request.inner();
    let success = <(Request, Response)>::SUCCESS_STATUS_CODE;
    let response = request.execute_as_user(user_id, db_conn)?.into();

    Ok((success, response))
}

fn app() {
    let router = router!((NewInstitution, Institution));
}
