use axum::{Json, Router, extract::State, http::StatusCode};
use axum_extra::routing::{RouterExt, TypedPath};
use scamplers_models::{
    institution::{self, Institution, InstitutionId},
    person::{self, Person, PersonId, PersonSummary},
};
use serde_qs::axum::QsQuery;
use uuid::Uuid;

use crate::{
    api::{
        error::ErrorResponse,
        extract::{
            ValidJson,
            auth::{self, AuthenticatedUser},
        },
    },
    db,
    state::AppState,
};

mod institutions;
mod people;

async fn inner_handler<Request, Response>(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    request: Request,
) -> Result<Json<Response>, ErrorResponse>
where
    Request: std::fmt::Debug + db::Operation<Response> + Send + serde::Serialize + 'static,
    Response: Send + 'static,
{
    tracing::info!("{request:?}");

    let db_conn = state.db_conn().await?;

    let response = db_conn
        .interact(move |db_conn| request.execute_as_user(user.id(), db_conn))
        .await??;

    Ok(Json(response))
}

type ApiResponse<T> = Result<(StatusCode, Json<T>), super::error::ErrorResponse>;

#[derive(TypedPath)]
#[typed_path("/institutions")]
struct InstitutionsEndpoint;

async fn create_institution(
    _: InstitutionsEndpoint,
    state: State<AppState>,
    user: AuthenticatedUser,
    ValidJson(request): ValidJson<institution::Creation>,
) -> ApiResponse<Institution> {
    let item = inner_handler(state, user, request).await?;
    Ok((StatusCode::CREATED, item))
}

async fn fetch_institution(
    request: InstitutionId,
    state: State<AppState>,
    user: AuthenticatedUser,
) -> ApiResponse<Institution> {
    let item = inner_handler(state, user, request).await?;
    Ok((StatusCode::FOUND, item))
}

async fn list_institutions(
    _: InstitutionsEndpoint,
    state: State<AppState>,
    user: AuthenticatedUser,
    QsQuery(request): QsQuery<institution::Query>,
) -> ApiResponse<Vec<Institution>> {
    let items = inner_handler(state, user, request).await?;
    Ok((StatusCode::OK, items))
}

#[derive(TypedPath)]
#[typed_path("/people")]
struct PeopleEndpoint;

async fn create_person(
    _: PeopleEndpoint,
    state: State<AppState>,
    user: AuthenticatedUser,
    ValidJson(request): ValidJson<person::Creation>,
) -> ApiResponse<Person> {
    let person = inner_handler(state, user, request).await?;
    Ok((StatusCode::CREATED, person))
}

async fn fetch_person(
    request: PersonId,
    state: State<AppState>,
    user: AuthenticatedUser,
) -> ApiResponse<Person> {
    let person = inner_handler(state, user, request).await?;
    Ok((StatusCode::OK, person))
}

async fn list_people(
    _: PeopleEndpoint,
    state: State<AppState>,
    user: AuthenticatedUser,
    QsQuery(request): QsQuery<person::Query>,
) -> ApiResponse<Vec<PersonSummary>> {
    let people = inner_handler(state, user, request).await?;
    Ok((StatusCode::OK, people))
}

#[derive(Debug, serde::Deserialize, serde::Serialize, TypedPath)]
#[typed_path("/people/{user_id}/api-keys/{api_key_prefix}")]
struct DeleteApiKeyEndpoint {
    user_id: Uuid,
    api_key_prefix: String,
}

async fn delete_api_key(
    request: DeleteApiKeyEndpoint,
    state: State<AppState>,
    user: AuthenticatedUser,
) -> ApiResponse<()> {
    // Check if the authenticated user is actually deleting their own API key. Technically this isn't necessary because of postgres's RLS
    if request.user_id != user.id() {
        return Err(super::ErrorResponse::from(auth::Error::invalid_api_key()));
    }

    let resp = inner_handler(state, user, request).await?;

    Ok((StatusCode::OK, resp))
}

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .typed_post(create_institution)
        .typed_get(fetch_institution)
        .typed_get(list_institutions)
        .typed_post(create_person)
        .typed_get(fetch_person)
        .typed_get(list_people)
        .typed_delete(delete_api_key)
}
