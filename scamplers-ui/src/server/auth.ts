import { betterAuth } from "better-auth";
import { sveltekitCookies } from "better-auth/svelte-kit";
import { getRequestEvent } from "$app/server";
import { createAuthMiddleware } from "better-auth/api";
import { SECRETS, SERVER_CONFIG } from "./config";
import { dbClient } from "./db-client";
import {
  ENCRYPTION_ALGORITHM,
  API_KEY_ENCRYPTION_SECRET,
  ApiKey,
} from "./auth/api-key";
import type { MicrosoftEntraIDProfile } from "better-auth/social-providers";

let microsoftEntraProfiles: Record<string, MicrosoftEntraIDProfile> = {};

const SECONDS_PER_MINUTE = 60;
const MINUTES_PER_HOUR = 60;
const HOURS_PER_DAY = 24;
const DAYS_PER_YEAR = 365;
const ONE_YEAR =
  SECONDS_PER_MINUTE * MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_YEAR;

async function createPerson(
  {
    name,
    email,
    emailVerified,
    tid,
    oid,
  }: {
    emailVerified: boolean;
  } & MicrosoftEntraIDProfile,
  apiKeyEncryptionSecret: CryptoKey,
  dbClient: Bun.SQL,
): Promise<{ apiKey: ApiKey; userId: string }> {
  const apiKey = await ApiKey.new(apiKeyEncryptionSecret);
  const newPerson = {
    name,
    email,
    email_verified: emailVerified,
    institution_id: tid,
    microsoft_entra_oid: oid,
  };

  const newPersonId = await dbClient.begin(async (tx) => {
    await tx`update people set email = ${null} where email = ${newPerson.email}`;

    const result =
      await tx`insert into people ${tx(newPerson)} on conflict (microsoft_entra_oid) do update set ${tx(newPerson)} returning id`;
    const newPersonId = result[0].id;

    const apiKeyData = {
      prefix: apiKey.prefix.toHex(),
      hash: apiKey.hash,
      user_id: newPersonId,
    };

    await tx`insert into api_keys ${tx(apiKeyData)}`;
    // Create a db user corresponding to this person so we can assign them a role. Note that we set a random password so that nobody can log into the database as that user.
    await tx`select create_user_if_not_exists(${newPersonId}, ${crypto.getRandomValues(new Uint8Array(32)).toHex()}, '{}')`;

    return newPersonId;
  });

  return { apiKey, userId: newPersonId };
}

async function deleteApiKey(
  {
    encryptedApiKey,
    encryptionSecret,
    initializationVector,
  }: {
    encryptedApiKey: string;
    encryptionSecret: CryptoKey;
    initializationVector: string;
  },
  dbClient: Bun.SQL,
) {
  const decrypted = await crypto.subtle.decrypt(
    { iv: Uint8Array.fromHex(initializationVector), ...ENCRYPTION_ALGORITHM },
    encryptionSecret,
    Uint8Array.fromHex(encryptedApiKey),
  );

  const prefix = new Uint8Array(
    decrypted.slice(0, SERVER_CONFIG.api_key_prefix_length! as number),
  ).toHex();

  // No two users share the same prefix
  await dbClient`delete from api_keys where prefix = ${prefix}`;
}

const COOKIE_NAMES = [
  "scamplers.encrypted_api_key",
  "scamplers.api_key_initialization_vector",
  "scamplers.user_id",
];
const COOKIE_OPTIONS = {
  secure: true,
  httpOnly: true,
  path: "/",
};

export const auth = betterAuth({
  baseURL: SERVER_CONFIG.baseUrl as string,
  secret: SECRETS.auth_secret,
  socialProviders: {
    microsoft: {
      clientId: SECRETS.microsoft_entra_client_id!,
      clientSecret: SECRETS.microsoft_entra_client_secret,
      tenantId: SECRETS.microsoft_entra_tenant,
      // This is a bit of a hack. We need the user's Microsoft Entra OID and tenant ID, which is only available in this function.
      mapProfileToUser: async (profile) => {
        // Using a user's email address as a unique key is typically poor practice because of one of the following (https://learn.microsoft.com/en-us/entra/identity-platform/id-token-claims-reference#payload-claims):
        // 1. an email address can be reassigned to a different person
        // 2. a user could sign in from two different browsers at the same time (same email address, different sessions)
        // It is extremely unlikely that either of these will cause a problem because this key-value pair exists for an infinitesimal period of time.
        microsoftEntraProfiles[profile.email] = profile;
      },
    },
  },
  session: {
    cookieCache: {
      enabled: true,
      strategy: "jwe",
      refreshCache: true,
      maxAge: ONE_YEAR,
    },
  },
  plugins: [sveltekitCookies(getRequestEvent)],
  hooks: {
    after: createAuthMiddleware(async (ctx) => {
      const { newSession, secret: cookieSigningSecret } = ctx.context;

      // If the user is signing out, erase cookies and delete the API key from the database. Note this check is necessarily performed before checking `newSession`
      if (ctx.path.includes("sign-out")) {
        const cookieDeletions = COOKIE_NAMES.map((cookieName) => {
          // Get the cookie
          const cookie = ctx.getSignedCookie(cookieName, cookieSigningSecret);

          // Delete the cookie from the user's cookie store
          ctx.setSignedCookie(cookieName, "", cookieSigningSecret, {
            maxAge: 0,
            ...COOKIE_OPTIONS,
          });

          return cookie;
        });

        const [encryptedApiKey, initializationVector, userId] =
          await Promise.all(cookieDeletions);

        // Delete the API key from the database
        await deleteApiKey(
          {
            encryptedApiKey: encryptedApiKey!,
            initializationVector: initializationVector!,
            encryptionSecret: API_KEY_ENCRYPTION_SECRET,
          },
          dbClient,
        );
      }

      if (!newSession) {
        return;
      }

      const { email, emailVerified } = newSession.user;
      const microsoftEntraProfile = microsoftEntraProfiles[email];

      if (!microsoftEntraProfile) {
        return;
      }

      delete microsoftEntraProfiles[email];

      // Upsert the user in the database, returning an encrypted API key
      const { apiKey, userId } = await createPerson(
        {
          emailVerified,
          ...microsoftEntraProfile,
        },
        API_KEY_ENCRYPTION_SECRET,
        dbClient,
      );

      const cookieValues = [
        apiKey.encryptedKey.toHex(),
        apiKey.initializationVector.toHex(),
        userId,
      ];
      const setCookies = COOKIE_NAMES.map((cookieName, i) => {
        return ctx.setSignedCookie(
          cookieName,
          cookieValues[i]!,
          cookieSigningSecret,
          { maxAge: ONE_YEAR, ...COOKIE_OPTIONS },
        );
      });

      await Promise.all(setCookies);
    }),
  },
});
