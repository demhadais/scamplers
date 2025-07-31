#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    base_api_model, base_api_model_with_default, db_enum, db_insertion, db_query, db_selection,
    db_update,
};
#[cfg(feature = "backend")]
use scamplers_schema::person;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::model::{Pagination, SortByGroup, institution::Institution};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[db_enum]
pub enum UserRole {
    AppAdmin,
    BiologyStaff,
    ComputationalStaff,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(not(target_arch = "wasm32"), json(python))]
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
    #[builder(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub roles: Vec<UserRole>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewPerson {
    #[new]
    #[pyo3(signature = (*, name, email, institution_id, roles=Vec::new()))]
    fn new(name: ValidString, email: String, institution_id: Uuid, roles: Vec<UserRole>) -> Self {
        Self {
            name,
            email,
            institution_id,
            roles,
            ms_user_id: None,
            orcid: None,
        }
    }
}

#[base_api_model]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[serde(transparent)]
pub struct NewMsLogin(#[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))] pub NewPerson);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewMsLogin {
    #[allow(clippy::new_ret_no_self)]
    #[wasm_bindgen]
    #[must_use]
    pub fn new() -> NewPersonEmpty {
        NewPersonEmpty
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NewPersonEmpty;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonEmpty {
    #[wasm_bindgen]
    #[must_use]
    pub fn name(self, name: String) -> NewPersonName {
        NewPersonName { name }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NewPersonName {
    name: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonName {
    #[wasm_bindgen]
    #[must_use]
    pub fn email(self, email: String) -> NewPersonEmail {
        NewPersonEmail { inner: self, email }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NewPersonEmail {
    inner: NewPersonName,
    email: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonEmail {
    #[wasm_bindgen]
    #[must_use]
    pub fn ms_user_id(self, ms_user_id: Uuid) -> NewPersonMsUserId {
        NewPersonMsUserId {
            inner: self,
            ms_user_id,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NewPersonMsUserId {
    inner: NewPersonEmail,
    ms_user_id: Uuid,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonMsUserId {
    #[wasm_bindgen]
    #[must_use]
    pub fn institution_id(self, institution_id: Uuid) -> NewPersonInstitutionId {
        NewPersonInstitutionId {
            inner: self,
            institution_id,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct NewPersonInstitutionId {
    inner: NewPersonMsUserId,
    institution_id: Uuid,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPersonInstitutionId {
    /// # Errors
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonHandle {
    pub id: Uuid,
    pub link: String,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: PersonHandle,
    pub name: String,
    pub email: Option<String>,
    pub orcid: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = person))]
pub struct PersonCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: PersonSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub institution: Institution,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyclass(get_all, str))]
#[base_api_model]
pub struct Person {
    #[serde(flatten)]
    pub core: PersonCore,
    pub roles: Vec<UserRole>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(inspectable, getter_with_clone))]
#[base_api_model]
pub struct CreatedUser {
    #[serde(flatten)]
    pub person: Person,
    pub api_key: String,
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[cfg_attr(feature = "python", pyclass(get_all, set_all, str))]
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_query]
pub struct PersonQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub orcid: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: SortByGroup<PersonOrdinalColumn>,
    #[builder(default)]
    pub pagination: Pagination,
}
