from uuid import UUID

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
    def from_json(json: str) -> InstitutionHandle: ...
    def to_json(self) -> str: ...
