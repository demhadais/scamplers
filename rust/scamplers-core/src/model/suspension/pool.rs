use scamplers_macros::{db_insertion, db_json, db_selection};
#[cfg(feature = "backend")]
use scamplers_schema::{suspension_pool, suspension_pool_measurement, suspension_pool_preparers};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::suspension::{common::MeasurementDataCore, suspension::NewSuspension};

#[db_json]
pub struct SuspensionPoolMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementDataCore,
    pub is_post_storage: bool,
}

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

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct NewSuspensionPool {
    #[garde(dive)]
    pub readable_id: ValidString,
    pub pooled_at: OffsetDateTime,
    #[garde(dive)]
    pub notes: Option<ValidString>,
    #[garde(dive)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub suspensions: Vec<NewSuspension>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    pub measurements: Vec<NewSuspensionPoolMeasurement>,
}

#[db_insertion]
pub struct SuspensionPoolPreparer {
    pub pool_id: Uuid,
    pub prepared_by: Uuid,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct SuspensionPoolHandle {
    pub id: Uuid,
    pub link: String,
}

#[db_selection]
#[cfg_attr(feature = "backend", diesel(table_name = suspension_pool))]
pub struct SuspensionPoolSummary {
    #[serde(flatten)]
    #[cfg_attr(feature = "backend", diesel(embed))]
    pub handle: SuspensionPoolHandle,
    pub readable_id: String,
    pub pooled_at: OffsetDateTime,
}
