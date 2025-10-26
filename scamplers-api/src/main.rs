use clap::Parser;
use scamplers_api::{api, config::Cli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap_or_default();
    let Cli { config, log_dir } = Cli::parse();

    api::serve(config, log_dir).await?;

    Ok(())
}
