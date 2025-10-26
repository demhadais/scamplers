import { API_BASE_URL } from "$env/static/public";
import { FRONTEND_TOKEN } from "./secrets.ts";
import type { PersonCreation } from "scamplers-models/person_creation.d.ts";
import type { CreatedUser } from "scamplers-models/created_user.d.ts";

class ApiClient {
  constructor(
    private api_base_url: string,
    private frontend_token: string,
  ) {}

  private async sendRequest<Resp>(
    endpoint: string,
    data: unknown,
    fetch_fn: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response> = fetch,
  ): Resp {
    const response = await fetch_fn(`${this.api_base_url}/${endpoint}`, {
      headers: { "Authorization: Bearer": this.frontend_token },
      body: JSON.stringify(data),
    });

    return await response.json();
  }

  async login_ms_user(user: PersonCreation): CreatedUser {
    return await this.sendRequest("users", user);
  }
}

export const serverApiClient = new ApiClient(API_BASE_URL, FRONTEND_TOKEN);
