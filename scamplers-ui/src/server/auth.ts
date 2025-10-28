import { betterAuth } from "better-auth";
import {
  MICROSOFT_ENTRA_ID_ID,
  MICROSOFT_ENTRA_ID_SECRET,
  MICROSOFT_ENTRA_ID_TENANT,
  AUTH_SECRET,
} from "./secrets";
import { sveltekitCookies } from "better-auth/svelte-kit";
import { getRequestEvent } from "$app/server";
import { createAuthMiddleware } from "better-auth/api";

export const auth = betterAuth({
  secret: AUTH_SECRET,
  socialProviders: {
    microsoft: {
      clientId: MICROSOFT_ENTRA_ID_ID,
      clientSecret: MICROSOFT_ENTRA_ID_SECRET,
      tenantId: MICROSOFT_ENTRA_ID_TENANT,
    },
  },
  session: {
    cookieCache: { enabled: true },
  },
  plugins: [sveltekitCookies(getRequestEvent)],
  advanced: { oauthConfig: { storeStateStrategy: "cookie" } },

  hooks: {
    after: createAuthMiddleware(async (ctx) => {
      // TODO
      // 1. create a person in the database
      // 2. create an API key for the user
      // 3. create the database user
      // 4. store the API key in the session
    }),
  },
});
