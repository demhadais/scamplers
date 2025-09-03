from datetime import UTC, datetime
from typing import Any
import pytest
import maturin_import_hook
import json

maturin_import_hook.install()

from scamplepy.common import (
    BiologicalMaterial,
    CellCountingMethod,
    LengthUnit,
    LibraryType,
    MassUnit,
    SuspensionMeasurementFields,
    VolumeUnit,
    ElectrophoreticMeasurementData,
)
from scamplepy.create import (
    # CellrangerMultiDataset,
    # CellrangerVdjDataset,
    # CellrangerarcCountDataset,
    # CellrangeratacCountDataset,
    ComplianceCommitteeType,
    NewCdnaGroup,
    NewSingleplexChipLoading,
    # ElectrophoreticMeasurementData,
    # JsonMetricsFile,
    NewSuspensionMeasurement,
    FrozenBlockEmbeddingMatrix,
    # LibraryMeasurementData,
    # NewCdnaMeasurement,
    NewCommitteeApproval,
    NewInstitution,
    # NewLibrary,
    # NewLibraryMeasurement,
    # NewOcmChipLoading,
    NewOcmGems,
    NewPerson,
    NewLab,
    NewFixedBlock,
    NewFrozenBlock,
    NewCryopreservedTissue,
    NewFixedTissue,
    NewFrozenTissue,
    NewPoolMultiplexChromiumRun,
    NewPoolMultiplexGems,
    NewSingleplexGems,
    NewSpecimenMeasurement,
    NewVirtualSpecimen,
    NewSuspension,
    NewSingleplexChromiumRun,
    NewOcmChromiumRun,
    NewCdna,
    NewCdnaMeasurement,
    FixedBlockEmbeddingMatrix,
    BlockFixative,
    # NucleicAcidConcentration,
    OcmChromiumChip,
    PoolMultiplexChromiumChip,
    # PoolMultiplexChromiumChip,
    # SingleRowCsvMetricsFile,
    SingleplexChromiumChip,
    SpecimenMeasurementData,
    TissueFixative,
    SuspensionFixative,
    Species,
    UserRole,
    NewSuspensionPool,
    NewSuspensionPoolMeasurement,
    # CellrangerCountDataset,
)
from uuid import UUID

ID = UUID(int=0)
TIME = datetime(year=1999, month=1, day=1, tzinfo=UTC)

# SINGLE_ROW_CSV = SingleRowCsvMetricsFile(
#     filename="summary.csv",
#     raw_contents="",
# )
# CELLRANGERATAC_COUNT_JSON = JsonMetricsFile(
#     filename="summary.json",
#     raw_contents="",
# )
# CELLRANGER_MULTI_CSV = MultiRowCsvMetricsFile(
#     filename="sample/summary.csv",
#     raw_contents="",
# )


@pytest.fixture
def new_institution() -> NewInstitution:
    return NewInstitution(id=ID, name="institution")


@pytest.fixture
def new_person(institution_id: UUID = ID) -> NewPerson:
    return NewPerson(
        name="ahmed",
        email="ahmed.said@jax.org",
        institution_id=institution_id,
        roles=[UserRole.AppAdmin],
    )


@pytest.fixture
def new_lab(pi_id: UUID = ID) -> NewLab:
    return NewLab(name="lab", pi_id=pi_id, delivery_dir="delivery")


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


@pytest.fixture
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


def _suspension_concentration() -> SuspensionMeasurementFields:
    return SuspensionMeasurementFields.Concentration(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, VolumeUnit.Microliter),
        instrument_name="",
        counting_method=CellCountingMethod.Aopi,
    )


def _suspension_mean_diameter() -> SuspensionMeasurementFields:
    return SuspensionMeasurementFields.MeanDiameter(
        measured_at=TIME,
        value=0,
        unit=(BiologicalMaterial.Cells, LengthUnit.Micrometer),
        instrument_name="",
    )


