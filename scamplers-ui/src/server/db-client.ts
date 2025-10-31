import { SECRETS } from "./config";

export const dbClient = new Bun.SQL({
  username: "scamplers_ui",
  password: SECRETS.db_password,
  hostname: SECRETS.db_host,
  port: SECRETS.db_port,
  database: SECRETS.db_name,
});
