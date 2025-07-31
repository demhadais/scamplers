from typing import Self
from uuid import UUID

class _Json:
    @classmethod
    def from_json(cls, json: str) -> Self: ...
    def to_json(self) -> str: ...

class _Handle(_Json):
    id: UUID
    link: str
