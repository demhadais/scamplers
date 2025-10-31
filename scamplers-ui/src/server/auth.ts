import { betterAuth } from "better-auth";
import { sveltekitCookies } from "better-auth/svelte-kit";
import { getRequestEvent } from "$app/server";
import { createAuthMiddleware } from "better-auth/api";
import { SECRETS, SERVER_CONFIG } from "./config";
import { dbClient } from "./db-client";

export const auth = betterAuth({
  baseURL: SERVER_CONFIG.baseUrl,
  secret: SECRETS.auth_secret,
  socialProviders: {
    microsoft: {
      clientId: SECRETS.microsoft_entra_id_id!,
      clientSecret: SECRETS.microsoft_entra_id_secret,
      tenantId: SECRETS.microsoft_entra_id_tenant,
      mapProfileToUser: (profile) => {
        return {
          id: profile.oid,
          microsoftEntraOid: profile.oid,
          institutionId: profile.tid,
        };
      },
    },
  },
  session: {
    cookieCache: { enabled: true },
  },
  plugins: [sveltekitCookies(getRequestEvent)],
  hooks: {
    after: createAuthMiddleware(async (ctx) => {
      const newSession = ctx.context.newSession;
      if (!newSession) {
        return;
      }

      const { id, name, email, emailVerified } = newSession.user;

      // TODO
      // 1. create a person in the database
      // 2. create an API key for the user
      // 3. create the database user
      // 4. store the API key in the session
    }),
  },
});
