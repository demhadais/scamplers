from dataclasses import dataclass

from .responses import Institution, DatasetSummary
from .requests import (
    CellrangerMultiDataset,
    CellrangerarcvdjCountDataset,
    CellrangeratacCountDataset,
    NewCryopreservedTissue,
    NewFixedBlock,
    NewFixedTissue,
    NewFrozenBlock,
    NewFrozenTissue,
    NewInstitution,
    NewVirtualSpecimen,
)

@dataclass(kw_only=True)
class ScamplersClient:
    api_base_url: str
    api_key: str | None = ...

    async def create_institution(self, data: NewInstitution) -> Institution: ...
    async def create_specimen(
        self,
        data: NewFixedBlock
        | NewFrozenBlock
        | NewCryopreservedTissue
        | NewFixedTissue
        | NewFrozenTissue
        | NewVirtualSpecimen,
    ) -> None: ...
    async def create_cellrangerarc_count_dataset(
        self, data: CellrangerarcvdjCountDataset
    ) -> DatasetSummary: ...
    async def create_cellrangeratac_count_dataset(
        self, data: CellrangeratacCountDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_count_dataset(
        self, data: CellrangerarcvdjCountDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_multi_dataset(
        self, data: CellrangerMultiDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_vdj_dataset(
        self, data: CellrangerarcvdjCountDataset
    ) -> DatasetSummary: ...
