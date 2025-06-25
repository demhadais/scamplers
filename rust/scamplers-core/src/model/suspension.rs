pub use multiplexed::NewMultiplexedSuspension;
pub use singleplexed::{
    NewSuspension, NewSuspensionMeasurement, Suspension, SuspensionCore, SuspensionMeasurement,
};

mod common;
mod multiplexed;
mod singleplexed;
