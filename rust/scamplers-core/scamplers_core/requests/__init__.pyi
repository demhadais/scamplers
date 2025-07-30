from dataclasses import dataclass, field
from datetime import datetime
from collections.abc import Sequence
from typing import Any
from uuid import UUID

@dataclass(kw_only=True)
class NewInstitution:
    id: UUID
    name: str

    @staticmethod
    def from_json(json: str) -> NewInstitution: ...
    def to_json(self) -> str: ...

class UserRole:
    AppAdmin = ...
    BiologyStaff = ...
    ComputationalStaff = ...

@dataclass(kw_only=True)
class NewPerson:
    name: str
    email: str
    orcid: str | None = field(init=False)
    institution_id: UUID
    ms_user_id: UUID | None = field(init=False)
    roles: list[UserRole]

    @staticmethod
    def from_json(json: str) -> NewPerson: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class NewLab:
    name: str
    pi_id: UUID
    delivery_dir: str
    member_ids: list[UUID] = ...

    @staticmethod
    def from_json(json: str) -> NewLab: ...
    def to_json(self) -> str: ...

class ComplianceCommitteeType:
    Ibc = ...
    Irb = ...
    Iacuc = ...

@dataclass(kw_only=True)
class NewCommitteeApproval:
    institution_id: UUID
    committee_type: ComplianceCommitteeType
    compliance_identifier: str
    specimen_id: UUID = ...

    @staticmethod
    def from_json(json: str) -> NewCommitteeApproval: ...
    def to_json(self) -> str: ...

class MassUnit:
    Nanogram = ...
    Picogram = ...

class VolumeUnit:
    Microliter = ...
    Millliter = ...

class LengthUnit:
    Micrometer = ...

@dataclass
class MeasurementData_Rin(SpecimenMeasurementData):
    measured_at: datetime
    instrument_name: str
    value: float

@dataclass
class MeasurementData_Dv200(SpecimenMeasurementData):
    measured_at: datetime
    instrument_name: str
    value: float

class SpecimenMeasurementData:
    Rin = MeasurementData_Rin
    Dv200 = MeasurementData_Dv200

    @staticmethod
    def from_json(json: str) -> SpecimenMeasurementData: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class NewSpecimenMeasurement:
    measured_by: UUID
    data: SpecimenMeasurementData
    specimen_id: UUID = ...

    @staticmethod
    def from_json(json: str) -> NewSpecimenMeasurement: ...
    def to_json(self) -> str: ...

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

class _NewSpecimenCommon:
    readable_id: str
    name: str
    submitted_by: UUID
    lab_id: UUID
    received_at: datetime
    species: list[Species]
    committee_approvals: list[NewCommitteeApproval]
    notes: str | None
    returned_at: datetime | None
    returned_by: UUID | None
    measurements: list[NewSpecimenMeasurement]

class FixedBlockEmbeddingMatrix:
    Paraffin = ...

class BlockFixative:
    FormaldehydeDerivative = ...

class NewFixedBlock:
    inner: _NewSpecimenCommon
    embedded_in: FixedBlockEmbeddingMatrix
    fixative: BlockFixative

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        embedded_in: FixedBlockEmbeddingMatrix,
        fixative: BlockFixative,
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
    ) -> NewFixedBlock: ...
    @staticmethod
    def from_json(json: str) -> NewFixedBlock: ...
    def to_json(self) -> str: ...

class FrozenBlockEmbeddingMatrix:
    CarboxymethylCellulose = ...
    OptimalCuttingTemperatureCompound = ...

class NewFrozenBlock:
    inner: _NewSpecimenCommon
    embedded_in: FrozenBlockEmbeddingMatrix
    fixative: BlockFixative | None
    frozen: bool

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        embedded_in: FrozenBlockEmbeddingMatrix,
        fixative: BlockFixative | None = ...,
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
    ) -> NewFrozenBlock: ...
    @staticmethod
    def from_json(json: str) -> NewFrozenBlock: ...
    def to_json(self) -> str: ...

class NewCryopreservedTissue:
    inner: _NewSpecimenCommon
    storage_buffer: str | None
    cryopreserved: bool

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
        storage_buffer: str | None = ...,
    ) -> NewCryopreservedTissue: ...
    @staticmethod
    def from_json(json: str) -> NewCryopreservedTissue: ...
    def to_json(self) -> str: ...

class TissueFixative:
    DithiobisSuccinimidylropionate = ...

class NewFixedTissue:
    inner: _NewSpecimenCommon
    fixative: TissueFixative
    storage_buffer: str | None

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        fixative: TissueFixative,
        storage_buffer: str | None = ...,
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
    ) -> NewFixedTissue: ...
    @staticmethod
    def from_json(json: str) -> NewFixedTissue: ...
    def to_json(self) -> str: ...

