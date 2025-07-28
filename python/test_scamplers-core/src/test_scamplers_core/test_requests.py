import maturin_import_hook

maturin_import_hook.install()
from scamplers_core.requests import NewInstitution
from uuid import uuid4


def test_new_institution():
    NewInstitution(id=uuid4(), name="")


def test_new_person(): ...
