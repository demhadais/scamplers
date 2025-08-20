#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::db_insertion;
use time::OffsetDateTime;
use uuid::Uuid;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::sequencing_submissions))]
pub struct NewSequencingSubmission {
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
