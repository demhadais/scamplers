#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{sequencing_run, sequencing_submissions};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = sequencing_submissions))]
pub struct NewSequencingSubmission {
    #[serde(default)]
    pub sequencing_run_id: Uuid,
    pub library_id: Uuid,
    pub fastq_paths: Vec<ValidString>,
    pub submitted_at: OffsetDateTime,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSequencingSubmission {
    #[new]
    #[pyo3(signature = (*, library_id, fastq_paths, submitted_at, sequencing_run_id=Uuid::default()))]
    fn new(
        library_id: Uuid,
        fastq_paths: Vec<ValidString>,
        submitted_at: OffsetDateTime,
        sequencing_run_id: Uuid,
    ) -> Self {
        Self {
            sequencing_run_id,
            library_id,
            fastq_paths,
            submitted_at,
        }
    }
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = sequencing_run))]
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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = sequencing_run))]
pub struct SequencingRunHandle {
    pub id: Uuid,
    pub link: String,
}

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = sequencing_run))]
pub struct SequencingRunSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub handle: SequencingRunHandle,
    pub readable_id: String,
    pub begun_at: OffsetDateTime,
    pub finished_at: Option<OffsetDateTime>,
    pub notes: Option<String>,
}
