import type { Institution } from "$lib/scamplers-models/institution";
import { getApiClient } from "$lib/server/scamplers-client";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async (event) => {
  const apiClient = await getApiClient();
  const institutions: Institution[] = await apiClient.listInstitutions(event);
  return { institutions };
};
