import maturin_import_hook

maturin_import_hook.install()

import asyncio
import scamplepy
# from scamplers_core.requests import *  # noqa: F403
# import fire

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


# def create_client(
#     api_base_url: str | None = None, api_key: str | None = None
# ) -> ScamplersClient:
#     accept_invalid_certificates = None

#     match (api_base_url, api_key):
#         case (None, None):
#             api_base_url = "http://localhost:8000"
#             accept_invalid_certificates = True
#         case (None, _):
#             api_base_url = "http://localhost/api"
#             accept_invalid_certificates = True
#         case (api_base_url, None):
#             raise Exception("unexpected combination of 'api_base_url' and 'api_key'")

#     return ScamplersClient(
#         api_base_url=api_base_url,
#         api_key=api_key,
#         accept_invalid_certificates=accept_invalid_certificates,
#     )


# async def main(api_base_url: str | None = None, api_key: str | None = None):
#     client = create_client(api_base_url, api_key)

#     institution = new_institution()
#     institution = await client.create_institution(institution)

#     person = new_person(institution_id=institution.handle.id)
#     person = await client.create_person(person)
#     person_id = person.core.summary.handle.id

#     lab = new_lab(pi_id=person_id)
#     lab = await client.create_lab(lab)
#     lab_id = lab.core.summary.handle.id

#     new_specimen_fns = [
#         new_fixed_block,
#         new_frozen_block,
#         new_cryopreserved_tissue,
#         new_fixed_tissue,
#         new_frozen_tissue,
#         new_virtual_specimen,
#     ]
#     specimen_creation_tasks = []
#     async with asyncio.TaskGroup() as tg:
#         for specimen_fn in new_specimen_fns:
#             specimen = specimen_fn(lab_id=lab_id, person_id=person_id)
#             specimen = tg.create_task(client.create_specimen(specimen))
#             specimen_creation_tasks.append(specimen)

#     specimens = [task.result() for task in specimen_creation_tasks]

#     suspension_pool = new_suspension_pool(
#         person_id=person_id,
#         parent_specimen_ids=[specimen.core.summary.handle.id for specimen in specimens],
#     )
#     await client.create_suspension_pool(suspension_pool)


# if __name__ == "__main__":
#     asyncio.run(fire.Fire(main))
