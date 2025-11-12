use anyhow::Context;
use scamplers_api::{api, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap_or_default();

    api::serve(Config::read().context(
        "failed to read configuration from command-line, environment, and configuration directory",
    )?)
    .await?;

    Ok(())
}
