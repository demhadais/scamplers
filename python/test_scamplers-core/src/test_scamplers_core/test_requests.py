from datetime import UTC, datetime
from typing_extensions import Any
import pytest
import maturin_import_hook
import json

maturin_import_hook.install()

from scamplers_core.requests import (
    CellCountingMethod,
    CellrangerMultiDataset,
    CellrangerVdjDataset,
    CellrangerarcCountDataset,
    CellrangeratacCountDataset,
    ComplianceCommitteeType,
    ElectrophoreticMeasurementData,
    JsonMetricsFile,
    LengthUnit,
    MassUnit,
    MultiRowCsvMetricsFile,
    NewSuspensionMeasurement,
    FrozenBlockEmbeddingMatrix,
    LibraryMeasurementData,
    NewCdnaMeasurement,
    NewCommitteeApproval,
    NewInstitution,
    NewLibrary,
    NewLibraryMeasurement,
    NewOcmChipLoading,
    NewOcmGems,
    NewPerson,
    NewLab,
    NewFixedBlock,
    NewFrozenBlock,
    NewCryopreservedTissue,
    NewFixedTissue,
    NewFrozenTissue,
    NewPoolMultiplexChipLoading,
    NewPoolMultiplexChromiumRun,
    NewPoolMultiplexGems,
    NewSingleplexChipLoading,
    NewSingleplexGems,
    NewSpecimenMeasurement,
    NewVirtualSpecimen,
    NewSuspension,
    NewSingleplexChromiumRun,
    NewOcmChromiumRun,
    NewCdna,
    FixedBlockEmbeddingMatrix,
    BlockFixative,
    NucleicAcidConcentration,
    OcmChromiumChip,
    PoolMultiplexChromiumChip,
    SingleRowCsvMetricsFile,
    SingleplexChromiumChip,
    SpecimenMeasurementData,
    SuspensionMeasurementDataCommon,
    TissueFixative,
    SuspensionFixative,
    BiologicalMaterial,
    Species,
    LibraryType,
    UserRole,
    VolumeUnit,
    NewSuspensionPool,
    NewSuspensionPoolMeasurement,
    CellrangerCountDataset,
)
from uuid import UUID

ID = UUID(int=0)
TIME = datetime(year=1999, month=1, day=1, tzinfo=UTC)


@pytest.fixture
def new_institution() -> NewInstitution:
    return NewInstitution(id=ID, name="")


@pytest.fixture
def new_person() -> NewPerson:
    return NewPerson(name="", email="", institution_id=ID, roles=[UserRole.AppAdmin])


@pytest.fixture
def new_lab() -> NewLab:
    return NewLab(name="", pi_id=ID, delivery_dir="")


@pytest.fixture
def specimen_dv200() -> SpecimenMeasurementData:
    return SpecimenMeasurementData.Dv200(measured_at=TIME, instrument_name="", value=0)


@pytest.fixture
def specimen_rin() -> SpecimenMeasurementData:
    return SpecimenMeasurementData.Rin(measured_at=TIME, instrument_name="", value=0)


@pytest.fixture
def new_committee_approval() -> NewCommitteeApproval:
    return NewCommitteeApproval(
        institution_id=ID,
        compliance_identifier="",
        committee_type=ComplianceCommitteeType.Ibc,
    )


@pytest.fixture
def new_fixed_block(
    specimen_dv200: SpecimenMeasurementData,
    specimen_rin: SpecimenMeasurementData,
    new_committee_approval: NewCommitteeApproval,
) -> NewFixedBlock:
    return NewFixedBlock(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.HomoSapiens],
        measurements=[
            NewSpecimenMeasurement(measured_by=ID, data=m)
            for m in [specimen_dv200, specimen_rin]
        ],
        committee_approvals=[new_committee_approval],
        embedded_in=FixedBlockEmbeddingMatrix.Paraffin,
        fixative=BlockFixative.FormaldehydeDerivative,
    )


