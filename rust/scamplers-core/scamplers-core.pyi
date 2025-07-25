from uuid import UUID

class NewInstitution:
    id: UUID
    name: str
    def __new__(cls, id: UUID, name: str) -> None: ...

class Institution:
    id: UUID
    name: str
    ...

class Client:
    def __new__(cls, api_base_url: str, api_key: str) -> None: ...

    async def create_institution(self, institution: NewInstitution) -> Institution: ...
