from uuid import UUID

from scamplers_core.requests import LibraryType

class ClientError:
    message: str

class DuplicateResourceError:
    entity: str
    fields: list[str]
    values: list[str]

class InvalidReferenceError:
    entity: str
    referenced_entity: str
    value: str | None

class ResourceNotFoundError:
    requested_resource_id: UUID

class InvalidDataError:
    message: str

class MalformedRequestError:
    message: str

class PermissionDeniedError:
    message: str

class ServerError:
    message: str
    raw_response_body: str

class DatasetCmdlineError:
    chemistry: str | None
    expected_cmdline: str
    found_cmdline: str

class DatasetNMetricsFilesError:
    expected_n_metrics_files: int
    found_n_metrics_files: int

class DatasetMetricsFileParseError:
    message: str

class CdnaLibraryTypeError:
    expected_library_types: list[LibraryType]
    found_library_types: list[LibraryType]

class CdnaGemsError:
    message: str

class InvalidMeasurementError:
    message: str

class ScamplersCoreErrorResponse:
    status: int | None
    error: (
        ClientError
        | DuplicateResourceError
        | InvalidReferenceError
        | ResourceNotFoundError
        | InvalidDataError
        | MalformedRequestError
        | PermissionDeniedError
        | ServerError
        | CdnaGemsError
        | CdnaLibraryTypeError
        | DatasetCmdlineError
        | DatasetNMetricsFilesError
        | DatasetMetricsFileParseError
        | InvalidMeasurementError
    )
