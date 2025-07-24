#![allow(uncommon_codepoints)]

#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod api_path;
pub mod client;
pub mod model;
pub mod result;

#[cfg(feature = "python")]
#[pymodule]
fn scamplers_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use client::ScamplersClient;
    use model::{
        dataset::DatasetSummary,
        institution::NewInstitution,
        lab::NewLab,
        person::{NewPerson, UserRole},
        specimen::{
            BlockFixative, FixedBlockEmbeddingMatrix, NewCommitteeApproval, NewCryoPreservedTissue,
            NewFixedBlock, NewFixedTissue, NewFrozenBlock, NewFrozenTissue, NewSpecimenMeasurement,
            NewVirtualSpecimen, Species, TissueFixative,
        },
    };

    use crate::{
        model::suspension::{NewSuspension, NewSuspensionPool},
        result::{
            CdnaGemsError, CdnaLibraryTypeError, ClientError, DatasetCmdlineError,
            DatasetMetricsFileParseError, DatasetNMetricsFilesError, DuplicateResourceError,
            InvalidDataError, InvalidMeasurementError, InvalidReferenceError,
            MalformedRequestError, PermissionDeniedError, ResourceNotFoundError,
            ScamplersCoreErrorResponse, ServerError,
        },
    };

    m.add_class::<ClientError>()?;
    m.add_class::<DuplicateResourceError>()?;
    m.add_class::<InvalidReferenceError>()?;
    m.add_class::<ResourceNotFoundError>()?;
    m.add_class::<InvalidDataError>()?;
    m.add_class::<MalformedRequestError>()?;
    m.add_class::<PermissionDeniedError>()?;
    m.add_class::<ServerError>()?;
    m.add_class::<CdnaGemsError>()?;
    m.add_class::<CdnaLibraryTypeError>()?;
    m.add_class::<DatasetCmdlineError>()?;
    m.add_class::<DatasetNMetricsFilesError>()?;
    m.add_class::<DatasetMetricsFileParseError>()?;
    m.add_class::<InvalidMeasurementError>()?;

    m.add_class::<ScamplersCoreErrorResponse>()?;

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
    m.add_class::<TissueFixative>()?;

    m.add_class::<NewVirtualSpecimen>()?;

    m.add_class::<Species>()?;
    m.add_class::<NewSpecimenMeasurement>()?;
    m.add_class::<NewCommitteeApproval>()?;

    m.add_class::<NewSuspension>()?;
    m.add_class::<NewSuspensionPool>()?;

    m.add_class::<DatasetSummary>()?;

    m.add_class::<ScamplersClient>()?;

    Ok(())
}
