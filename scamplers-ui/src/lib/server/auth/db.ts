import type { MicrosoftEntraIDProfile } from "better-auth/social-providers";
import { EncryptedApiKey } from "./api-key";
import { decryptApiKey } from "./crypto";

export async function insertPerson(
  {
    name,
    email,
    emailVerified,
    tid,
    oid,
  }: {
    emailVerified: boolean;
  } & MicrosoftEntraIDProfile,
  dbClient: Bun.SQL,
): Promise<string> {
  const newPerson = {
    name,
    email,
    email_verified: emailVerified,
    institution_id: tid,
    microsoft_entra_oid: oid,
  };

  const newPersonId = await dbClient.begin(async (tx) => {
    // Anyone else with this email should have it removed
    await tx`update people set email = ${null}, email_verified = ${false} where email = ${newPerson.email}`;

    const result = await tx`insert into people ${
      tx(
        newPerson,
      )
    } on conflict (microsoft_entra_oid) do update set ${
      tx(
        newPerson,
      )
    } returning id`;
    const newPersonId = result[0].id;

    // Create a db user corresponding to this person so we can assign them roles later on. Note that we set a random
    // password and no roles so that nobody can log into the database as that user.
    await tx`select create_user_if_not_exists(${newPersonId}, ${
      crypto
        .getRandomValues(new Uint8Array(32))
        .toHex()
    }, '{}')`;

    return newPersonId;
  });

  return newPersonId;
}

export async function insertApiKey(
  encryptedApiKey: EncryptedApiKey,
  personId: string,
  dbClient: Bun.SQL,
) {
  const apiKeyData = {
    prefix: encryptedApiKey.prefix,
    hash: encryptedApiKey.hash,
    user_id: personId,
  };

  await dbClient.begin(async (tx) => {
    await tx`insert into api_keys ${tx(apiKeyData)}`;
  });
}

export async function deleteApiKey(
  {
    encryptedApiKey,
    encryptionSecret,
    initializationVector,
    apiKeyPrefixLength,
  }: {
    encryptedApiKey: string;
    encryptionSecret: CryptoKey;
    initializationVector: string;
    apiKeyPrefixLength: number;
  },
  dbClient: Bun.SQL,
) {
  const decrypted = await decryptApiKey(
    initializationVector,
    encryptionSecret,
    encryptedApiKey,
  );

  const prefix = new Uint8Array(decrypted.slice(0, apiKeyPrefixLength));

  // No two users share the same prefix
  await dbClient`delete from api_keys where prefix = ${prefix}`;
}
