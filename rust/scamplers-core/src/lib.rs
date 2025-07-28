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
        chromium_run::{NewSingleplexChromiumRun, NewSingleplexGems},
        institution::{Institution, NewInstitution},
        lab::NewLab,
        person::{NewPerson, UserRole},
        specimen::{
            self, BlockFixative, FixedBlockEmbeddingMatrix, NewCommitteeApproval,
            NewCryopreservedTissue, NewFixedBlock, NewFixedTissue, NewFrozenBlock, NewFrozenTissue,
            NewSpecimenMeasurement, NewVirtualSpecimen, Species, TissueFixative,
        },
        suspension::{
            self, BiologicalMaterial, CellCountingMethod, NewSuspension, NewSuspensionPool,
        },
    };
    use result::{
        CdnaGemsError, CdnaLibraryTypeError, ClientError, DatasetCmdlineError,
        DatasetMetricsFileParseError, DatasetNMetricsFilesError, DuplicateResourceError,
        InvalidDataError, InvalidMeasurementError, InvalidReferenceError, MalformedRequestError,
        PermissionDeniedError, ResourceNotFoundError, ScamplersCoreErrorResponse, ServerError,
    };

    use crate::model::chromium_run::{
        NewOcmChipLoading, NewOcmChromiumRun, NewOcmGems, NewSingleplexChipLoading,
    };

    m.add_class::<ScamplersClient>()?;

    let submodule_names = ["errors", "requests", "responses"];
    let [errors_module, requests_module, responses_module] = submodule_names;

    // All the error types
    let errors = PyModule::new(m.py(), errors_module)?;
    errors.add_class::<ClientError>()?;
    errors.add_class::<DuplicateResourceError>()?;
    errors.add_class::<InvalidReferenceError>()?;
    errors.add_class::<ResourceNotFoundError>()?;
    errors.add_class::<InvalidDataError>()?;
    errors.add_class::<MalformedRequestError>()?;
    errors.add_class::<PermissionDeniedError>()?;
    errors.add_class::<ServerError>()?;
    errors.add_class::<CdnaGemsError>()?;
    errors.add_class::<CdnaLibraryTypeError>()?;
    errors.add_class::<DatasetCmdlineError>()?;
    errors.add_class::<DatasetNMetricsFilesError>()?;
    errors.add_class::<DatasetMetricsFileParseError>()?;
    errors.add_class::<InvalidMeasurementError>()?;
    // The error type that wraps them all
    errors.add_class::<ScamplersCoreErrorResponse>()?;
    m.add_submodule(&errors)?;

    // All the request models, grouped by domain
    let requests = PyModule::new(m.py(), requests_module)?;
    requests.add_class::<NewInstitution>()?;

    requests.add_class::<UserRole>()?;
    requests.add_class::<NewPerson>()?;

    requests.add_class::<NewLab>()?;

    requests.add_class::<NewFixedBlock>()?;
    requests.add_class::<FixedBlockEmbeddingMatrix>()?;
    requests.add_class::<NewFrozenBlock>()?;
    requests.add_class::<BlockFixative>()?;

    requests.add_class::<NewCryopreservedTissue>()?;
    requests.add_class::<NewFixedTissue>()?;
    requests.add_class::<NewFrozenTissue>()?;
    requests.add_class::<TissueFixative>()?;

    requests.add_class::<NewVirtualSpecimen>()?;

    requests.add_class::<Species>()?;
    requests.add_class::<specimen::MeasurementData>()?;
    requests.add_class::<NewSpecimenMeasurement>()?;
    requests.add_class::<NewCommitteeApproval>()?;

    requests.add_class::<suspension::MeasurementDataCore>()?;
    requests.add_class::<BiologicalMaterial>()?;
    requests.add_class::<CellCountingMethod>()?;

    requests.add_class::<NewSuspension>()?;
    requests.add_class::<NewSuspensionPool>()?;

    requests.add_class::<NewSingleplexChromiumRun>()?;
    requests.add_class::<NewSingleplexGems>()?;
    requests.add_class::<NewSingleplexChipLoading>()?;

    requests.add_class::<NewOcmChromiumRun>()?;
    requests.add_class::<NewOcmGems>()?;
    requests.add_class::<NewOcmChipLoading>()?;
    m.add_submodule(&requests)?;

    // All the response types, grouped by domain
    let responses = PyModule::new(m.py(), responses_module)?;
    responses.add_class::<Institution>()?;
    m.add_submodule(&responses)?;

    let python = m.py();
    let sys_modules = python.import("sys")?.getattr("modules")?;

    let submodules = [errors, requests, responses];
    for (module_name, submodule) in submodule_names.iter().zip(&submodules) {
        sys_modules.set_item(format!("scamplers_core.{module_name}"), submodule)?;
    }

    Ok(())
}
