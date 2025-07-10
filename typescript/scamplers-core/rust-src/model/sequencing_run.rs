use {crate::string::ValidString, time::OffsetDateTime, uuid::Uuid};
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_insertion, backend_with_getters},
    scamplers_schema::{sequencing_run, sequencing_submissions},
};

#[cfg_attr(feature = "backend", backend_insertion(sequencing_submissions))]
pub struct NewSequencingSubmission {
    #[cfg_attr(feature = "backend", serde(default))]
    sequencing_run_id: Uuid,
    library_id: Uuid,
    fastq_paths: Vec<ValidString>,
    #[cfg_attr(feature = "backend", valuable(skip))]
    submitted_at: OffsetDateTime,
}

#[cfg_attr(feature = "backend", backend_insertion(sequencing_run))]
pub struct NewSequencingRun {
    #[cfg_attr(feature = "backend", garde(dive))]
    readable_id: ValidString,
    #[cfg_attr(feature = "backend", valuable(skip))]
    begun_at: OffsetDateTime,
    #[cfg_attr(feature = "backend", valuable(skip))]
    finished_at: Option<OffsetDateTime>,
    #[cfg_attr(feature = "backend", garde(dive))]
    notes: Option<ValidString>,
    #[cfg_attr(feature = "backend", garde(dive), diesel(skip_insertion))]
    libraries: Vec<NewSequencingSubmission>,
}
impl NewSequencingRun {
    pub fn libraries(&mut self, sequencing_run_id: Uuid) -> &[NewSequencingSubmission] {
        for submission in &mut self.libraries {
            submission.sequencing_run_id = sequencing_run_id;
        }

        &self.libraries
    }
}

#[cfg_attr(feature = "backend", backend_with_getters)]
mod with_getters {
    use time::OffsetDateTime;
    use uuid::Uuid;
    #[cfg(feature = "backend")]
    use {scamplers_macros::backend_selection, scamplers_schema::sequencing_run};

    #[cfg_attr(feature = "backend", backend_selection(sequencing_run))]
    pub struct SequencingRunHandle {
        id: Uuid,
        link: String,
    }

    #[cfg_attr(feature = "backend", backend_selection(sequencing_run))]
    pub struct SequencingRunSummary {
        #[cfg_attr(feature = "backend", diesel(embed), serde(flatten))]
        handle: SequencingRunHandle,
        readable_id: String,
        #[cfg_attr(feature = "backend", valuable(skip))]
        begun_at: OffsetDateTime,
        #[cfg_attr(feature = "backend", valuable(skip))]
        finished_at: OffsetDateTime,
        notes: Option<String>,
    }
}
pub use with_getters::*;
