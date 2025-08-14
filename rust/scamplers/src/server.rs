use axum::Router;
use axum::extract::{FromRequest, State};
use axum::http::StatusCode;

use crate::auth::User;
use crate::db::models::institution::{InstitutionId, InstitutionQuery};
use crate::db::models::person::{NewPerson, Person, PersonId, PersonQuery};
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

type ApiResponse<Req, Resp> =
    Result<(StatusCode, <(Req, Resp) as ApiEndpoint>::ResponseWrapper), ScamplersErrorResponse>;

macro_rules! router {
    ($(($handler_name:ident, $request:ty, $response:ty));*) => {{
        use crate::endpoints::ApiEndpoint;
        use axum::{http::Method, routing::*};

        let mut router = axum::Router::new();

        $(
            let method = <($request, $response)>::METHOD;
            let path = <($request, $response)>::PATH;

            #[axum::debug_handler]
            async fn $handler_name(State(state): State<AppState>, User(user_id): User, request: <($request, $response) as ApiEndpoint>::RequestExtractor) -> ApiResponse<$request, $response> {
                let db_conn = state.db_conn().await?;
                let request = request.inner();
                let success = <($request, $response)>::SUCCESS_STATUS_CODE;

                let response: $response = db_conn
                    .interact(move |db_conn| request.execute_as_user(user_id, db_conn))
                    .await??;

                Ok((success, response.into()))
            }

            router = match method {
                Method::GET => router.route(path, get($handler_name)),
                Method::POST => router.route(path, post($handler_name)),
                Method::PATCH => router.route(path, patch($handler_name)),
                Method::DELETE => router.route(path, delete($handler_name)),
                _ => {anyhow::bail!("unexpected method: {method}")}
            };
        )*

        router
    }};
}

fn app() -> anyhow::Result<Router<AppState>> {
    Ok(router!(
        (create_institution, NewInstitution, Institution);
        (read_institution, InstitutionId, Institution);
        (read_institutions, InstitutionQuery, Vec<Institution>);
        (create_person, NewPerson, Person);
        (read_person, PersonId, Person);
        (read_people, PersonQuery, Vec<Person>)
    ))
}
