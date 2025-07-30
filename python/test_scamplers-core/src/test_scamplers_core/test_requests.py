from datetime import UTC, datetime
import maturin_import_hook
import pytest

maturin_import_hook.install()
from scamplers_core.requests import (
    ComplianceCommitteeType,
    CellCountingMethod,
    LengthUnit,
    ElectrophoreticMeasurementData,
    NewSuspensionMeasurement,
    FrozenBlockEmbeddingMatrix,
    LibraryMeasurementData,
    MassUnit,
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
)
from uuid import UUID

ID = UUID(int=0)
TIME = datetime(year=1999, month=1, day=1, tzinfo=UTC)


def test_new_institution():
    NewInstitution(id=ID, name="")


def test_new_person():
    NewPerson(name="", email="", institution_id=ID, roles=[UserRole.AppAdmin])


def test_new_lab():
    NewLab(name="", pi_id=ID, delivery_dir="")


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


def test_new_fixed_block(
    specimen_dv200: SpecimenMeasurementData,
    specimen_rin: SpecimenMeasurementData,
    new_committee_approval: NewCommitteeApproval,
):
    NewFixedBlock(
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


def test_new_frozen_block():
    NewFrozenBlock(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=datetime.now(UTC),
        species=[Species.MusMusculus],
        embedded_in=FrozenBlockEmbeddingMatrix.CarboxymethylCellulose,
    )


def test_new_cryopreserved_tissue():
    NewCryopreservedTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=datetime.now(UTC),
        species=[Species.RattusNorvegicus],
    )


def test_new_fixed_tissue():
    NewFixedTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=datetime.now(UTC),
        species=[Species.HomoSapiens],
        fixative=TissueFixative.DithiobisSuccinimidylropionate,
    )


def test_new_frozen_tissue():
    NewFrozenTissue(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=datetime.now(UTC),
        species=[Species.CallithrixJacchus],
    )


def test_new_virtual_specimen():
    NewVirtualSpecimen(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=datetime.now(UTC),
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


def test_new_suspension(
    suspension_concentration: SuspensionMeasurementDataCommon,
    suspension_mean_diameter: SuspensionMeasurementDataCommon,
    suspension_viability: SuspensionMeasurementDataCommon,
    suspension_volume: SuspensionMeasurementDataCommon,
):
    NewSuspension(
        readable_id="",
        parent_specimen_id=ID,
        biological_material=BiologicalMaterial.Cells,
        created_at=datetime.now(UTC),
        target_cell_recovery=0,
        target_reads_per_cell=0,
        preparer_ids=[ID],
        measurements=[
            NewSuspensionMeasurement(measured_by=ID, data=m, is_post_hybridization=True)
            for m in [
                suspension_concentration,
                suspension_mean_diameter,
                suspension_viability,
                suspension_volume,
            ]
        ],
    )


def test_new_singleplex_chromium_run(
    suspension_volume: SuspensionMeasurementDataCommon,
):
    NewSingleplexChromiumRun(
        readable_id="",
        run_at=datetime.now(UTC),
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


def test_new_pool_multiplex_chromium_run(
    suspension_volume: SuspensionMeasurementDataCommon,
):
    NewPoolMultiplexChromiumRun(
        readable_id="",
        run_at=datetime.now(UTC),
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


def test_new_ocm_chromium_run(suspension_volume: SuspensionMeasurementDataCommon):
    NewOcmChromiumRun(
        readable_id="",
        run_at=datetime.now(UTC),
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


def test_new_cdna(electrophoretic_measurement_data: ElectrophoreticMeasurementData):
    NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="",
        prepared_at=datetime.now(UTC),
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


def test_new_library(
    library_electrophoretic_measurement: LibraryMeasurementData,
    library_fluormetric_measurement: LibraryMeasurementData,
):
    NewLibrary(
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
