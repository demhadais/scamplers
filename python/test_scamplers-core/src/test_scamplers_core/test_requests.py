from datetime import UTC, datetime
from pathlib import Path
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

TEST_DATA_DIR = Path(__file__).parent / "test-data"

WEB_SUMMARY = (TEST_DATA_DIR / "web-summary.html").read_text()
SINGLE_ROW_CSV = SingleRowCsvMetricsFile(
    filename="summary.csv",
    raw_contents=(TEST_DATA_DIR / "single-row.csv").read_text(),
)
CELLRANGERATAC_COUNT_JSON = JsonMetricsFile(
    filename="summary.json",
    raw_contents=(TEST_DATA_DIR / "cellranger-atac_count.json").read_text(),
)
CELLRANGER_MULTI_CSV = MultiRowCsvMetricsFile(
    filename="sample/summary.csv",
    raw_contents=(TEST_DATA_DIR / "cellranger_multi.csv").read_text(),
)


def new_institution() -> NewInstitution:
    return NewInstitution(id=ID, name="institution")


@pytest.fixture
def new_institution_fixture() -> NewInstitution:
    return new_institution()


def new_person(institution_id: UUID = ID) -> NewPerson:
    return NewPerson(
        name="ahmed",
        email="ahmed.said@jax.org",
        institution_id=institution_id,
        roles=[UserRole.AppAdmin],
    )


@pytest.fixture
def new_person_fixture() -> NewPerson:
    return new_person()


def new_lab(pi_id: UUID = ID) -> NewLab:
    return NewLab(name="lab", pi_id=pi_id, delivery_dir="delivery")


@pytest.fixture
def new_lab_fixture() -> NewLab:
    return new_lab()


def _specimen_dv200() -> SpecimenMeasurementData:
    return SpecimenMeasurementData.Dv200(
        measured_at=TIME, instrument_name="mayonnaise", value=0.5
    )


def _specimen_rin() -> SpecimenMeasurementData:
    return SpecimenMeasurementData.Rin(
        measured_at=TIME, instrument_name="mayonnaise", value=5
    )


def _new_committee_approval() -> NewCommitteeApproval:
    return NewCommitteeApproval(
        institution_id=ID,
        compliance_identifier="compliance",
        committee_type=ComplianceCommitteeType.Ibc,
    )


def new_fixed_block(person_id: UUID = ID, lab_id: UUID = ID) -> NewFixedBlock:
    return NewFixedBlock(
        readable_id="fixedblock",
        name="f",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.HomoSapiens],
        measurements=[
            NewSpecimenMeasurement(measured_by=person_id, data=m)
            for m in [_specimen_dv200(), _specimen_rin()]
        ],
        committee_approvals=[_new_committee_approval()],
        embedded_in=FixedBlockEmbeddingMatrix.Paraffin,
        fixative=BlockFixative.FormaldehydeDerivative,
    )


@pytest.fixture
def new_fixed_block_fixture() -> NewFixedBlock:
    return new_fixed_block()


def new_frozen_block(person_id: UUID = ID, lab_id: UUID = ID) -> NewFrozenBlock:
    return NewFrozenBlock(
        readable_id="frozenblock",
        name="f",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.MusMusculus],
        embedded_in=FrozenBlockEmbeddingMatrix.CarboxymethylCellulose,
    )


@pytest.fixture
def new_frozen_block_fixture() -> NewFrozenBlock:
    return new_frozen_block()


def new_cryopreserved_tissue(
    person_id: UUID = ID, lab_id: UUID = ID
) -> NewCryopreservedTissue:
    return NewCryopreservedTissue(
        readable_id="cryopreservedtissue",
        name="c",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.RattusNorvegicus],
    )


@pytest.fixture
def new_cryopreserved_tissue_fixture() -> NewCryopreservedTissue:
    return new_cryopreserved_tissue()


def new_fixed_tissue(person_id: UUID = ID, lab_id: UUID = ID) -> NewFixedTissue:
    return NewFixedTissue(
        readable_id="fixedtissue",
        name="f",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.HomoSapiens],
        fixative=TissueFixative.DithiobisSuccinimidylropionate,
    )


@pytest.fixture
def new_fixed_tissue_fixture() -> NewFixedTissue:
    return new_fixed_tissue()


