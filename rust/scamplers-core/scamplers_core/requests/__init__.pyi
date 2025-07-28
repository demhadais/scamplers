from datetime import datetime
from uuid import UUID

class NewInstitution:
    id: UUID
    name: str

    def __new__(cls, id: UUID, name: str) -> NewInstitution: ...
    @staticmethod
    def from_json(json: str) -> NewInstitution: ...
    def to_json(self) -> str: ...

class UserRole:
    AppAdmin = ...
    BiologyStaff = ...
    ComputationalStaff = ...

class NewPerson:
    name: str
    email: str
    orcid: str | None
    institution_id: UUID
    ms_user_id: UUID | None
    roles: list[UserRole]

    def __new__(
        cls, name: str, email: str, institution_id: UUID, roles: list[UserRole]
    ) -> NewPerson: ...
    @staticmethod
    def from_json(json: str) -> NewPerson: ...
    def to_json(self) -> str: ...

class NewLab:
    name: str
    pi_id: UUID
    delivery_dir: str
    member_ids: list[UUID]

    def __new__(
        cls, name: str, pi_id: UUID, delivery_dir: str, member_ids: list[UUID]
    ) -> NewLab: ...
    @staticmethod
    def from_json(json: str) -> NewLab: ...
    def to_json(self) -> str: ...

class FixedBlockEmbeddingMatrix:
    Paraffin = ...

class BlockFixative:
    FormaldehydeDerivative = ...

class NewFixedBlock:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        embedded_in: FixedBlockEmbeddingMatrix,
        fixative: BlockFixative,
    ) -> NewFixedBlock: ...
    @staticmethod
    def from_json(json: str) -> NewFixedBlock: ...
    def to_json(self) -> str: ...

class FrozenBlockEmbeddingMatrix:
    CarboxymethylCellulose = ...
    OptimalCuttingTemperatureCompound = ...

class NewFrozenBlock:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        embedded_in: FrozenBlockEmbeddingMatrix,
        fixative: BlockFixative | None,
    ) -> NewFrozenBlock: ...
    @staticmethod
    def from_json(json: str) -> NewFrozenBlock: ...
    def to_json(self) -> str: ...

class TissueFixative:
    DithiobisSuccinimidylropionate = ...

class NewCryopreservedTissue:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        storage_buffer: str | None,
    ) -> NewCryopreservedTissue: ...
    @staticmethod
    def from_json(json: str) -> NewCryopreservedTissue: ...
    def to_json(self) -> str: ...

class NewFixedTissue:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        fixative: TissueFixative,
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        storage_buffer: str | None,
    ) -> NewFixedTissue: ...
    @staticmethod
    def from_json(json: str) -> NewFixedTissue: ...
    def to_json(self) -> str: ...

class NewFrozenTissue:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        storage_buffer: str | None,
    ) -> NewFrozenTissue: ...
    @staticmethod
    def from_json(json: str) -> NewFrozenTissue: ...
    def to_json(self) -> str: ...

class NewVirtualSpecimen:
    def __new__(
        cls,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement],
        committee_approvals: list[NewCommitteeApproval],
        notes: str | None,
        returned_at: datetime | None,
        returned_by: UUID | None,
        fixative: SuspensionFixative,
    ) -> NewVirtualSpecimen: ...
    @staticmethod
    def from_json(json: str) -> NewVirtualSpecimen: ...
    def to_json(self) -> str: ...

class SuspensionFixative:
    FormaldehydeDerivative = ...

class Species:
    AmbystomaMexicanum = ...
    CanisFamiliaris = ...
    CallithrixJacchus = ...
    DrosophilaMelanogaster = ...
    GasterosteusAculeatus = ...
    HomoSapiens = ...
    MusMusculus = ...
    RattusNorvegicus = ...
    SminthopsisCrassicaudata = ...

class SpecimenMeasurementData:
    pass

class NewSpecimenMeasurement:
    def __new__(
        cls, measured_by: UUID, data: SpecimenMeasurementData, specimen_id: UUID = ...
    ) -> NewSpecimenMeasurement: ...
    @staticmethod
    def from_json(json: str) -> NewSpecimenMeasurement: ...
    def to_json(self) -> str: ...

class ComplianceCommitteeType:
    Ibc = ...
    Irb = ...
    Iacuc = ...

class NewCommitteeApproval:
    specimen_id: UUID
    institution_id: UUID
    committee_type: ComplianceCommitteeType
    compliance_identifier: str

    @staticmethod
    def from_json(json: str) -> NewCommitteeApproval: ...
    def to_json(self) -> str: ...

class MeasurementData_Rin:
    measured_at: datetime
    instrument_name: str
    value: float

class SuspensionMeasurementData: ...

class BiologicalMaterial:
    Cells = ...
    Nuclei = ...

class CellCountingMethod:
    BrightField = ...
    Aopi = ...
    TrypanBlue = ...

class NewSuspension:
    def __new__(
        cls,
        readable_id: str,
        parent_specimen_id: UUID,
        biological_material: BiologicalMaterial,
        created_at: datetime | None,
        pooled_into_id: UUID | None,
        multiplexing_tag_id: UUID | None,
        lysis_duration_minutes: float | None,
        target_cell_recovery: float,
        target_reads_per_cell: int,
        notes: str | None,
        preparer_ids: list[UUID],
        measurements: list[NewSuspensionMeasurement],
    ) -> NewSuspension: ...
    @staticmethod
    def from_json(json: str) -> NewSuspension: ...
    def to_json(self) -> str: ...

