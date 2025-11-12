use std::{fs, str::FromStr};

use anyhow::{Context, anyhow};
use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;

use crate::initial_data::InitialData;

#[derive(Clone, Copy, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Development,
    #[default]
    Production,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => "development".fmt(f),
            Self::Production => "production".fmt(f),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0} is an invalid mode")]
pub struct ModeError(String);

impl std::str::FromStr for Mode {
    type Err = ModeError;

    fn from_str(s: &str) -> Result<Self, ModeError> {
        match s {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            s => Err(ModeError(s.to_string())),
        }
    }
}

#[derive(Debug, Parser, serde::Serialize)]
struct Cli {
    #[arg(long, env = "SCAMPLERS_CONFIG_DIRS")]
    config_dirs: Vec<Utf8PathBuf>,
    #[arg(long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<Mode>,
    #[arg(long, env = "SCAMPLERS_DB_ROOT_USER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    db_root_user: Option<String>,
    #[arg(long, env = "SCAMPLERS_DB_ROOT_PASSWORD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    db_root_password: Option<String>,
    #[arg(long, env = "SCAMPLERS_API_DB_PASSWORD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scamplers_api_db_password: Option<String>,
    #[arg(long, env = "SCAMPLERS_UI_DB_PASSWORD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    scamplers_ui_db_password: Option<String>,
    #[arg(long, env = "SCAMPLERS_DB_HOST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    db_host: Option<String>,
    #[arg(long, env = "SCAMPLERS_DB_PORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    db_port: Option<u16>,
    #[arg(long, env = "SCAMPLERS_DB_NAME")]
    #[serde(skip_serializing_if = "Option::is_none")]
    db_name: Option<String>,
    #[arg(long, env = "SCAMPLERS_API_KEY_PREFIX_LENGTH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_prefix_length: Option<usize>,
    #[arg(long, env = "SCAMPLERS_API_HOST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<String>,
    #[arg(long, env = "SCAMPLERS_API_PORT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[arg(long, env = "SCAMPLERS_INITIAL_DATA_PATH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    log_dir: Option<Utf8PathBuf>,
}

fn read_config_file<T>(config_dir: &Utf8Path, filename: &str) -> anyhow::Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Send + std::error::Error + Sync + 'static,
{
    let file_path = config_dir.join(filename);
    if !file_path.exists() {
        return Ok(None);
    }

    let contents = fs::read_to_string(&file_path).context(format!(
        "failed to read configuration {filename} at {file_path}"
    ))?;

    Ok(Some(contents.parse()?))
}

fn read_config_file_group<T>(
    config_dirs: &[Utf8PathBuf],
    files: &mut [(&mut Option<T>, &str)],
) -> anyhow::Result<()>
where
    T: FromStr,
    <T as FromStr>::Err: Send + std::error::Error + Sync + 'static,
{
    for (value, filename) in files {
        for dir in &*config_dirs {
            if value.is_some() {
                break;
            }
            let Some(file_contents) = read_config_file(dir, filename)? else {
                continue;
            };

            value.get_or_insert(file_contents);
        }
    }

    Ok(())
}

impl Cli {
    pub fn read_config_dirs(&mut self) -> anyhow::Result<()> {
        let Self {
            config_dirs,
            mode,
            db_root_user,
            db_root_password,
            scamplers_api_db_password,
            scamplers_ui_db_password,
            db_host,
            db_port,
            db_name,
            api_key_prefix_length,
            host,
            port,
            log_dir,
        } = self;

        if config_dirs.is_empty() {
            return Ok(());
        }

        read_config_file_group(config_dirs, &mut [(mode, "mode")])?;

        // Each variable type needs to have its own array
        let mut config_files = [
            (db_root_user, "db_root_user"),
            (db_root_password, "db_root_password"),
            (scamplers_api_db_password, "scamplers_api_db_password"),
            (scamplers_ui_db_password, "scamplers_ui_db_password"),
            (db_host, "db_host"),
            (db_name, "db_name"),
            (host, "host"),
        ];
        read_config_file_group(config_dirs, &mut config_files)?;

        let mut config_files = [(db_port, "db_port"), (port, "port")];
        read_config_file_group(config_dirs, &mut config_files)?;

        read_config_file_group(config_dirs, &mut [(log_dir, "log_dir")])?;

        read_config_file_group(
            config_dirs,
            &mut [(api_key_prefix_length, "api_key_prefix_length")],
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    mode: Mode,
    db_root_user: String,
    db_root_password: String,
    scamplers_api_db_password: String,
    scamplers_ui_db_password: String,
    db_host: String,
    db_port: u16,
    db_name: String,
    api_key_prefix_length: usize,
    host: String,
    #[serde(default = "Config::default_port")]
    port: u16,
    initial_data: Option<InitialData>,
    log_dir: Option<Utf8PathBuf>,
}

enum DatabaseUser {
    Root,
    ScamplersApi,
}

impl Config {
    pub fn read() -> anyhow::Result<Self> {
        let mut cli = Cli::parse();
        cli.read_config_dirs().context(format!(
            "failed to read configuration directories {:?}",
            cli.config_dirs
        ))?;

        let mut initial_data = None;

        for config_dir in &cli.config_dirs {
            if initial_data.is_some() {
                break;
            }

            let initial_data_path = config_dir.join("initial_data");
            let err = || format!("failed to read initial data at {initial_data_path}");
            let contents = fs::read(&initial_data_path).with_context(err)?;
            initial_data = serde_json::from_slice(&contents).with_context(err)?;
        }

        let mut config: Self = serde_json::to_value(&cli).map(serde_json::from_value)??;
        config.initial_data = initial_data;

        Ok(config)
    }

    #[must_use]
    pub fn log_dir(&self) -> Option<&Utf8Path> {
        self.log_dir.as_ref().map(Utf8PathBuf::as_path)
    }

    #[must_use]
    pub fn mode(&self) -> Mode {
        self.mode
    }

    fn default_port() -> u16 {
        8000
    }

    #[must_use]
    pub fn address(&self) -> String {
        let Self { host, port, .. } = self;

        format!("{host}:{port}")
    }

    #[must_use]
    pub fn scamplers_api_db_password(&self) -> &str {
        &self.scamplers_api_db_password
    }

    #[must_use]
    pub fn scamplers_ui_db_password(&self) -> &str {
        &self.scamplers_ui_db_password
    }

    fn db_url(&self, database_user: DatabaseUser) -> String {
        let Self {
            db_root_user,
            db_root_password,
            scamplers_api_db_password,
            db_host,
            db_port,
            db_name,
            ..
        } = self;

        let base = "postgres://";
        let db_spec = format!("{db_host}:{db_port}/{db_name}");

        match database_user {
            DatabaseUser::Root => {
                format!("{base}{db_root_user}:{db_root_password}@{db_spec}")
            }
            DatabaseUser::ScamplersApi => {
                format!("{base}scamplers_api:{scamplers_api_db_password}@{db_spec}")
            }
        }
    }

    #[must_use]
    pub fn db_root_url(&self) -> String {
        self.db_url(DatabaseUser::Root)
    }

    #[must_use]
    pub fn scamplers_api_db_url(&self) -> String {
        self.db_url(DatabaseUser::ScamplersApi)
    }

    #[must_use]
    pub fn api_key_prefix_length(&self) -> usize {
        self.api_key_prefix_length
    }

    pub fn initial_data(&mut self) -> anyhow::Result<InitialData> {
        let Self { initial_data, .. } = self;

        initial_data
            .take()
            .ok_or(anyhow!("initial data must be supplied"))
    }
}