class NewFrozenTissue:
    inner: _NewSpecimenCommon
    storage_buffer: str | None
    frozen: bool

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        storage_buffer: str | None = ...,
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
    ) -> NewFrozenTissue: ...
    @staticmethod
    def from_json(json: str) -> NewFrozenTissue: ...
    def to_json(self) -> str: ...

class SuspensionFixative:
    FormaldehydeDerivative = ...

class NewVirtualSpecimen:
    inner: _NewSpecimenCommon
    fixative: SuspensionFixative | None

    def __new__(
        cls,
        *,
        readable_id: str,
        name: str,
        submitted_by: UUID,
        lab_id: UUID,
        received_at: datetime,
        species: list[Species],
        fixative: SuspensionFixative | None = ...,
        measurements: list[NewSpecimenMeasurement] = ...,
        committee_approvals: list[NewCommitteeApproval] = ...,
        notes: str | None = ...,
        returned_at: datetime | None = ...,
        returned_by: UUID | None = ...,
    ) -> NewVirtualSpecimen: ...
    @staticmethod
    def from_json(json: str) -> NewVirtualSpecimen: ...
    def to_json(self) -> str: ...

class BiologicalMaterial:
    Cells = ...
    Nuclei = ...

class CellCountingMethod:
    BrightField = ...
    Aopi = ...
    TrypanBlue = ...

@dataclass(kw_only=True)
class MeasurementDataCore_Concentration(SuspensionMeasurementDataCommon):
    measured_at: datetime
    instrument_name: str
    counting_method: CellCountingMethod
    value: float
    unit: tuple[BiologicalMaterial, VolumeUnit]

@dataclass(kw_only=True)
class MeasurementDataCore_Volume(SuspensionMeasurementDataCommon):
    measured_at: datetime
    value: float
    unit: VolumeUnit

@dataclass(kw_only=True)
class MeasurementDataCore_Viability(SuspensionMeasurementDataCommon):
    measured_at: datetime
    instrument_name: str
    value: float

@dataclass(kw_only=True)
class MeasurementDataCore_MeanDiameter(SuspensionMeasurementDataCommon):
    measured_at: datetime
    instrument_name: str
    value: float
    unit: tuple[BiologicalMaterial, LengthUnit]

class SuspensionMeasurementDataCommon:
    Concentration = MeasurementDataCore_Concentration
    Volume = MeasurementDataCore_Volume
    Viability = MeasurementDataCore_Viability
    MeanDiameter = MeasurementDataCore_MeanDiameter

    @staticmethod
    def from_json(json: str) -> SuspensionMeasurementDataCommon: ...
    def to_json(self) -> str: ...

class _SuspensionMeasurementData:
    core: SuspensionMeasurementDataCommon
    is_post_hybridization: bool

class NewSuspensionMeasurement:
    measured_by: UUID
    data: _SuspensionMeasurementData
    suspension_id: UUID = ...

    def __new__(
        cls,
        *,
        measured_by: UUID,
        data: SuspensionMeasurementDataCommon,
        is_post_hybridization: bool,
        suspension_id: UUID = ...,
    ) -> NewSuspensionMeasurement: ...

@dataclass(kw_only=True)
class NewSuspension:
    readable_id: str
    parent_specimen_id: UUID
    biological_material: BiologicalMaterial
    target_cell_recovery: float
    target_reads_per_cell: int
    preparer_ids: list[UUID]
    measurements: list[NewSuspensionMeasurement] = ...
    created_at: datetime | None = ...
    pooled_into_id: UUID | None = ...
    multiplexing_tag_id: UUID | None = ...
    lysis_duration_minutes: float | None = ...
    notes: str | None = ...

    @staticmethod
    def from_json(json: str) -> NewSuspension: ...
    def to_json(self) -> str: ...

class _SuspensionPoolMeasurementData:
    data: SuspensionMeasurementDataCommon
    is_post_storage: bool

class NewSuspensionPoolMeasurement:
    measured_by: UUID
    data: _SuspensionPoolMeasurementData
    pool_id: UUID = ...

    def __new__(
        cls,
        *,
        measured_by: UUID,
        data: SuspensionMeasurementDataCommon,
        is_post_storage: bool,
        pool_id: UUID = ...,
    ) -> NewSuspensionPoolMeasurement: ...

@dataclass(kw_only=True)
class NewSuspensionPool:
    readable_id: str
    name: str
    pooled_at: datetime
    suspensions: list[NewSuspension]
    preparer_ids: list[UUID]
    measurements: list[NewSuspensionPoolMeasurement] = ...
    notes: str | None = ...

    @staticmethod
    def from_json(json: str) -> NewSuspensionPool: ...
    def to_json(self) -> str: ...

class SingleplexChromiumChip:
    J = ...
    H = ...
    GemxFx = ...
    Gemx3p = ...
    Gemx5p = ...

