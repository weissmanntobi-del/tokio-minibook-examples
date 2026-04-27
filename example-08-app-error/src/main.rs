use anyhow::Result;
use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;

use error::AppError;

/// Demonstrates readable application errors mapped to stable HTTP responses.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/items/{id}", get(get_item))
        .route("/boom", get(boom));

    let addr: SocketAddr = "127.0.0.1:3002".parse()?;
    tracing::info!(%addr, "listening");
    tracing::info!("try: curl http://127.0.0.1:3002/items/1");
    tracing::info!("try: curl http://127.0.0.1:3002/items/99");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[derive(Serialize)]
struct Item {
    id: u64,
    name: &'static str,
}

async fn get_item(Path(id): Path<u64>) -> Result<Json<Item>, AppError> {
    match id {
        0 => Err(AppError::BadRequest("id must be greater than zero".to_string())),
        1 => Ok(Json(Item { id, name: "Tokio" })),
        _ => Err(AppError::NotFound),
    }
}

async fn boom() -> Result<Json<Item>, AppError> {
    Err(AppError::Internal)
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::warn!("shutting down");
}
