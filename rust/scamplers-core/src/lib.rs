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
        chromium_run::{
            NewOcmChipLoading, NewOcmChromiumRun, NewOcmGems, NewPoolMultiplexChipLoading,
            NewPoolMultiplexChromiumRun, NewPoolMultiplexGems, NewSingleplexChipLoading,
            NewSingleplexChromiumRun, NewSingleplexGems, OcmChromiumChip,
            PoolMultiplexChromiumChip, SingleplexChromiumChip,
        },
        institution::{Institution, NewInstitution},
        lab::NewLab,
        library_type_specification::{LibraryType, NewLibraryTypeSpecification},
        nucleic_acid::{self, NewCdna, NewCdnaMeasurement, NewLibrary, NewLibraryMeasurement},
        person::{NewPerson, UserRole},
        specimen::{
            self, BlockFixative, ComplianceCommitteeType, FixedBlockEmbeddingMatrix,
            FrozenBlockEmbeddingMatrix, NewCommitteeApproval, NewCryopreservedTissue,
            NewFixedBlock, NewFixedTissue, NewFrozenBlock, NewFrozenTissue, NewSpecimenMeasurement,
            NewVirtualSpecimen, Species, SuspensionFixative, TissueFixative,
        },
        suspension::{
            self, BiologicalMaterial, CellCountingMethod, NewSuspension, NewSuspensionMeasurement,
            NewSuspensionPool,
        },
        units::{LengthUnit, MassUnit, VolumeUnit},
    };
    use result::{
        CdnaGemsError, CdnaLibraryTypeError, ClientError, DatasetCmdlineError,
        DatasetMetricsFileParseError, DatasetNMetricsFilesError, DuplicateResourceError,
        InvalidDataError, InvalidMeasurementError, InvalidReferenceError, MalformedRequestError,
        PermissionDeniedError, ResourceNotFoundError, ScamplersCoreErrorResponse, ServerError,
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
    requests.add_class::<FrozenBlockEmbeddingMatrix>()?;
    requests.add_class::<NewCryopreservedTissue>()?;
    requests.add_class::<NewFixedTissue>()?;
    requests.add_class::<NewFrozenTissue>()?;
    requests.add_class::<TissueFixative>()?;
    requests.add_class::<NewVirtualSpecimen>()?;
    requests.add_class::<SuspensionFixative>()?;
    requests.add_class::<MassUnit>()?;
    requests.add_class::<VolumeUnit>()?;
    requests.add_class::<LengthUnit>()?;
    requests.add_class::<Species>()?;
    requests.add_class::<specimen::MeasurementData>()?;
    requests.add_class::<NewSpecimenMeasurement>()?;
    requests.add_class::<NewCommitteeApproval>()?;
    requests.add_class::<ComplianceCommitteeType>()?;

    requests.add_class::<suspension::MeasurementDataCore>()?;
    requests.add_class::<BiologicalMaterial>()?;
    requests.add_class::<CellCountingMethod>()?;
    requests.add_class::<NewSuspension>()?;
    requests.add_class::<NewSuspensionMeasurement>()?;
    requests.add_class::<NewSuspensionPool>()?;

    requests.add_class::<NewSingleplexChromiumRun>()?;
    requests.add_class::<NewSingleplexGems>()?;
    requests.add_class::<NewSingleplexChipLoading>()?;
    requests.add_class::<SingleplexChromiumChip>()?;
    requests.add_class::<NewOcmChromiumRun>()?;
    requests.add_class::<NewOcmGems>()?;
    requests.add_class::<NewOcmChipLoading>()?;
    requests.add_class::<OcmChromiumChip>()?;
    requests.add_class::<NewPoolMultiplexChromiumRun>()?;
    requests.add_class::<NewPoolMultiplexGems>()?;
    requests.add_class::<NewPoolMultiplexChipLoading>()?;
    requests.add_class::<PoolMultiplexChromiumChip>()?;

    requests.add_class::<NewLibraryTypeSpecification>()?;

    requests.add_class::<nucleic_acid::common::ElectrophoreticMeasurementData>()?;
    requests.add_class::<nucleic_acid::common::Concentration>()?;
    requests.add_class::<LibraryType>()?;
    requests.add_class::<NewCdnaMeasurement>()?;
    requests.add_class::<NewCdna>()?;
    requests.add_class::<nucleic_acid::library::MeasurementData>()?;
    requests.add_class::<NewLibraryMeasurement>()?;
    requests.add_class::<NewLibrary>()?;

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
