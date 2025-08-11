pub use common::{BiologicalMaterial, CellCountingMethod, MeasurementDataCore};
pub use pool::{
    NewSuspensionPool, NewSuspensionPoolMeasurement, SuspensionPoolHandle,
    SuspensionPoolMeasurementData, SuspensionPoolPreparer,
};
pub use suspension::{
    MultiplexingTag, MultiplexingTagType, NewMultiplexingTag, NewSuspension,
    NewSuspensionMeasurement, Suspension, SuspensionCore, SuspensionMeasurement,
    SuspensionMeasurementData, SuspensionPreparer,
};

mod common;
mod pool;
#[allow(clippy::module_inception)]
mod suspension;
use serde_json::from_
