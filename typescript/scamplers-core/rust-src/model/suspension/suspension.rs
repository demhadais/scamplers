use {
    crate::{
        model::suspension::common::{BiologicalMaterial, MeasurementDataCore},
        string::NonEmptyString,
    },
    time::OffsetDateTime,
    uuid::Uuid,
};
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_enum, backend_db_json, backend_insertion},
    scamplers_schema::multiplexing_tag,
    scamplers_schema::{suspension, suspension_measurement, suspension_preparers},
};

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum MultiplexingTagType {
    FlexBarcode,
    OnChipMultiplexing,
    #[cfg_attr(feature = "backend", serde(rename = "TotalSeqA"))]
    TotalSeqA,
    #[cfg_attr(feature = "backend", serde(rename = "TotalSeqB"))]
    TotalSeqB,
    #[cfg_attr(feature = "backend", serde(rename = "TotalSeqC"))]
    TotalSeqC,
}

#[cfg_attr(feature = "backend", backend_insertion(multiplexing_tag))]
pub struct NewMultiplexingTag {
    tag_id: String,
    type_: MultiplexingTagType,
}

#[cfg_attr(feature = "backend", backend_db_json)]
pub struct MeasurementData {
    #[cfg_attr(feature = "backend", serde(flatten), garde(dive))]
    data: MeasurementDataCore,
    is_post_hybridization: bool,
}

#[cfg_attr(feature = "backend", backend_insertion(suspension_measurement))]
pub struct NewSuspensionMeasurement {
    #[cfg_attr(feature = "backend", serde(default))]
    suspension_id: Uuid,
    measured_by: Uuid,
    #[cfg_attr(feature = "backend", garde(dive), serde(flatten))]
    data: MeasurementData,
}

#[cfg_attr(feature = "backend", backend_insertion(suspension))]
pub struct NewSuspension {
    readable_id: NonEmptyString,
    parent_specimen_id: Uuid,
    biological_material: BiologicalMaterial,
    #[cfg_attr(feature = "backend", valuable(skip))]
    created_at: Option<OffsetDateTime>,
    pooled_into_id: Option<Uuid>,
    multiplexing_tag_id: Option<Uuid>,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
    lysis_duration_min: Option<f32>,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
    target_cell_recovery: f32,
    #[cfg_attr(feature = "backend", garde(range(min = 0)))]
    target_reads_per_cell: i32,
    #[cfg_attr(feature = "backend", garde(dive))]
    notes: Option<NonEmptyString>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    preparer_ids: Vec<Uuid>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion), garde(dive))]
    measurements: Vec<NewSuspensionMeasurement>,
}

#[cfg_attr(feature = "backend", backend_insertion(suspension_preparers))]
pub struct SuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}
