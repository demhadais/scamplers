import json
from collections.abc import Callable
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

import maturin_import_hook
import pytest

maturin_import_hook.install()

from uuid import UUID

from scamplepy.common import (
    BiologicalMaterial,
    CellCountingMethod,
    LengthUnit,
    LibraryType,
    MassUnit,
    NucleicAcidConcentration,
    NucleicAcidMeasurementData,
    SuspensionMeasurementFields,
    VolumeUnit,
)
from scamplepy.create import (
    BlockFixative,
    ComplianceCommitteeType,
    FixedBlockEmbeddingMatrix,
    FrozenBlockEmbeddingMatrix,
    NewCdna,
    NewCdnaGroup,
    NewCdnaMeasurement,
    NewCellrangerCountDataset,
    NewCommitteeApproval,
    NewCryopreservedTissue,
    NewFixedBlock,
    NewFixedTissue,
    NewFrozenBlock,
    NewFrozenTissue,
    NewInstitution,
    NewLab,
    NewLibrary,
    NewLibraryMeasurement,
    NewOcmChromiumRun,
    NewOcmGems,
    NewPerson,
    NewPoolMultiplexChromiumRun,
    NewPoolMultiplexGems,
    NewSingleplexChipLoading,
    NewSingleplexChromiumRun,
    NewSingleplexGems,
    NewSpecimenMeasurement,
    NewSuspension,
    NewSuspensionMeasurement,
    NewSuspensionPool,
    NewSuspensionPoolMeasurement,
    NewFixedOrFreshSuspension,
    SingleRowCsvMetricsFile,
    Species,
    SpecimenMeasurementData,
    SuspensionFixative,
    TissueFixative,
    UserRole,
)

ID = UUID(int=0)
TIME = datetime(year=1999, month=1, day=1, tzinfo=UTC)
TISSUE = "napkin hehe"

SINGLE_ROW_CSV = SingleRowCsvMetricsFile(
    filename="summary.csv",
    raw_contents=(
        Path(__file__).parent.parent.parent.parent.parent
        / "rust"
        / "scamplers"
        / "src"
        / "db"
        / "models"
        / "dataset"
        / "chromium"
        / "test-data"
        / "single-row.csv"
    ).read_text(),
)


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
        measured_at=TIME,
        instrument_name="mayonnaise",
        value=0.5,
    )


def _specimen_rin() -> SpecimenMeasurementData:
    return SpecimenMeasurementData.Rin(
        measured_at=TIME,
        instrument_name="mayonnaise",
        value=5,
    )


def _new_committee_approval() -> NewCommitteeApproval:
    return NewCommitteeApproval(
        institution_id=ID,
        compliance_identifier="compliance",
        committee_type=ComplianceCommitteeType.Ibc,
    )


@pytest.fixture
def new_fixed_block(person_id: UUID = ID, lab_id: UUID = ID) -> NewFixedBlock:
    additional_data = {
        "boolean": True,
        "list": [1, 2, 3],
        "string": "foo",
        "float": 1.3,
        "int": 10,
        "dict": {"foo": "bar"},
    }
    block = NewFixedBlock(
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
        tissue=TISSUE,
        additional_data=additional_data,
    )

    assert block.inner.additional_data == additional_data
    if reloaded_additional_data := block.inner.additional_data:
        assert type(reloaded_additional_data["boolean"]) is bool

    return block


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
        tissue=TISSUE,
    )


@pytest.fixture
def new_cryopreserved_tissue(
    person_id: UUID = ID,
    lab_id: UUID = ID,
) -> NewCryopreservedTissue:
    tissue = NewCryopreservedTissue(
        readable_id="cryopreservedtissue",
        name="c",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.RattusNorvegicus],
        tissue=TISSUE,
    )
    tissue.cryopreserved = True

    try:
        tissue.cryopreserved = False
    except ValueError as e:
        assert str(e) == "field can only be true"

    return tissue


@pytest.fixture
def new_fixed_tissue(person_id: UUID = ID, lab_id: UUID = ID) -> NewFixedTissue:
    return NewFixedTissue(
        readable_id="fixedtissue",
        name="f",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.HomoSapiens],
        fixative=TissueFixative.DithiobisSuccinimidylpropionate,
        tissue=TISSUE,
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
        tissue=TISSUE,
    )


