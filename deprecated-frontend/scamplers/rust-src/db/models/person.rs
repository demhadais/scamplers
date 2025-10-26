#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection,
    db_simple_enum, db_update,
};
#[cfg(feature = "app")]
use scamplers_schema::{institution, person};
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
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
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
#[pymethods]
impl NewPerson {
    #[new]
    #[pyo3(signature = (*, name, email, institution_id, ms_user_id=None, roles=Vec::new()))]
    fn new(
        name: ValidString,
        email: String,
        institution_id: Uuid,
        ms_user_id: Option<Uuid>,
        roles: Vec<UserRole>,
    ) -> Self {
        Self {
            name,
            email,
            institution_id,
            ms_user_id,
            roles,
            orcid: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl NewPerson {
    #[wasm_bindgen(constructor)]
    #[must_use]
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
#[cfg_attr(feature = "app", diesel(table_name = person, base_query = person::table.inner_join(institution::table)))]
pub struct PersonSummaryWithParents {
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
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[base_model]
#[derive(Jsonify, WasmJsonify, PyJsonify)]
pub struct Person {
    #[serde(flatten)]
    pub info: PersonSummaryWithParents,
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
pub struct PersonUpdate {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<ValidString>,
    #[garde(email)]
    pub email: Option<String>,
    pub ms_user_id: Option<Uuid>,
    #[garde(dive)]
    pub orcid: Option<ValidString>,
    pub institution_id: Option<Uuid>,
    #[builder(default)]
    #[cfg_attr(feature = "app", diesel(skip_update))]
    pub grant_roles: Vec<UserRole>,
    #[builder(default)]
    #[cfg_attr(feature = "app", diesel(skip_update))]
    pub revoke_roles: Vec<UserRole>,
}

define_ordering_enum! { PersonOrderBy { Name, Email }, default = Name }

#[db_query]
pub struct PersonQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub emails: Vec<String>,
    #[builder(default)]
    pub orcids: Vec<String>,
    #[builder(default)]
    pub ms_user_ids: Vec<Uuid>,
    #[builder(default)]
    pub order_by: DefaultVec<PersonOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[pymethods]
impl PersonQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::new(), names=Vec::new(), emails=Vec::new(), orcids=Vec::new(), ms_user_ids=Vec::new(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        names: Vec<String>,
        emails: Vec<String>,
        orcids: Vec<String>,
        ms_user_ids: Vec<Uuid>,
        order_by: DefaultVec<PersonOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            names,
            emails,
            orcids,
            ms_user_ids,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}
