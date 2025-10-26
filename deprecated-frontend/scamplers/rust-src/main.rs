use clap::Parser;
use scamplers::{config::Cli, server::serve_app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap_or_default();
    let Cli { config, log_dir } = Cli::parse();

    serve_app(config, log_dir).await?;

    Ok(())
}