@pytest.fixture
def new_fixed_suspension_specimen(
    person_id: UUID = ID, lab_id: UUID = ID
) -> NewFixedOrFreshSuspension:
    return NewFixedOrFreshSuspension(
        readable_id="fixedorfreshsuspension",
        name="v",
        submitted_by=person_id,
        lab_id=lab_id,
        received_at=TIME,
        species=[Species.DrosophilaMelanogaster],
        fixative=SuspensionFixative.FormaldehydeDerivative,
        tissue=TISSUE,
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
        measured_at=TIME,
        value=0,
        instrument_name="",
    )


def _suspension_volume() -> SuspensionMeasurementFields:
    return SuspensionMeasurementFields.Volume(
        measured_at=TIME,
        value=0,
        unit=VolumeUnit.Microliter,
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
            measured_by=measured_by,
            data=m,
            is_post_hybridization=True,
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
            measured_by=measured_by,
            data=m,
            is_post_storage=True,
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
        assay_id=ID,
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        gems=[
            NewSingleplexGems(
                readable_id=gems_readable_id,
                suspension_id=suspension_id,
                suspension_volume_loaded=_suspension_volume(),
                buffer_volume_loaded=_suspension_volume(),
            ),
        ],
    )


@pytest.fixture
def new_pool_multiplex_chromium_run(
    run_by: UUID = ID,
    suspension_pool_id: UUID = ID,
) -> NewPoolMultiplexChromiumRun:
    return NewPoolMultiplexChromiumRun(
        readable_id="poolmultiplexchromiumrun",
        assay_id=ID,
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        gems=[
            NewPoolMultiplexGems(
                readable_id="",
                suspension_pool_id=suspension_pool_id,
                suspension_volume_loaded=_suspension_volume(),
                buffer_volume_loaded=_suspension_volume(),
            ),
        ],
    )


@pytest.fixture
def new_ocm_chromium_run(
    run_by: UUID = ID,
    suspension_id: UUID = ID,
) -> NewOcmChromiumRun:
    return NewOcmChromiumRun(
        readable_id="ocmchromiumrun",
        assay_id=ID,
        run_at=TIME,
        succeeded=True,
        run_by=run_by,
        gems=[
            NewOcmGems(
                readable_id="",
                loading=[
                    NewSingleplexChipLoading(
                        suspension_id=suspension_id,
                        suspension_volume_loaded=_suspension_volume(),
                        buffer_volume_loaded=_suspension_volume(),
                    ),
                ],
            ),
        ],
    )


def _electrophoretic_measurement_data() -> NucleicAcidMeasurementData:
    return NucleicAcidMeasurementData.Electrophoretic(
        measured_at=TIME,
        instrument_name="mayonnaise",
        mean_size_bp=0,
        sizing_range=(0, 0),
        concentration=NucleicAcidConcentration(
            value=0, unit=(MassUnit.Nanogram, VolumeUnit.Microliter)
        ),
    )


def _fluormetric_measurement_data() -> NucleicAcidMeasurementData:
    return NucleicAcidMeasurementData.Fluorometric(
        measured_at=TIME,
        instrument_name="mayonnaise",
        concentration=NucleicAcidConcentration(
            value=0,
            unit=(MassUnit.Picogram, VolumeUnit.Microliter),
        ),
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
            ),
        ],
        preparer_ids=[person_id],
    )


@pytest.fixture
def new_cdna_group() -> NewCdnaGroup:
    return NewCdnaGroup.Single(_new_cdna())


@pytest.fixture
def new_library(cdna_id: UUID = ID, person_id: UUID = ID) -> NewLibrary:
    return NewLibrary(
        readable_id="library",
        cdna_id=cdna_id,
        number_of_sample_index_pcr_cycles=0,
        target_reads_per_cell=0,
        prepared_at=TIME,
        preparer_ids=[person_id],
        volume_mcl=40.0,
        measurements=[
            NewLibraryMeasurement(measured_by=person_id, data=m)
            for m in [
                _electrophoretic_measurement_data(),
                _fluormetric_measurement_data(),
            ]
        ],
    )


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


