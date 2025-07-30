from typing import Any
from typing_extensions import Callable
import maturin_import_hook

maturin_import_hook.install()

from scamplers_core import ScamplersClient
from scamplers_core.requests import (
    CellrangerMultiDataset,
    CellrangerarcvdjCountDataset,
    CellrangeratacCountDataset,
    NewFixedBlock,
    NewFrozenBlock,
)
from .test_requests import (
    new_cellrangerarcvdj_count_dataset,
    new_cellrangeratac_count_dataset,
    new_cellranger_multi_dataset,
    new_fixed_block,
    specimen_dv200,
    specimen_rin,
    new_committee_approval,
    new_frozen_block,
)

__all__ = [
    "new_cellrangerarcvdj_count_dataset",
    "new_cellrangeratac_count_dataset",
    "new_cellranger_multi_dataset",
    "new_fixed_block",
    "specimen_dv200",
    "specimen_rin",
    "new_committee_approval",
    "new_frozen_block",
]


def test_client(
    new_fixed_block: NewFixedBlock,
    new_frozen_block: NewFrozenBlock,
    new_cellrangerarcvdj_count_dataset: CellrangerarcvdjCountDataset,
    new_cellrangeratac_count_dataset: CellrangeratacCountDataset,
    new_cellranger_multi_dataset: CellrangerMultiDataset,
):
    client = ScamplersClient(api_base_url="")
    methods_params: list[tuple[Callable, Any]] = [
        (client.create_specimen, new_fixed_block),
        (client.create_specimen, new_frozen_block),
        (
            client.create_cellrangerarc_count_dataset,
            new_cellrangerarcvdj_count_dataset,
        ),
        (client.create_cellrangeratac_count_dataset, new_cellrangeratac_count_dataset),
        (client.create_cellranger_count_dataset, new_cellrangerarcvdj_count_dataset),
        (client.create_cellranger_multi_dataset, new_cellranger_multi_dataset),
        (client.create_cellranger_vdj_dataset, new_cellrangerarcvdj_count_dataset),
    ]

    for method, data in methods_params:
        _ = method(data)
