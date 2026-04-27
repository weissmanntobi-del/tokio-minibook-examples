use tokio::time::{sleep, Duration};

/// Demonstrates cancellation propagation with `tokio::select!`.
///
/// The PDF snippet used placeholder functions `do_work()` and `shutdown_signal()`.
/// This runnable version performs five small async steps unless Ctrl+C wins first.
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    println!("Work started. Press Ctrl+C to cancel early.");

    tokio::select! {
        _ = do_work() => {
            println!("Work finished normally.");
        }
        _ = shutdown_signal() => {
            println!("Shutdown won the race. In-flight work was cancelled.");
        }
    }
}

async fn do_work() {
    for step in 1..=5 {
        println!("working step {step}/5");
        sleep(Duration::from_millis(500)).await;
    }
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
