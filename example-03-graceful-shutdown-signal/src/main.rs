/// Demonstrates a minimal graceful shutdown signal handler.
///
/// Run with `cargo run`, then press Ctrl+C. On Unix, this also handles SIGTERM.
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Service is running. Press Ctrl+C to trigger shutdown...");
    shutdown_signal().await;
    println!("Shutdown signal received. Clean up resources here.");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};

        let mut term = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        term.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => (),
        _ = terminate => (),
    }
}
