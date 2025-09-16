#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::gen_stub_pymethods;
use scamplers_macros::{
    Jsonify, PyJsonify, WasmJsonify, base_model, db_insertion, db_query, db_selection,
};
use time::OffsetDateTime;
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
#[cfg_attr(
    feature = "app",
    derive(Identifiable, Associations, Selectable, Queryable)
)]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::sequencing_submissions, primary_key(sequencing_run_id, library_id), belongs_to(SequencingRunSummary, foreign_key = sequencing_run_id)))]
pub struct NewSequencingSubmission {
    #[builder(default)]
    #[serde(default)]
    pub sequencing_run_id: Uuid,
    pub library_id: Uuid,
    pub submitted_at: OffsetDateTime,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSequencingSubmission {
    #[new]
    #[pyo3(signature = (*, library_id, submitted_at, sequencing_run_id=Uuid::default()))]
    fn new(library_id: Uuid, submitted_at: OffsetDateTime, sequencing_run_id: Uuid) -> Self {
        Self {
            sequencing_run_id,
            library_id,
            submitted_at,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::sequencing_run))]
pub struct NewSequencingRun {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub begun_at: OffsetDateTime,
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub libraries: Vec<NewSequencingSubmission>,
    pub finished_at: Option<OffsetDateTime>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSequencingRun {
    #[new]
    #[pyo3(signature = (*, readable_id, begun_at, libraries, finished_at=None, notes=None))]
    fn new(
        readable_id: ValidString,
        begun_at: OffsetDateTime,
        libraries: Vec<NewSequencingSubmission>,
        finished_at: Option<OffsetDateTime>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            readable_id,
            begun_at,
            libraries,
            finished_at,
            notes,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::sequencing_run))]
pub struct SequencingRunSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub begun_at: OffsetDateTime,
    pub finished_at: Option<OffsetDateTime>,
    pub notes: Option<String>,
}

#[base_model]
#[cfg_attr(
    target_arch = "wasm32",
    ::wasm_bindgen::prelude::wasm_bindgen(getter_with_clone)
)]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[derive(Jsonify, WasmJsonify, PyJsonify)]
pub struct SequencingRun {
    #[serde(flatten)]
    pub summary: SequencingRunSummary,
    pub libraries: Vec<Uuid>,
}

define_ordering_enum! { SequencingRunOrderBy { BegunAt, FinishedAt, ReadableId }, default = BegunAt }

#[db_query]
pub struct SequencingRunQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub order_by: DefaultVec<SequencingRunOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl SequencingRunQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::default(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default().offset))]
    fn new(
        ids: Vec<Uuid>,
        order_by: DefaultVec<SequencingRunOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}

uuid_newtype!(SequencingRunId);
