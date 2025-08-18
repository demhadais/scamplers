import maturin_import_hook
maturin_import_hook.install()

from uuid import UUID
from scamplepy.create import NewInstitution

NewInstitution(id = UUID(int=0), name="foo")