class _NewGemsCommon:
    readable_id: str
    chemistry: str
    chromium_run_id: UUID

class _NewChipLoadingCommon:
    gems_id: UUID
    suspension_volume_loaded: SuspensionMeasurementDataCommon
    buffer_volume_loaded: SuspensionMeasurementDataCommon
    notes: str | None

class _NewChromiumRunCommon:
    readable_id: str
    run_at: datetime
    run_by: UUID
    succeeded: bool
    notes: str | None

class NewSingleplexChipLoading:
    suspension_id: UUID
    inner: _NewChipLoadingCommon

    def __new__(
        cls,
        *,
        suspension_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementDataCommon,
        buffer_volume_loaded: SuspensionMeasurementDataCommon,
        notes: str | None = ...,
    ) -> NewSingleplexChipLoading: ...

class NewSingleplexGems:
    inner: _NewGemsCommon
    loading: NewSingleplexChipLoading

    def __new__(
        cls, *, readable_id: str, chemistry: str, loading: NewSingleplexChipLoading
    ) -> NewSingleplexGems: ...

class NewSingleplexChromiumRun:
    inner: _NewChromiumRunCommon
    chip: SingleplexChromiumChip
    gems: list[NewSingleplexGems]

    def __new__(
        cls,
        *,
        readable_id: str,
        run_at: datetime,
        run_by: UUID,
        succeeded: bool,
        chip: SingleplexChromiumChip,
        gems: list[NewSingleplexGems],
        notes: str | None = ...,
    ) -> NewSingleplexChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewSingleplexChromiumRun: ...
    def to_json(self) -> str: ...

class NewOcmChipLoading:
    _0: NewSingleplexChipLoading

    def __new__(
        cls,
        *,
        suspension_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementDataCommon,
        buffer_volume_loaded: SuspensionMeasurementDataCommon,
        notes: str | None = ...,
    ) -> NewOcmChipLoading: ...

class NewOcmGems:
    inner: _NewGemsCommon
    loading: list[NewOcmChipLoading]

    def __new__(
        cls, *, readable_id: str, chemistry: str, loading: list[NewOcmChipLoading]
    ) -> NewOcmGems: ...

class OcmChromiumChip:
    GemxOcm3p = ...

class NewOcmChromiumRun:
    inner: _NewChromiumRunCommon
    chip: OcmChromiumChip
    gems: list[NewOcmGems]

    def __new__(
        cls,
        *,
        readable_id: str,
        run_at: datetime,
        succeeded: bool,
        run_by: UUID,
        chip: OcmChromiumChip,
        gems: list[NewOcmGems],
        notes: str | None = ...,
    ) -> NewOcmChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewOcmChromiumRun: ...
    def to_json(self) -> str: ...

class NewPoolMultiplexChipLoading:
    suspension_id: UUID
    inner: _NewChipLoadingCommon

    def __new__(
        cls,
        *,
        suspension_pool_id: UUID,
        suspension_volume_loaded: SuspensionMeasurementDataCommon,
        buffer_volume_loaded: SuspensionMeasurementDataCommon,
        notes: str | None = ...,
    ) -> NewPoolMultiplexChipLoading: ...

class NewPoolMultiplexGems:
    inner: _NewGemsCommon
    loading: NewPoolMultiplexChipLoading

    def __new__(
        cls, *, readable_id: str, chemistry: str, loading: NewPoolMultiplexChipLoading
    ) -> NewPoolMultiplexGems: ...

class PoolMultiplexChromiumChip:
    Q = ...
    GemxFx = ...

class NewPoolMultiplexChromiumRun:
    inner: _NewChromiumRunCommon
    chip: PoolMultiplexChromiumChip
    gems: list[NewPoolMultiplexGems]

    def __new__(
        cls,
        *,
        readable_id: str,
        run_at: datetime,
        run_by: UUID,
        succeeded: bool,
        chip: PoolMultiplexChromiumChip,
        gems: list[NewPoolMultiplexGems],
        notes: str | None = ...,
    ) -> NewPoolMultiplexChromiumRun: ...
    @staticmethod
    def from_json(json: str) -> NewPoolMultiplexChromiumRun: ...
    def to_json(self) -> str: ...

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

@dataclass
class NewLibraryTypeSpecification:
    chemistry: str
    library_type: LibraryType
    index_kit: str
    cdna_volume_µl: float
    library_volume_µl: float

    @staticmethod
    def from_json(json: str) -> NewPoolMultiplexChromiumRun: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class NucleicAcidConcentration:
    value: float
    unit: tuple[MassUnit, VolumeUnit]

