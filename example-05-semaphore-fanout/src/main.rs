use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

/// Demonstrates bounded fan-out with a Tokio semaphore.
///
/// The PDF snippet used placeholders `items` and `process(item)`. This example
/// creates 20 items but allows only five to run concurrently.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let items: Vec<u32> = (1..=20).collect();
    let sem = Arc::new(Semaphore::new(5)); // At most five concurrent jobs.
    let mut handles = Vec::new();

    for item in items {
        let permit = sem.clone().acquire_owned().await?;

        let handle = tokio::spawn(async move {
            let _permit = permit; // Released automatically when this task ends.
            process(item).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    println!("All jobs completed with bounded concurrency.");
    Ok(())
}

async fn process(item: u32) {
    println!("start job {item}");
    sleep(Duration::from_millis(250)).await;
    println!("finish job {item}");
}
