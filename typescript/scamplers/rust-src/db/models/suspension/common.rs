use scamplers_macros::{db_json, db_simple_enum};
use time::OffsetDateTime;
use valid_string::ValidString;

use crate::db::models::units::{LengthUnit, VolumeUnit};

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum CellCountingMethod {
    BrightField,
    Aopi,
    TrypanBlue,
}

#[db_simple_enum]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
pub enum BiologicalMaterial {
    Cells,
    Nuclei,
}

#[cfg_attr(
    feature = "python",
    pyo3_stub_gen::derive::gen_stub_pyclass_complex_enum
)]
#[db_json]
#[serde(tag = "quantity", rename_all = "snake_case")]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common", set_all))]
pub enum SuspensionMeasurementFields {
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

impl SuspensionMeasurementFields {
    pub fn is_volume(&self, (): &()) -> garde::Result {
        if !matches!(self, Self::Volume { .. }) {
            return Err(garde::Error::new(format!(
                "expected volume, found {self:?}"
            )));
        }

        Ok(())
    }
}
