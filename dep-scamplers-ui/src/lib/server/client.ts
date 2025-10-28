import { SCAMPLERS_PUBLIC_API_URL } from "$env/static/private";
import { UI_AUTH_TOKEN } from "./secrets";
import type { PersonCreation } from "$lib/scamplers-models/person_creation";
import type { CreatedUser } from "$lib/scamplers-models/created_user";

interface Error {
  status: number;
  error: object;
}

export function isError<T>(result: T | Error): result is Error {
  return (result as Error).error !== undefined;
}

class ApiClient {
  constructor(
    private api_url: string,
    private ui_auth_token: string,
  ) {}

  private async sendRequest<Resp>({
    method,
    path,
    apiKey = "",
    data,
    fetch_fn = fetch,
  }: {
    method: string;
    path: string;
    apiKey?: string;
    data?: unknown;
    fetch_fn?: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response>;
  }): Promise<Resp> {
    const request: RequestInit = {
      method,
      headers: {
        authorization: `Bearer ${this.ui_auth_token}`,
        "content-type": "application/json",
        "X-API-Key": apiKey,
      },
    };

    if (data) {
      request.body = JSON.stringify(data);
    }

    const response = await fetch_fn(`${this.api_url}/${path}`, request);

    return await response.json();
  }

  private async get<Resp>({
    path,
    apiKey,
    fetch_fn = fetch,
  }: {
    path: string;
    apiKey?: string;
    fetch_fn?: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response>;
  }): Promise<Resp> {
    return await this.sendRequest({ method: "GET", path, apiKey, fetch_fn });
  }

  private async post<Resp>({
    path,
    apiKey,
    data,
    fetch_fn = fetch,
  }: {
    path: string;
    apiKey?: string;
    data: unknown;
    fetch_fn?: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response>;
  }): Promise<Resp> {
    return await this.sendRequest({
      method: "POST",
      path,
      apiKey,
      data,
      fetch_fn,
    });
  }

  private async delete<Resp>({
    path,
    apiKey,
    fetch_fn = fetch,
  }: {
    path: string;
    apiKey?: string;
    fetch_fn?: (
      input: RequestInfo | URL,
      init?: RequestInit,
    ) => Promise<Response>;
  }): Promise<Resp> {
    return await this.sendRequest({
      method: "DELETE",
      path,
      apiKey,
      fetch_fn,
    });
  }

  async microsoftSignIn(user: PersonCreation): Promise<CreatedUser | Error> {
    return await this.post({ path: "sign-in/microsoft", data: user });
  }

  async deleteApiKey(apiKey: string): Promise<null | Error> {
    return await this.delete({
      path: `api-keys/${apiKey.slice(0, 8)}`,
      apiKey,
    });
  }
}

export const apiClient = new ApiClient(SCAMPLERS_PUBLIC_API_URL, UI_AUTH_TOKEN);