def new_frozen_tissue(person_id: UUID = ID, lab_id: UUID = ID) -> NewFrozenTissue:
    return NewFrozenTissue(
        readable_id="frozentissue",
        name="f",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.CallithrixJacchus],
    )


@pytest.fixture
def new_frozen_tissue_fixture() -> NewFrozenTissue:
    return new_frozen_tissue()


def new_virtual_specimen(person_id: UUID = ID, lab_id: UUID = ID) -> NewVirtualSpecimen:
    return NewVirtualSpecimen(
        readable_id="virtualspecimen",
        name="v",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.DrosophilaMelanogaster],
        fixative=SuspensionFixative.FormaldehydeDerivative,
    )


@pytest.fixture
def new_virtual_specimen_fixture() -> NewVirtualSpecimen:
    return new_virtual_specimen()


def _suspension_concentration() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.Concentration(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, VolumeUnit.Microliter),
        instrument_name="",
        counting_method=CellCountingMethod.Aopi,
    )


def _suspension_mean_diameter() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.MeanDiameter(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, LengthUnit.Micrometer),
        instrument_name="",
    )


def _suspension_viability() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.Viability(
        measured_at=TIME, value=0, instrument_name=""
    )


def _suspension_volume() -> SuspensionMeasurementDataCommon:
    return SuspensionMeasurementDataCommon.Volume(
        measured_at=TIME, value=0, unit=VolumeUnit.Microliter
    )


def _all_suspension_measurement_data_common() -> list[SuspensionMeasurementDataCommon]:
    return [
        _suspension_concentration(),
        _suspension_mean_diameter(),
        _suspension_viability(),
        _suspension_volume(),
    ]


def _new_suspension_measurements(
    measured_by: UUID = ID,
) -> list[NewSuspensionMeasurement]:
    return [
        NewSuspensionMeasurement(
            measured_by=measured_by, data=m, is_post_hybridization=True
        )
        for m in _all_suspension_measurement_data_common()
    ]


def new_suspension(
    readable_id: str = "suspension",
    parent_specimen_id: UUID = ID,
    person_id: UUID = ID,
    multiplexing_tag_id: UUID | None = None,
) -> NewSuspension:
    return NewSuspension(
        readable_id=readable_id,
        parent_specimen_id=parent_specimen_id,
        biological_material=BiologicalMaterial.Cells,
        created_at=TIME,
        target_cell_recovery=0,
        target_reads_per_cell=0,
        preparer_ids=[person_id],
        measurements=_new_suspension_measurements(person_id),
        multiplexing_tag_id=multiplexing_tag_id,
    )


@pytest.fixture
def new_suspension_fixture() -> NewSuspension:
    return new_suspension()


def _new_suspension_pool_measurements(
    measured_by: UUID = ID,
) -> list[NewSuspensionPoolMeasurement]:
    return [
        NewSuspensionPoolMeasurement(
            measured_by=measured_by, data=m, is_post_storage=True
        )
        for m in _all_suspension_measurement_data_common()
    ]


def new_suspension_pool(
    person_id: UUID = ID,
    parent_specimen_ids: list[UUID] = [ID, ID],
) -> NewSuspensionPool:
    return NewSuspensionPool(
        readable_id="suspensionpool",
        name="p",
        pooled_at=TIME,
        suspensions=[
            new_suspension(
                readable_id=f"suspension{i}",
                person_id=person_id,
                parent_specimen_id=parent_id,
            )
            for i, parent_id in enumerate(parent_specimen_ids)
        ],
        preparer_ids=[person_id],
        measurements=_new_suspension_pool_measurements(person_id),
    )


@pytest.fixture
def new_suspension_pool_fixture() -> NewSuspensionPool:
    return new_suspension_pool()


def new_singleplex_chromium_run(
    run_by: UUID = ID,
    suspension_id: UUID = ID,
    chemistry: str = "chemistry",
    gems_readable_id: str = "gems",
) -> NewSingleplexChromiumRun:
    return NewSingleplexChromiumRun(
        readable_id="singleplexchromiumrun",
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        chip=SingleplexChromiumChip.Gemx3p,
        gems=[
            NewSingleplexGems(
                readable_id=gems_readable_id,
                chemistry=chemistry,
                loading=NewSingleplexChipLoading(
                    suspension_id=suspension_id,
                    suspension_volume_loaded=_suspension_volume(),
                    buffer_volume_loaded=_suspension_volume(),
                ),
            )
        ],
    )


