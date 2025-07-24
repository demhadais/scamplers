use axum::{
    Router,
    routing::{get, post},
};
use scamplers_core::{
    api_path::ToApiPath,
    model::{
        chromium_run::{ChromiumRun, NewChromiumRun},
        dataset::{DatasetSummary, NewDataset},
        institution::{Institution, InstitutionQuery, NewInstitution},
        lab::{Lab, LabQuery, LabSummary, NewLab},
        nucleic_acid::{CdnaHandle, LibraryHandle, NewCdnaGroup, NewLibrary},
        person::{CreatedUser, NewMsLogin, NewPerson, Person, PersonQuery, PersonSummary},
        sequencing_run::{NewSequencingRun, SequencingRunSummary},
        specimen::{NewSpecimen, Specimen, SpecimenQuery, SpecimenSummary},
        suspension::{NewSuspension, NewSuspensionPool, Suspension, SuspensionPoolHandle},
    },
};
use scamplers_schema::lab::dsl::lab;
use uuid::Uuid;

use super::AppState;
use crate::server::api::handler::{by_id, by_query, relatives, write};

mod handler;

// trait RouterExt {
//     fn post_to_db_route<Req>(self) -> Self
//     where
//         Req: WriteToDb + Send + Valuable,
//         Req::Returns: Send,
//         (Req, Req::Returns): ToApiPath;
//     fn fetch_by_id_route<Req, Resp>(self) -> Self;
//     fn fetch_by_query_route<Req, Resp>(self) -> Self;
// }

// impl RouterExt for Router {
//     fn post_to_db_route<Req>(self) -> Self
//     where
//         Req: WriteToDb + Send + Valuable,
//         Req::Returns: Send,
//         (Req, Req::Returns): ToApiPath,
//     {
//         self.route(&<(Req, Req::Returns)>::to_api_path(), post(write::<Req>))
//     }
// }

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async {}))
        .route(
            &<(NewInstitution, Institution)>::to_api_path(),
            post(write::<NewInstitution>),
        )
        .route(
            &<(Uuid, Institution)>::to_api_path(),
            get(by_id::<Institution>),
        )
        .route(
            &<(InstitutionQuery, Institution)>::to_api_path(),
            post(by_query::<Institution>),
        )
        .route(
            &<(NewPerson, Person)>::to_api_path(),
            post(write::<NewPerson>),
        )
        .route(
            &<(NewMsLogin, CreatedUser)>::to_api_path(),
            post(write::<NewMsLogin>),
        )
        .route(&<(Uuid, Person)>::to_api_path(), get(by_id::<Person>))
        .route(
            &<(PersonQuery, PersonSummary)>::to_api_path(),
            post(by_query::<PersonSummary>),
        )
        .route(&<(NewLab, Lab)>::to_api_path(), post(write::<NewLab>))
        .route(&<(Uuid, Lab)>::to_api_path(), get(by_id::<Lab>))
        .route(
            &<(LabQuery, LabSummary)>::to_api_path(),
            post(by_query::<LabSummary>),
        )
        .route(
            &format!("{}/members", <(Uuid, Lab)>::to_api_path()),
            get(relatives::<lab, PersonSummary>),
        )
        .route(
            &<(NewSpecimen, Specimen)>::to_api_path(),
            post(write::<NewSpecimen>),
        )
        .route(&<(Uuid, Specimen)>::to_api_path(), get(by_id::<Specimen>))
        .route(
            &<(SpecimenQuery, SpecimenSummary)>::to_api_path(),
            post(by_query::<SpecimenSummary>),
        )
        .route(
            &<(NewSuspension, Suspension)>::to_api_path(),
            post(write::<NewSuspension>),
        )
        .route(
            &<(NewSuspensionPool, SuspensionPoolHandle)>::to_api_path(),
            post(write::<NewSuspensionPool>),
        )
        .route(
            &<(NewChromiumRun, ChromiumRun)>::to_api_path(),
            post(write::<NewChromiumRun>),
        )
        .route(
            &<(NewCdnaGroup, Vec<CdnaHandle>)>::to_api_path(),
            post(write::<NewCdnaGroup>),
        )
        .route(
            &<(NewLibrary, LibraryHandle)>::to_api_path(),
            post(write::<NewLibrary>),
        )
        .route(
            &<(NewSequencingRun, SequencingRunSummary)>::to_api_path(),
            post(write::<NewSequencingRun>),
        )
        .route(
            &<(NewDataset, DatasetSummary)>::to_api_path(),
            post(write::<NewDataset>),
        )
}
