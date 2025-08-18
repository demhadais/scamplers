#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection,
    db_simple_enum, db_update,
};
#[cfg(feature = "app")]
use scamplers_schema::person;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{DefaultVec, Links, Pagination, institution::Institution},
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;
#[cfg(feature = "app")]
mod update;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum UserRole {
    AppAdmin,
    BiologyStaff,
    ComputationalStaff,
}

#[db_insertion]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone, setter))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::person))]
pub struct NewPerson {
    #[garde(dive)]
    pub name: ValidString,
    #[garde(email)]
    pub email: String,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[garde(dive)]
    pub orcid: Option<ValidString>,
    pub institution_id: Uuid,
    pub ms_user_id: Option<Uuid>,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    #[serde(default)]
    #[builder(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub roles: Vec<UserRole>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPerson {
    #[wasm_bindgen(constructor)]
    pub fn new(ms_user_id: Uuid) -> Self {
        Self {
            name: ValidString::new(),
            email: String::new(),
            orcid: None,
            institution_id: Uuid::nil(),
            ms_user_id: Some(ms_user_id),
            roles: vec![],
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = person))]
pub struct PersonSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub name: String,
    pub email: Option<String>,
    pub orcid: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = person))]
pub struct PersonSummaryWithRelations {
    #[cfg_attr(feature = "app", diesel(column_name = id))]
    pub id_: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: PersonSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub institution: Institution,
}

uuid_newtype!(PersonId);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(feature = "python", pyclass(get_all, module = "scamplepy.responses"))]
#[base_model]
#[derive(Jsonify, WasmJsonify, PyJsonify)]
pub struct Person {
    #[serde(flatten)]
    pub info: PersonSummaryWithRelations,
    pub roles: Vec<UserRole>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[base_model]
pub struct CreatedUser {
    #[serde(flatten)]
    pub person: Person,
    pub api_key: String,
}

#[db_update]
#[cfg_attr(feature = "app", diesel(table_name = person))]
pub struct PersonUpdateFields {
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

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, module = "scamplepy.update")
)]
#[base_model]
#[derive(Default)]
pub struct PersonUpdate {
    pub grant_roles: Vec<UserRole>,
    pub revoke_roles: Vec<UserRole>,
    #[serde(flatten)]
    #[garde(dive)]
    pub fields: PersonUpdateFields,
}

define_ordering_enum! { PersonOrderBy { Name, Email }, default = Name }

#[db_query]
pub struct PersonQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub orcid: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[builder(default)]
    pub order_by: DefaultVec<PersonOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}
