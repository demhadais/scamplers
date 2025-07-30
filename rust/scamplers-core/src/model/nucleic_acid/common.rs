#[cfg(feature = "python")]
use pyo3::prelude::*;
use scamplers_macros::{db_json, to_from_json};
use time::OffsetDateTime;
use valid_string::ValidString;

use crate::model::units::{MassUnit, VolumeUnit};

#[db_json]
#[cfg_attr(
    feature = "python",
    pyo3(name = "NucleicAcidConcentration", get_all, set_all)
)]
pub struct Concentration {
    #[garde(range(min = 0.0))]
    pub value: f32,
    pub unit: (MassUnit, VolumeUnit),
}

#[cfg(feature = "python")]
#[pymethods]
impl Concentration {
    #[new]
    #[pyo3(signature = (*, value, unit))]
    fn new(value: f32, unit: (MassUnit, VolumeUnit)) -> Self {
        Self { value, unit }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn electrophoretic_sizing_range((min, max): &(u16, u16), _: &()) -> garde::Result {
    if min >= max {
        return Err(garde::Error::new("min must be less than max"));
    }
    if *max > 10_000 {
        return Err(garde::Error::new("max must be less than 10,000"));
    }

    Ok(())
}

#[to_from_json(python)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(get_all, set_all))]
pub struct ElectrophoreticMeasurementData {
    pub measured_at: OffsetDateTime,
    #[garde(dive)]
    pub instrument_name: ValidString,
    #[garde(range(min = 0.0))]
    pub mean_library_size_bp: f32,
    #[garde(custom(electrophoretic_sizing_range))]
    pub sizing_range: (u16, u16),
    #[garde(dive)]
    pub concentration: Concentration,
}

#[cfg(feature = "python")]
#[pymethods]
impl ElectrophoreticMeasurementData {
    #[new]
    #[pyo3(signature = (*, measured_at, instrument_name, mean_library_size_bp, sizing_range, concentration_value, concentration_unit))]
    fn new(
        measured_at: OffsetDateTime,
        instrument_name: ValidString,
        mean_library_size_bp: f32,
        sizing_range: (u16, u16),
        concentration_value: f32,
        concentration_unit: (MassUnit, VolumeUnit),
    ) -> Self {
        Self {
            measured_at,
            instrument_name,
            mean_library_size_bp,
            sizing_range,
            concentration: Concentration {
                value: concentration_value,
                unit: concentration_unit,
            },
        }
    }
}
