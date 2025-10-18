use anyhow::Context;
use axum::{Router, routing::get};
use camino::Utf8PathBuf;
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;

use crate::{config::Config, state::AppState};
mod api;

/// # Errors
async fn serve(mut config: Config) -> anyhow::Result<()> {
    config
        .read_secrets()
        .context("failed to read secrets directory")?;
    let app_addr = config.app_address();

    let app_state = AppState::initialize(config)
        .await
        .context("failed to initialize app state")?;
    tracing::info!("initialized app state");

    let app = app(app_state.clone());

    let listener = TcpListener::bind(&app_addr)
        .await
        .context(format!("failed to listen on {app_addr}"))?;
    tracing::info!("scamplers listening on {app_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(app_state))
        .await
        .context("failed to serve app")?;

    Ok(())
}

/// # Errors
pub async fn serve_integration_test(config: Config) -> anyhow::Result<()> {
    serve(config).await
}

/// # Errors
pub async fn serve_app(config: Config, log_dir: Option<Utf8PathBuf>) -> anyhow::Result<()> {
    initialize_logging(log_dir);
    serve(config).await
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

fn app(app_state: AppState) -> Router {
    let path = app_state.api_path().to_string();
    let api_router = api::router()
        .layer(TraceLayer::new_for_http())
        .route("/health", get(async || ()))
        .with_state(app_state);

    Router::new()
        .nest(&path, api_router)
        .layer(axum::extract::DefaultBodyLimit::disable())
}

async fn shutdown_signal(app_state: AppState) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        () = ctrl_c => {drop(app_state)},
        () = terminate => {drop(app_state)},
    }
}
