#[cfg(feature = "backend")]
use scamplers_macros::{backend_db_enum, backend_db_json};
use time::OffsetDateTime;

use crate::model::units::{LengthUnit, VolumeUnit};

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum CellCountingMethod {
    BrightField,
    Aopi,
    TrypanBlue,
}

#[cfg_attr(feature = "backend", backend_db_enum)]
pub enum BiologicalMaterial {
    Cells,
    Nuclei,
}

#[cfg_attr(feature = "backend", backend_db_json)]
pub enum MeasurementDataCore {
    Concentration {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        instrument_name: String,
        counting_method: CellCountingMethod,
        #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
        value: f32,
        unit: (BiologicalMaterial, VolumeUnit),
    },
    Volume {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
        value: f32,
        unit: VolumeUnit,
    },
    Viability {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        instrument_name: String,
        #[cfg_attr(feature = "backend", garde(range(min = 0.0, max = 1.0)))]
        value: f32,
    },
    MeanDiameter {
        #[cfg_attr(feature = "backend", valuable(skip))]
        measured_at: OffsetDateTime,
        instrument_name: String,
        #[cfg_attr(feature = "backend", garde(range(min = 0.0)))]
        value: f32,
        unit: (BiologicalMaterial, LengthUnit),
    },
}
