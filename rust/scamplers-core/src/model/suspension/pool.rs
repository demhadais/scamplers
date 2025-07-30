#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_insertion, db_json, db_selection, to_from_json};
#[cfg(feature = "backend")]
use scamplers_schema::{suspension_pool, suspension_pool_measurement, suspension_pool_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::suspension::{common::MeasurementDataCore, suspension::NewSuspension};

#[db_json]
#[cfg_attr(
    feature = "python",
    pyo3(name = "_SuspensionPoolMeasurementData", get_all, set_all)
)]
pub struct SuspensionPoolMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementDataCore,
    pub is_post_storage: bool,
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool_measurement))]
pub struct NewSuspensionPoolMeasurement {
    #[serde(default)]
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
        data: MeasurementDataCore,
        is_post_storage: bool,
        pool_id: Uuid,
    ) -> Self {
        Self {
            pool_id,
            measured_by,
            data: SuspensionPoolMeasurementData {
                data,
                is_post_storage,
            },
        }
    }
}

#[to_from_json(python)]
#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct NewSuspensionPool {
    #[garde(dive)]
    pub readable_id: ValidString,
    #[garde(dive)]
    pub name: ValidString,
    pub pooled_at: OffsetDateTime,
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub suspensions: Vec<NewSuspension>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
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

#[db_insertion]
pub struct SuspensionPoolPreparer {
    pub pool_id: Uuid,
    pub prepared_by: Uuid,
}

#[to_from_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct SuspensionPoolHandle {
    pub id: Uuid,
    pub link: String,
}

#[to_from_json(python)]
#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct SuspensionPoolSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: SuspensionPoolHandle,
    pub readable_id: String,
    pub pooled_at: OffsetDateTime,
}
