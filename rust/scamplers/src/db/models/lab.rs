#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_model, db_insertion, db_query, db_selection, db_update};
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{Links, Pagination, person::PersonSummary},
    define_ordering_enum, impl_order_by, impl_python_order_by, impl_wasm_order_by, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;
#[cfg(feature = "app")]
mod update;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct NewLab {
    #[garde(dive)]
    pub name: ValidString,
    pub pi_id: Uuid,
    #[garde(dive)]
    pub delivery_dir: ValidString,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    #[builder(default)]
    pub member_ids: Vec<Uuid>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewLab {
    #[new]
    #[pyo3(signature = (*, name, pi_id, delivery_dir, member_ids=Vec::new()))]
    fn new(
        name: ValidString,
        pi_id: Uuid,
        delivery_dir: ValidString,
        member_ids: Vec<Uuid>,
    ) -> Self {
        Self {
            name,
            pi_id,
            delivery_dir,
            member_ids,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct LabSummary {
    pub id: Uuid,
    pub links: Links,
    pub name: String,
    pub delivery_dir: String,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct LabCore {
    // We include the ID even though it's already in `LabSummary` so that we can have `diesel::Identifiable` on this struct. Consequently, we skip serializing it since it's already in `LabSummary`
    #[serde(skip)]
    pub id: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: LabSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub pi: PersonSummary,
}

#[cfg_attr(
    target_arch = "wasm32",
    wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(feature = "python", pyclass(get_all))]
#[base_model]
pub struct Lab {
    #[serde(flatten)]
    pub core: LabCore,
    pub members: Vec<PersonSummary>,
}

#[db_update]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct LabUpdateCore {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<ValidString>,
    pub pi_id: Option<Uuid>,
    #[garde(dive)]
    pub delivery_dir: Option<ValidString>,
}

#[cfg_attr(feature = "python", pyclass(get_all, set_all))]
#[base_model]
#[derive(Default)]
pub struct LabUpdate {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: LabUpdateCore,
    pub add_members: Vec<Uuid>,
    pub remove_members: Vec<Uuid>,
}

define_ordering_enum! {LabOrderBy { Name }, default = Name}

#[db_query]
pub struct LabQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub name: Option<String>,
    #[builder(default)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub order_by: Vec<LabOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

impl_order_by!(LabQuery);
impl_wasm_order_by!(LabQuery);
impl_python_order_by!(LabQuery);

uuid_newtype!(LabId);
