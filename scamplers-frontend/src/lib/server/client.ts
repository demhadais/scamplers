import { PUBLIC_API_BASE_URL } from "$env/static/public";
import { FRONTEND_TOKEN } from "./secrets";
import type { PersonCreation } from "scamplers-models/person_creation";
import type { CreatedUser } from "scamplers-models/created_user";

class ApiClient {
  constructor(
    private api_base_url: string,
    private frontend_token: string,
  ) {}

  private async post<Resp>({
    endpoint,
    data,
    fetch_fn = fetch,
  }: {
    endpoint: string;
    data: unknown;
    fetch_fn?: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response>;
  }): Promise<Resp> {
    const response = await fetch_fn(`${this.api_base_url}/${endpoint}`, {
      method: "POST",
      headers: {
        Authorization: `Bearer: ${this.frontend_token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    return await response.json();
  }

  async microsoftSignin(user: PersonCreation): Promise<CreatedUser> {
    return await this.post({ endpoint: "signin/microsoft", data: user });
  }

  async signOut(apiKey: String) {
    const response = await fetch(
      `${this.api_base_url}/api-keys/${apiKey.substring(0, 8)}`,
      {
        method: "DELETE",
        headers: {
          "X-API-Key": apiKey,
        },
      },
    );

    return await response.json();
  }
}

export const serverApiClient = new ApiClient(
  PUBLIC_API_BASE_URL,
  FRONTEND_TOKEN,
);
