use axum::Router;
use axum::extract::{FromRequest, State};
use axum::http::StatusCode;

use crate::auth::User;
use crate::db::models::person::{NewPerson, Person};
use crate::extract::RequestExtractorExt;
use crate::state::AppState;
use crate::{
    db::{
        DbOperation,
        models::institution::{Institution, NewInstitution},
    },
    endpoints::ApiEndpoint,
    result::ScamplersErrorResponse,
};

macro_rules! router {
    ($(($request:ty, $response:ty));*) => {{
        use crate::endpoints::ApiEndpoint;
        use axum::{http::Method, routing::*};

        let mut router = axum::Router::new();

        $(
            let method = <($request, $response)>::METHOD;
            let path = <($request, $response)>::PATH;

            router = match method {
                Method::GET => router.route(path, get(handle::<$request, $response>)),
                Method::POST => router.route(path, post(handle::<$request, $response>)),
                Method::PATCH => router.route(path, patch(handle::<$request, $response>)),
                Method::DELETE => router.route(path, delete(handle::<$request, $response>)),
                _ => {anyhow::bail!("unexpected method: {method}")}
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
        FromRequest<AppState> + RequestExtractorExt<Request> + Send + Sync,
    <(Request, Response) as ApiEndpoint>::ResponseWrapper: From<Response> + Send + Sync,
    Request: DbOperation<Response> + Send + Sync + 'static,
    Response: Send + Sync + 'static,
{
    let db_conn = state.db_conn().await?;
    let request = request.inner();
    let success = <(Request, Response)>::SUCCESS_STATUS_CODE;

    let response = db_conn
        .interact(move |db_conn| request.execute_as_user(user_id, db_conn))
        .await??
        .into();

    Ok((success, response))
}

fn app() -> anyhow::Result<Router<AppState>> {
    Ok(router!(
        (NewInstitution, Institution);
        (NewPerson, Person)
    ))
}
