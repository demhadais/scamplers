use axum::{Json, Router, extract::State, http::StatusCode};
use valuable::Valuable;

use crate::{
    auth::{Frontend, User},
    db::{
        DbOperation,
        models::{
            chromium_run::{ChromiumRun, ChromiumRunId, ChromiumRunQuery, NewChromiumRun},
            dataset::chromium::{
                ChromiumDataset, ChromiumDatasetId, ChromiumDatasetQuery, NewChromiumDataset,
            },
            institution::{Institution, InstitutionId, InstitutionQuery, NewInstitution},
            lab::{Lab, LabId, LabQuery, LabUpdate, NewLab},
            multiplexing_tag::MultiplexingTag,
            nucleic_acid::{
                cdna::{Cdna, CdnaId, CdnaQuery, NewCdnaGroup},
                library::{Library, LibraryId, LibraryQuery, NewLibrary},
            },
            person::{CreatedUser, NewPerson, Person, PersonId, PersonQuery, PersonUpdate},
            sequencing_run::{
                NewSequencingRun, SequencingRun, SequencingRunId, SequencingRunQuery,
            },
            specimen::{NewSpecimen, Specimen, SpecimenId, SpecimenQuery},
            suspension::{
                pool::{NewSuspensionPool, SuspensionPool, SuspensionPoolId, SuspensionPoolQuery},
                suspension::{NewSuspension, Suspension, SuspensionId, SuspensionQuery},
            },
            tenx_assay::{NewTenxAssay, TenxAssay, TenxAssayId, TenxAssayQuery},
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
    (router = $router:expr, $($handler_name:ident($request_type:ty) -> $response_type:ty);*) => {{
        use crate::endpoints::{Api, Endpoint};
        use axum::{http::Method, routing::*};

        $(
            #[allow(clippy::items_after_statements)]
            #[axum::debug_handler]
            async fn $handler_name(state: State<AppState>, user: User, request: <Api as Endpoint<$request_type, $response_type>>::RequestExtractor) -> ScamplersApiResponse<$request_type, $response_type> {
                inner_handler::<$request_type, $response_type>(state, user, request.0).await
            }

            let path = <Api as Endpoint<$request_type, $response_type>>::PATH;
            let method = <Api as Endpoint<$request_type, $response_type>>::METHOD;

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

    let ms_login_path = <Api as Endpoint<NewPerson, CreatedUser>>::PATH;
    router = router.route(ms_login_path, axum::routing::post(new_ms_login));

    router = router!(
        router = router,
        create_institution(NewInstitution) -> Institution;
        fetch_institution(InstitutionId) -> Institution;
        list_institutions(InstitutionQuery) -> Vec<Institution>;
        create_person(NewPerson) -> Person;
        fetch_person(PersonId) -> Person;
        list_people(PersonQuery) -> Vec<Person>;
        update_person(PersonUpdate) -> Person;
        create_lab(NewLab) -> Lab;
        fetch_lab(LabId) -> Lab;
        list_labs(LabQuery) -> Vec<Lab>;
        update_lab(LabUpdate) -> Lab;
        create_specimen(NewSpecimen) -> Specimen;
        fetch_specimen(SpecimenId) -> Specimen;
        list_specimens(SpecimenQuery) -> Vec<Specimen>;
        list_person_specimens((PersonId, SpecimenQuery)) -> Vec<Specimen>;
        create_sequencing_run(NewSequencingRun) -> SequencingRun;
        list_sequencing_runs(SequencingRunQuery) -> Vec<SequencingRun>;
        fetch_sequencing_run(SequencingRunId) -> SequencingRun;
        create_tenx_assays(NewTenxAssay) -> TenxAssay;
        fetch_tenx_assay(TenxAssayId) -> TenxAssay;
        list_tenx_assays(TenxAssayQuery) -> Vec<TenxAssay>;
        list_multiplexing_tags(()) -> Vec<MultiplexingTag>;
        create_suspension(NewSuspension) -> Suspension;
        fetch_suspension(SuspensionId) -> Suspension;
        list_suspensions(SuspensionQuery) -> Vec<Suspension>;
        create_suspension_pool(NewSuspensionPool) -> SuspensionPool;
        fetch_suspension_pool(SuspensionPoolId) -> SuspensionPool;
        list_suspension_pools(SuspensionPoolQuery) -> Vec<SuspensionPool>;
        create_chromium_run(NewChromiumRun) -> ChromiumRun;
        fetch_chromium_run(ChromiumRunId) -> ChromiumRun;
        list_chromium_runs(ChromiumRunQuery) -> Vec<ChromiumRun>;
        create_cdna(NewCdnaGroup) -> Vec<Cdna>;
        fetch_cdna(CdnaId) -> Cdna;
        list_cdna(CdnaQuery) -> Vec<Cdna>;
        create_library(NewLibrary) -> Library;
        fetch_library(LibraryId) -> Library;
        list_libraries(LibraryQuery) -> Vec<Library>;
        create_chromium_dataset(NewChromiumDataset) -> ChromiumDataset;
        fetch_chromium_dataset(ChromiumDatasetId) -> ChromiumDataset;
        list_chromium_datasets(ChromiumDatasetQuery) -> Vec<ChromiumDataset>
    );

    router
}
