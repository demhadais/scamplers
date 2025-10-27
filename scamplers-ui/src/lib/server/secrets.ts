import { env } from "$env/dynamic/private";

async function read_secret(name: string): Promise<string> {
  if (env.IN_DOCKER) {
    // @ts-ignore: Deno exists
    return await Deno.readTextFile(`/run/secrets/${name}`);
  }

  const key = name.toUpperCase();
  const val = env[key] || env[`SCAMPLERS_${key}`];
  if (val === undefined) {
    throw `secret ${name} not set`;
  }

  return val;
}

export const AUTH_SECRET = await read_secret("auth_secret");
export const MICROSOFT_ENTRA_ID_ID = await read_secret(
  "auth_microsoft_entra_id_id",
);
export const MICROSOFT_ENTRA_ID_SECRET = await read_secret(
  "auth_microsoft_entra_id_secret",
);
export const MICROSOFT_ENTRA_ID_ISSUER = await read_secret(
  "auth_microsoft_entra_id_issuer",
);
export const UI_AUTH_TOKEN = await read_secret("ui_auth_token");
