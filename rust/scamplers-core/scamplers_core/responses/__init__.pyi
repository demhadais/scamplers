from datetime import datetime
from uuid import UUID

from ..requests import JsonMetricsFile, MultiRowCsvMetricsFile, SingleRowCsvMetricsFile

class InstitutionHandle:
    id: UUID
    link: str

    @staticmethod
    def from_json(json: str) -> InstitutionHandle: ...
    def to_json(self) -> str: ...

class Institution:
    handle: InstitutionHandle
    name: str

    @staticmethod
    def from_json(json: str) -> Institution: ...
    def to_json(self) -> str: ...

class DatasetHandle:
    id: UUID
    link: str

    @staticmethod
    def from_json(json: str) -> DatasetSummary: ...
    def to_json(self) -> str: ...

class DatasetSummary:
    handle: DatasetHandle
    data_path: str
    delivered_at: datetime
    web_summary: str
    metrics: (
        SingleRowCsvMetricsFile | list[MultiRowCsvMetricsFile] | JsonMetricsFile | None
    )

    @staticmethod
    def from_json(json: str) -> DatasetSummary: ...
    def to_json(self) -> str: ...
