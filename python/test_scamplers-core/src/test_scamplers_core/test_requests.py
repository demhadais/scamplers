import maturin_import_hook

maturin_import_hook.install()
from scamplers_core.requests import (
    NewInstitution,
    NewPerson,
    NewLab,
    NewFixedBlock,
    NewFrozenBlock,
    NewCryopreservedTissue,
    NewFixedTissue,
    NewFrozenTissue,
    NewSpecimenMeasurement,
    NewVirtualSpecimen,
    NewSuspension,
    NewSingleplexChromiumRun,
    NewOcmChromiumRun,
    NewCdna,
    FixedBlockEmbeddingMatrix,
    BlockFixative,
    SpecimenMeasurementData,
    TissueFixative,
    SuspensionFixative,
    BiologicalMaterial,
    Species,
    LibraryType,
    UserRole,
)
from uuid import uuid4
from datetime import UTC, datetime


def test_new_institution():
    NewInstitution(id=uuid4(), name="")


def test_new_person():
    NewPerson(name="", email="", institution_id=uuid4(), roles=[UserRole.AppAdmin])


def test_new_lab():
    NewLab(name="", pi_id=uuid4(), delivery_dir="", member_ids=[uuid4()])


def test_new_fixed_block():
    NewFixedBlock(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.HomoSapiens],
        measurements=[
            NewSpecimenMeasurement(
                measured_by=uuid4(), data=SpecimenMeasurementData.Rin()
            )
        ],
        committee_approvals=[],
        notes=None,
        returned_at=None,
        returned_by=None,
        embedded_in=FixedBlockEmbeddingMatrix.Paraffin,
        fixative=BlockFixative.FormaldehydeDerivative,
    )


def test_new_frozen_block():
    NewFrozenBlock(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.MusMusculus],
        measurements=[],
        committee_approvals=[],
        notes=None,
        returned_at=None,
        returned_by=None,
        embedded_in=None,
        fixative=None,
    )


def test_new_cryopreserved_tissue():
    NewCryopreservedTissue(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.RattusNorvegicus],
        measurements=[],
        committee_approvals=[],
        notes=None,
        returned_at=None,
        returned_by=None,
        storage_buffer=None,
    )


def test_new_fixed_tissue():
    NewFixedTissue(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.HomoSapiens],
        measurements=[],
        committee_approvals=[],
        fixative=TissueFixative.DithiobisSuccinimidylropionate,
        notes=None,
        returned_at=None,
        returned_by=None,
        storage_buffer=None,
    )


def test_new_frozen_tissue():
    NewFrozenTissue(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.CallithrixJacchus],
        measurements=[],
        committee_approvals=[],
        notes=None,
        returned_at=None,
        returned_by=None,
        storage_buffer=None,
    )


def test_new_virtual_specimen():
    NewVirtualSpecimen(
        readable_id="",
        name="",
        submitted_by=uuid4(),
        lab_id=uuid4(),
        received_at=datetime.now(UTC),
        species=[Species.DrosophilaMelanogaster],
        measurements=[],
        committee_approvals=[],
        notes=None,
        returned_at=None,
        returned_by=None,
        fixative=SuspensionFixative.FormaldehydeDerivative,
    )


def test_new_suspension():
    NewSuspension(
        readable_id="",
        parent_specimen_id=uuid4(),
        biological_material=BiologicalMaterial.Cells,
        created_at=datetime.now(UTC),
        pooled_into_id=None,
        multiplexing_tag_id=None,
        lysis_duration_minutes=None,
        target_cell_recovery=0,
        target_reads_per_cell=0,
        notes=None,
        preparer_ids=[uuid4()],
        measurements=[],
    )


def test_new_singleplex_chromium_run():
    NewSingleplexChromiumRun(
        readable_id="",
        run_at=datetime.now(UTC),
        succeeded=True,
        run_by=uuid4(),
        chip=None,
        gems=[],
        notes=None,
    )


def test_new_ocm_chromium_run():
    NewOcmChromiumRun(
        readable_id="",
        run_at=datetime.now(UTC),
        succeeded=True,
        run_by=uuid4(),
        chip=None,
        gems=[],
        notes=None,
    )


def test_new_cdna():
    NewCdna(
        library_type=LibraryType.GeneExpression,
        readable_id="",
        prepared_at=datetime.now(UTC),
        gems_id=uuid4(),
        n_amplification_cycles=0,
        measurements=[],
        preparer_ids=[uuid4()],
        storage_location=None,
        notes=None,
    )