@pytest.fixture
def new_frozen_block() -> NewFrozenBlock:
    return NewFrozenBlock(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.MusMusculus],
        embedded_in=FrozenBlockEmbeddingMatrix.CarboxymethylCellulose,
    )


@pytest.fixture
def new_cryopreserved_tissue() -> NewCryopreservedTissue:
    return NewCryopreservedTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.RattusNorvegicus],
    )


@pytest.fixture
def new_fixed_tissue() -> NewFixedTissue:
    return NewFixedTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.HomoSapiens],
        fixative=TissueFixative.DithiobisSuccinimidylropionate,
    )


@pytest.fixture
def new_frozen_tissue() -> NewFrozenTissue:
    return NewFrozenTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.CallithrixJacchus],
    )


@pytest.fixture
def new_virtual_specimen() -> NewVirtualSpecimen:
    return NewVirtualSpecimen(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.DrosophilaMelanogaster],
        fixative=SuspensionFixative.FormaldehydeDerivative,
    )


@pytest.fixture
def suspension_concentration() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.Concentration(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, VolumeUnit.Microliter),
        instrument_name="",
        counting_method=CellCountingMethod.Aopi,
    )


@pytest.fixture
def suspension_mean_diameter() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.MeanDiameter(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, LengthUnit.Micrometer),
        instrument_name="",
    )


@pytest.fixture
def suspension_viability():
    return SuspensionMeasurementDataCommon.Viability(
        measured_at=TIME, value=0, instrument_name=""
    )


@pytest.fixture
def suspension_volume() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.Volume(
        measured_at=TIME, value=0, unit=VolumeUnit.Microliter
    )


@pytest.fixture
def all_suspension_measurement_data_common(
    suspension_concentration: SuspensionMeasurementDataCommon,
    suspension_mean_diameter: SuspensionMeasurementDataCommon,
    suspension_viability: SuspensionMeasurementDataCommon,
    suspension_volume: SuspensionMeasurementDataCommon,
) -> list[SuspensionMeasurementDataCommon]:
    return [
        suspension_concentration,
        suspension_mean_diameter,
        suspension_viability,
        suspension_volume,
    ]


@pytest.fixture
def new_suspension_measurements(
    all_suspension_measurement_data_common: list[SuspensionMeasurementDataCommon],
) -> list[NewSuspensionMeasurement]:
    return [
        NewSuspensionMeasurement(measured_by=ID, data=m, is_post_hybridization=True)
        for m in all_suspension_measurement_data_common
    ]


@pytest.fixture
def new_suspension(
    new_suspension_measurements: list[NewSuspensionMeasurement],
) -> NewSuspension:
    return NewSuspension(
        readable_id="",
        parent_specimen_id=ID,
        biological_material=BiologicalMaterial.Cells,
        created_at=TIME,
        target_cell_recovery=0,
        target_reads_per_cell=0,
        preparer_ids=[ID],
        measurements=new_suspension_measurements,
    )


@pytest.fixture
def new_suspension_pool_measurements(
    all_suspension_measurement_data_common: list[SuspensionMeasurementDataCommon],
) -> list[NewSuspensionPoolMeasurement]:
    return [
        NewSuspensionPoolMeasurement(measured_by=ID, data=m, is_post_storage=True)
        for m in all_suspension_measurement_data_common
    ]


@pytest.fixture
def new_suspension_pool(
    new_suspension: NewSuspension,
    new_suspension_pool_measurements: list[NewSuspensionPoolMeasurement],
) -> NewSuspensionPool:
    return NewSuspensionPool(
        readable_id="",
        name="",
        pooled_at=TIME,
        suspensions=[new_suspension] * 2,
        preparer_ids=[ID],
        measurements=new_suspension_pool_measurements,
    )


