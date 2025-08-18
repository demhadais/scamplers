import pytest
import maturin_import_hook

maturin_import_hook.install()

from scamplepy import ScamplersClient
from .test_requests import (
    new_cellranger_count_dataset_fixture,
    new_cellranger_multi_dataset_fixture,
    new_cellrangeratac_count_dataset_fixture,
    new_cryopreserved_tissue_fixture,
    new_fixed_block_fixture,
    new_fixed_tissue_fixture,
    new_frozen_block_fixture,
    new_frozen_tissue_fixture,
    new_singleplex_chromium_run_fixture,
)


__all__ = [
    "new_cellranger_count_dataset_fixture",
    "new_cellranger_multi_dataset_fixture",
    "new_cellrangeratac_count_dataset_fixture",
    "new_cryopreserved_tissue_fixture",
    "new_fixed_block_fixture",
    "new_fixed_tissue_fixture",
    "new_frozen_block_fixture",
    "new_frozen_tissue_fixture",
    "new_singleplex_chromium_run_fixture",
]


@pytest.fixture
def scamplers_client() -> ScamplersClient:
    return ScamplersClient(api_base_url="")


@pytest.mark.parametrize(
    "method_name, fixture",
    [
        ("create_specimen", "new_fixed_block_fixture"),
        ("create_specimen", "new_frozen_block_fixture"),
        ("create_specimen", "new_cryopreserved_tissue_fixture"),
        ("create_specimen", "new_fixed_tissue_fixture"),
        ("create_specimen", "new_frozen_tissue_fixture"),
        ("create_cellranger_count_dataset", "new_cellranger_count_dataset_fixture"),
        ("create_chromium_run", "new_singleplex_chromium_run_fixture"),
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