@pytest.fixture
def new_singleplex_chromium_run_fixture() -> NewSingleplexChromiumRun:
    return new_singleplex_chromium_run()


def new_pool_multiplex_chromium_run(
    run_by: UUID = ID, suspension_pool_id: UUID = ID
) -> NewPoolMultiplexChromiumRun:
    return NewPoolMultiplexChromiumRun(
        readable_id="poolmultiplexchromiumrun",
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        chip=PoolMultiplexChromiumChip.GemxFx,
        gems=[
            NewPoolMultiplexGems(
                readable_id="",
                chemistry="",
                loading=NewPoolMultiplexChipLoading(
                    suspension_pool_id=suspension_pool_id,
                    suspension_volume_loaded=_suspension_volume(),
                    buffer_volume_loaded=_suspension_volume(),
                ),
            )
        ],
    )


@pytest.fixture
def new_pool_multiplex_chromium_run_fixture() -> NewPoolMultiplexChromiumRun:
    return new_pool_multiplex_chromium_run()


def new_ocm_chromium_run(
    run_by: UUID = ID, suspension_id: UUID = ID
) -> NewOcmChromiumRun:
    return NewOcmChromiumRun(
        readable_id="ocmchromiumrun",
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        chip=OcmChromiumChip.GemxOcm3p,
        gems=[
            NewOcmGems(
                readable_id="",
                chemistry="",
                loading=[
                    NewOcmChipLoading(
                        suspension_id=suspension_id,
                        suspension_volume_loaded=_suspension_volume(),
                        buffer_volume_loaded=_suspension_volume(),
                    )
                ],
            )
        ],
    )


@pytest.fixture
def new_ocm_chromium_run_fixture() -> NewOcmChromiumRun:
    return new_ocm_chromium_run()


def _electrophoretic_measurement_data() -> ElectrophoreticMeasurementData:
    return ElectrophoreticMeasurementData(
        measured_at=TIME,
        instrument_name="mayonnaise",
        mean_library_size_bp=0,
        sizing_range=(0, 0),
        concentration_value=0,
        concentration_unit=(MassUnit.Nanogram, VolumeUnit.Microliter),
    )


def new_cdna(gems_id: UUID = ID, person_id: UUID = ID) -> NewCdna:
    return NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="cdna",
        prepared_at=TIME,
        gems_id=gems_id,
        n_amplification_cycles=0,
        measurements=[
            NewCdnaMeasurement(
                measured_by=person_id,
                data=_electrophoretic_measurement_data(),
            )
        ],
        preparer_ids=[person_id],
    )


@pytest.fixture
def new_cdna_fixture() -> NewCdna:
    return new_cdna()


def _library_electrophoretic_measurement() -> LibraryMeasurementData:
    return LibraryMeasurementData.Electrophoretic(_electrophoretic_measurement_data())


def _library_fluormetric_measurement() -> LibraryMeasurementData:
    return LibraryMeasurementData.Fluorometric(
        measured_at=TIME,
        instrument_name="mayonnaise",
        concentration=NucleicAcidConcentration(
            value=0, unit=(MassUnit.Picogram, VolumeUnit.Microliter)
        ),
    )


def new_library(cdna_id: UUID = ID, person_id: UUID = ID) -> NewLibrary:
    return NewLibrary(
        readable_id="library",
        cdna_id=cdna_id,
        number_of_sample_index_pcr_cycles=0,
        target_reads_per_cell=0,
        prepared_at=TIME,
        preparer_ids=[person_id],
        measurements=[
            NewLibraryMeasurement(measured_by=person_id, data=m)
            for m in [
                _library_electrophoretic_measurement(),
                _library_fluormetric_measurement(),
            ]
        ],
    )


@pytest.fixture
def new_library_fixture() -> NewLibrary:
    return new_library()


def new_cellrangerarc_count_dataset(
    lab_id: UUID = ID, gems_id: UUID = ID
) -> CellrangerarcCountDataset:
    return CellrangerarcCountDataset(
        name="dataset",
        lab_id=lab_id,
        data_path="data",
        delivered_at=TIME,
        gems_id=gems_id,
        web_summary=WEB_SUMMARY,
        metrics=SINGLE_ROW_CSV,
    )


