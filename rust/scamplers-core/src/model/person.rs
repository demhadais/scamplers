use crate::model::{Pagination, SortByGroup, institution::Institution};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_enum, db_insertion, db_query, db_selection,
    db_update, getters_impl,
};
#[cfg(feature = "backend")]
use scamplers_schema::person;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[db_enum]
#[derive(PartialEq)]
pub enum UserRole {
    AppAdmin,
    ComputationalStaff,
    BiologyStaff,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct NewPerson {
    #[garde(dive)]
    pub name: ValidString,
    #[garde(email)]
    pub email: String,
    #[garde(dive)]
    pub orcid: Option<ValidString>,
    pub institution_id: Uuid,
    pub ms_user_id: Option<Uuid>,
    #[serde(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub roles: Vec<UserRole>,
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[serde(transparent)]
pub struct NewMsLogin(#[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))] pub NewPerson);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewMsLogin {
    #[wasm_bindgen]
    pub fn new() -> NewPersonEmpty {
        NewPersonEmpty
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct NewPersonEmpty;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonEmpty {
    #[wasm_bindgen]
    pub fn name(self, name: String) -> NewPersonName {
        NewPersonName { name }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct NewPersonName {
    name: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonName {
    #[wasm_bindgen]
    pub fn email(self, email: String) -> NewPersonEmail {
        NewPersonEmail { inner: self, email }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct NewPersonEmail {
    inner: NewPersonName,
    email: String,
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonEmail {
    #[wasm_bindgen]
    pub fn ms_user_id(self, ms_user_id: Uuid) -> NewPersonMsUserId {
        NewPersonMsUserId {
            inner: self,
            ms_user_id,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct NewPersonMsUserId {
    inner: NewPersonEmail,
    ms_user_id: Uuid,
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonMsUserId {
    #[wasm_bindgen]
    pub fn institution_id(self, institution_id: Uuid) -> NewPersonInstitutionId {
        NewPersonInstitutionId {
            inner: self,
            institution_id,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
struct NewPersonInstitutionId {
    inner: NewPersonMsUserId,
    institution_id: Uuid,
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonInstitutionId {
    #[wasm_bindgen]
    pub fn build(self) -> std::result::Result<NewMsLogin, valid_string::EmptyStringError> {
        use std::str::FromStr;

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

        Ok(NewMsLogin(NewPerson {
            name: ValidString::from_str(&name)?,
            email,
            orcid: None,
            institution_id,
            ms_user_id: Some(ms_user_id),
            roles: vec![],
        }))
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonHandle {
    #[cfg_attr(feature = "python", pyo3(get))]
    pub id: Uuid,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: PersonHandle,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub name: String,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub email: Option<String>,
    #[cfg_attr(feature = "python", pyo3(get))]
    pub orcid: Option<String>,
}

#[getters_impl]
impl PersonSummary {
    pub fn id(&self) -> Uuid {
        self.handle.id
    }

    pub fn link(&self) -> String {
        self.handle.link.to_string()
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    #[cfg_attr(feature = "python", pyo3(get))]
    pub institution: Institution,
}

#[getters_impl]
impl PersonCore {
    pub fn id(&self) -> Uuid {
        self.summary.id()
    }

    pub fn link(&self) -> String {
        self.summary.link()
    }

    pub fn name(&self) -> String {
        self.summary.name.to_string()
    }

    pub fn email(&self) -> Option<String> {
        self.summary.email.to_owned()
    }

    pub fn orcid(&self) -> Option<String> {
        self.summary.orcid.to_owned()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass)]
#[base_api_model]
pub struct Person {
    #[serde(flatten)]
    pub core: PersonCore,
    pub roles: Vec<UserRole>,
}

#[getters_impl]
impl Person {
    pub fn id(&self) -> Uuid {
        self.core.id()
    }

    pub fn link(&self) -> String {
        self.core.link()
    }

    pub fn name(&self) -> String {
        self.core.name()
    }

    pub fn email(&self) -> Option<String> {
        self.core.email()
    }

    pub fn orcid(&self) -> Option<String> {
        self.core.orcid()
    }

    pub fn institution(&self) -> Institution {
        self.core.institution.clone()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[base_api_model]
pub struct CreatedUser {
    #[serde(flatten)]
    pub person: Person,
    pub api_key: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CreatedUser {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn id(&self) -> Uuid {
        self.person.id()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn link(&self) -> String {
        self.person.link()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn name(&self) -> String {
        self.person.name()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn email(&self) -> Option<String> {
        self.person.email()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn orcid(&self) -> Option<String> {
        self.person.orcid()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn roles(&self) -> Vec<UserRole> {
        self.person.roles.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
    pub fn institution(&self) -> Institution {
        self.person.institution()
    }
}

#[db_update]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonUpdateCore {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<ValidString>,
    #[garde(email)]
    pub email: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[garde(dive)]
    pub orcid: Option<ValidString>,
    pub institution_id: Option<Uuid>,
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