class NewSuspensionMeasurement:
    def __new__(
        cls,
        measured_by: UUID,
        data: SuspensionMeasurementData,
        is_post_hybridization: bool,
        suspension_id: UUID = ...,
    ) -> NewSuspensionMeasurement: ...

class NewSuspensionPool:
    @staticmethod
    def from_json(json: str) -> NewSuspensionPool: ...
    def to_json(self) -> str: ...

class NewSuspensionPoolMeasurement:
    def __new__(
        cls,
        measured_by: UUID,
        data: SuspensionMeasurementData,
        is_post_storage: bool,
        pool_id: UUID = ...,
    ) -> NewSuspensionPoolMeasurement: ...

class SingleplexChromiumChip:
    J = ...
    H = ...
    GemxFx = ...
    Gemx3p = ...
    Gemx5p = ...

class NewSingleplexChromiumRun:
    def __new__(
        cls,
        readable_id: str,
        run_at: datetime,
        succeeded: bool,
        run_by: UUID,
        chip: SingleplexChromiumChip,
        gems: list[NewSingleplexGems],
        notes: str | None,
    ) -> NewSingleplexChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewSingleplexChromiumRun: ...
    def to_json(self) -> str: ...

class NewSingleplexGems:
    def __new__(
        cls, readable_id: str, chemistry: str, loading: NewSingleplexChipLoading
    ) -> NewSingleplexGems: ...

class NewSingleplexChipLoading:
    def __new__(
        cls,
        suspension_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementData,
        buffer_volume_loaded: SuspensionMeasurementData,
        notes: str | None,
    ) -> NewSingleplexChipLoading: ...

class OcmChromiumChip:
    GemxOcm3p = ...

class NewOcmChromiumRun:
    def __new__(
        cls,
        readable_id: str,
        run_at: datetime,
        succeeded: bool,
        run_by: UUID,
        chip: OcmChromiumChip,
        gems: list[NewOcmGems],
        notes: str | None,
    ) -> NewOcmChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewOcmChromiumRun: ...
    def to_json(self) -> str: ...

class NewOcmGems:
    def __new__(
        cls, readable_id: str, chemistry: str, loading: list[NewOcmChipLoading]
    ) -> NewOcmGems: ...

class NewOcmChipLoading:
    def __new__(
        cls,
        suspension_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementData,
        buffer_volume_loaded: SuspensionMeasurementData,
        notes: str | None,
    ) -> NewOcmChipLoading: ...

class PoolMultiplexChromiumChip:
    Q = ...
    GemxFx = ...

class NewPoolMultiplexChromiumRun:
    def __new__(
        cls,
        readable_id: str,
        run_at: datetime,
        succeeded: bool,
        run_by: UUID,
        chip: PoolMultiplexChromiumChip,
        gems: list[NewPoolMultiplexGems],
        notes: str | None,
    ) -> NewPoolMultiplexChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewPoolMultiplexChromiumRun: ...
    def to_json(self) -> str: ...

class NewPoolMultiplexGems:
    def __new__(
        cls, readable_id: str, chemistry: str, loading: NewPoolMultiplexChipLoading
    ) -> NewPoolMultiplexGems: ...

class NewPoolMultiplexChipLoading:
    def __new__(
        cls,
        suspension_pool_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementData,
        buffer_volume_loaded: SuspensionMeasurementData,
        notes: str | None,
    ) -> NewPoolMultiplexChipLoading: ...

class LibraryType:
    AntibodyCapture = ...
    AntigenCapture = ...
    ChromatinAccessibility = ...
    CrisprGuideCapture = ...
    Custom = ...
    GeneExpression = ...
    MultiplexingCapture = ...
    Vdj = ...
    VdjB = ...
    VdjT = ...
    VdjTGd = ...

class Concentration:
    pass

class MassUnit:
    Nanogram = ...
    Picogram = ...

class VolumeUnit:
    Microliter = ...
    Millliter = ...

class LengthUnit:
    Micrometer = ...

class NewCdnaMeasurement:
    def __new__(
        cls,
        measured_by: UUID,
        measured_at: datetime,
        instrument_name: str,
        mean_library_size_bp: float,
        sizing_range: tuple[int, int],
        concentration_value: float,
        concentration_unit: tuple[MassUnit, VolumeUnit],
        cdna_id: UUID = ...,
    ) -> NewCdnaMeasurement: ...
    @staticmethod
    def from_json(json: str) -> NewCdnaMeasurement: ...
    def to_json(self) -> str: ...

class NewCdna:
    def __new__(
        cls,
        library_type: LibraryType,
        readable_id: str,
        prepared_at: datetime,
        gems_id: UUID,
        n_amplification_cycles: int,
        measurements: list[NewCdnaMeasurement],
        preparer_ids: list[UUID],
        storage_location: str | None,
        notes: str | None,
    ) -> NewCdna: ...
    @staticmethod
    def from_json(json: str) -> NewCdna: ...
    def to_json(self) -> str: ...
