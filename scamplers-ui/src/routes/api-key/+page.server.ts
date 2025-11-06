import { EncryptedApiKey } from "../../server/auth/api-key";
import { apiKeyFromCookies } from "../../server/auth/cookies";
import { API_KEY_ENCRYPTION_SECRET } from "../../server/auth/crypto";
import { insertApiKey } from "../../server/auth/db";
import { SERVER_CONFIG } from "../../server/config";
import { dbClient } from "../../server/db-client";

// In theory, we could just make a call to the REST API to
export const actions = {
  default: async ({ cookies }) => {
    const apiKeyPrefixLength = SERVER_CONFIG.apiKeyPrefixLength;

    const newUnencryptedApiKey = EncryptedApiKey.newUnencrypted();

    const userApiKey = await apiKeyFromCookies(
      cookies,
      API_KEY_ENCRYPTION_SECRET,
    );
    if (!userApiKey) {
      throw "unauthorized";
    }

    const userApiKeyPrefix = userApiKey.slice(0, apiKeyPrefixLength);

    // In theory, we could just make a call to the REST API to get the user's ID, but this is a simple query so it's easier to do it here than to write an API route in scamplers-api
    const [results, newEncryptedApiKey] = await Promise.all([
      dbClient`select user_id from api_keys where prefix = ${userApiKeyPrefix}`,
      EncryptedApiKey.fromRandomValues(
        newUnencryptedApiKey,
        API_KEY_ENCRYPTION_SECRET,
        apiKeyPrefixLength,
      ),
    ]);
    const personId = results[0].user_id;

    await insertApiKey(newEncryptedApiKey, personId, dbClient);

    return newUnencryptedApiKey.toHex();
  },
};
