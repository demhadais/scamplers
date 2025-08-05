use axum::{
    Json,
    extract::{FromRequest, OptionalFromRequest, Path, State},
    response::{IntoResponse, Response},
};
use diesel_async::{AsyncConnection, scoped_futures::ScopedFutureExt};
use garde::Validate;
use scamplers_core::model::person::{CreatedUser, NewMsLogin};
use serde::{Serialize, de::DeserializeOwned};
use valuable::Valuable;

use crate::{
    db::{
        DbTransaction,
        model::{self, FetchRelatives, WriteToDb},
    },
    result::{ScamplersError, ScamplersResult},
    server::{
        AppState,
        auth::{Frontend, User},
    },
};

#[derive(Default)]
pub(super) struct ValidJson<T>(T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
    T::Context: std::default::Default,
{
    type Rejection = ScamplersError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let Json(data) = <Json<T> as FromRequest<S>>::from_request(req, state).await?;
        data.validate()?;

        Ok(Self(data))
    }
}

impl<S, T> OptionalFromRequest<S> for ValidJson<T>
where
    S: Send + Sync,
    T: Validate + DeserializeOwned,
    T::Context: std::default::Default,
{
    type Rejection = ScamplersError;

    async fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> std::result::Result<Option<Self>, Self::Rejection> {
        let Some(Json(data)) =
            <Json<T> as OptionalFromRequest<S>>::from_request(req, state).await?
        else {
            return Ok(None);
        };

        data.validate()?;

        Ok(Some(Self(data)))
    }
}

impl<T: Serialize> IntoResponse for ValidJson<T> {
    fn into_response(self) -> Response {
        let Self(inner) = self;

        axum::Json(inner).into_response()
    }
}

pub async fn new_ms_login(
    _: Frontend,
    State(app_state): State<AppState>,
    ValidJson(login): ValidJson<NewMsLogin>,
) -> ScamplersResult<Json<CreatedUser>> {
    tracing::info!(deserialized_data = login.as_value());

    let mut db_conn = app_state.db_conn().await?;

    let item = db_conn
        .transaction(|conn| async move { login.write_to_db(conn).await }.scope_boxed())
        .await?;

    Ok(Json(item))
}

pub async fn write<Data>(
    User(user_id): User,
    State(app_state): State<AppState>,
    ValidJson(data): ValidJson<Data>,
) -> ScamplersResult<Json<Data::Returns>>
where
    Data: WriteToDb + Send + valuable::Valuable,
    Data::Returns: Send,
{
    tracing::info!(deserialized_data = data.as_value());

    let mut db_conn = app_state.db_conn().await?;

    let item = db_conn
        .transaction(|conn| {
            async move {
                conn.set_transaction_user(&user_id.to_string()).await?;

                data.write_to_db(conn).await
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json(item))
}

pub async fn by_id<Resource>(
    User(user_id): User,
    State(app_state): State<AppState>,
    Path(resource_id): Path<Resource::Id>,
) -> ScamplersResult<Json<Resource>>
where
    Resource: model::FetchById + Send,
    Resource::Id: Send + Sync + valuable::Valuable,
{
    tracing::info!(deserialized_id = resource_id.as_value());

    let mut db_conn = app_state.db_conn().await?;

    let item = db_conn
        .transaction(|conn| {
            async move {
                conn.set_transaction_user(&user_id.to_string()).await?;

                Resource::fetch_by_id(&resource_id, conn).await
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json(item))
}

pub async fn by_query<Resource>(
    User(user_id): User,
    State(app_state): State<AppState>,
    query: Option<ValidJson<Resource::QueryParams>>,
) -> ScamplersResult<Json<Vec<Resource>>>
where
    Resource: model::FetchByQuery + Send,
    Resource::QueryParams: Send + valuable::Valuable + Default,
{
    let ValidJson(query) = query.unwrap_or_default();
    tracing::info!(deserialized_query = query.as_value());

    let mut db_conn = app_state.db_conn().await?;

    let item = db_conn
        .transaction(|conn| {
            async move {
                conn.set_transaction_user(&user_id.to_string()).await?;

                Resource::fetch_by_query(&query, conn).await
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json(item))
}

pub(super) async fn relatives<Table, Relative>(
    User(user_id): User,
    State(app_state): State<AppState>,
    Path(id): Path<Table::Id>,
) -> ScamplersResult<Json<Vec<Relative>>>
where
    Table: FetchRelatives<Relative>,
    Table::Id: Valuable + Send,
    Relative: Send,
{
    tracing::info!(deserialized_id = id.as_value());

    let mut db_conn = app_state.db_conn().await?;

    let item = db_conn
        .transaction(|conn| {
            async move {
                conn.set_transaction_user(&user_id.to_string()).await?;

                Table::fetch_relatives(&id, conn).await
            }
            .scope_boxed()
        })
        .await?;

    Ok(Json(item))
}
