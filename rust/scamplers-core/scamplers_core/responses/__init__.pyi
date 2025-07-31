from datetime import datetime
from uuid import UUID

from ..requests import (
    JsonMetricsFile,
    MultiRowCsvMetricsFile,
    SingleRowCsvMetricsFile,
    SpecimenMeasurementData,
    UserRole,
    _SuspensionMeasurementData,
)
from .._baseclass import _Handle, _Json

class InstitutionHandle(_Handle): ...

class Institution(_Json):
    handle: InstitutionHandle
    name: str

class LabHandle(_Handle): ...

class LabSummary(_Json):
    handle: LabHandle
    name: str
    delivery_dir: str

class PersonHandle(_Handle): ...

class PersonSummary(_Json):
    handle: PersonHandle
    name: str
    email: str | None
    orcid: str | None

class PersonCore:
    summary: PersonSummary
    institution: Institution

class Person(_Json):
    core: PersonCore
    roles: list[UserRole]

class LabCore:
    summary: LabSummary
    pi: PersonSummary

class Lab(_Json):
    core: LabCore
    members: list[PersonSummary]

class SpecimenHandle(_Handle): ...

class SpecimenSummary(_Json):
    handle: SpecimenHandle
    readable_id: str
    name: str
    received_at: datetime
    species: list[str | None]
    notes: str | None
    returned_at: datetime | None
    type_: str
    embedded_in: str | None
    fixative: str | None
    frozen: bool
    cryopreserved: bool
    storage_buffer: str | None

class SpecimenCore:
    summary: SpecimenSummary
    lab: LabSummary
    submitted_by: PersonSummary

class SpecimenMeasurement:
    measured_by: PersonHandle
    data: SpecimenMeasurementData

class Specimen(_Json):
    core: SpecimenCore
    measurements: list[SpecimenMeasurement]

class SequencingRunHandle(_Handle): ...
class SequencingRunSummary(_Json): ...
class SuspensionHandle(_Handle): ...

class SuspensionSummary(_Json):
    handle: SuspensionHandle
    readable_id: str
    biological_material: str
    created_at: datetime | None
    lysis_duration_minutes: float | None
    target_cell_recovery: float
    target_reads_per_cell: int
    notes: str | None

class MultiplexingTag:
    id: UUID
    tag_id: str
    type_: str

class SuspensionCore:
    summary: SuspensionSummary
    parent_specimen: SpecimenSummary
    multiplexing_tag: MultiplexingTag

class SuspensionMeasurement:
    measured_by: PersonHandle
    data: _SuspensionMeasurementData

class Suspension(_Json):
    core: SuspensionCore
    preparers: list[PersonHandle]
    measurements: list[SuspensionMeasurement]

class SuspensionPoolHandle(_Handle): ...

class SuspensionPoolSummary(_Json):
    handle: SuspensionPoolHandle
    readable_id: str
    pooled_at: datetime

class ChromiumRunHandle(_Handle): ...

class ChromiumRunSummary(_Json):
    readable_id: str
    chip: str
    run_at: datetime
    succeeded: bool
    notes: str | None

class _GemsHandle(_Handle): ...

class ChromiumRun(_Json):
    summary: ChromiumRunSummary
    gems: list[_GemsHandle]

class CdnaHandle(_Handle): ...
class LibraryHandle(_Handle): ...
class DatasetHandle(_Handle): ...

class DatasetSummary(_Json):
    handle: DatasetHandle
    data_path: str
    delivered_at: datetime
    web_summary: str
    metrics: (
        SingleRowCsvMetricsFile | list[MultiRowCsvMetricsFile] | JsonMetricsFile | None
    )
