import type { Institution } from "../scamplers-models/institution";
import type { InstitutionQuery } from "../scamplers-models/institution_query";
import type { ServerLoadEvent } from "@sveltejs/kit";
import { hexEncodedApiKeyFromCookies } from "../../server/auth/cookies";
import { API_KEY_ENCRYPTION_SECRET } from "../../server/auth/crypto";
import { SERVER_CONFIG } from "../../server/config";

class ApiClient {
  readonly apiBaseUrl: string;

  constructor(apiBaseUrl: string) {
    this.apiBaseUrl = apiBaseUrl;
  }

  private async sendRequest<T>(
    { cookies, fetch, url }: ServerLoadEvent,
    {
      endpoint,
      method,
      data,
    }: { endpoint: string; method: string; data?: unknown },
  ): Promise<T> {
    const apiUrl = `${this.apiBaseUrl}/${endpoint}?${url.search}`;

    const apiKey = await hexEncodedApiKeyFromCookies(
      cookies,
      API_KEY_ENCRYPTION_SECRET,
    );
    const options: RequestInit = {
      method,
      headers: {
        "X-API-Key": apiKey || "",
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    };

    const response = await fetch(apiUrl, options);
    const asJson = await response.json();

    if (asJson.error) {
      throw asJson;
    }

    return asJson;
  }

  private async get<T>(event: ServerLoadEvent, endpoint: string): Promise<T> {
    return await this.sendRequest(event, { endpoint, method: "GET" });
  }

  private async post<T>(
    event: ServerLoadEvent,
    endpoint: string,
    data: unknown,
  ): Promise<T> {
    return await this.sendRequest(event, { endpoint, method: "POST", data });
  }

  async listInstitutions(event: ServerLoadEvent): Promise<Institution[]> {
    return await this.get(event, "institutions");
  }
}

export const apiClient = new ApiClient(SERVER_CONFIG.apiUrl as string);
