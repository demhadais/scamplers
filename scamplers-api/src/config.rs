use std::fs;

use anyhow::{Context, anyhow, bail};
use camino::Utf8PathBuf;
use clap::{Args, Parser};

use crate::initial_data::InitialData;

#[derive(Debug, Args, serde::Deserialize, Clone)]
pub struct Config {
    #[arg(long, default_value_t)]
    dev: bool,
    #[arg(long)]
    secrets_dir: Option<Utf8PathBuf>,
    #[arg(long, env = "SCAMPLERS_DB_ROOT_USER", default_value_t = String::from("postgres"))]
    db_root_user: String,
    #[arg(long, env = "SCAMPLERS_DB_ROOT_PASSWORD", default_value_t)]
    db_root_password: String,
    #[arg(long, env = "SCAMPLERS_API_DB_PASSWORD", default_value_t)]
    scamplers_api_db_password: String,
    #[arg(long, env = "SCAMPLERS_UI_DB_PASSWORD", default_value_t)]
    scamplers_ui_db_password: String,
    #[arg(long, env = "SCAMPLERS_DB_HOST", default_value_t = String::from("localhost"))]
    db_host: String,
    #[arg(long, env = "SCAMPLERS_DB_PORT", default_value_t = 5432)]
    db_port: u16,
    #[arg(long, env = "SCAMPLERS_DB_NAME", default_value_t = String::from("postgres"))]
    db_name: String,
    #[arg(long, env = "SCAMPLERS_API_KEY_PREFIX_LENGTH", default_value_t = 8)]
    api_key_prefix_length: usize,
    #[arg(long, env = "SCAMPLERS_API_HOST", default_value_t = String::from("localhost"))]
    host: String,
    #[arg(long, env = "SCAMPLERS_API_PORT", default_value_t = 8000)]
    port: u16,
    #[arg(skip)]
    initial_data: Option<InitialData>,
    #[arg(long, env = "SCAMPLERS_INITIAL_DATA_PATH")]
    initial_data_path: Option<Utf8PathBuf>,
}
impl Config {
    #[must_use]
    pub fn dev(&self) -> bool {
        self.dev
    }

    pub fn read_secrets(&mut self) -> anyhow::Result<()> {
        let Self {
            secrets_dir,
            db_root_user,
            db_root_password,
            scamplers_api_db_password,
            scamplers_ui_db_password,
            db_name,
            initial_data,
            initial_data_path,
            ..
        } = self;

        let Some(secrets_dir) = secrets_dir else {
            return Ok(());
        };

        let read_secret = |name: &str| {
            fs::read_to_string(secrets_dir.join(name))
                .context(format!("failed to read secret {name}"))
        };

        *db_root_user = read_secret("db_root_user")?;
        *db_root_password = read_secret("db_root_password")?;
        *scamplers_api_db_password = read_secret("scamplers_api_db_password")?;
        *scamplers_ui_db_password = read_secret("scamplers_ui_db_password")?;
        *db_name = read_secret("db_name")?;
        *initial_data = serde_json::from_str(&read_secret("seed_data")?)?;
        *initial_data_path = None;

        Ok(())
    }

    #[must_use]
    pub fn app_address(&self) -> String {
        let Self {
            host: app_host,
            port: app_port,
            ..
        } = self;

        format!("{app_host}:{app_port}")
    }

    #[must_use]
    pub fn scamplers_api_db_password(&self) -> &str {
        &self.scamplers_api_db_password
    }

    #[must_use]
    pub fn scamplers_ui_db_password(&self) -> &str {
        &self.scamplers_ui_db_password
    }

    fn db_url(&self, root: bool) -> String {
        let Self {
            db_root_password,
            scamplers_api_db_password,
            db_host,
            db_port,
            db_name,
            ..
        } = self;

        let base = "postgres://";
        let db_spec = format!("{db_host}:{db_port}/{db_name}");

        if root {
            format!("{base}{root}:{db_root_password}@{db_spec}")
        } else {
            format!("{base}scamplers_api:{scamplers_api_db_password}@{db_spec}")
        }
    }

    #[must_use]
    pub fn db_root_url(&self) -> String {
        self.db_url(true)
    }

    #[must_use]
    pub fn db_login_url(&self) -> String {
        self.db_url(false)
    }

    #[must_use]
    pub fn api_key_prefix_length(&self) -> usize {
        self.api_key_prefix_length
    }

    pub fn initial_data(&mut self) -> anyhow::Result<InitialData> {
        let Self {
            initial_data,
            initial_data_path,
            ..
        } = self;

        match (initial_data, initial_data_path) {
            (data, None) => data.take().ok_or(anyhow!(
                "`initial_data` must be supplied if `initial_data_path` is not supplied"
            )),
            (None, Some(seed_data_path)) => {
                Ok(serde_json::from_str(&fs::read_to_string(seed_data_path)?)?)
            }
            (Some(..), Some(..)) => {
                bail!("`initial_data` and `initial_data_path` are mutually exclusive")
            }
        }
    }
}

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(flatten)]
    pub config: Config,
    #[arg(long, env = "SCAMPLERS_LOG_DIR")]
    pub log_dir: Option<Utf8PathBuf>,
}
