use scamplers_macros::{db_enum, db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{
    multiplexing_tag, suspension, suspension_measurement, suspension_preparers,
};
use {
    crate::{
        model::{
            person::PersonHandle,
            specimen::SpecimenSummary,
            suspension::common::{BiologicalMaterial, MeasurementDataCore},
        },
        string::NonEmptyString,
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
    tag_id: String,
    type_: MultiplexingTagType,
}

#[db_json]
pub struct SuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    core: MeasurementDataCore,
    is_post_hybridization: bool,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_measurement))]
pub struct NewSuspensionMeasurement {
    #[serde(default)]
    suspension_id: Uuid,
    measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    data: SuspensionMeasurementData,
}

#[db_insertion]
#[derive(getset::Setters)]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct NewSuspension {
    readable_id: NonEmptyString,
    parent_specimen_id: Uuid,
    biological_material: BiologicalMaterial,
    created_at: Option<OffsetDateTime>,
    #[getset(set = "pub(super)")]
    pooled_into_id: Option<Uuid>,
    multiplexing_tag_id: Option<Uuid>,
    #[garde(range(min = 0.0))]
    lysis_duration_minutes: Option<f32>,
    #[garde(range(min = 0.0))]
    target_cell_recovery: f32,
    #[garde(range(min = 0))]
    target_reads_per_cell: i32,
    #[garde(dive)]
    notes: Option<NonEmptyString>,
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    measurements: Vec<NewSuspensionMeasurement>,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_preparers))]
pub struct SingleplexedSuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}

impl NewSuspension {
    #[must_use]
    pub fn preparers(&self, self_id: Uuid) -> Vec<SingleplexedSuspensionPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| SingleplexedSuspensionPreparer {
                prepared_by: *p,
                suspension_id: self_id,
            })
            .collect()
    }

    pub fn measurements(&mut self, self_id: Uuid) -> &[NewSuspensionMeasurement] {
        for measurement in &mut self.measurements {
            measurement.suspension_id = self_id;
        }

        &self.measurements
    }
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionHandle {
    id: Uuid,
    link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    handle: SuspensionHandle,
    readable_id: String,
    biological_material: String,
    created_at: Option<OffsetDateTime>,
    lysis_duration_minutes: Option<f32>,
    target_cell_recovery: f32,
    target_reads_per_cell: i32,
    notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexing_tag))]
pub struct MultiplexingTag {
    id: Uuid,
    tag_id: String,
    type_: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension))]
pub struct SuspensionCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    summary: SuspensionSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    parent_specimen: SpecimenSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    multiplexing_tag: MultiplexingTag,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_measurement))]
pub struct SuspensionMeasurement {
    #[cfg_attr(feature = "backend", diesel(embed))]
    measured_by: PersonHandle,
    #[serde(flatten)]
    data: SuspensionMeasurementData,
}

#[derive(serde::Serialize)]
pub struct Suspension {
    #[serde(flatten)]
    core: SuspensionCore,
    preparers: Vec<PersonHandle>,
    measurements: Vec<SuspensionMeasurement>,
}
