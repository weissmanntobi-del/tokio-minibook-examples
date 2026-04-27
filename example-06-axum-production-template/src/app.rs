use axum::{http::StatusCode, routing::get, Router};
use std::time::Duration;
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

/// Builds the HTTP router and middleware stack in one place.
pub fn build() -> Router {
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(ConcurrencyLimitLayer::new(1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(2),
        ));

    Router::new()
        .route("/health", get(health))
        .layer(middleware)
}

async fn health() -> &'static str {
    "ok"
}

/// Minimal shutdown signal used by `axum::serve(...).with_graceful_shutdown(...)`.
pub async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::warn!("shutting down");
}
