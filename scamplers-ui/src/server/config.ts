async function read_config_var(name: string, secret: boolean): Promise<string> {
  if (Bun.env.inDocker && secret) {
    return await Bun.file(`/run/secrets/${name}`).text();
  }

  const key = name.toUpperCase();
  const val = Bun.env[key] || Bun.env[`SCAMPLERS_UI_${key}`];
  if (val === undefined) {
    throw `secret ${name} not set`;
  }

  return val;
}

const secret_names = [
  "auth_secret",
  "microsoft_entra_id_id",
  "microsoft_entra_id_secret",
  "microsoft_entra_id_tenant",
  "db_host",
  "db_port",
  "db_password",
];
let SECRETS: Record<string, string> = {};
for (const s of secret_names) {
  SECRETS[s] = await read_config_var(s, true);
}

const serverConfigVars = ["protocol", "host", "port"];
let SERVER_CONFIG: Record<string, string> = {};
for (const v of serverConfigVars) {
  SERVER_CONFIG[v] = await read_config_var(v, false);
}
SERVER_CONFIG.baseUrl = `${SERVER_CONFIG.protocol}://${SERVER_CONFIG.host}:${SERVER_CONFIG.port}`;

export { SECRETS, SERVER_CONFIG };
