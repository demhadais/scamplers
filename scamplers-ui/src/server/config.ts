async function readEnvVar(name: string): Promise<string | undefined> {
  const key = name.toUpperCase();
  const val = Bun.env[key] || Bun.env[`SCAMPLERS_${key}`];

  return val;
}

async function readRequiredEnvVar(name: string): Promise<string> {
  const val = await readEnvVar(name);

  if (val === undefined) {
    throw `required environment variable ${name} is unset`;
  }

  return val;
}

async function readSecret(name: string): Promise<string> {
  if (Bun.env.inDocker) {
    return await Bun.file(`/run/secrets/${name}`).text();
  }

  return await readRequiredEnvVar(name);
}

const SECRETS = {
  dbHost: await readSecret("db_host"),
  dbPort: await readSecret("db_port"),
  uiDbPassword: await readSecret("ui_db_password"),
  dbName: await readSecret("db_name"),
  microsoft_entra_client_id: await readSecret("microsoft_entra_client_id"),
  microsoft_entra_client_secret: await readSecret(
    "microsoft_entra_client_secret",
  ),
  microsoft_entra_tenant: await readSecret("microsoft_entra_tenant"),
};

const SERVER_CONFIG = {
  apiKeyPrefixLength: parseInt(
    await readRequiredEnvVar("api_key_prefix_length"),
  ),
  publicUrl: await readEnvVar("public_url"),
  apiUrl: await readRequiredEnvVar("api_url"),
};

if (
  SERVER_CONFIG.apiUrl === undefined &&
  SERVER_CONFIG.publicUrl === undefined
) {
  throw `must have at least one of API_URL or PUBLIC_URL set`;
}

// In production, the API URL should just default to the public URL
if (SERVER_CONFIG.apiUrl === undefined) {
  SERVER_CONFIG.apiUrl == `${SERVER_CONFIG.publicUrl}/api`;
}

export { SECRETS, SERVER_CONFIG };
