use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use scamplers::{
    db::models::{
        institution::NewInstitution,
        person::{NewPerson, UserRole},
        units::{LengthUnit, MassUnit, VolumeUnit},
    },
    result::{
        CdnaGemsError, CdnaLibraryTypeError, ClientError, DatasetCmdlineError,
        DatasetMetricsFileParseError, DatasetNMetricsFilesError, DuplicateResourceError,
        InvalidDataError, InvalidMeasurementError, InvalidReferenceError, MalformedRequestError,
        PermissionDeniedError, ResourceNotFoundError, ScamplersErrorResponse, ServerError,
    },
};

const ERRORS_SUBMODULE_NAME: &'static str = "errors";
const REQUESTS_SUBMODULE_NAME: &'static str = "requests";

fn errors_submodule(parent: &Bound<PyModule>) -> PyResult<()> {
    let errors = PyModule::new(parent.py(), ERRORS_SUBMODULE_NAME)?;

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

    errors.add_class::<ScamplersErrorResponse>()?;

    parent.add_submodule(&errors)?;

    Ok(())
}

fn requests_create_subsubmodule(parent: &Bound<PyModule>) -> PyResult<()> {
    let create = PyModule::new(parent.py(), "create")?;

    create.add_class::<NewInstitution>()?;

    create.add_class::<UserRole>()?;
    create.add_class::<NewPerson>()?;

    // requests.add_class::<NewLab>()?;

    // requests.add_class::<NewFixedBlock>()?;
    // requests.add_class::<FixedBlockEmbeddingMatrix>()?;
    // requests.add_class::<NewFrozenBlock>()?;
    // requests.add_class::<BlockFixative>()?;
    // requests.add_class::<FrozenBlockEmbeddingMatrix>()?;
    // requests.add_class::<NewCryopreservedTissue>()?;
    // requests.add_class::<NewFixedTissue>()?;
    // requests.add_class::<NewFrozenTissue>()?;
    // requests.add_class::<TissueFixative>()?;
    // requests.add_class::<NewVirtualSpecimen>()?;
    // requests.add_class::<SuspensionFixative>()?;
    // requests.add_class::<Species>()?;
    // requests.add_class::<specimen::common::MeasurementData>()?;
    // requests.add_class::<NewSpecimenMeasurement>()?;
    // requests.add_class::<NewCommitteeApproval>()?;
    // requests.add_class::<ComplianceCommitteeType>()?;

    // requests.add_class::<suspension::MeasurementDataCore>()?;
    // requests.add_class::<BiologicalMaterial>()?;
    // requests.add_class::<CellCountingMethod>()?;
    // requests.add_class::<NewSuspension>()?;
    // requests.add_class::<NewSuspensionMeasurement>()?;
    // requests.add_class::<NewSuspensionPool>()?;
    // requests.add_class::<NewSuspensionPoolMeasurement>()?;

    // requests.add_class::<NewSingleplexChromiumRun>()?;
    // requests.add_class::<NewSingleplexGems>()?;
    // requests.add_class::<NewSingleplexChipLoading>()?;
    // requests.add_class::<SingleplexChromiumChip>()?;
    // requests.add_class::<NewOcmChromiumRun>()?;
    // requests.add_class::<NewOcmGems>()?;
    // requests.add_class::<NewOcmChipLoading>()?;
    // requests.add_class::<OcmChromiumChip>()?;
    // requests.add_class::<NewPoolMultiplexChromiumRun>()?;
    // requests.add_class::<NewPoolMultiplexGems>()?;
    // requests.add_class::<NewPoolMultiplexChipLoading>()?;
    // requests.add_class::<PoolMultiplexChromiumChip>()?;

    // requests.add_class::<LibraryType>()?;
    // requests.add_class::<NewLibraryTypeSpecification>()?;

    // requests.add_class::<nucleic_acid::common::ElectrophoreticMeasurementData>()?;
    // requests.add_class::<nucleic_acid::common::Concentration>()?;
    // requests.add_class::<NewCdnaMeasurement>()?;
    // requests.add_class::<NewCdna>()?;
    // requests.add_class::<NewCdnaGroup>()?;
    // requests.add_class::<nucleic_acid::library::MeasurementData>()?;
    // requests.add_class::<NewLibraryMeasurement>()?;
    // requests.add_class::<NewLibrary>()?;

    // requests.add_class::<SingleRowCsvMetricsFile>()?;
    // requests.add_class::<MultiRowCsvMetricsFile>()?;
    // requests.add_class::<JsonMetricsFile>()?;

    // requests.add_class::<dataset::chromium::CellrangerarcCountDataset>()?;
    // requests.add_class::<dataset::chromium::CellrangerCountDataset>()?;
    // requests.add_class::<dataset::chromium::CellrangerMultiDataset>()?;
    // requests.add_class::<dataset::chromium::CellrangerVdjDataset>()?;
    // requests.add_class::<dataset::chromium::CellrangeratacCountDataset>()?;

    parent.add_submodule(&create)?;

    Ok(())
}

fn requests_submodule(parent: &Bound<PyModule>) -> PyResult<()> {
    let requests = PyModule::new(parent.py(), REQUESTS_SUBMODULE_NAME)?;

    requests_create_subsubmodule(&requests)?;

    parent.add_submodule(&requests)?;

    Ok(())
}

fn units_submodule(parent: &Bound<PyModule>) -> PyResult<()> {
    let units = PyModule::new(parent.py(), "units")?;

    units.add_class::<MassUnit>()?;
    units.add_class::<VolumeUnit>()?;
    units.add_class::<LengthUnit>()?;

    parent.add_submodule(&units)?;

    Ok(())
}

#[pymodule]
fn scamplepy(module: &Bound<'_, PyModule>) -> PyResult<()> {
    use scamplers::client::ScamplersClient;

    module.add_class::<ScamplersClient>()?;

    errors_submodule(module)?;
    requests_submodule(module)?;
    units_submodule(module)?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
