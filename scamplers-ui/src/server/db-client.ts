import { SECRETS } from "./config";

export const dbClient = new Bun.SQL({
  username: "scamplers_ui",
  password: SECRETS.uiDbPassword,
  hostname: SECRETS.dbHost,
  port: SECRETS.dbPort,
  database: SECRETS.dbName,
});