@pytest.fixture
def new_cellrangerarc_count_dataset_fixture() -> CellrangerarcCountDataset:
    return new_cellrangerarc_count_dataset()


def new_cellrangeratac_count_dataset(
    lab_id: UUID = ID, gems_id: UUID = ID
) -> CellrangeratacCountDataset:
    return CellrangeratacCountDataset(
        name="dataset",
        lab_id=lab_id,
        data_path="data",
        delivered_at=TIME,
        gems_id=gems_id,
        web_summary=WEB_SUMMARY,
        metrics=CELLRANGERATAC_COUNT_JSON,
    )


@pytest.fixture
def new_cellrangeratac_count_dataset_fixture() -> CellrangeratacCountDataset:
    return new_cellrangeratac_count_dataset()


def new_cellranger_count_dataset(
    lab_id: UUID = ID, gems_id: UUID = ID
) -> CellrangerCountDataset:
    return CellrangerCountDataset(
        name="",
        lab_id=lab_id,
        data_path="data",
        delivered_at=TIME,
        gems_id=gems_id,
        web_summary=WEB_SUMMARY,
        metrics=SINGLE_ROW_CSV,
    )


@pytest.fixture
def new_cellranger_count_dataset_fixture() -> CellrangerCountDataset:
    return new_cellranger_count_dataset()


def new_cellranger_multi_dataset(
    lab_id: UUID = ID, gems_id: UUID = ID, n_samples: int = 1
) -> CellrangerMultiDataset:
    return CellrangerMultiDataset(
        name="dataset",
        lab_id=lab_id,
        data_path="data",
        delivered_at=TIME,
        gems_id=gems_id,
        web_summary=WEB_SUMMARY,
        metrics=[CELLRANGER_MULTI_CSV for i in range(n_samples)],
    )


@pytest.fixture
def new_cellranger_multi_dataset_fixture() -> CellrangerMultiDataset:
    return new_cellranger_multi_dataset()


def new_cellranger_vdj_dataset(
    lab_id: UUID = ID, gems_id: UUID = ID
) -> CellrangerVdjDataset:
    return CellrangerVdjDataset(
        name="dataset",
        lab_id=lab_id,
        data_path="data",
        delivered_at=TIME,
        gems_id=gems_id,
        web_summary="",
        metrics=SINGLE_ROW_CSV,
    )


@pytest.fixture
def new_cellranger_vdj_dataset_fixture() -> CellrangerVdjDataset:
    return new_cellranger_vdj_dataset()


@pytest.mark.parametrize(
    "data, key, expected_value",
    [
        ("new_institution_fixture", "name", "institution"),
        ("new_person_fixture", "name", "ahmed"),
        ("new_lab_fixture", "name", "lab"),
        ("new_fixed_block_fixture", "type", "fixed_block"),
        ("new_frozen_block_fixture", "type", "frozen_block"),
        ("new_cryopreserved_tissue_fixture", "type", "cryopreserved_tissue"),
        ("new_fixed_tissue_fixture", "type", "fixed_tissue"),
        ("new_frozen_tissue_fixture", "type", "frozen_tissue"),
        ("new_virtual_specimen_fixture", "type", "suspension"),
        ("new_suspension_fixture", "target_cell_recovery", 0),
        ("new_suspension_pool_fixture", "preparer_ids", [ID]),
        ("new_singleplex_chromium_run_fixture", "plexy", "singleplex"),
        ("new_pool_multiplex_chromium_run_fixture", "plexy", "pool_multiplex"),
        ("new_ocm_chromium_run_fixture", "plexy", "ocm"),
        ("new_cdna_fixture", "preparer_ids", [ID]),
        ("new_library_fixture", "cdna_id", ID),
        ("new_cellrangerarc_count_dataset_fixture", "cmdline", "cellranger-arc count"),
        (
            "new_cellrangeratac_count_dataset_fixture",
            "cmdline",
            "cellranger-atac count",
        ),
        ("new_cellranger_count_dataset_fixture", "cmdline", "cellranger count"),
        ("new_cellranger_multi_dataset_fixture", "cmdline", "cellranger multi"),
        ("new_cellranger_vdj_dataset_fixture", "cmdline", "cellranger vdj"),
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
