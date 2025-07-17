#![allow(uncommon_codepoints)]

#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod api_path;
pub mod client;
pub mod model;

#[cfg(feature = "python")]
#[pymodule]
fn scamplers_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use client::Client;
    use model::institution::{Institution, NewInstitution};

    m.add_class::<NewInstitution>()?;
    m.add_class::<Institution>()?;
    m.add_class::<Client>()?;

    Ok(())
}
