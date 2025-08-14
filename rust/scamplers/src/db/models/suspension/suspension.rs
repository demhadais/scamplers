#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_model, db_insertion, db_json, db_selection};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::db::models::{
    Links,
    multiplexing_tag::MultiplexingTag,
    person::PersonSummary,
    specimen::SpecimenSummary,
    suspension::common::{BiologicalMaterial, MeasurementDataCore},
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(name = "_SuspensionMeasurementData"))]
pub struct SuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub core: MeasurementDataCore,
    pub is_post_hybridization: bool,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_measurement))]
pub struct NewSuspensionMeasurement {
    #[serde(default)]
    #[builder(default)]
    pub suspension_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: SuspensionMeasurementData,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSuspensionMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, is_post_hybridization, suspension_id=Uuid::default()))]
    fn new(
        measured_by: Uuid,
        data: MeasurementDataCore,
        is_post_hybridization: bool,
        suspension_id: Uuid,
    ) -> Self {
        Self {
            suspension_id,
            measured_by,
            data: SuspensionMeasurementData {
                core: data,
                is_post_hybridization,
            },
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension))]
pub struct NewSuspension {
    pub readable_id: ValidString,
    pub parent_specimen_id: Uuid,
    pub biological_material: BiologicalMaterial,
    #[garde(range(min = 0.0))]
    pub target_cell_recovery: f32,
    #[garde(range(min = 0))]
    pub target_reads_per_cell: i32,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionMeasurement>,
    pub created_at: Option<OffsetDateTime>,
    pub pooled_into_id: Option<Uuid>,
    pub multiplexing_tag_id: Option<Uuid>,
    #[garde(range(min = 0.0))]
    pub lysis_duration_minutes: Option<f32>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSuspension {
    #[new]
    #[pyo3(signature = (*, readable_id, parent_specimen_id, biological_material, target_cell_recovery, target_reads_per_cell, preparer_ids, measurements=Vec::new(), created_at=None, pooled_into_id=None, multiplexing_tag_id=None,lysis_duration_minutes=None,notes=None))]
    fn new(
        readable_id: ValidString,
        parent_specimen_id: Uuid,
        biological_material: BiologicalMaterial,
        target_cell_recovery: f32,
        target_reads_per_cell: i32,
        preparer_ids: Vec<Uuid>,
        measurements: Vec<NewSuspensionMeasurement>,
        created_at: Option<OffsetDateTime>,
        pooled_into_id: Option<Uuid>,
        multiplexing_tag_id: Option<Uuid>,
        lysis_duration_minutes: Option<f32>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            readable_id,
            parent_specimen_id,
            biological_material,
            target_cell_recovery,
            target_reads_per_cell,
            preparer_ids,
            measurements,
            created_at,
            pooled_into_id,
            multiplexing_tag_id,
            lysis_duration_minutes,
            notes,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension))]
pub struct SuspensionSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub biological_material: String,
    pub created_at: Option<OffsetDateTime>,
    pub lysis_duration_minutes: Option<f32>,
    pub target_cell_recovery: f32,
    pub target_reads_per_cell: i32,
    pub notes: Option<String>,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension))]
pub struct SuspensionCore {
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: SuspensionSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub parent_specimen: SpecimenSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub multiplexing_tag: MultiplexingTag,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_measurement))]
pub struct SuspensionMeasurement {
    #[cfg_attr(feature = "app", diesel(embed))]
    pub measured_by: PersonSummary,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub data: SuspensionMeasurementData,
}

#[cfg_attr(feature = "python", pyclass(get_all))]
#[base_model]
pub struct Suspension {
    #[serde(flatten)]
    pub core: SuspensionCore,
    pub preparers: Vec<PersonSummary>,
    pub measurements: Vec<SuspensionMeasurement>,
}
