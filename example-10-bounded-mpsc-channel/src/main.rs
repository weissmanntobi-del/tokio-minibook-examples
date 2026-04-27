use anyhow::Result;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct Job {
    id: usize,
}

/// Demonstrates bounded `mpsc` channels for backpressure.
///
/// The PDF snippet used placeholders `Job`, `job`, and `handle(job)`. This
/// example sends eight jobs into a channel that can queue only three at a time.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<Job>(3); // At most three jobs queued.

    let worker = tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            handle(job).await;
        }
    });

    for id in 1..=8 {
        println!("producer: sending job {id}");
        tx.send(Job { id }).await?; // Awaits when the bounded queue is full.
        println!("producer: queued job {id}");
    }

    drop(tx); // Close the channel so the worker loop can finish.
    worker.await?;

    println!("All queued jobs processed.");
    Ok(())
}

async fn handle(job: Job) {
    println!("worker: handling job {}", job.id);
    sleep(Duration::from_millis(350)).await;
    println!("worker: finished job {}", job.id);
}
