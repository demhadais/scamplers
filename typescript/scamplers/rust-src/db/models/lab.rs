#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::gen_stub_pymethods;
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection, db_update,
};
#[cfg(feature = "app")]
use scamplers_schema::{lab, person};
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{DefaultVec, Links, Pagination, person::PersonSummary},
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;
#[cfg(feature = "app")]
mod update;

#[db_insertion]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
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
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub name: String,
    pub delivery_dir: String,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = lab, base_query=lab::table.inner_join(person::table)))]
pub struct LabSummaryWithParents {
    // We include the ID even though it's already in `LabSummary` so that we can have
    // `diesel::Identifiable` on this struct
    #[cfg_attr(feature = "app", diesel(column_name = id))]
    pub id_: Uuid,
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
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[base_model]
#[derive(Jsonify, WasmJsonify, PyJsonify)]
pub struct Lab {
    #[serde(flatten)]
    pub info: LabSummaryWithParents,
    pub members: Vec<PersonSummary>,
}

#[db_update]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::lab))]
pub struct LabUpdateFields {
    pub id: Uuid,
    #[garde(dive)]
    pub name: Option<ValidString>,
    pub pi_id: Option<Uuid>,
    #[garde(dive)]
    pub delivery_dir: Option<ValidString>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(get_all, set_all, module = "scamplepy.update")
)]
#[base_model]
#[derive(Default)]
pub struct LabUpdate {
    #[serde(flatten)]
    #[garde(dive)]
    pub fields: LabUpdateFields,
    pub add_members: Vec<Uuid>,
    pub remove_members: Vec<Uuid>,
}

define_ordering_enum! { LabOrderBy { Name }, default = Name }

#[db_query]
pub struct LabQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub names: Vec<String>,
    #[builder(default)]
    pub order_by: DefaultVec<LabOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl LabQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::default(), names=Vec::default(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        names: Vec<String>,
        order_by: DefaultVec<LabOrderBy>,
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

uuid_newtype!(LabId);
