from datetime import UTC, datetime
import maturin_import_hook

maturin_import_hook.install()
from scamplers_core.requests import (
    ComplianceCommitteeType,
    ElectrophoreticMeasurementData,
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
SUSPENSION_VOLUME = SuspensionMeasurementDataCommon.Volume(
    measured_at=TIME, value=0, unit=VolumeUnit.Millliter
)
ELECTROPHORETIC_MEASUREMENT_DATA = ElectrophoreticMeasurementData(
    measured_at=TIME,
    instrument_name="",
    mean_library_size_bp=0,
    sizing_range=(0, 0),
    concentration_value=0,
    concentration_unit=(MassUnit.Nanogram, VolumeUnit.Microliter),
)


def test_new_institution():
    NewInstitution(id=ID, name="")


def test_new_person():
    NewPerson(name="", email="", institution_id=ID, roles=[UserRole.AppAdmin])


def test_new_lab():
    NewLab(name="", pi_id=ID, delivery_dir="")


def test_new_fixed_block():
    NewFixedBlock(
        readable_id="",
        name="",
        submitted_by=ID,
        lab_id=ID,
        received_at=TIME,
        species=[Species.HomoSapiens],
        measurements=[
            NewSpecimenMeasurement(
                measured_by=ID,
                data=SpecimenMeasurementData.Rin(
                    measured_at=TIME, instrument_name="", value=0
                ),
            )
        ],
        committee_approvals=[
            NewCommitteeApproval(
                institution_id=ID,
                compliance_identifier="",
                committee_type=ComplianceCommitteeType.Ibc,
            )
        ],
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


def test_new_suspension():
    NewSuspension(
        readable_id="",
        parent_specimen_id=ID,
        biological_material=BiologicalMaterial.Cells,
        created_at=datetime.now(UTC),
        target_cell_recovery=0,
        target_reads_per_cell=0,
        preparer_ids=[ID],
    )


def test_new_singleplex_chromium_run():
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
                    suspension_volume_loaded=SUSPENSION_VOLUME,
                    buffer_volume_loaded=SUSPENSION_VOLUME,
                ),
            )
        ],
    )


def test_new_pool_multiplex_chromium_run():
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
                    suspension_volume_loaded=SUSPENSION_VOLUME,
                    buffer_volume_loaded=SUSPENSION_VOLUME,
                ),
            )
        ],
    )


def test_new_ocm_chromium_run():
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
                        suspension_volume_loaded=SUSPENSION_VOLUME,
                        buffer_volume_loaded=SUSPENSION_VOLUME,
                    )
                ],
            )
        ],
    )


def test_new_cdna():
    NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="",
        prepared_at=datetime.now(UTC),
        gems_id=ID,
        n_amplification_cycles=0,
        measurements=[
            NewCdnaMeasurement(
                measured_by=ID,
                data=ELECTROPHORETIC_MEASUREMENT_DATA,
                cdna_id=ID,
            )
        ],
        preparer_ids=[ID],
    )


def test_new_library():
    m1 = NewLibraryMeasurement(
        measured_by=ID,
        data=LibraryMeasurementData.Fluorometric(
            measured_at=TIME,
            instrument_name="",
            concentration=NucleicAcidConcentration(
                value=0, unit=(MassUnit.Picogram, VolumeUnit.Microliter)
            ),
        ),
    )
    m2 = NewLibraryMeasurement(
        measured_by=ID,
        data=LibraryMeasurementData.Electrophoretic(ELECTROPHORETIC_MEASUREMENT_DATA),
    )

    NewLibrary(
        readable_id="",
        cdna_id=ID,
        number_of_sample_index_pcr_cycles=0,
        target_reads_per_cell=0,
        prepared_at=TIME,
        preparer_ids=[ID],
        measurements=[m1, m2],
    )
