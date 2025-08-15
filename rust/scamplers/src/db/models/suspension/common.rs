use scamplers_macros::{db_json, db_simple_enum};
use time::OffsetDateTime;
use valid_string::ValidString;

use crate::db::models::units::{LengthUnit, VolumeUnit};

#[db_simple_enum]
pub enum CellCountingMethod {
    BrightField,
    Aopi,
    TrypanBlue,
}

#[db_simple_enum]
pub enum BiologicalMaterial {
    Cells,
    Nuclei,
}

#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass_complex_enum
)]
#[db_json]
#[cfg_attr(
    feature = "python",
    pyo3(name = "SuspensionMeasurementDataCommon", module = "scamplepy.common")
)]
pub enum MeasurementDataCore {
    Concentration {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
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
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(range(min = 0.0, max = 1.0))]
        value: f32,
    },
    MeanDiameter {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(range(min = 0.0))]
        value: f32,
        unit: (BiologicalMaterial, LengthUnit),
    },
}
