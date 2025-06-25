use {crate::string::NonEmptyString, time::OffsetDateTime, uuid::Uuid};
#[cfg(feature = "backend")]
use {
    scamplers_macros::backend_insertion,
    scamplers_schema::{sequencing_run, sequencing_submissions},
};

#[cfg_attr(feature = "backend", backend_insertion(sequencing_submissions))]
struct NewSequencingSubmission {
    #[cfg_attr(feature = "backend", serde(default))]
    sequencing_run_id: Uuid,
    library_id: Uuid,
    fastq_paths: Vec<NonEmptyString>,
    #[cfg_attr(feature = "backend", valuable(skip))]
    submitted_at: OffsetDateTime,
}

#[cfg_attr(feature = "backend", backend_insertion(sequencing_run))]
pub struct NewSequencingRun {
    #[cfg_attr(feature = "backend", garde(dive))]
    readable_id: NonEmptyString,
    #[cfg_attr(feature = "backend", valuable(skip))]
    begun_at: OffsetDateTime,
    #[cfg_attr(feature = "backend", valuable(skip))]
    finished_at: Option<OffsetDateTime>,
    #[cfg_attr(feature = "backend", garde(dive))]
    notes: Option<NonEmptyString>,
    #[cfg_attr(feature = "backend", garde(dive), diesel(skip_insertion))]
    libraries: Vec<NewSequencingSubmission>,
}