@pytest.fixture
def new_cellranger_count_dataset(
    lab_id: UUID = ID,
    gems_id: UUID = ID,
) -> NewCellrangerCountDataset:
    ds = NewCellrangerCountDataset(
        name="",
        lab_id=lab_id,
        library_ids=[ID],
        data_path="data",
        delivered_at=TIME,
        web_summary="",
        metrics=SINGLE_ROW_CSV,
    )
    assert ds.metrics.contents["estimated_number_of_cells"] == 65_558  # type: ignore

    return ds


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


def block_embedding(block) -> str:
    return str(block.embedded_in).lower().split(".")[-1]


def block_embedding_from_dict(block: dict[str, str]) -> str:
    return block["embedded_in"].replace("_", "")


def specimen_name(specimen) -> str:
    return specimen.inner.name


def specimen_name_from_dict(specimen: dict[str, str]):
    return specimen["name"]


def chromium_run_readable_id(chromium_run) -> str:
    return chromium_run.inner.readable_id


def chromium_run_readable_id_from_dict(chromium_run: dict[str, str]):
    return chromium_run["readable_id"]


@pytest.mark.parametrize(
    "data, accessor",
    [
        ("new_institution", "name"),
        ("new_person", "name"),
        ("new_lab", "name"),
        ("new_fixed_block", (block_embedding, block_embedding_from_dict)),
        ("new_frozen_block", (block_embedding, block_embedding_from_dict)),
        ("new_cryopreserved_tissue", (specimen_name, specimen_name_from_dict)),
        ("new_fixed_tissue", (specimen_name, specimen_name_from_dict)),
        ("new_frozen_tissue", (specimen_name, specimen_name_from_dict)),
        ("new_fixed_suspension_specimen", (specimen_name, specimen_name_from_dict)),
        (
            "new_suspension_fixture",
            (
                lambda pool: pool.measurements[0].data.fields.value,
                lambda dict_: dict_["measurements"][0]["value"],
            ),
        ),
        ("new_suspension_pool", "readable_id"),
        (
            "new_singleplex_chromium_run",
            (chromium_run_readable_id, chromium_run_readable_id_from_dict),
        ),
        (
            "new_pool_multiplex_chromium_run",
            (chromium_run_readable_id, chromium_run_readable_id_from_dict),
        ),
        (
            "new_ocm_chromium_run",
            (chromium_run_readable_id, chromium_run_readable_id_from_dict),
        ),
        (
            "new_cdna_group",
            (
                lambda cdna_group: cdna_group.cdna.n_amplification_cycles,
                lambda dict_: dict_["cdna"]["n_amplification_cycles"],
            ),
        ),
        ("new_library", "number_of_sample_index_pcr_cycles"),
        # ("new_cellrangerarc_count_dataset", "cmdline", "cellranger-arc count"),
        # (
        #     "new_cellrangeratac_count_dataset",
        #     "cmdline",
        #     "cellranger-atac count",
        # ),
        (
            "new_cellranger_count_dataset",
            (
                lambda dataset: dataset.metrics.contents["estimated_number_of_cells"],
                lambda dict_: dict_["metrics"]["contents"]["estimated_number_of_cells"],
            ),
        ),
        # ("new_cellranger_multi_dataset", "cmdline", "cellranger multi"),
        # ("new_cellranger_vdj_dataset", "cmdline", "cellranger vdj"),
    ],
)
def test_jsonification(
    data: Any,
    accessor: tuple[Callable, Callable] | str,
    request: pytest.FixtureRequest,
):
    # Get the original data and convert it to a json string
    original_data = request.getfixturevalue(data)
    jsonified = original_data.to_json_bytes()

    # Read that json string as a python object (we expect it to be a `dict`)
    pythonized = json.loads(jsonified)

    if isinstance(accessor, str):
        assert getattr(original_data, accessor) == pytest.approx(pythonized[accessor])

    else:
        data_accessor, dict_accessor = accessor
        assert data_accessor(original_data) == pytest.approx(dict_accessor(pythonized))
