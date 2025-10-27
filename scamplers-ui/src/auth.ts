import { type DefaultSession, SvelteKitAuth } from "@auth/sveltekit";
import type { PersonCreation } from "$lib/scamplers-models/person_creation";
import {
  AUTH_SECRET,
  MICROSOFT_ENTRA_ID_ID,
  MICROSOFT_ENTRA_ID_ISSUER,
  MICROSOFT_ENTRA_ID_SECRET,
} from "$lib/server/secrets";
import { apiClient, isError } from "$lib/server/client";
import { type JWT } from "@auth/core/jwt";
import MicrosoftEntraID from "@auth/sveltekit/providers/microsoft-entra-id";
import type { AdapterSession } from "@auth/core/adapters";

declare module "@auth/sveltekit" {
  interface Session {
    user: {
      apiKey: string;
      roles: ("app_admin" | "biology_staff" | "computational_staff")[];
    } & DefaultSession["user"];
  }
}
declare module "@auth/core/jwt" {
  interface JWT {
    userApiKey: string;
    userRoles: ("app_admin" | "biology_staff" | "computational_staff")[];
  }
}

export const { handle, signIn, signOut } = SvelteKitAuth({
  secret: AUTH_SECRET,
  providers: [
    MicrosoftEntraID({
      clientId: MICROSOFT_ENTRA_ID_ID,
      clientSecret: MICROSOFT_ENTRA_ID_SECRET,
      issuer: MICROSOFT_ENTRA_ID_ISSUER,
    }),
  ],
  callbacks: {
    signIn({ profile }): boolean {
      if (profile && profile.tid && profile.oid) {
        return true;
      }
      return false;
    },

    async jwt({ token, profile }): Promise<JWT | null> {
      // Do not remove this line or else authentication breaks
      if (!profile) {
        return token;
      }

      profile = profile!;
      profile.ms_user_id = profile.oid;
      profile.institution_id = profile.tid;

      const result = await apiClient.microsoftSignIn(profile as PersonCreation);

      if (isError(result)) {
        return null;
      }

      token.userApiKey = result.api_key;
      token.userRoles = result.roles;

      return token;
    },

    session({ session, token }) {
      session.user.apiKey = token.userApiKey;
      session.user.roles = token.userRoles;

      return session;
    },
  },
  trustHost: true,
  events: {
    async signOut(
      message:
        | { session: AdapterSession | null | void | undefined }
        | { token: JWT | null },
    ) {
      if ("token" in message && message.token) {
        await apiClient.deleteApiKey(message.token.userApiKey);
      }
    },
  },
});
