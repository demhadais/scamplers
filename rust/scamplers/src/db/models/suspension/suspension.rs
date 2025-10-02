use std::collections::HashSet;

use any_value::AnyValue;
#[cfg(feature = "app")]
use diesel::{Associations, prelude::*};
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::gen_stub_pymethods;
use scamplers_macros::{base_model, db_insertion, db_json, db_query, db_selection};
#[cfg(feature = "app")]
use scamplers_schema::{multiplexing_tag, specimen, suspension, suspension_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "app")]
use crate::db::models::suspension::pool::SuspensionPoolSummary;
use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        multiplexing_tag::MultiplexingTag,
        specimen::{SpecimenQuery, SpecimenSummary},
        suspension::common::{BiologicalMaterial, SuspensionMeasurementFields},
    },
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common", set_all))]
pub struct SuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub fields: SuspensionMeasurementFields,
    pub is_post_probe_hybridization: bool,
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
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSuspensionMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, is_post_probe_hybridization, suspension_id=Uuid::default()))]
    fn new(
        measured_by: Uuid,
        data: SuspensionMeasurementFields,
        is_post_probe_hybridization: bool,
        suspension_id: Uuid,
    ) -> Self {
        Self {
            suspension_id,
            measured_by,
            data: SuspensionMeasurementData {
                fields: data,
                is_post_probe_hybridization,
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
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive, custom(|measurements, (): &()| validate_suspension_measurement_biological_materials(measurements, self.biological_material)))]
    #[serde(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionMeasurement>,
    pub created_at: Option<OffsetDateTime>,
    pub pooled_into: Option<Uuid>,
    pub multiplexing_tag_id: Option<Uuid>,
    #[garde(range(min = 0.0))]
    pub lysis_duration_minutes: Option<f32>,
    pub additional_data: Option<AnyValue>,
}

fn validate_suspension_measurement_biological_materials(
    measurements: &[NewSuspensionMeasurement],
    self_biological_material: BiologicalMaterial,
) -> garde::Result {
    let mut measurement_biological_materials = HashSet::with_capacity(measurements.len());

    for NewSuspensionMeasurement {
        data: SuspensionMeasurementData { fields, .. },
        ..
    } in measurements
    {
        match fields {
            SuspensionMeasurementFields::Concentration {
                unit: (biological_material, ..),
                ..
            }
            | SuspensionMeasurementFields::MeanDiameter {
                unit: (biological_material, ..),
                ..
            } => {
                measurement_biological_materials.insert(*biological_material);
            }
            _ => {}
        }
    }

    if measurement_biological_materials.is_empty() {
        return Ok(());
    }

    if measurement_biological_materials != HashSet::from_iter([self_biological_material]) {
        return Err(garde::Error::new(
            "all suspension measurements must have the same biological material as the suspension \
             itself",
        ));
    }

    Ok(())
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSuspension {
    #[new]
    #[pyo3(signature = (*, readable_id, parent_specimen_id, biological_material, target_cell_recovery, preparer_ids, measurements=Vec::new(), created_at=None, multiplexing_tag_id=None, lysis_duration_minutes=None, additional_data=None))]
    fn new(
        readable_id: ValidString,
        parent_specimen_id: Uuid,
        biological_material: BiologicalMaterial,
        target_cell_recovery: f32,
        preparer_ids: Vec<Uuid>,
        measurements: Vec<NewSuspensionMeasurement>,
        created_at: Option<OffsetDateTime>,
        multiplexing_tag_id: Option<Uuid>,
        lysis_duration_minutes: Option<f32>,
        additional_data: Option<AnyValue>,
    ) -> Self {
        Self {
            readable_id,
            parent_specimen_id,
            biological_material,
            target_cell_recovery,
            preparer_ids,
            measurements,
            created_at,
            pooled_into: None,
            multiplexing_tag_id,
            lysis_duration_minutes,
            additional_data,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = suspension, belongs_to(SuspensionPoolSummary, foreign_key = pooled_into)))]
pub struct SuspensionSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub biological_material: String,
    pub pooled_into: Option<Uuid>,
    pub created_at: Option<OffsetDateTime>,
    pub lysis_duration_minutes: Option<f32>,
    pub target_cell_recovery: f32,
    pub additional_data: Option<AnyValue>,
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = suspension, base_query = suspension::table.inner_join(specimen::table).left_join(multiplexing_tag::table)))]
pub struct SuspensionSummaryWithParents {
    #[cfg_attr(feature = "app", diesel(column_name = id))]
    pub id_: Uuid,
    #[serde(flatten)]
    #[cfg_attr(feature = "app", diesel(embed))]
    pub summary: SuspensionSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub parent_specimen: SpecimenSummary,
    #[cfg_attr(feature = "app", diesel(embed))]
    pub multiplexing_tag: Option<MultiplexingTag>,
}

#[cfg(feature = "app")]
#[derive(Insertable, Identifiable, Associations, Selectable, Queryable)]
#[diesel(primary_key(suspension_id, prepared_by), belongs_to(SuspensionSummaryWithParents, foreign_key = suspension_id))]
struct SuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_measurement, belongs_to(SuspensionSummaryWithParents, foreign_key = suspension_id)))]
pub struct SuspensionMeasurement {
    pub id: Uuid,
    pub measured_by: Uuid,
    pub suspension_id: Uuid,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub data: SuspensionMeasurementData,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[base_model]
pub struct Suspension {
    #[serde(flatten)]
    pub info: SuspensionSummaryWithParents,
    pub prepared_by: Vec<Uuid>,
    pub measurements: Vec<SuspensionMeasurement>,
}

define_ordering_enum! { SuspensionOrderBy{ CreatedAt, ReadableId }, default = CreatedAt }

// TODO: add more fields
#[db_query]
pub struct SuspensionQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    pub parent_specimen: Option<SpecimenQuery>,
    #[builder(default)]
    pub order_by: DefaultVec<SuspensionOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl SuspensionQuery {
    #[new]
    #[pyo3(signature = (*, ids = Vec::new(), parent_specimen = None, order_by = DefaultVec::default(), limit = Pagination::default().limit, offset = Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        parent_specimen: Option<SpecimenQuery>,
        order_by: DefaultVec<SuspensionOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            parent_specimen,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}

uuid_newtype!(SuspensionId);
