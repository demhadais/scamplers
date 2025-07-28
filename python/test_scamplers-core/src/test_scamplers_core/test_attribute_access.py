import maturin_import_hook

maturin_import_hook.install()
import pytest
from scamplers_core.requests import NewInstitution
from scamplers_core.responses import Institution


@pytest.mark.parametrize(
    "class_, attributes", [(NewInstitution, []), (Institution, [])]
)
def test_attribute_access(class_: type, attributes: list[str]): ...