@pytest.fixture
def new_singleplex_chromium_run(
    suspension_volume: SuspensionMeasurementDataCommon,
) -> NewSingleplexChromiumRun:
    return NewSingleplexChromiumRun(
        readable_id="",
        run_at=TIME,
        succeeded=True,
        run_by=ID,
        chip=SingleplexChromiumChip.Gemx3p,
        gems=[
            NewSingleplexGems(
                readable_id="",
                chemistry="",
                loading=NewSingleplexChipLoading(
                    suspension_id=ID,
                    suspension_volume_loaded=suspension_volume,
                    buffer_volume_loaded=suspension_volume,
                ),
            )
        ],
    )


@pytest.fixture
def new_pool_multiplex_chromium_run(
    suspension_volume: SuspensionMeasurementDataCommon,
) -> NewPoolMultiplexChromiumRun:
    return NewPoolMultiplexChromiumRun(
        readable_id="",
        run_at=TIME,
        succeeded=True,
        run_by=ID,
        chip=PoolMultiplexChromiumChip.GemxFx,
        gems=[
            NewPoolMultiplexGems(
                readable_id="",
                chemistry="",
                loading=NewPoolMultiplexChipLoading(
                    suspension_pool_id=ID,
                    suspension_volume_loaded=suspension_volume,
                    buffer_volume_loaded=suspension_volume,
                ),
            )
        ],
    )


@pytest.fixture
def new_ocm_chromium_run(
    suspension_volume: SuspensionMeasurementDataCommon,
) -> NewOcmChromiumRun:
    return NewOcmChromiumRun(
        readable_id="",
        run_at=TIME,
        succeeded=True,
        run_by=ID,
        chip=OcmChromiumChip.GemxOcm3p,
        gems=[
            NewOcmGems(
                readable_id="",
                chemistry="",
                loading=[
                    NewOcmChipLoading(
                        suspension_id=ID,
                        suspension_volume_loaded=suspension_volume,
                        buffer_volume_loaded=suspension_volume,
                    )
                ],
            )
        ],
    )


@pytest.fixture
def electrophoretic_measurement_data() -> ElectrophoreticMeasurementData:
    return ElectrophoreticMeasurementData(
        measured_at=TIME,
        instrument_name="",
        mean_library_size_bp=0,
        sizing_range=(0, 0),
        concentration_value=0,
        concentration_unit=(MassUnit.Nanogram, VolumeUnit.Microliter),
    )


@pytest.fixture
def new_cdna(
    electrophoretic_measurement_data: ElectrophoreticMeasurementData,
) -> NewCdna:
    return NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="",
        prepared_at=TIME,
        gems_id=ID,
        n_amplification_cycles=0,
        measurements=[
            NewCdnaMeasurement(
                measured_by=ID,
                data=electrophoretic_measurement_data,
                cdna_id=ID,
            )
        ],
        preparer_ids=[ID],
    )


@pytest.fixture
def library_electrophoretic_measurement(
    electrophoretic_measurement_data: ElectrophoreticMeasurementData,
) -> LibraryMeasurementData:
    return LibraryMeasurementData.Electrophoretic(electrophoretic_measurement_data)


@pytest.fixture
def library_fluormetric_measurement() -> LibraryMeasurementData:
    return LibraryMeasurementData.Fluorometric(
        measured_at=TIME,
        instrument_name="",
        concentration=NucleicAcidConcentration(
            value=0, unit=(MassUnit.Picogram, VolumeUnit.Microliter)
        ),
    )


@pytest.fixture
def new_library(
    library_electrophoretic_measurement: LibraryMeasurementData,
    library_fluormetric_measurement: LibraryMeasurementData,
) -> NewLibrary:
    return NewLibrary(
        readable_id="",
        cdna_id=ID,
        number_of_sample_index_pcr_cycles=0,
        target_reads_per_cell=0,
        prepared_at=TIME,
        preparer_ids=[ID],
        measurements=[
            NewLibraryMeasurement(measured_by=ID, data=m)
            for m in [
                library_electrophoretic_measurement,
                library_fluormetric_measurement,
            ]
        ],
    )


