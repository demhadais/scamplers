use scamplers_macros::{db_insertion, db_selection, to_json};
#[cfg(feature = "backend")]
use scamplers_schema::{sequencing_run, sequencing_submissions};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_submissions))]
pub struct NewSequencingSubmission {
    #[serde(default)]
    pub sequencing_run_id: Uuid,
    pub library_id: Uuid,
    pub fastq_paths: Vec<ValidString>,
    pub submitted_at: OffsetDateTime,
}

#[to_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct NewSequencingRun {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub begun_at: OffsetDateTime,
    pub finished_at: Option<OffsetDateTime>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub libraries: Vec<NewSequencingSubmission>,
}

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct SequencingRunHandle {
    pub id: Uuid,
    pub link: String,
}

#[to_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct SequencingRunSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: SequencingRunHandle,
    pub readable_id: String,
    pub begun_at: OffsetDateTime,
    pub finished_at: OffsetDateTime,
    pub notes: Option<String>,
}