def _suspension_viability() -> SuspensionMeasurementFields:
    return SuspensionMeasurementFields.Viability(
        measured_at=TIME, value=0, instrument_name=""
    )


def _suspension_volume() -> SuspensionMeasurementFields:
    return SuspensionMeasurementFields.Volume(
        measured_at=TIME, value=0, unit=VolumeUnit.Microliter
    )


def _all_suspension_measurement_data_common() -> list[SuspensionMeasurementFields]:
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
def new_suspension_fixture():
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


@pytest.fixture
def new_suspension_pool(
    request: pytest.FixtureRequest,
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
                suspension_id=suspension_id,
                suspension_volume_loaded=_suspension_volume(),
                buffer_volume_loaded=_suspension_volume(),
            )
        ],
    )


@pytest.fixture
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
                suspension_pool_id=suspension_pool_id,
                suspension_volume_loaded=_suspension_volume(),
                buffer_volume_loaded=_suspension_volume(),
            )
        ],
    )


@pytest.fixture
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
                    NewSingleplexChipLoading(
                        suspension_id=suspension_id,
                        suspension_volume_loaded=_suspension_volume(),
                        buffer_volume_loaded=_suspension_volume(),
                    )
                ],
            )
        ],
    )


def _electrophoretic_measurement_data() -> ElectrophoreticMeasurementData:
    return ElectrophoreticMeasurementData(
        measured_at=TIME,
        instrument_name="mayonnaise",
        mean_library_size_bp=0,
        sizing_range=(0, 0),
        concentration_value=0,
        concentration_unit=(MassUnit.Nanogram, VolumeUnit.Microliter),
    )


def _new_cdna(gems_id: UUID = ID, person_id: UUID = ID) -> NewCdna:
    return NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="cdna",
        prepared_at=TIME,
        gems_id=gems_id,
        n_amplification_cycles=0,
        volume_mcl=40.0,
        measurements=[
            NewCdnaMeasurement(
                measured_by=person_id,
                data=_electrophoretic_measurement_data(),
            )
        ],
        preparer_ids=[person_id],
    )


@pytest.fixture
def new_cdna_group() -> NewCdnaGroup:
    return NewCdnaGroup.Single(_new_cdna())


# def _library_electrophoretic_measurement() -> LibraryMeasurementData:
#     return LibraryMeasurementData.Electrophoretic(_electrophoretic_measurement_data())


# def _library_fluormetric_measurement() -> LibraryMeasurementData:
#     return LibraryMeasurementData.Fluorometric(
#         measured_at=TIME,
#         instrument_name="mayonnaise",
#         concentration=NucleicAcidConcentration(
#             value=0, unit=(MassUnit.Picogram, VolumeUnit.Microliter)
#         ),
#     )

# @pytest.fixture
# def new_library(cdna_id: UUID = ID, person_id: UUID = ID) -> NewLibrary:
#     return NewLibrary(
#         readable_id="library",
#         cdna_id=cdna_id,
#         number_of_sample_index_pcr_cycles=0,
#         target_reads_per_cell=0,
#         prepared_at=TIME,
#         preparer_ids=[person_id],
#         measurements=[
#             NewLibraryMeasurement(measured_by=person_id, data=m)
#             for m in [
#                 _library_electrophoretic_measurement(),
#                 _library_fluormetric_measurement(),
#             ]
#         ],
#     )

# @pytest.fixture
# def new_cellrangerarc_count_dataset(
#     lab_id: UUID = ID, gems_id: UUID = ID
# ) -> CellrangerarcCountDataset:
#     return CellrangerarcCountDataset(
#         name="dataset",
#         lab_id=lab_id,
#         data_path="data",
#         delivered_at=TIME,
#         gems_id=gems_id,
#         web_summary=WEB_SUMMARY,
#         metrics=SINGLE_ROW_CSV,
#     )


