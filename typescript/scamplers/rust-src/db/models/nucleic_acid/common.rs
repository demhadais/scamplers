#[cfg(feature = "app")]
use diesel::prelude::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pyclass_complex_enum, gen_stub_pymethods};
use scamplers_macros::db_json;
#[cfg(feature = "app")]
use scamplers_schema::{chromium_run, gems, tenx_assay};
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
    pyo3(
        name = "NucleicAcidConcentration",
        module = "scamplepy.common",
        set_all
    )
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
#[gen_stub_pymethods]
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

#[cfg_attr(feature = "python", gen_stub_pyclass_complex_enum)]
#[db_json]
#[serde(tag = "type")]
#[cfg_attr(
    feature = "python",
    pyo3(
        name = "NucleicAcidMeasurementData",
        module = "scamplepy.common",
        set_all
    )
)]
pub enum MeasurementData {
    Electrophoretic {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(range(min = 0.0))]
        mean_size_bp: Option<f32>,
        #[garde(custom(electrophoretic_sizing_range))]
        sizing_range: (u16, u16),
        #[garde(dive)]
        concentration: Concentration,
    },
    Fluorometric {
        measured_at: OffsetDateTime,
        #[garde(dive)]
        instrument_name: ValidString,
        #[garde(dive)]
        concentration: Concentration,
    },
}

#[cfg(feature = "app")]
#[diesel::dsl::auto_type]
#[must_use]
pub fn gems_to_assay() -> _ {
    gems::table.inner_join(chromium_run::table.inner_join(tenx_assay::table))
}
