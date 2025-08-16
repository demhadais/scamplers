use axum::{Json, Router, extract::State, http::StatusCode};
use valuable::Valuable;

use crate::{
    auth::{Frontend, User},
    db::{
        DbOperation,
        models::{
            institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
            lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
            person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
        },
    },
    endpoints::{Api, Endpoint},
    extract::ValidJsonBody,
    result::ScamplersErrorResponse,
    state::AppState,
};

type ScamplersApiResponse<Request, Response> = Result<
    (
        StatusCode,
        <Api as Endpoint<Request, Response>>::ResponseWrapper,
    ),
    ScamplersErrorResponse,
>;

async fn inner_handler<Request, Response>(
    State(state): State<AppState>,
    User(user_id): User,
    request: Request,
) -> ScamplersApiResponse<Request, Response>
where
    Api: Endpoint<Request, Response>,
    <Api as Endpoint<Request, Response>>::ResponseWrapper: From<Response>,
    Request: DbOperation<Response> + Send + 'static + valuable::Valuable,
    Response: Send + 'static,
{
    tracing::info!(deserialized_request = request.as_value());

    let db_conn = state.db_conn().await?;

    let response = db_conn
        .interact(move |db_conn| request.execute_as_user(user_id, db_conn))
        .await??;
    let status = Api::SUCCESS_STATUS_CODE;

    Ok((status, response.into()))
}

macro_rules! router {
    ($router:expr; $(($handler_name:ident, $request_type:ty, $response_type:ty));*) => {{
        use crate::endpoints::{Api, Endpoint};
        use axum::{http::Method, routing::*};

        $(
            let path = <Api as Endpoint<$request_type, $response_type>>::PATH;
            let method = <Api as Endpoint<$request_type, $response_type>>::METHOD;

            #[axum::debug_handler]
            async fn $handler_name(state: State<AppState>, user: User, request: <Api as Endpoint<$request_type, $response_type>>::RequestExtractor) -> ScamplersApiResponse<$request_type, $response_type> {
                inner_handler::<$request_type, $response_type>(state, user, request.0).await
            }

            $router = match method {
                Method::GET => $router.route(path, get($handler_name)),
                Method::POST => $router.route(path, post($handler_name)),
                Method::PATCH => $router.route(path, patch($handler_name)),
                Method::DELETE => $router.route(path, delete($handler_name)),
                _ => unreachable!()
            };
        )*

        $router
    }};
}

async fn new_ms_login(
    _: Frontend,
    State(state): State<AppState>,
    ValidJsonBody(login): ValidJsonBody<NewPerson>,
) -> ScamplersApiResponse<NewPerson, CreatedUser> {
    tracing::info!(deserialized_request = login.as_value());

    let db_conn = state.db_conn().await?;

    let created_user = db_conn.interact(|db_conn| login.execute(db_conn)).await??;

    Ok((StatusCode::CREATED, Json(created_user)))
}

pub fn router() -> Router<AppState> {
    let mut router = Router::new();

    router = router!(
        router;
        (create_institution, NewInstitution, Institution);
        (read_institution, InstitutionId, Institution);
        (read_institutions, InstitutionQuery, Vec<Institution>);
        (create_person, NewPerson, Person);
        (read_person, PersonId, Person);
        (read_people, PersonQuery, Vec<Person>);
        (update_person, PersonUpdate, Person);
        (create_lab, NewLab, Lab);
        (read_lab, LabId, Lab);
        (read_labs, LabQuery, Vec<Lab>);
        (update_lab, LabUpdate, Lab)
    );

    router
}
