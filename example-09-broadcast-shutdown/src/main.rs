use anyhow::Result;
use tokio::sync::broadcast;
use tokio::time::{interval, sleep, Duration};

/// Demonstrates using `tokio::sync::broadcast` as a shared shutdown signal.
///
/// The PDF snippet showed the pattern with `background_loop()` as a placeholder.
/// This runnable version starts three background workers and broadcasts shutdown
/// after two seconds.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let (shutdown_tx, _shutdown_rx) = broadcast::channel::<()>(16);
    let mut handles = Vec::new();

    for worker_id in 1..=3 {
        let mut rx = shutdown_tx.subscribe();

        let handle = tokio::spawn(async move {
            background_loop(worker_id, &mut rx).await;
        });

        handles.push(handle);
    }

    sleep(Duration::from_secs(2)).await;
    println!("Broadcasting shutdown...");
    let _ = shutdown_tx.send(());

    for handle in handles {
        handle.await?;
    }

    println!("All workers stopped cleanly.");
    Ok(())
}

async fn background_loop(worker_id: u8, rx: &mut broadcast::Receiver<()>) {
    let mut ticker = interval(Duration::from_millis(400));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                println!("worker {worker_id}: tick");
            }
            _ = rx.recv() => {
                println!("worker {worker_id}: shutdown received");
                break;
            }
        }
    }
}
