use std::collections::HashSet;

use any_value::AnyValue;
#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::gen_stub_pymethods;
use scamplers_macros::{base_model, db_insertion, db_query, db_selection};
#[cfg(feature = "app")]
use scamplers_schema::suspension_pool_preparers;
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{
    db::models::{
        DefaultVec, Links, Pagination,
        suspension::{
            common::SuspensionMeasurementFields,
            suspension::{NewSuspension, SuspensionSummary},
        },
    },
    define_ordering_enum, uuid_newtype,
};

#[cfg(feature = "app")]
mod create;
#[cfg(feature = "app")]
mod read;

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_pool_measurement))]
pub struct NewSuspensionPoolMeasurement {
    #[serde(default)]
    #[builder(default)]
    pub pool_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: SuspensionMeasurementFields,
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSuspensionPoolMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, pool_id=Uuid::default()))]
    fn new(measured_by: Uuid, data: SuspensionMeasurementFields, pool_id: Uuid) -> Self {
        Self {
            pool_id,
            measured_by,
            data,
        }
    }
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_pool))]
pub struct NewSuspensionPool {
    #[garde(dive)]
    pub readable_id: ValidString,
    #[garde(dive)]
    pub name: ValidString,
    pub pooled_at: OffsetDateTime,
    #[garde(
        dive,
        length(min = 1),
        custom(|suspensions, (): &()| validate_suspension_biological_materials(suspensions, &self.measurements))
    )]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub suspensions: Vec<NewSuspension>,
    #[garde(length(min = 1))]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionPoolMeasurement>,
    pub additional_data: Option<AnyValue>,
}

fn validate_suspension_biological_materials(
    suspensions: &[NewSuspension],
    measurements: &[NewSuspensionPoolMeasurement],
) -> garde::Result {
    let biological_materials: HashSet<_, _> =
        suspensions.iter().map(|s| s.biological_material).collect();

    if biological_materials.len() != 1 {
        return Err(garde::Error::new(
            "suspensions pooled together must have the same biological material",
        ));
    }

    let mut measurement_biological_materials = HashSet::with_capacity(measurements.len());

    for NewSuspensionPoolMeasurement { data, .. } in measurements {
        match data {
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

    if measurement_biological_materials != biological_materials {
        return Err(garde::Error::new(
            "all suspension pool measurements must have the same biological material as the \
             constituent suspensions",
        ));
    }

    Ok(())
}

#[cfg(feature = "python")]
#[pyo3_stub_gen::derive::gen_stub_pymethods]
#[pymethods]
impl NewSuspensionPool {
    #[new]
    #[pyo3(signature = (*, readable_id, name, pooled_at, suspensions, preparer_ids, measurements=Vec::new(), additional_data=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        pooled_at: OffsetDateTime,
        suspensions: Vec<NewSuspension>,
        preparer_ids: Vec<Uuid>,
        measurements: Vec<NewSuspensionPoolMeasurement>,
        additional_data: Option<AnyValue>,
    ) -> Self {
        Self {
            readable_id,
            name,
            pooled_at,
            suspensions,
            preparer_ids,
            measurements,
            additional_data,
        }
    }
}

#[db_selection]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_pool))]
pub struct SuspensionPoolSummary {
    pub id: Uuid,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(readonly))]
    pub links: Links,
    pub readable_id: String,
    pub name: String,
    pub pooled_at: OffsetDateTime,
    pub additional_data: Option<AnyValue>,
}

#[cfg(feature = "app")]
#[derive(Insertable, Identifiable, Associations, Selectable, Queryable)]
#[diesel(primary_key(pool_id, prepared_by), belongs_to(SuspensionPoolSummary, foreign_key = pool_id))]
struct SuspensionPoolPreparer {
    pub pool_id: Uuid,
    pub prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "app", derive(Associations))]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_pool_measurement, belongs_to(SuspensionPoolSummary, foreign_key = pool_id)))]
pub struct SuspensionPoolMeasurement {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub data: SuspensionMeasurementFields,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[base_model]
pub struct SuspensionPool {
    #[serde(flatten)]
    pub summary: SuspensionPoolSummary,
    pub suspensions: Vec<SuspensionSummary>,
    pub preparers: Vec<Uuid>,
    pub measurements: Vec<SuspensionPoolMeasurement>,
}

define_ordering_enum! { SuspensionPoolOrderBy{ Name, PooledAt, ReadableId }, default = PooledAt }

#[db_query]
pub struct SuspensionPoolQuery {
    #[builder(default)]
    pub ids: Vec<Uuid>,
    #[builder(default)]
    pub order_by: DefaultVec<SuspensionPoolOrderBy>,
    #[builder(default)]
    pub pagination: Pagination,
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl SuspensionPoolQuery {
    #[new]
    #[pyo3(signature = (*, ids=Vec::default(), order_by=DefaultVec::default(), limit=Pagination::default().limit, offset=Pagination::default_offset()))]
    fn new(
        ids: Vec<Uuid>,
        order_by: DefaultVec<SuspensionPoolOrderBy>,
        limit: i64,
        offset: i64,
    ) -> Self {
        Self {
            ids,
            order_by,
            pagination: Pagination { limit, offset },
        }
    }
}

uuid_newtype!(SuspensionPoolId);
