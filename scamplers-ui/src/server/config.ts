async function read_config_var(name: string, secret: boolean): Promise<string> {
  if (Bun.env.inDocker && secret) {
    return await Bun.file(`/run/secrets/${name}`).text();
  }

  const key = name.toUpperCase();
  const val = Bun.env[key] || Bun.env[`SCAMPLERS_UI_${key}`];
  if (val === undefined) {
    throw `environment variable ${name} not set`;
  }

  return val;
}

const secret_names = [
  "auth_secret",
  "db_host",
  "db_port",
  "db_password",
  "db_name",
  "microsoft_entra_client_id",
  "microsoft_entra_client_secret",
  "microsoft_entra_tenant",
];
let SECRETS: Record<string, string> = {};
for (const s of secret_names) {
  SECRETS[s] = await read_config_var(s, true);
}

const serverConfigVars = ["api_key_prefix_length", "protocol", "host", "port"];
let SERVER_CONFIG: Record<string, string | number> = {};
for (const v of serverConfigVars) {
  SERVER_CONFIG[v] = await read_config_var(v, false);
}
SERVER_CONFIG.api_key_prefix_length = parseInt(
  SERVER_CONFIG.api_key_prefix_length as string,
);
SERVER_CONFIG.baseUrl = `${SERVER_CONFIG.protocol}://${SERVER_CONFIG.host}:${SERVER_CONFIG.port}`;

export { SECRETS, SERVER_CONFIG };
