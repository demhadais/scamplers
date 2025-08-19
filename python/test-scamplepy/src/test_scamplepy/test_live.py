import maturin_import_hook

maturin_import_hook.install()
from datetime import UTC, datetime
import uuid
from scamplepy.query import OrderBy, SpecimenQuery, SpecimenType
from scamplepy import ScamplersClient
from scamplepy.create import *  # noqa: F403
import fire

# from test_scamplers_core.test_requests import (
#     new_cryopreserved_tissue,
#     new_institution,
#     new_person,
#     new_lab,
#     new_fixed_block,
#     new_frozen_block,
#     new_fixed_tissue,
#     new_frozen_tissue,
#     new_virtual_specimen,
#     new_suspension_pool,
# )


def create_client(
    api_base_url: str | None = None, api_key: str | None = None
) -> ScamplersClient:
    accept_invalid_certificates = False

    match (api_base_url, api_key):
        case (None, None):
            api_base_url = "http://localhost:8000"
            accept_invalid_certificates = True
        case (None, _):
            api_base_url = "http://localhost/api"
            accept_invalid_certificates = True
        case (api_base_url, None):
            raise Exception("unexpected combination of 'api_base_url' and 'api_key'")

    return ScamplersClient(
        api_base_url=api_base_url,
        api_key=api_key,
        accept_invalid_certificates=accept_invalid_certificates,
    )


async def main(api_base_url: str | None = None, api_key: str | None = None):
    client = create_client(api_base_url, api_key)

    institution = NewInstitution(id=uuid.uuid4(), name="institution")
    institution = await client.create_institution(institution)

    person = NewPerson(
        name="name",
        email="name@example.com",
        institution_id=institution.id,
        roles=[UserRole.AppAdmin],
    )
    person = await client.create_person(person)

    lab = NewLab(name="lab", pi_id=person.info.id_, delivery_dir="delivery")
    lab = await client.create_lab(lab)

    dv200 = SpecimenMeasurementData.Dv200(
        measured_at=datetime.now(UTC), instrument_name="mayonnaise", value=0.5
    )
    rin = SpecimenMeasurementData.Rin(
        measured_at=datetime.now(UTC), instrument_name="mayonnaise", value=5
    )

    specimen = NewFixedBlock(
        readable_id="SP01",
        name="f",
        submitted_by=person.info.id_,
        lab_id=lab.info.id_,
        received_at=datetime.now(UTC),
        species=[Species.HomoSapiens],
        measurements=[
            NewSpecimenMeasurement(measured_by=person.info.id_, data=m)
            for m in [dv200, rin]
        ],
        committee_approvals=[],
        embedded_in=FixedBlockEmbeddingMatrix.Paraffin,
        fixative=BlockFixative.FormaldehydeDerivative,
    )

    created_specimen = await client.create_specimen(specimen)
    fetched_specimens = await client.list_person_specimens(
        person.info.id_,
        SpecimenQuery(
            name="f",
            order_by=[OrderBy(field="received_at", descending=False)],
            types=[SpecimenType.Block],
        ),
    )

    assert len(fetched_specimens) == 1
    assert created_specimen == fetched_specimens[0]


if __name__ == "__main__":
    fire.Fire(main)
