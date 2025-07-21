use scamplers_macros::db_json;
use time::OffsetDateTime;
use valid_string::ValidString;

use crate::model::units::{MassUnit, VolumeUnit};

#[db_json]
pub struct ElectrophoreticSizingRange(
    #[garde(range(min = 0, max = self.1))] i32,
    #[garde(range(min = self.0, max = 10_000))] i32,
);

#[db_json]
pub struct Concentration {
    #[garde(range(min = 0.0))]
    pub value: f32,
    pub unit: (MassUnit, VolumeUnit),
}

#[db_json]
pub struct ElectrophoreticMeasurementData {
    pub measured_at: OffsetDateTime,
    #[garde(dive)]
    pub instrument_name: ValidString,
    #[garde(range(min = 0.0))]
    pub mean_library_size_bp: f32,
    #[garde(dive)]
    pub sizing_range: ElectrophoreticSizingRange,
    #[garde(dive)]
    pub concentration: Concentration,
}
