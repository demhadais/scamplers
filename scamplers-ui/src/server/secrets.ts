async function read_secret(name: string): Promise<string> {
  if (process.env.IN_DOCKER) {
    return await Bun.file(`/run/secrets/${name}`).text();
  }

  const key = name.toUpperCase();
  const val = process.env[key] || process.env[`SCAMPLERS_${key}`];
  if (val === undefined) {
    throw `secret ${name} not set`;
  }

  return val;
}

export const AUTH_SECRET = await read_secret("auth_secret");
export const MICROSOFT_ENTRA_ID_ID = await read_secret("microsoft_entra_id_id");
export const MICROSOFT_ENTRA_ID_SECRET = await read_secret(
  "microsoft_entra_id_secret",
);
export const MICROSOFT_ENTRA_ID_TENANT = await read_secret(
  "microsoft_entra_id_tenant",
);
