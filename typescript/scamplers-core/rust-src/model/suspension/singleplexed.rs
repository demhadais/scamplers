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
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_enum, backend_db_json, backend_insertion, backend_selection},
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
pub struct SuspensionMeasurementData {
    #[cfg_attr(feature = "backend", serde(flatten), garde(dive))]
    core: MeasurementDataCore,
    is_post_hybridization: bool,
}

#[cfg_attr(feature = "backend", backend_insertion(suspension_measurement))]
pub struct NewSuspensionMeasurement {
    #[cfg_attr(feature = "backend", serde(default))]
    suspension_id: Uuid,
    measured_by: Uuid,
    #[cfg_attr(feature = "backend", garde(dive), serde(flatten))]
    data: SuspensionMeasurementData,
}
impl NewSuspensionMeasurement {
    #[must_use]
    pub fn suspension_id(&self) -> &Uuid {
        &self.suspension_id
    }
}

#[cfg_attr(feature = "backend", backend_insertion(suspension))]
pub struct NewSuspension {
    readable_id: NonEmptyString,
    parent_specimen_id: Uuid,
    biological_material: BiologicalMaterial,
    #[cfg_attr(feature = "backend", valuable(skip))]
    created_at: Option<OffsetDateTime>,
    pub(super) pooled_into_id: Option<Uuid>,
    multiplexing_tag_id: Option<Uuid>,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
    lysis_duration_minutes: Option<f32>,
    #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
    target_cell_recovery: f32,
    #[cfg_attr(feature = "backend", garde(range(min = 0)))]
    target_reads_per_cell: i32,
    #[cfg_attr(feature = "backend", garde(dive))]
    notes: Option<NonEmptyString>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    preparer_ids: Vec<Uuid>,
    #[cfg_attr(
        feature = "backend",
        diesel(skip_insertion),
        garde(dive),
        serde(default)
    )]
    measurements: Vec<NewSuspensionMeasurement>,
}

#[cfg_attr(feature = "backend", backend_insertion(suspension_preparers))]
pub struct SingleplexedSuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}

impl NewSuspension {
    #[must_use]
    pub fn preparers(&self, suspension_id: Uuid) -> Vec<SingleplexedSuspensionPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| SingleplexedSuspensionPreparer {
                prepared_by: *p,
                suspension_id,
            })
            .collect()
    }

    pub fn measurements(&mut self, suspension_id: Uuid) -> &[NewSuspensionMeasurement] {
        for measurement in &mut self.measurements {
            measurement.suspension_id = suspension_id;
        }

        &self.measurements
    }
}

#[cfg_attr(feature = "backend", backend_selection(suspension))]
pub struct SuspensionHandle {
    id: Uuid,
    link: String,
}

#[cfg_attr(feature = "backend", backend_selection(suspension))]
pub struct SuspensionSummary {
    #[cfg_attr(feature = "backend", diesel(embed), serde(flatten))]
    handle: SuspensionHandle,
    readable_id: String,
    biological_material: String,
    #[cfg_attr(feature = "backend", valuable(skip))]
    created_at: Option<OffsetDateTime>,
    lysis_duration_minutes: Option<f32>,
    target_cell_recovery: f32,
    target_reads_per_cell: i32,
    notes: Option<String>,
}

#[cfg_attr(feature = "backend", backend_selection(multiplexing_tag))]
pub struct MultiplexingTag {
    id: Uuid,
    tag_id: String,
    type_: String,
}

#[cfg_attr(feature = "backend", backend_selection(suspension))]
pub struct SuspensionCore {
    #[cfg_attr(feature = "backend", diesel(embed), serde(flatten))]
    summary: SuspensionSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    parent_specimen: SpecimenSummary,
    #[cfg_attr(feature = "backend", diesel(embed))]
    multiplexing_tag: MultiplexingTag,
}

#[cfg_attr(feature = "backend", backend_selection(suspension_measurement))]
pub struct SuspensionMeasurement {
    #[cfg_attr(feature = "backend", diesel(embed))]
    measured_by: PersonHandle,
    #[cfg_attr(feature = "backend", serde(flatten))]
    data: SuspensionMeasurementData,
}

#[cfg_attr(feature = "backend", derive(serde::Serialize, bon::Builder))]
pub struct Suspension {
    #[cfg_attr(feature = "backend", serde(flatten))]
    core: SuspensionCore,
    preparers: Vec<PersonHandle>,
    measurements: Vec<SuspensionMeasurement>,
}
