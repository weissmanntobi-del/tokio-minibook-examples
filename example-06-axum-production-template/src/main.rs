use anyhow::Result;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;

/// Demonstrates the split production-template style from the PDF:
/// - logging setup in main
/// - router/middleware in app.rs
/// - graceful shutdown on Ctrl+C
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let app = app::build();
    let addr: SocketAddr = "127.0.0.1:3000".parse()?;

    tracing::info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(app::shutdown_signal())
        .await?;

    Ok(())
}
