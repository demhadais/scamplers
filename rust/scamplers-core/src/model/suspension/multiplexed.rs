use scamplers_macros::{db_insertion, db_json};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    model::suspension::{common::MeasurementDataCore, singleplexed::NewSuspension},
    string::NonEmptyString,
};
#[cfg(feature = "backend")]
use scamplers_schema::{
    multiplexed_suspension, multiplexed_suspension_measurement, multiplexed_suspension_preparers,
};

#[db_json]
pub struct MultiplexedSuspensionMeasurementData {
    #[serde(flatten)]
    #[garde(dive)]
    data: MeasurementDataCore,
    is_post_storage: bool,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexed_suspension_measurement))]
pub struct NewMultiplexedSuspensionMeasurement {
    #[serde(default)]
    suspension_id: Uuid,
    measured_by: Uuid,
    #[serde(flatten)]
    #[garde(dive)]
    data: MultiplexedSuspensionMeasurementData,
}

#[db_insertion]
#[cfg_attr(feature = "backend", diesel(table_name = multiplexed_suspension))]
pub struct NewMultiplexedSuspension {
    readable_id: NonEmptyString,
    pooled_at: OffsetDateTime,
    notes: Option<NonEmptyString>,
    #[garde(dive)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    suspensions: Vec<NewSuspension>,
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    preparer_ids: Vec<Uuid>,
    #[garde(dive)]
    #[serde(default)]
    #[getset(skip)]
    #[cfg_attr(feature = "backend", diesel(skip_insertion))]
    measurements: Vec<NewMultiplexedSuspensionMeasurement>,
}

#[db_insertion]
pub struct MultiplexedSuspensionPreparer {
    suspension_id: Uuid,
    prepared_by: Uuid,
}

impl NewMultiplexedSuspension {
    pub fn suspensions(&mut self, self_id: Uuid) -> &[NewSuspension] {
        for suspension in &mut self.suspensions {
            suspension.set_pooled_into_id(Some(self_id));
        }

        &self.suspensions
    }

    #[must_use]
    pub fn preparers(&self, self_id: Uuid) -> Vec<MultiplexedSuspensionPreparer> {
        self.preparer_ids
            .iter()
            .map(|p| MultiplexedSuspensionPreparer {
                prepared_by: *p,
                suspension_id: self_id,
            })
            .collect()
    }

    pub fn measurements(&mut self, self_id: Uuid) -> &[NewMultiplexedSuspensionMeasurement] {
        for measurement in &mut self.measurements {
            measurement.suspension_id = self_id;
        }

        &self.measurements
    }
}
