use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use scamplers::{
    db::models::{
        WasmPythonOrderBy,
        chromium_run::{
            ChromiumRunQuery, NewOcmChromiumRun, NewOcmGems, NewPoolMultiplexChromiumRun,
            NewPoolMultiplexGems, NewSingleplexChipLoading, NewSingleplexChromiumRun,
            NewSingleplexGems,
        },
        institution::{InstitutionQuery, NewInstitution},
        lab::{LabQuery, NewLab},
        nucleic_acid::{
            cdna::{CdnaQuery, NewCdna, NewCdnaGroup, NewCdnaMeasurement},
            common::ElectrophoreticMeasurementData,
            library::{self, LibraryQuery, NewLibrary, NewLibraryMeasurement},
        },
        person::{NewPerson, PersonQuery, UserRole},
        sequencing_run::{NewSequencingRun, SequencingRunQuery},
        specimen::{
            self, Species, SpecimenQuery, SpecimenType,
            block::{
                BlockFixative, FixedBlockEmbeddingMatrix, FrozenBlockEmbeddingMatrix,
                NewFixedBlock, NewFrozenBlock,
            },
            common::{ComplianceCommitteeType, NewCommitteeApproval, NewSpecimenMeasurement},
            tissue::{NewCryopreservedTissue, NewFixedTissue, NewFrozenTissue, TissueFixative},
            virtual_::{NewVirtualSpecimen, SuspensionFixative},
        },
        suspension::{
            common::{BiologicalMaterial, CellCountingMethod, SuspensionMeasurementFields},
            pool::{NewSuspensionPool, NewSuspensionPoolMeasurement, SuspensionPoolQuery},
            suspension::{NewSuspension, NewSuspensionMeasurement, SuspensionQuery},
        },
        tenx_assay::chromium::LibraryType,
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
        register_common_submodule(module)?,
        register_create_submodule(module)?,
        register_query_submodule(module)?,
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

macro_rules! add_classes {
    ($module:expr, $($class:ty),*) => {{
        $(
            $module.add_class::<$class>()?;
        )*
    }};
}

fn register_errors_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let errors_module =
        PyModule::new_scamplepy_module(parent.py(), scamplers::ERRORS_SUBMODULE_NAME)?;

    add_classes!(
        errors_module,
        ClientError,
        DuplicateResourceError,
        InvalidReferenceError,
        ResourceNotFoundError,
        InvalidDataError,
        MalformedRequestError,
        PermissionDeniedError,
        ServerError,
        CdnaGemsError,
        CdnaLibraryTypeError,
        DatasetCmdlineError,
        DatasetNMetricsFilesError,
        DatasetMetricsFileParseError,
        InvalidMeasurementError,
        ScamplersErrorResponse
    );

    errors_module.add_class::<ScamplersErrorResponse>()?;

    parent.add_submodule(&errors_module)?;

    Ok((scamplers::ERRORS_SUBMODULE_NAME, errors_module))
}

fn register_common_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let common_submodule =
        PyModule::new_scamplepy_module(parent.py(), scamplers::COMMON_SUBMODULE_NAME)?;

    add_classes!(
        common_submodule,
        MassUnit,
        VolumeUnit,
        LengthUnit,
        CellCountingMethod,
        SuspensionMeasurementFields,
        BiologicalMaterial,
        ElectrophoreticMeasurementData,
        LibraryType,
        library::MeasurementData
    );

    parent.add_submodule(&common_submodule)?;

    Ok((scamplers::COMMON_SUBMODULE_NAME, common_submodule))
}

fn register_create_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let create_submodule = PyModule::new(parent.py(), scamplers::CREATE_SUBMODULE_NAME)?;

    add_classes!(
        create_submodule,
        NewInstitution,
        UserRole,
        NewPerson,
        NewLab,
        NewFixedBlock,
        FixedBlockEmbeddingMatrix,
        NewFrozenBlock,
        BlockFixative,
        FrozenBlockEmbeddingMatrix,
        NewCryopreservedTissue,
        NewFixedTissue,
        NewFrozenTissue,
        TissueFixative,
        NewVirtualSpecimen,
        SuspensionFixative,
        Species,
        specimen::common::MeasurementData,
        NewSequencingRun,
        NewSpecimenMeasurement,
        NewCommitteeApproval,
        ComplianceCommitteeType,
        NewSuspension,
        NewSuspensionMeasurement,
        NewSuspensionPool,
        NewSuspensionPoolMeasurement,
        NewSingleplexChromiumRun,
        NewSingleplexGems,
        NewSingleplexChipLoading,
        NewOcmChromiumRun,
        NewOcmGems,
        NewPoolMultiplexChromiumRun,
        NewPoolMultiplexGems,
        NewCdnaMeasurement,
        NewCdna,
        NewCdnaGroup,
        NewLibraryMeasurement,
        NewLibrary
    );

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

    // create_submodule.
    // add_class::<nucleic_acid::common::ElectrophoreticMeasurementData>()?
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

    // create_submodule.add_class::<dataset::chromium::CellrangerarcCountDataset>()?
    // ; create_submodule.
    // add_class::<dataset::chromium::CellrangerCountDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangerMultiDataset>()?;
    // create_submodule.add_class::<dataset::chromium::CellrangerVdjDataset>()?;
    // create_submodule.
    // add_class::<dataset::chromium::CellrangeratacCountDataset>()?;

    parent.add_submodule(&create_submodule)?;

    Ok((scamplers::CREATE_SUBMODULE_NAME, create_submodule))
}

fn register_query_submodule<'a>(parent: &'a Bound<PyModule>) -> PyResult<ModuleWithName<'a>> {
    let query_submodule = PyModule::new(parent.py(), scamplers::QUERY_SUBMODULE_NAME)?;

    add_classes!(
        query_submodule,
        WasmPythonOrderBy,
        InstitutionQuery,
        PersonQuery,
        LabQuery,
        SpecimenType,
        SpecimenQuery,
        SequencingRunQuery,
        SuspensionQuery,
        SuspensionPoolQuery,
        ChromiumRunQuery,
        CdnaQuery,
        LibraryQuery
    );

    parent.add_submodule(&query_submodule)?;

    Ok((scamplers::QUERY_SUBMODULE_NAME, query_submodule))
}

define_stub_info_gatherer!(stub_info);
