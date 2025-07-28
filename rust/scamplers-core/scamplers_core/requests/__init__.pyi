from uuid import UUID
from enum import Enum

class NewInstitution:
    id: UUID
    name: str
    def __init__(self, id: UUID, name: str): ...

class UserRole(Enum):
    AppAdmin = ...
    BiologyStaff = ...
    ComputationalStaff = ...

class NewPerson:
    def __init__(
        self, name: str, email: str, institution_id: UUID, roles: list[UserRole]
    ): ...
