use uuid::Uuid;

use crate::model::{
    institution::{Institution, InstitutionQuery, NewInstitution},
    lab::{Lab, LabQuery, LabSummary, NewLab},
    person::{CreatedUser, NewMsLogin, NewPerson, Person, PersonQuery, PersonSummary},
    sequencing_run::NewSequencingRun,
    specimen::{NewSpecimen, Specimen, SpecimenQuery, SpecimenSummary},
    suspension::{NewSuspension, Suspension},
};

const SEARCH_SUFFIX: &str = "search";

pub trait ToApiPath {
    fn to_api_path() -> String;
}

const INSTITUTIONS: &str = "/institutions";
impl ToApiPath for (NewInstitution, Institution) {
    #[must_use]
    fn to_api_path() -> String {
        INSTITUTIONS.to_string()
    }
}

impl ToApiPath for (Uuid, Institution) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{INSTITUTIONS}/{{id}}")
    }
}

impl ToApiPath for (InstitutionQuery, Institution) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{INSTITUTIONS}/{SEARCH_SUFFIX}")
    }
}

const PEOPLE: &str = "/people";
impl ToApiPath for (NewPerson, Person) {
    #[must_use]
    fn to_api_path() -> String {
        PEOPLE.to_string()
    }
}

impl ToApiPath for (Uuid, Person) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{PEOPLE}/{{id}}")
    }
}

impl ToApiPath for (PersonQuery, PersonSummary) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{PEOPLE}/{SEARCH_SUFFIX}")
    }
}

const MICROSOFT_LOGIN: &str = "/microsoft-login";
impl ToApiPath for (NewMsLogin, CreatedUser) {
    #[must_use]
    fn to_api_path() -> String {
        MICROSOFT_LOGIN.to_string()
    }
}

const LABS: &str = "/labs";
impl ToApiPath for (NewLab, Lab) {
    #[must_use]
    fn to_api_path() -> String {
        LABS.to_string()
    }
}

impl ToApiPath for (Uuid, Lab) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{LABS}/{{id}}")
    }
}

impl ToApiPath for (LabQuery, LabSummary) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{LABS}/{SEARCH_SUFFIX}")
    }
}

const SPECIMENS: &str = "/specimens";
impl ToApiPath for (NewSpecimen, Specimen) {
    #[must_use]
    fn to_api_path() -> String {
        SPECIMENS.to_string()
    }
}

impl ToApiPath for (Uuid, Specimen) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{SPECIMENS}/{{id}}")
    }
}

impl ToApiPath for (SpecimenQuery, SpecimenSummary) {
    #[must_use]
    fn to_api_path() -> String {
        format!("{SPECIMENS}/{SEARCH_SUFFIX}")
    }
}

const SEQUENCING_RUNS: &str = "/sequencing_runs";
impl ToApiPath for (NewSequencingRun, ()) {
    #[must_use]
    fn to_api_path() -> String {
        SEQUENCING_RUNS.to_string()
    }
}

const SUSPENSIONS: &str = "/suspensions";
impl ToApiPath for (NewSuspension, Suspension) {
    #[must_use]
    fn to_api_path() -> String {
        SUSPENSIONS.to_string()
    }
}
