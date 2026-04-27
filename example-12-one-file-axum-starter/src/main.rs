use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Compact one-file starter from the PDF's final checklist section.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(2),
        ));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .layer(middleware);

    let addr: SocketAddr = "127.0.0.1:3003".parse()?;
    tracing::info!(%addr, "listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::warn!("shutting down");
}
