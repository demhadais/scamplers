use crate::{
    model::{IsUpdate, Pagination, SortByGroup, institution::Institution},
    string::NonEmptyString,
};
use pyo3::{pyclass, pymethods};
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_enum, db_insertion, db_query, db_selection,
    db_update,
};
#[cfg(feature = "backend")]
use scamplers_schema::person;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[db_enum]
#[derive(PartialEq)]
pub enum UserRole {
    AppAdmin,
    ComputationalStaff,
    BiologyStaff,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct NewPerson {
    #[garde(dive)]
    pub name: NonEmptyString,
    #[garde(email)]
    pub email: String,
    #[garde(dive)]
    pub orcid: Option<NonEmptyString>,
    pub institution_id: Uuid,
    pub ms_user_id: Option<Uuid>,
    #[serde(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub roles: Vec<UserRole>,
}
impl NewPerson {
    #[must_use]
    pub fn new_user_route() -> String {
        "/users".to_string()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPerson {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn new() -> NewPersonEmpty {
        NewPersonEmpty
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
struct NewPersonEmpty;
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPersonEmpty {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn name(self, name: String) -> NewPersonName {
        NewPersonName { name }
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
struct NewPersonName {
    name: String,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPersonName {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn email(self, email: String) -> NewPersonEmail {
        NewPersonEmail { inner: self, email }
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
struct NewPersonEmail {
    inner: NewPersonName,
    email: String,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPersonEmail {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn ms_user_id(self, ms_user_id: Uuid) -> NewPersonMsUserId {
        NewPersonMsUserId {
            inner: self,
            ms_user_id,
        }
    }
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
struct NewPersonMsUserId {
    inner: NewPersonEmail,
    ms_user_id: Uuid,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPersonMsUserId {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn institution_id(self, institution_id: Uuid) -> NewPersonInstitutionId {
        NewPersonInstitutionId {
            inner: self,
            institution_id,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
struct NewPersonInstitutionId {
    inner: NewPersonMsUserId,
    institution_id: Uuid,
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl NewPersonInstitutionId {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn build(self) -> NewPerson {
        let Self {
            inner:
                NewPersonMsUserId {
                    inner:
                        NewPersonEmail {
                            inner: NewPersonName { name },
                            email,
                        },
                    ms_user_id,
                },
            institution_id,
        } = self;

        NewPerson {
            name: NonEmptyString::new(&name).unwrap(),
            email,
            orcid: None,
            institution_id,
            ms_user_id: Some(ms_user_id),
            roles: vec![],
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[pyclass]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: PersonHandle,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub email: Option<String>,
    #[pyo3(get)]
    pub orcid: Option<String>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pymethods]
impl PersonSummary {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn id(&self) -> Uuid {
        self.handle.id
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn link(&self) -> String {
        self.handle.link.to_string()
    }
}

#[db_selection]
#[pyclass]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    summary: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    #[pyo3(get)]
    institution: Institution,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pymethods]
impl PersonCore {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn id(&self) -> Uuid {
        self.summary.id()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn link(&self) -> String {
        self.summary.link()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn name(&self) -> String {
        self.summary.name.to_string()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn email(&self) -> Option<String> {
        self.summary.email.to_owned()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn orcid(&self) -> Option<String> {
        self.summary.orcid.to_owned()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[pyclass]
#[base_api_model]
pub struct Person {
    #[serde(flatten)]
    pub core: PersonCore,
    #[pyo3(get)]
    pub roles: Vec<UserRole>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pymethods]
impl Person {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn id(&self) -> Uuid {
        self.core.id()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn link(&self) -> String {
        self.core.link()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn name(&self) -> String {
        self.core.name()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn email(&self) -> Option<String> {
        self.core.email()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn orcid(&self) -> Option<String> {
        self.core.orcid()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn institution(&self) -> Institution {
        self.core.institution.clone()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[pyclass]
#[base_api_model]
pub struct CreatedUser {
    #[serde(flatten)]
    pub person: Person,
    pub api_key: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pymethods]
impl CreatedUser {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn id(&self) -> Uuid {
        self.person.id()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn link(&self) -> String {
        self.person.link()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn name(&self) -> String {
        self.person.name()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn email(&self) -> Option<String> {
        self.person.email()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn orcid(&self) -> Option<String> {
        self.person.orcid()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn roles(&self) -> Vec<UserRole> {
        self.person.roles.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    #[getter]
    pub fn institution(&self) -> Institution {
        self.person.institution()
    }
}

#[db_update]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonUpdateCore {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<NonEmptyString>,
    #[garde(email)]
    pub email: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[garde(dive)]
    pub orcid: Option<NonEmptyString>,
    pub institution_id: Option<Uuid>,
}

impl IsUpdate for PersonUpdateCore {
    fn is_update(&self) -> bool {
        matches!(
            self,
            Self {
                name: Some(_),
                email: Some(_),
                orcid: Some(_),
                institution_id: Some(_),
                ms_user_id: Some(_),
                ..
            }
        )
    }
}

#[base_api_model_with_default]
pub struct PersonUpdate {
    pub grant_roles: Vec<UserRole>,
    pub revoke_roles: Vec<UserRole>,
    #[serde(flatten)]
    #[garde(dive)]
    pub core: PersonUpdateCore,
}

#[base_api_model_with_default]
pub enum PersonOrdinalColumn {
    #[default]
    Name,
    Email,
}

#[db_query]
pub struct PersonQuery {
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub orcid: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<PersonOrdinalColumn>,
    pub pagination: Pagination,
}
