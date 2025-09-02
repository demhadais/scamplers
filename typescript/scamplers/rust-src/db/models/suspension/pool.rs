#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{base_model, db_insertion, db_json, db_query, db_selection};
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

#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub struct SuspensionPoolMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub fields: SuspensionMeasurementFields,
    pub is_post_storage: bool,
}

#[db_insertion]
#[cfg_attr(feature = "app", diesel(table_name = scamplers_schema::suspension_pool_measurement))]
pub struct NewSuspensionPoolMeasurement {
    #[serde(default)]
    #[builder(default)]
    pub pool_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: SuspensionPoolMeasurementData,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSuspensionPoolMeasurement {
    #[new]
    #[pyo3(signature = (*, measured_by, data, is_post_storage, pool_id=Uuid::default()))]
    fn new(
        measured_by: Uuid,
        data: SuspensionMeasurementFields,
        is_post_storage: bool,
        pool_id: Uuid,
    ) -> Self {
        Self {
            pool_id,
            measured_by,
            data: SuspensionPoolMeasurementData {
                fields: data,
                is_post_storage,
            },
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
    #[garde(dive)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub suspensions: Vec<NewSuspension>,
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "app", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionPoolMeasurement>,
    #[garde(dive)]
    pub notes: Option<ValidString>,
}

#[cfg(feature = "python")]
#[pymethods]
impl NewSuspensionPool {
    #[new]
    #[pyo3(signature = (*, readable_id, name, pooled_at, suspensions, preparer_ids, measurements=Vec::new(), notes=None))]
    fn new(
        readable_id: ValidString,
        name: ValidString,
        pooled_at: OffsetDateTime,
        suspensions: Vec<NewSuspension>,
        preparer_ids: Vec<Uuid>,
        measurements: Vec<NewSuspensionPoolMeasurement>,
        notes: Option<ValidString>,
    ) -> Self {
        Self {
            readable_id,
            name,
            pooled_at,
            suspensions,
            preparer_ids,
            measurements,
            notes,
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
    pub notes: Option<String>,
}

#[db_insertion]
#[cfg_attr(
    feature = "app",
    derive(Identifiable, Associations, Selectable, Queryable)
)]
#[cfg_attr(feature = "app", diesel(primary_key(pool_id, prepared_by), belongs_to(SuspensionPoolSummary, foreign_key = pool_id)))]
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
    pub data: SuspensionPoolMeasurementData,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[cfg_attr(
    feature = "python",
    pyclass(eq, get_all, module = "scamplepy.responses")
)]
#[base_model]
pub struct SuspensionPool {
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

uuid_newtype!(SuspensionPoolId);
