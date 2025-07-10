use scamplers_macros::{db_insertion, db_json};
use time::OffsetDateTime;
use uuid::Uuid;
use valid_string::ValidString;

use crate::model::suspension::{common::MeasurementDataCore, suspension::NewSuspension};
#[cfg(feature = "backend")]
use scamplers_schema::{
    multiplexed_suspension, multiplexed_suspension_measurement, multiplexed_suspension_preparers,
};

#[db_json]
pub struct MultiplexedSuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MeasurementDataCore,
    pub is_post_storage: bool,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexed_suspension_measurement))]
pub struct NewMultiplexedSuspensionMeasurement {
    #[serde(default)]
    pub suspension_id: Uuid,
    pub measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    pub data: MultiplexedSuspensionMeasurementData,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexed_suspension))]
pub struct NewMultiplexedSuspension {
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
    pub measurements: Vec<NewMultiplexedSuspensionMeasurement>,
}

#[db_insertion]
pub struct MultiplexedSuspensionPreparer {
    pub suspension_id: Uuid,
    pub prepared_by: Uuid,
}
