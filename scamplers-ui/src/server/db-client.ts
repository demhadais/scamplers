import { SECRETS } from "./config";

// Bun will pre-connect using environment variables so there is no need to pass in connection options explicitly
export const dbClient = new Bun.SQL({
  host: SECRETS.db_host,
  username: "scamplers_ui",
  password: SECRETS.db_password,
  database: SECRETS.db_name,
});
