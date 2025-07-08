use crate::model::units::{LengthUnit, VolumeUnit};
use scamplers_macros::{db_enum, db_json};
use time::OffsetDateTime;

#[db_enum]
pub enum CellCountingMethod {
    BrightField,
    Aopi,
    TrypanBlue,
}

#[db_enum]
pub enum BiologicalMaterial {
    Cells,
    Nuclei,
}

#[db_json]
pub enum MeasurementDataCore {
    Concentration {
        measured_at: OffsetDateTime,
        instrument_name: String,
        counting_method: CellCountingMethod,
        #[garde(range(min = 0.0))]
        value: f32,
        unit: (BiologicalMaterial, VolumeUnit),
    },
    Volume {
        measured_at: OffsetDateTime,
        #[garde(range(min = 0.0))]
        value: f32,
        unit: VolumeUnit,
    },
    Viability {
        measured_at: OffsetDateTime,
        instrument_name: String,
        #[garde(range(min = 0.0, max = 1.0))]
        value: f32,
    },
    MeanDiameter {
        measured_at: OffsetDateTime,
        instrument_name: String,
        #[garde(range(min = 0.0))]
        value: f32,
        unit: (BiologicalMaterial, LengthUnit),
    },
}
