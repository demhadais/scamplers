use axum::{
    Router,
    routing::{get, post},
};
use scamplers_core::{
    api_path::ToApiPath,
    model::{
        institution::{Institution, InstitutionQuery, NewInstitution},
        lab::{Lab, LabQuery, LabSummary, NewLab},
        person::{CreatedUser, NewMsLogin, NewPerson, Person, PersonQuery, PersonSummary},
        sequencing_run::NewSequencingRun,
        specimen::{NewSpecimen, Specimen, SpecimenQuery, SpecimenSummary},
        suspension::{NewSuspension, Suspension},
    },
};
use scamplers_schema::lab::dsl::lab;
use uuid::Uuid;

use crate::server::api::handler::{by_id, by_query, new_user, relatives, write};

use super::AppState;

mod error;
mod handler;

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
        .route(&<(NewMsLogin, CreatedUser)>::to_api_path(), post(new_user))
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
            &<(NewSequencingRun, ())>::to_api_path(),
            post(write::<NewSequencingRun>),
        )
        .route(
            &<(NewSuspension, Suspension)>::to_api_path(),
            post(write::<NewSuspension>),
        )
}
