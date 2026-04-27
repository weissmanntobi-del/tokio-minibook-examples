use anyhow::Result;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};

/// Demonstrates a small Tokio TCP echo server with production hygiene:
/// - one task per connection
/// - semaphore-based backpressure
/// - read timeout to avoid slow clients holding resources forever
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server listening on 127.0.0.1:8080");
    println!("Try: nc 127.0.0.1 8080");

    // Concurrency guard: prevent unbounded connection handling.
    let limit = Arc::new(Semaphore::new(1024));

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let permit = limit.clone().acquire_owned().await?;

        tokio::spawn(async move {
            let _permit = permit; // The permit is held until this task ends.
            let mut buf = [0_u8; 1024];

            loop {
                // Time-bound each read to tame slow clients.
                let n = match timeout(Duration::from_secs(30), socket.read(&mut buf)).await {
                    Ok(Ok(0)) => {
                        println!("client {addr} disconnected");
                        return;
                    }
                    Ok(Ok(n)) => n,
                    Ok(Err(e)) => {
                        eprintln!("read error from {addr}: {e}");
                        return;
                    }
                    Err(_) => {
                        eprintln!("read timeout from {addr}");
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("write error to {addr}: {e}");
                    return;
                }
            }
        });
    }
}
