use scamplers_macros::db_selection;
#[cfg(feature = "backend")]
use scamplers_schema::{sequencing_run, sequencing_submissions};
use {
    crate::string::NonEmptyString, scamplers_macros::db_insertion, time::OffsetDateTime, uuid::Uuid,
};

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_submissions))]
pub struct NewSequencingSubmission {
    #[serde(default)]
    sequencing_run_id: Uuid,
    library_id: Uuid,
    fastq_paths: Vec<NonEmptyString>,
    submitted_at: OffsetDateTime,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct NewSequencingRun {
    #[garde(dive)]
    readable_id: NonEmptyString,
    begun_at: OffsetDateTime,
    finished_at: Option<OffsetDateTime>,
    #[garde(dive)]
    notes: Option<NonEmptyString>,
    #[garde(dive)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    libraries: Vec<NewSequencingSubmission>,
}
impl NewSequencingRun {
    pub fn libraries(&mut self, self_id: Uuid) -> &[NewSequencingSubmission] {
        for submission in &mut self.libraries {
            submission.sequencing_run_id = self_id;
        }

        &self.libraries
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct SequencingRunHandle {
    id: Uuid,
    link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = sequencing_run))]
pub struct SequencingRunSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    handle: SequencingRunHandle,
    readable_id: String,
    begun_at: OffsetDateTime,
    finished_at: OffsetDateTime,
    notes: Option<String>,
}
