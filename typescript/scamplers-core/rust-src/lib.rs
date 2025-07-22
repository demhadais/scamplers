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
    use model::institution::NewInstitution;

    use crate::model::{
        dataset::DatasetSummary,
        lab::NewLab,
        person::{NewPerson, UserRole},
        specimen::{
            BlockFixative, FixedBlockEmbeddingMatrix, NewCommitteeApproval, NewCryoPreservedTissue,
            NewFixedBlock, NewFixedTissue, NewFrozenBlock, NewFrozenTissue, NewSpecimenMeasurement,
            NewVirtualSpecimen, Species,
        },
    };

    m.add_class::<NewInstitution>()?;

    m.add_class::<UserRole>()?;
    m.add_class::<NewPerson>()?;

    m.add_class::<NewLab>()?;

    m.add_class::<NewFixedBlock>()?;
    m.add_class::<FixedBlockEmbeddingMatrix>()?;
    m.add_class::<NewFrozenBlock>()?;
    m.add_class::<BlockFixative>()?;

    m.add_class::<NewCryoPreservedTissue>()?;
    m.add_class::<NewFixedTissue>()?;
    m.add_class::<NewFrozenTissue>()?;

    m.add_class::<NewVirtualSpecimen>()?;

    m.add_class::<Species>()?;
    m.add_class::<NewSpecimenMeasurement>()?;
    m.add_class::<NewCommitteeApproval>()?;

    m.add_class::<DatasetSummary>()?;
    m.add_class::<Client>()?;

    Ok(())
}
