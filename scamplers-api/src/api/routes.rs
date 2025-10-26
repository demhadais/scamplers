use axum::{Json, Router, debug_handler, extract::State, http::StatusCode, routing::post};
use scamplers_models::person::{CreatedUser, Person};

use crate::{
    api::{
        auth::{Frontend, User},
        error::ErrorResponse,
        extract::ValidJson,
    },
    db::{self, Operation},
    state::AppState,
};

pub mod institution;
pub mod person;

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

async fn new_ms_login(
    _: Frontend,
    State(state): State<AppState>,
    ValidJson(login): ValidJson<scamplers_models::person::Creation>,
) -> ApiResponse<CreatedUser> {
    tracing::info!(deserialized_request = serde_json::to_string(&login).unwrap());

    let db_conn = state.db_conn().await?;

    let created_user = db_conn.interact(|db_conn| login.execute(db_conn)).await??;

    Ok((StatusCode::CREATED, Json(created_user)))
}

#[debug_handler]
async fn create_person(
    state: State<AppState>,
    user: User,
    ValidJson(request): ValidJson<scamplers_models::person::Creation>,
) -> ApiResponse<Person> {
    let person = inner_handler(state, user, request).await?;
    Ok((StatusCode::CREATED, person))
}

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/people", post(create_person))
        .route("/login", post(new_ms_login))
    // .typed_post(create_institution)
}
