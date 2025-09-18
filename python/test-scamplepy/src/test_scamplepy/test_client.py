from collections.abc import Callable
from typing import Any
import maturin_import_hook
import pytest

maturin_import_hook.install()

from scamplepy import ScamplersClient

from .test_requests import (
    # new_cellranger_multi_dataset,
    # new_cellrangeratac_count_dataset,
    new_cdna_group,
    new_cellranger_count_dataset,
    new_cryopreserved_tissue,
    new_fixed_block,
    new_fixed_tissue,
    new_frozen_block,
    new_frozen_tissue,
    new_library,
    new_singleplex_chromium_run,
)

# This is to prevent ruff from removing "unused imports"
__all__ = [
    "new_cellranger_count_dataset",
    # "new_cellranger_multi_dataset",
    # "new_cellrangeratac_count_dataset",
    "new_cdna_group",
    "new_cryopreserved_tissue",
    "new_fixed_block",
    "new_fixed_tissue",
    "new_frozen_block",
    "new_frozen_tissue",
    "new_singleplex_chromium_run",
    "new_library",
]


@pytest.fixture
def scamplers_client() -> ScamplersClient:
    return ScamplersClient(api_base_url="")


@pytest.mark.parametrize(
    "method, fixture",
    [
        (ScamplersClient.create_specimen, "new_fixed_block"),
        (ScamplersClient.create_specimen, "new_frozen_block"),
        (ScamplersClient.create_specimen, "new_cryopreserved_tissue"),
        (ScamplersClient.create_specimen, "new_fixed_tissue"),
        (ScamplersClient.create_specimen, "new_frozen_tissue"),
        (ScamplersClient.create_chromium_run, "new_singleplex_chromium_run"),
        (ScamplersClient.create_cdna, "new_cdna_group"),
        (ScamplersClient.create_library, "new_library"),
        (ScamplersClient.create_chromium_dataset, "new_cellranger_count_dataset"),
    ],
)
def test_client(
    scamplers_client: ScamplersClient,
    method: Callable[[ScamplersClient, Any], Any],
    fixture: str,
    request: pytest.FixtureRequest,
):
    data = request.getfixturevalue(fixture)
    _ = method(scamplers_client, data)
