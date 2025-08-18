use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use scamplers::{
    db::models::{
        institution::NewInstitution,
        lab::NewLab,
        person::{NewPerson, UserRole},
        specimen::{
            self, Species,
            block::{
                BlockFixative, FixedBlockEmbeddingMatrix, FrozenBlockEmbeddingMatrix,
                NewFixedBlock, NewFrozenBlock,
            },
            tissue::{NewCryopreservedTissue, NewFixedTissue, NewFrozenTissue, TissueFixative},
            virtual_::{NewVirtualSpecimen, SuspensionFixative},
        },
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

    create_submodule.add_class::<NewLab>()?;

    create_submodule.add_class::<NewFixedBlock>()?;
    create_submodule.add_class::<FixedBlockEmbeddingMatrix>()?;
    create_submodule.add_class::<NewFrozenBlock>()?;
    create_submodule.add_class::<BlockFixative>()?;
    create_submodule.add_class::<FrozenBlockEmbeddingMatrix>()?;
    create_submodule.add_class::<NewCryopreservedTissue>()?;
    create_submodule.add_class::<NewFixedTissue>()?;
    create_submodule.add_class::<NewFrozenTissue>()?;
    create_submodule.add_class::<TissueFixative>()?;
    create_submodule.add_class::<NewVirtualSpecimen>()?;
    create_submodule.add_class::<SuspensionFixative>()?;
    create_submodule.add_class::<Species>()?;
    create_submodule.add_class::<specimen::common::MeasurementData>()?;
    create_submodule.add_class::<NewSpecimenMeasurement>()?;
    create_submodule.add_class::<NewCommitteeApproval>()?;
    create_submodule.add_class::<ComplianceCommitteeType>()?;

    // create_submodule.add_class::<suspension::MeasurementDataCore>()?;
    // create_submodule.add_class::<BiologicalMaterial>()?;
    // create_submodule.add_class::<CellCountingMethod>()?;
    // create_submodule.add_class::<NewSuspension>()?;
    // create_submodule.add_class::<NewSuspensionMeasurement>()?;
    // create_submodule.add_class::<NewSuspensionPool>()?;
    // create_submodule.add_class::<NewSuspensionPoolMeasurement>()?;

    // create_submodule.add_class::<NewSingleplexChromiumRun>()?;
    // create_submodule.add_class::<NewSingleplexGems>()?;
    // create_submodule.add_class::<NewSingleplexChipLoading>()?;
    // create_submodule.add_class::<SingleplexChromiumChip>()?;
    // create_submodule.add_class::<NewOcmChromiumRun>()?;
    // create_submodule.add_class::<NewOcmGems>()?;
    // create_submodule.add_class::<NewOcmChipLoading>()?;
    // create_submodule.add_class::<OcmChromiumChip>()?;
    // create_submodule.add_class::<NewPoolMultiplexChromiumRun>()?;
    // create_submodule.add_class::<NewPoolMultiplexGems>()?;
    // create_submodule.add_class::<NewPoolMultiplexChipLoading>()?;
    // create_submodule.add_class::<PoolMultiplexChromiumChip>()?;

    // create_submodule.add_class::<LibraryType>()?;
    // create_submodule.add_class::<NewLibraryTypeSpecification>()?;

    // create_submodule.add_class::<nucleic_acid::common::ElectrophoreticMeasurementData>()?
    // ; create_submodule.add_class::<nucleic_acid::common::Concentration>()?;
    // create_submodule.add_class::<NewCdnaMeasurement>()?;
    // create_submodule.add_class::<NewCdna>()?;
    // create_submodule.add_class::<NewCdnaGroup>()?;
    // create_submodule.add_class::<nucleic_acid::library::MeasurementData>()?;
    // create_submodule.add_class::<NewLibraryMeasurement>()?;
    // create_submodule.add_class::<NewLibrary>()?;

    // create_submodule.add_class::<SingleRowCsvMetricsFile>()?;
    // create_submodule.add_class::<MultiRowCsvMetricsFile>()?;
    // create_submodule.add_class::<JsonMetricsFile>()?;

    // create_submodule.add_class::<dataset::chromium::CellrangerarcCountDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangerCountDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangerMultiDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangerVdjDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangeratacCountDataset>()?;

    parent.add_submodule(&create_submodule)?;

    Ok((scamplers::CREATE_SUBMODULE_NAME, create_submodule))
}

define_stub_info_gatherer!(stub_info);