# @pytest.fixture
# def new_cellrangeratac_count_dataset(
#     lab_id: UUID = ID, gems_id: UUID = ID
# ) -> CellrangeratacCountDataset:
#     return CellrangeratacCountDataset(
#         name="dataset",
#         lab_id=lab_id,
#         data_path="data",
#         delivered_at=TIME,
#         gems_id=gems_id,
#         web_summary=WEB_SUMMARY,
#         metrics=CELLRANGERATAC_COUNT_JSON,
#     )


# @pytest.fixture
# def new_cellranger_count_dataset(
#     lab_id: UUID = ID, gems_id: UUID = ID
# ) -> CellrangerCountDataset:
#     return CellrangerCountDataset(
#         name="",
#         lab_id=lab_id,
#         data_path="data",
#         delivered_at=TIME,
#         gems_id=gems_id,
#         web_summary=WEB_SUMMARY,
#         metrics=SINGLE_ROW_CSV,
#     )


# @pytest.fixture
# def new_cellranger_multi_dataset(
#     lab_id: UUID = ID, gems_id: UUID = ID, n_samples: int = 1
# ) -> CellrangerMultiDataset:
#     return CellrangerMultiDataset(
#         name="dataset",
#         lab_id=lab_id,
#         data_path="data",
#         delivered_at=TIME,
#         gems_id=gems_id,
#         web_summary=WEB_SUMMARY,
#         metrics=[CELLRANGER_MULTI_CSV for i in range(n_samples)],
#     )


# @pytest.fixture
# def new_cellranger_vdj_dataset(
#     lab_id: UUID = ID, gems_id: UUID = ID
# ) -> CellrangerVdjDataset:
#     return CellrangerVdjDataset(
#         name="dataset",
#         lab_id=lab_id,
#         data_path="data",
#         delivered_at=TIME,
#         gems_id=gems_id,
#         web_summary="",
#         metrics=SINGLE_ROW_CSV,
#     )


@pytest.mark.parametrize(
    "data, key, expected_value",
    [
        ("new_institution", "name", "institution"),
        ("new_person", "name", "ahmed"),
        ("new_lab", "name", "lab"),
        ("new_fixed_block", "embedded_in", "paraffin"),
        ("new_frozen_block", "embedded_in", "carboxymethyl_cellulose"),
        ("new_cryopreserved_tissue", "name", "c"),
        ("new_fixed_tissue", "name", "f"),
        ("new_frozen_tissue", "name", "f"),
        ("new_virtual_specimen", "name", "v"),
        ("new_suspension_fixture", "target_cell_recovery", 0),
        ("new_suspension_pool", "preparer_ids", [ID]),
        ("new_singleplex_chromium_run", "chip", "GEM-X 3'"),
        ("new_pool_multiplex_chromium_run", "chip", "GEM-X FX"),
        ("new_ocm_chromium_run", "chip", "GEM-X OCM 3'"),
        ("new_cdna_group", "preparer_ids", [ID]),
        # ("new_library", "cdna_id", ID),
        # ("new_cellrangerarc_count_dataset", "cmdline", "cellranger-arc count"),
        # (
        #     "new_cellrangeratac_count_dataset",
        #     "cmdline",
        #     "cellranger-atac count",
        # ),
        # ("new_cellranger_count_dataset", "cmdline", "cellranger count"),
        # ("new_cellranger_multi_dataset", "cmdline", "cellranger multi"),
        # ("new_cellranger_vdj_dataset", "cmdline", "cellranger vdj"),
    ],
)
def test_jsonification(
    data: Any, key: str | None, expected_value: Any, request: pytest.FixtureRequest
):
    data = request.getfixturevalue(data)
    json_str = data.to_json_bytes()
    pythonized = json.loads(json_str)
    found_value = pythonized[key]

    if not isinstance(expected_value, (str, list, datetime)):
        found_value = type(expected_value)(found_value)

    elif isinstance(expected_value, list) and isinstance(found_value, list):
        for i, v in enumerate(found_value):
            found_value[i] = type(expected_value[0])(found_value[i])

    assert found_value == expected_value

    dataclass = type(data)
    assert dataclass.from_json_bytes(json_str) == data
