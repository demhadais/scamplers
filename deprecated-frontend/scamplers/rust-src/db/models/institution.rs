#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_query, db_selection};
use uuid::Uuid;
use valid_string::ValidString;

use crate::{
    db::models::{DefaultVec, Links, Pagination},
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;
#[cfg(feature = "app")]
mod update;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::institution))]
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
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::institution))]
pub struct Institution {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub name: String,
}

define_ordering_enum! { InstitutionOrderBy { Name }, default = Name }

#[db_query]
pub struct InstitutionQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub order_by: DefaultVec<InstitutionOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[pymethods]
impl InstitutionQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::default(), names=Vec::default(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        names: Vec<String>,
        order_by: DefaultVec<InstitutionOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            names,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}
