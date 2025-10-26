use axum::{Json, Router, debug_handler, extract::State, http::StatusCode, routing::post};
use axum_extra::routing::{RouterExt, TypedPath};
use scamplers_models::{
    institution::{self, Institution, InstitutionId},
    person::{self, CreatedUser, Person, PersonId, PersonSummary},
};
use serde_qs::axum::QsQuery;

use crate::{
    api::{
        error::ErrorResponse,
        extract::{
            ValidJson,
            auth::{ApiKey, Frontend, User},
        },
    },
    db::{self, Operation},
    state::AppState,
};

mod institutions;
mod people;

async fn inner_handler<Request, Response>(
    State(state): State<AppState>,
    User(user_id): User,
    request: Request,
) -> Result<Json<Response>, ErrorResponse>
where
    Request: db::Operation<Response> + Send + serde::Serialize + 'static,
    Response: Send + 'static,
{
    tracing::info!(deserialized_request = serde_json::to_string(&request).unwrap());

    let db_conn = state.db_conn().await?;

    let response = db_conn
        .interact(move |db_conn| request.execute_as_user(user_id, db_conn))
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
    user: User,
    ValidJson(request): ValidJson<institution::Creation>,
) -> ApiResponse<Institution> {
    let item = inner_handler(state, user, request).await?;
    Ok((StatusCode::CREATED, item))
}

async fn fetch_institution(
    request: InstitutionId,
    state: State<AppState>,
    user: User,
) -> ApiResponse<Institution> {
    let item = inner_handler(state, user, request).await?;
    Ok((StatusCode::FOUND, item))
}

async fn list_institutions(
    _: InstitutionsEndpoint,
    state: State<AppState>,
    user: User,
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
    user: User,
    ValidJson(request): ValidJson<person::Creation>,
) -> ApiResponse<Person> {
    let person = inner_handler(state, user, request).await?;
    Ok((StatusCode::CREATED, person))
}

async fn fetch_person(
    request: PersonId,
    state: State<AppState>,
    user: User,
) -> ApiResponse<Person> {
    let person = inner_handler(state, user, request).await?;
    Ok((StatusCode::FOUND, person))
}

async fn list_people(
    _: PeopleEndpoint,
    state: State<AppState>,
    user: User,
    QsQuery(request): QsQuery<person::Query>,
) -> ApiResponse<Vec<PersonSummary>> {
    let people = inner_handler(state, user, request).await?;
    Ok((StatusCode::OK, people))
}

#[derive(TypedPath)]
#[typed_path("/api-keys")]
struct ApiKeysEndpoint;

async fn create_api_key(
    _: ApiKeysEndpoint,
    state: State<AppState>,
    user: User,
) -> ApiResponse<serde_json::Value> {
    // Extract the API key and put it inside a JSON object
    let Json(api_key) = inner_handler(state, user, user).await?;
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"api_key": api_key})),
    ))
}

#[derive(serde::Deserialize, serde::Serialize, TypedPath)]
#[typed_path("/api-keys/{prefix}")]
struct ApiKeyPrefix(String);

async fn delete_api_key(
    ApiKeyPrefix(prefix): ApiKeyPrefix,
    state: State<AppState>,
    user: User,
) -> ApiResponse<()> {
    let resp = inner_handler(state, user, (prefix, user)).await?;
    Ok((StatusCode::OK, resp))
}

#[derive(TypedPath)]
#[typed_path("/signin/microsoft")]
struct MicrosoftSigninEndpoint;

async fn microsoft_signin(
    _: MicrosoftSigninEndpoint,
    _: Frontend,
    State(state): State<AppState>,
    ValidJson(signin): ValidJson<scamplers_models::person::Creation>,
) -> ApiResponse<CreatedUser> {
    tracing::info!(deserialized_request = serde_json::to_string(&signin).unwrap());

    let db_conn = state.db_conn().await?;

    let created_user = db_conn
        .interact(|db_conn| signin.execute(db_conn))
        .await??;

    Ok((StatusCode::CREATED, Json(created_user)))
}

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .typed_post(create_institution)
        .typed_get(fetch_institution)
        .typed_get(list_institutions)
        .typed_post(create_person)
        .typed_get(fetch_person)
        .typed_get(list_people)
        .typed_post(create_api_key)
        .typed_delete(delete_api_key)
        .typed_post(microsoft_signin)
}
