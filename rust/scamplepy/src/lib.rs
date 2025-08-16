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

#[pymodule]
fn scamplepy(module: &Bound<'_, PyModule>) -> PyResult<()> {
    use scamplers::client::ScamplersClient;

    module.add_class::<ScamplersClient>()?;

    let submodules = [
        register_errors_submodule(module)?,
        register_units_submodule(module)?,
        register_create_submodule(module)?,
    ];

    let python = module.py();

    for (submodule_name, submodule) in &submodules {
        add_module_to_sys_modules(python, submodule_name, submodule)?;
    }

    Ok(())
}

fn add_module_to_sys_modules(
    python: Python<'_>,
    module_name: &str,
    submodule: &Bound<'_, PyModule>,
) -> anyhow::Result<()> {
    let sys_modules = python.import("sys")?.getattr("modules")?;

    sys_modules.set_item(module_name, submodule)?;

    Ok(())
}

type ModuleWithName<'a> = (&'static str, Bound<'a, PyModule>);

trait NewScamplepyModule {
    fn new_scamplepy_module<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyModule>>;
}

impl NewScamplepyModule for PyModule {
    fn new_scamplepy_module<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyModule>> {
        Self::new(py, name.trim_start_matches("scamplepy."))
    }
}

fn register_errors_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let errors_module =
        PyModule::new_scamplepy_module(parent.py(), scamplers::ERRORS_SUBMODULE_NAME)?;

    errors_module.add_class::<ClientError>()?;
    errors_module.add_class::<DuplicateResourceError>()?;
    errors_module.add_class::<InvalidReferenceError>()?;
    errors_module.add_class::<ResourceNotFoundError>()?;
    errors_module.add_class::<InvalidDataError>()?;
    errors_module.add_class::<MalformedRequestError>()?;
    errors_module.add_class::<PermissionDeniedError>()?;
    errors_module.add_class::<ServerError>()?;
    errors_module.add_class::<CdnaGemsError>()?;
    errors_module.add_class::<CdnaLibraryTypeError>()?;
    errors_module.add_class::<DatasetCmdlineError>()?;
    errors_module.add_class::<DatasetNMetricsFilesError>()?;
    errors_module.add_class::<DatasetMetricsFileParseError>()?;
    errors_module.add_class::<InvalidMeasurementError>()?;

    errors_module.add_class::<ScamplersErrorResponse>()?;

    parent.add_submodule(&errors_module)?;

    Ok((scamplers::ERRORS_SUBMODULE_NAME, errors_module))
}

fn register_units_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let units_module =
        PyModule::new_scamplepy_module(parent.py(), scamplers::COMMON_SUBMODULE_NAME)?;

    units_module.add_class::<MassUnit>()?;
    units_module.add_class::<VolumeUnit>()?;
    units_module.add_class::<LengthUnit>()?;

    parent.add_submodule(&units_module)?;

    Ok((scamplers::COMMON_SUBMODULE_NAME, units_module))
}

fn register_create_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let create_submodule = PyModule::new(parent.py(), scamplers::CREATE_SUBMODULE_NAME)?;

    create_submodule.add_class::<NewInstitution>()?;

    create_submodule.add_class::<UserRole>()?;
    create_submodule.add_class::<NewPerson>()?;

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

    // requests.add_class::<nucleic_acid::common::ElectrophoreticMeasurementData>()?
    // ; requests.add_class::<nucleic_acid::common::Concentration>()?;
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

    parent.add_submodule(&create_submodule)?;

    Ok((scamplers::CREATE_SUBMODULE_NAME, create_submodule))
}

define_stub_info_gatherer!(stub_info);
