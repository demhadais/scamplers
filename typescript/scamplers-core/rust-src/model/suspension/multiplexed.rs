use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    model::suspension::{common::MeasurementDataCore, singleplexed::NewSuspension},
    string::ValidString,
};
#[cfg(feature = "backend")]
use {
    scamplers_macros::{backend_db_json, backend_insertion},
    scamplers_schema::multiplexed_suspension_preparers,
    scamplers_schema::{multiplexed_suspension, multiplexed_suspension_measurement},
};

#[cfg_attr(feature = "backend", backend_db_json)]
pub struct MultiplexedSuspensionMeasurementData {
    #[cfg_attr(feature = "backend", serde(flatten), garde(dive))]
    data: MeasurementDataCore,
    is_post_storage: bool,
}

#[cfg_attr(
    feature = "backend",
    backend_insertion(multiplexed_suspension_measurement)
)]
pub struct NewMultiplexedSuspensionMeasurement {
    #[cfg_attr(feature = "backend", serde(default))]
    suspension_id: Uuid,
    measured_by: Uuid,
    #[cfg_attr(feature = "backend", serde(flatten), garde(dive))]
    data: MultiplexedSuspensionMeasurementData,
}

#[cfg_attr(feature = "backend", backend_insertion(multiplexed_suspension))]
pub struct NewMultiplexedSuspension {
    readable_id: ValidString,
    #[cfg_attr(feature = "backend", valuable(skip))]
    pooled_at: OffsetDateTime,
    notes: Option<ValidString>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion), garde(dive))]
    suspensions: Vec<NewSuspension>,
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    preparer_ids: Vec<Uuid>,
    #[cfg_attr(
        feature = "backend",
        diesel(skip_insertion),
        garde(dive),
        serde(default)
    )]
    measurements: Vec<NewMultiplexedSuspensionMeasurement>,
}

#[cfg_attr(
    feature = "backend",
    backend_insertion(multiplexed_suspension_preparers)
)]
pub struct MultiplexedSuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}

impl NewMultiplexedSuspension {
    pub fn suspensions(&mut self, suspension_id: Uuid) -> &[NewSuspension] {
        for suspension in &mut self.suspensions {
            suspension.pooled_into_id = Some(suspension_id);
        }

        &self.suspensions
    }

    #[must_use]
    pub fn preparers(&self, suspension_id: Uuid) -> Vec<MultiplexedSuspensionPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| MultiplexedSuspensionPreparer {
                prepared_by: *p,
                suspension_id,
            })
            .collect()
    }

    pub fn measurements(&mut self, suspension_id: Uuid) -> &[NewMultiplexedSuspensionMeasurement] {
        for measurement in &mut self.measurements {
            measurement.suspension_id = suspension_id;
        }

        &self.measurements
    }
}
