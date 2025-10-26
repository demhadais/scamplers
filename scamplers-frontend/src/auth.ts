import { type DefaultSession, SvelteKitAuth } from "@auth/sveltekit";
import type { PersonCreation } from "scamplers-models/person_creation.d.ts";
import {
  AUTH_SECRET,
  MICROSOFT_ENTRA_ID_ID,
  MICROSOFT_ENTRA_ID_ISSUER,
  MICROSOFT_ENTRA_ID_SECRET,
} from "$lib/server/secrets.ts";
import { serverApiClient } from "$lib/server/client.ts";
import { type JWT } from "@auth/core/jwt";
import MicrosoftEntraID from "@auth/sveltekit/providers/microsoft-entra-id";
import type { CreatedUser } from "scamplers-models/created_user.d.ts";

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
        console.log("are we doing something here");
        return token;
      }

      profile = profile!;
      profile.ms_user_id = profile.oid;
      profile.institution_id = profile.tid;

      const createdUser = await serverApiClient.login_ms_user(
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
});