class ElectrophoreticMeasurementData:
    measured_at: datetime
    instrument_name: str
    mean_library_size_bp: float
    sizing_range: tuple[int, int]
    concentration: NucleicAcidConcentration

    def __new__(
        cls,
        *,
        measured_at: datetime,
        instrument_name: str,
        mean_library_size_bp: float,
        sizing_range: tuple[int, int],
        concentration_value: float,
        concentration_unit: tuple[MassUnit, VolumeUnit],
    ) -> ElectrophoreticMeasurementData: ...
    @staticmethod
    def from_json(json: str) -> NewPoolMultiplexChromiumRun: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class NewCdnaMeasurement:
    measured_by: UUID
    data: ElectrophoreticMeasurementData
    cdna_id: UUID = ...

    @staticmethod
    def from_json(json: str) -> NewCdnaMeasurement: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class NewCdna:
    library_type: LibraryType
    readable_id: str
    prepared_at: datetime
    gems_id: UUID
    n_amplification_cycles: int
    preparer_ids: list[UUID]
    measurements: list[NewCdnaMeasurement] = ...
    storage_location: str | None = ...
    notes: str | None = ...

    @staticmethod
    def from_json(json: str) -> NewCdna: ...
    def to_json(self) -> str: ...

@dataclass
class MeasurementData_Electrophoretic(LibraryMeasurementData):
    _0: ElectrophoreticMeasurementData

@dataclass(kw_only=True)
class MeasurementData_Fluorometric(LibraryMeasurementData):
    measured_at: datetime
    instrument_name: str
    concentration: NucleicAcidConcentration

class LibraryMeasurementData:
    Electrophoretic = MeasurementData_Electrophoretic
    Fluorometric = MeasurementData_Fluorometric

@dataclass(kw_only=True)
class NewLibraryMeasurement:
    measured_by: UUID
    data: LibraryMeasurementData
    library_id: UUID = ...

@dataclass(kw_only=True)
class NewLibrary:
    readable_id: str
    cdna_id: UUID
    number_of_sample_index_pcr_cycles: int
    target_reads_per_cell: int
    prepared_at: datetime
    preparer_ids: list[UUID]
    single_index_set_name: str | None = ...
    dual_index_set_name: str | None = ...
    measurements: list[NewLibraryMeasurement] = ...
    notes: str | None = ...

    @staticmethod
    def from_json(json: str) -> NewLibrary: ...
    def to_json(self) -> str: ...

@dataclass(kw_only=True)
class _MetricsFile:
    filename: str
    raw_contents: str

@dataclass(kw_only=True)
class SingleRowCsvMetricsFile(_MetricsFile):
    contents: dict[str, Any] = field(init=False)

@dataclass(kw_only=True)
class MultiRowCsvMetricsFile(_MetricsFile):
    contents: list[dict[str, Any]] = field(init=False)

class _MultiRowCsvMetricsFileGroup(Sequence[MultiRowCsvMetricsFile]): ...

@dataclass(kw_only=True)
class JsonMetricsFile(_MetricsFile):
    contents: Any = field(init=False)

class _NewDatasetCommon:
    name: str
    lab_id: UUID
    data_path: str
    delivered_at: datetime

class _NewChromiumDatasetCore:
    inner: _NewDatasetCommon
    gems_id: UUID
    web_summary: str

class CellrangerarcvdjCountDataset:
    core: _NewChromiumDatasetCore
    metrics: SingleRowCsvMetricsFile

    def __new__(
        cls,
        *,
        name: str,
        lab_id: UUID,
        data_path: str,
        delivered_at: datetime,
        gems_id: UUID,
        web_summary: str,
        metrics: SingleRowCsvMetricsFile,
    ) -> CellrangerarcvdjCountDataset: ...
    @staticmethod
    def from_json(json: str) -> CellrangerarcvdjCountDataset: ...
    def to_json(self) -> str: ...

class CellrangerMultiDataset:
    core: _NewChromiumDatasetCore
    metrics: list[MultiRowCsvMetricsFile]

    def __new__(
        cls,
        *,
        name: str,
        lab_id: UUID,
        data_path: str,
        delivered_at: datetime,
        gems_id: UUID,
        web_summary: str,
        metrics: list[MultiRowCsvMetricsFile],
    ) -> CellrangerMultiDataset: ...
    @staticmethod
    def from_json(json: str) -> CellrangerMultiDataset: ...
    def to_json(self) -> str: ...

class CellrangeratacCountDataset:
    core: _NewChromiumDatasetCore
    metrics: JsonMetricsFile

    def __new__(
        cls,
        *,
        name: str,
        lab_id: UUID,
        data_path: str,
        delivered_at: datetime,
        gems_id: UUID,
        web_summary: str,
        metrics: JsonMetricsFile,
    ) -> CellrangeratacCountDataset: ...
    @staticmethod
    def from_json(json: str) -> CellrangeratacCountDataset: ...
    def to_json(self) -> str: ...
