import { type DefaultSession, SvelteKitAuth } from "@auth/sveltekit";
import type { PersonCreation } from "scamplers-models/person_creation";
import {
  AUTH_SECRET,
  MICROSOFT_ENTRA_ID_ID,
  MICROSOFT_ENTRA_ID_ISSUER,
  MICROSOFT_ENTRA_ID_SECRET,
} from "$lib/server/secrets";
import { serverApiClient } from "$lib/server/client";
import { type JWT } from "@auth/core/jwt";
import MicrosoftEntraID from "@auth/sveltekit/providers/microsoft-entra-id";
import type { CreatedUser } from "scamplers-models/created_user";

declare module "@auth/sveltekit" {
  interface Session {
    person: CreatedUser;
  }
}
declare module "@auth/core/jwt" {
  interface JWT {
    createdUser: CreatedUser;
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

      const createdUser = await serverApiClient.microsoftSignin(
        profile as PersonCreation,
      );
      token.createdUser = createdUser;

      return token;
    },

    session({ session, token }) {
      session.person = token.createdUser;

      return session;
    },
  },
  trustHost: true,
  events: {
    signOut({ token }) {
      serverApiClient.signOut(token.createdUser.api_key);
    },
  },
});
