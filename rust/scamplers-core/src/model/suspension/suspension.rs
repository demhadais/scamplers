use scamplers_macros::{db_enum, db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{
    multiplexing_tag, suspension, suspension_measurement, suspension_preparers,
};
use valid_string::ValidString;
use {
    crate::model::{
        person::PersonHandle,
        specimen::SpecimenSummary,
        suspension::common::{BiologicalMaterial, MeasurementDataCore},
    },
    time::OffsetDateTime,
    uuid::Uuid,
};

#[db_enum]
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

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexing_tag))]
pub struct NewMultiplexingTag {
    pub tag_id: String,
    pub type_: MultiplexingTagType,
}

#[db_json]
pub struct SuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: MeasurementDataCore,
    pub is_post_hybridization: bool,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_measurement))]
pub struct NewSuspensionMeasurement {
    #[serde(default)]
    pub suspension_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: SuspensionMeasurementData,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct NewSuspension {
    pub readable_id: ValidString,
    pub parent_specimen_id: Uuid,
    pub biological_material: BiologicalMaterial,
    pub created_at: Option<OffsetDateTime>,
    pub pooled_into_id: Option<Uuid>,
    pub multiplexing_tag_id: Option<Uuid>,
    #[garde(range(min = 0.0))]
    pub lysis_duration_minutes: Option<f32>,
    #[garde(range(min = 0.0))]
    pub target_cell_recovery: f32,
    #[garde(range(min = 0))]
    pub target_reads_per_cell: i32,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionMeasurement>,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_preparers))]
pub struct SuspensionPreparer {
    pub suspension_id: Uuid,
    pub prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: SuspensionHandle,
    pub readable_id: String,
    pub biological_material: String,
    pub created_at: Option<OffsetDateTime>,
    pub lysis_duration_minutes: Option<f32>,
    pub target_cell_recovery: f32,
    pub target_reads_per_cell: i32,
    pub notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexing_tag))]
pub struct MultiplexingTag {
    pub id: Uuid,
    pub tag_id: String,
    pub type_: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub summary: SuspensionSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub parent_specimen: SpecimenSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub multiplexing_tag: MultiplexingTag,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_measurement))]
pub struct SuspensionMeasurement {
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub measured_by: PersonHandle,
    #[serde(flatten)]
    pub data: SuspensionMeasurementData,
}

#[derive(serde::Serialize)]
pub struct Suspension {
    #[serde(flatten)]
    pub core: SuspensionCore,
    pub preparers: Vec<PersonHandle>,
    pub measurements: Vec<SuspensionMeasurement>,
}
