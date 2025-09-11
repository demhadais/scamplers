import pytest
import maturin_import_hook

maturin_import_hook.install()

from scamplepy import ScamplersClient
from .test_requests import (
    new_cellranger_count_dataset,
    # new_cellranger_multi_dataset,
    # new_cellrangeratac_count_dataset,
    new_cdna_group,
    new_cryopreserved_tissue,
    new_fixed_block,
    new_fixed_tissue,
    new_frozen_block,
    new_frozen_tissue,
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
]


@pytest.fixture
def scamplers_client() -> ScamplersClient:
    return ScamplersClient(api_base_url="")


@pytest.mark.parametrize(
    "method_name, fixture",
    [
        ("create_specimen", "new_fixed_block"),
        ("create_specimen", "new_frozen_block"),
        ("create_specimen", "new_cryopreserved_tissue"),
        ("create_specimen", "new_fixed_tissue"),
        ("create_specimen", "new_frozen_tissue"),
        ("create_chromium_run", "new_singleplex_chromium_run"),
        ("create_cdna", "new_cdna_group"),
        ("create_library", "new_library"),
        ("create_chromium_dataset", "new_cellranger_count_dataset"),
    ],
)
def test_client(
    scamplers_client: ScamplersClient,
    method_name: str,
    fixture: str,
    request: pytest.FixtureRequest,
):
    method = scamplers_client.__getattribute__(method_name)
    data = request.getfixturevalue(fixture)
    _ = method(data)
