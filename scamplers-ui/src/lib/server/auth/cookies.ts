import type { Cookies } from "@sveltejs/kit";
import { decryptApiKey } from "./crypto";

export class CookieNames {
  static get encryptedApiKey(): string {
    return "scamplers.encrypted_api_key";
  }
  static get apiKeyInitializationVector(): string {
    return "scamplers.api_key_initialization_vector";
  }
}

export async function apiKeyFromCookies(
  cookies: Cookies,
  encryptionSecret: CryptoKey,
) {
  const initializationVector = cookies.get(
    CookieNames.apiKeyInitializationVector,
  );
  const hexEncodedEncryptedApiKey = cookies.get(CookieNames.encryptedApiKey);

  if (!initializationVector || !hexEncodedEncryptedApiKey) {
    return null;
  }

  return await decryptApiKey(
    initializationVector,
    encryptionSecret,
    hexEncodedEncryptedApiKey,
  );
}

export async function hexEncodedApiKeyFromCookies(
  cookies: Cookies,
  encryptionSecret: CryptoKey,
): Promise<string | null> {
  const decryptedBytes = await apiKeyFromCookies(cookies, encryptionSecret);
  if (!decryptedBytes) {
    return null;
  }

  return new Uint8Array(decryptedBytes).toHex();
}
