#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_query, db_selection};
#[cfg(feature = "app")]
use scamplers_schema::institution;
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::models::{Jsonify, Pagination},
    define_ordering_enum, impl_order_by, impl_python_order_by, impl_wasm_order_by, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;
#[cfg(feature = "app")]
mod update;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = institution))]
pub struct NewInstitution {
    pub id: Uuid,
    #[garde(dive)]
    pub name: ValidString,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewInstitution {
    #[new]
    #[pyo3(signature = (*, id, name))]
    fn new(id: Uuid, name: ValidString) -> Self {
        Self { id, name }
    }
}

uuid_newtype!(InstitutionId);

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = institution))]
pub struct InstitutionHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = institution))]
pub struct Institution {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub handle: InstitutionHandle,
    pub name: String,
}

define_ordering_enum!(InstitutionOrderBy; Name; default = Name;);

#[db_query]
pub struct InstitutionQuery {
    #[serde(alias = "id")]
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    #[builder(default)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: Vec<InstitutionOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

impl_order_by!(InstitutionQuery);
impl_wasm_order_by!(InstitutionQuery);
impl_python_order_by!(InstitutionQuery);