@pytest.fixture
def new_cellrangerarc_count_dataset() -> CellrangerarcCountDataset:
    return CellrangerarcCountDataset(
        name="",
        lab_id=ID,
        data_path="",
        delivered_at=TIME,
        gems_id=ID,
        web_summary="",
        metrics=SingleRowCsvMetricsFile(filename="", raw_contents=""),
    )


@pytest.fixture
def new_cellrangeratac_count_dataset() -> CellrangeratacCountDataset:
    return CellrangeratacCountDataset(
        name="",
        lab_id=ID,
        data_path="",
        delivered_at=TIME,
        gems_id=ID,
        web_summary="",
        metrics=JsonMetricsFile(filename="", raw_contents=""),
    )


@pytest.fixture
def new_cellranger_count_dataset() -> CellrangerCountDataset:
    return CellrangerCountDataset(
        name="",
        lab_id=ID,
        data_path="",
        delivered_at=TIME,
        gems_id=ID,
        web_summary="",
        metrics=SingleRowCsvMetricsFile(filename="", raw_contents=""),
    )


@pytest.fixture
def new_cellranger_multi_dataset() -> CellrangerMultiDataset:
    return CellrangerMultiDataset(
        name="",
        lab_id=ID,
        data_path="",
        delivered_at=TIME,
        gems_id=ID,
        web_summary="",
        metrics=[MultiRowCsvMetricsFile(filename="", raw_contents="")],
    )


@pytest.fixture
def new_cellranger_vdj_dataset() -> CellrangerVdjDataset:
    return CellrangerVdjDataset(
        name="",
        lab_id=ID,
        data_path="",
        delivered_at=TIME,
        gems_id=ID,
        web_summary="",
        metrics=SingleRowCsvMetricsFile(filename="", raw_contents=""),
    )


@pytest.mark.parametrize(
    "data, key, expected_value",
    [
        ("new_institution", "name", ""),
        ("new_person", "name", ""),
        ("new_lab", "name", ""),
        ("new_fixed_block", "type", "fixed_block"),
        ("new_frozen_block", "type", "frozen_block"),
        ("new_cryopreserved_tissue", "type", "cryopreserved_tissue"),
        ("new_fixed_tissue", "type", "fixed_tissue"),
        ("new_frozen_tissue", "type", "frozen_tissue"),
        ("new_virtual_specimen", "type", "suspension"),
        ("new_suspension", "target_cell_recovery", 0),
        ("new_suspension_pool", "preparer_ids", [ID]),
        ("new_singleplex_chromium_run", "plexy", "singleplex"),
        ("new_pool_multiplex_chromium_run", "plexy", "pool_multiplex"),
        ("new_ocm_chromium_run", "plexy", "ocm"),
        ("new_cdna", "preparer_ids", [ID]),
        ("new_library", "cdna_id", ID),
        ("new_cellrangerarc_count_dataset", "cmdline", "cellranger-arc count"),
        ("new_cellrangeratac_count_dataset", "cmdline", "cellranger-atac count"),
        ("new_cellranger_count_dataset", "cmdline", "cellranger count"),
        ("new_cellranger_multi_dataset", "cmdline", "cellranger multi"),
        ("new_cellranger_vdj_dataset", "cmdline", "cellranger vdj"),
    ],
)
def test_jsonification(
    data: Any, key: str | None, expected_value: Any, request: pytest.FixtureRequest
):
    data = request.getfixturevalue(data)
    json_str = data.to_json()
    pythonized = json.loads(json_str)
    found_value = pythonized[key]

    if not isinstance(expected_value, (str, list, datetime)):
        found_value = type(expected_value)(found_value)

    elif isinstance(expected_value, list) and isinstance(found_value, list):
        for i, v in enumerate(found_value):
            found_value[i] = type(expected_value[0])(found_value[i])

    assert found_value == expected_value

    dataclass = type(data)
    assert dataclass.from_json(json_str) == data
