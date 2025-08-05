from dataclasses import dataclass

from .responses import (
    CdnaHandle,
    ChromiumRun,
    Institution,
    DatasetSummary,
    Lab,
    LibraryHandle,
    Person,
    SequencingRunSummary,
    Specimen,
    Suspension,
    SuspensionPoolHandle,
)
from .requests import (
    CellrangerMultiDataset,
    CellrangerarcCountDataset,
    CellrangerCountDataset,
    CellrangerVdjDataset,
    CellrangeratacCountDataset,
    NewCdna,
    NewCryopreservedTissue,
    NewFixedBlock,
    NewFixedTissue,
    NewFrozenBlock,
    NewFrozenTissue,
    NewInstitution,
    NewLab,
    NewLibrary,
    NewOcmChromiumRun,
    NewPerson,
    NewPoolMultiplexChromiumRun,
    NewSequencingRun,
    NewSingleplexChromiumRun,
    NewSuspension,
    NewSuspensionPool,
    NewVirtualSpecimen,
)

@dataclass(kw_only=True)
class ScamplersClient:
    api_base_url: str
    api_key: str | None = ...
    accept_invalid_certificates: bool | None = ...

    async def create_institution(self, data: NewInstitution) -> Institution: ...
    async def create_person(self, data: NewPerson) -> Person: ...
    async def create_lab(self, data: NewLab) -> Lab: ...
    async def create_specimen(
        self,
        data: NewFixedBlock
        | NewFrozenBlock
        | NewCryopreservedTissue
        | NewFixedTissue
        | NewFrozenTissue
        | NewVirtualSpecimen,
    ) -> Specimen: ...
    async def create_suspension_pool(
        self, data: NewSuspensionPool
    ) -> SuspensionPoolHandle: ...
    async def create_suspension(self, data: NewSuspension) -> Suspension: ...
    async def create_sequencing_run(
        self, data: NewSequencingRun
    ) -> SequencingRunSummary: ...
    async def create_chromium_run(
        self,
        data: NewSingleplexChromiumRun
        | NewOcmChromiumRun
        | NewPoolMultiplexChromiumRun,
    ) -> ChromiumRun: ...
    async def create_cdna(self, data: NewCdna) -> CdnaHandle: ...
    async def create_library(self, data: NewLibrary) -> LibraryHandle: ...
    async def create_cellrangerarc_count_dataset(
        self, data: CellrangerarcCountDataset
    ) -> DatasetSummary: ...
    async def create_cellrangeratac_count_dataset(
        self, data: CellrangeratacCountDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_count_dataset(
        self, data: CellrangerCountDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_multi_dataset(
        self, data: CellrangerMultiDataset
    ) -> DatasetSummary: ...
    async def create_cellranger_vdj_dataset(
        self, data: CellrangerVdjDataset
    ) -> DatasetSummary: ...
