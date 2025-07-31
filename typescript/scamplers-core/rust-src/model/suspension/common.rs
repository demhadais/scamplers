use scamplers_macros::{db_enum, db_json};
use time::OffsetDateTime;

use crate::model::units::{LengthUnit, VolumeUnit};

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

#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(scamplers_macros::FromJson, scamplers_macros::ToJson)
)]
#[db_json]
#[cfg_attr(
    feature = "python",
    pyo3(name = "SuspensionMeasurementDataCommon", set_all)
)]
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
