#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use scamplers_macros::db_json;
use time::OffsetDateTime;
use valid_string::ValidString;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::db::models::units::{MassUnit, VolumeUnit};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", gen_stub_pyclass)]
#[db_json]
#[cfg_attr(
    feature = "python",
    pyo3(name = "NucleicAcidConcentration", module = "scamplepy.common")
)]
pub struct Concentration {
    #[garde(range(min = 0.0))]
    pub value: f32,
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub unit: (MassUnit, VolumeUnit),
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Concentration {
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn unit(&self) -> String {
        let (num, denom) = self.unit;

        let num: &'static str = num.into();
        let denom: &'static str = denom.into();

        format!("{num}/{denom}")
    }
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[cfg_attr(feature = "python", gen_stub_pyclass)]
#[db_json]
#[cfg_attr(feature = "python", pyo3(module = "scamplepy.common"))]
#[derive(bon::Builder)]
#[builder(on(_, into))]
pub struct ElectrophoreticMeasurementData {
    pub measured_at: OffsetDateTime,
    #[garde(dive)]
    pub instrument_name: ValidString,
    #[garde(range(min = 0.0))]
    pub mean_size_bp: Option<f32>,
    #[garde(custom(electrophoretic_sizing_range))]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(skip))]
    pub sizing_range: (u16, u16),
    #[garde(dive)]
    pub concentration: Concentration,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ElectrophoreticMeasurementData {
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn sizing_range(&self) -> Vec<u16> {
        let (min, max) = self.sizing_range;
        [min, max].to_vec()
    }
}

#[cfg(feature = "python")]
#[gen_stub_pymethods]
#[pymethods]
impl ElectrophoreticMeasurementData {
    #[new]
    #[pyo3(signature = (*, measured_at, instrument_name, mean_size_bp, sizing_range, concentration_value, concentration_unit))]
    fn new(
        measured_at: OffsetDateTime,
        instrument_name: ValidString,
        mean_size_bp: Option<f32>,
        sizing_range: (u16, u16),
        concentration_value: f32,
        concentration_unit: (MassUnit, VolumeUnit),
    ) -> Self {
        Self {
            measured_at,
            instrument_name,
            mean_size_bp,
            sizing_range,
            concentration: Concentration {
                value: concentration_value,
                unit: concentration_unit,
            },
        }
    }
}
