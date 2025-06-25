use std::marker::PhantomData;

use uuid::Uuid;

use crate::model::{
    institution::{Institution, InstitutionQuery, NewInstitution},
    lab::{Lab, LabQuery, LabSummary, NewLab},
    person::{NewPerson, Person, PersonQuery, PersonSummary},
    sequencing_run::NewSequencingRun,
    specimen::{NewSpecimen, Specimen, SpecimenQuery, SpecimenSummary},
    suspension::{NewSuspension, Suspension},
};

pub struct Endpoint<Req, Resp>(PhantomData<Req>, PhantomData<Resp>);

const SEARCH_SUFFIX: &str = "search";

const INSTITUTIONS: &str = "/institutions";
impl Endpoint<NewInstitution, Institution> {
    #[must_use]
    pub fn route() -> String {
        INSTITUTIONS.to_string()
    }
}

impl Endpoint<Uuid, Institution> {
    #[must_use]
    pub fn route() -> String {
        format!("{INSTITUTIONS}/{{id}}")
    }
}

impl Endpoint<InstitutionQuery, Institution> {
    #[must_use]
    pub fn route() -> String {
        format!("{INSTITUTIONS}/{SEARCH_SUFFIX}")
    }
}

const PEOPLE: &str = "/people";
impl Endpoint<NewPerson, Person> {
    #[must_use]
    pub fn route() -> String {
        PEOPLE.to_string()
    }
}

impl Endpoint<Uuid, Person> {
    #[must_use]
    pub fn route() -> String {
        format!("{PEOPLE}/{{id}}")
    }
}

impl Endpoint<PersonQuery, PersonSummary> {
    #[must_use]
    pub fn route() -> String {
        format!("{PEOPLE}/{SEARCH_SUFFIX}")
    }
}

const LABS: &str = "/labs";
impl Endpoint<NewLab, Lab> {
    #[must_use]
    pub fn route() -> String {
        LABS.to_string()
    }
}

impl Endpoint<Uuid, Lab> {
    #[must_use]
    pub fn route() -> String {
        format!("{LABS}/{{id}}")
    }
}

impl Endpoint<LabQuery, LabSummary> {
    #[must_use]
    pub fn route() -> String {
        format!("{LABS}/{SEARCH_SUFFIX}")
    }
}

const SPECIMENS: &str = "/specimens";
impl Endpoint<NewSpecimen, Specimen> {
    #[must_use]
    pub fn route() -> String {
        SPECIMENS.to_string()
    }
}

impl Endpoint<Uuid, Specimen> {
    #[must_use]
    pub fn route() -> String {
        format!("{SPECIMENS}/{{id}}")
    }
}

impl Endpoint<SpecimenQuery, SpecimenSummary> {
    #[must_use]
    pub fn route() -> String {
        format!("{SPECIMENS}/{SEARCH_SUFFIX}")
    }
}

const SEQUENCING_RUNS: &str = "/sequencing_runs";
impl Endpoint<NewSequencingRun, ()> {
    #[must_use]
    pub fn route() -> String {
        SEQUENCING_RUNS.to_string()
    }
}

const SUSPENSIONS: &str = "/suspensions";
impl Endpoint<NewSuspension, Suspension> {
    #[must_use]
    pub fn route() -> String {
        SUSPENSIONS.to_string()
    }
}
