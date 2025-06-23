use {crate::string::NonEmptyString, time::OffsetDateTime, uuid::Uuid};
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_insertion, backend_with_getters},
    scamplers_schema::dataset_metadata,
};

#[cfg_attr(feature = "backend", backend_insertion(dataset_metadata))]
pub struct NewDatasetMetadata {
    #[cfg_attr(feature = "backend", garde(dive))]
    name: NonEmptyString,
    lab_id: Uuid,
    data_path: NonEmptyString,
    #[cfg_attr(feature = "backend", valuable(skip))]
    delivered_at: OffsetDateTime,
}

#[cfg_attr(feature = "backend", backend_with_getters)]
mod with_getters {
    use crate::model::lab::LabSummary;
    use time::OffsetDateTime;
    use uuid::Uuid;

    #[cfg(feature = "backend")]
    use {scamplers_macros::backend_selection, scamplers_schema::dataset_metadata};

    #[cfg_attr(feature = "backend", backend_selection(dataset_metadata))]
    pub struct DatasetMetadataSummary {
        #[cfg_attr(feature = "backend", serde(skip))]
        id: Uuid,
        name: String,
        data_path: String,
        #[cfg_attr(feature = "backend", valuable(skip))]
        delivered_at: OffsetDateTime,
    }

    #[cfg_attr(feature = "backend", backend_selection(dataset_metadata))]
    pub struct DatasetMetadata {
        #[cfg_attr(feature = "backend", serde(flatten), diesel(embed))]
        summary: DatasetMetadataSummary,
        #[cfg_attr(feature = "backend", serde(flatten), diesel(embed))]
        lab: LabSummary,
    }
}
pub use with_getters::*;
