use anyhow::Context;
use axum::{Router, routing::get};
use camino::Utf8PathBuf;
use tokio::net::TcpListener;

use crate::{config::Config, state::AppState};

mod auth;
mod error;
mod extract;
mod routes;
pub use error::{Error, ErrorResponse};

pub async fn serve_integration_test(config: Config) -> anyhow::Result<()> {
    serve_inner(config).await
}

pub async fn serve(config: Config, log_dir: Option<Utf8PathBuf>) -> anyhow::Result<()> {
    initialize_logging(log_dir);
    serve_inner(config).await
}

async fn serve_inner(mut config: Config) -> anyhow::Result<()> {
    config
        .read_secrets()
        .context("failed to read secrets directory")?;

    let app_state = AppState::initialize(&mut config)
        .await
        .context("failed to initialize app state")?;
    tracing::info!("initialized app state");

    let app = app(app_state.clone(), config.api_path());

    let app_addr = config.app_address();
    let listener = TcpListener::bind(&app_addr)
        .await
        .context(format!("failed to listen on {app_addr}"))?;
    tracing::info!("scamplers listening on {app_addr}");

    axum::serve(listener, app)
        .await
        .context("failed to serve app")?;

    Ok(())
}

fn initialize_logging(log_dir: Option<Utf8PathBuf>) {
    use tracing::Level;
    use tracing_subscriber::{filter::Targets, prelude::*};

    let log_layer = tracing_subscriber::fmt::layer();

    match log_dir {
        None => {
            let dev_test_log_filter = Targets::new()
                .with_target("scamplers", Level::DEBUG)
                .with_target("tower_http", Level::TRACE);
            let log_layer = log_layer.pretty().with_filter(dev_test_log_filter);

            tracing_subscriber::registry().with(log_layer).init();
        }
        Some(path) => {
            let log_writer = tracing_appender::rolling::daily(path, "scamplers.log");
            let prod_log_filter = Targets::new().with_target("scamplers", Level::INFO);
            let log_layer = log_layer
                .json()
                .with_writer(log_writer)
                .with_filter(prod_log_filter);

            tracing_subscriber::registry().with(log_layer).init();
        }
    }
}

fn app(app_state: AppState, api_path: &str) -> Router {
    let api_router = routes::router()
        .route("/health", get(async || ()))
        .with_state(app_state);

    Router::new().nest(&api_path, api_router)
}
