use anyhow::Result;
use tokio::task;

/// Demonstrates `tokio::task::spawn_blocking`.
///
/// The PDF snippet used a placeholder `do_expensive_thing()` function.
/// Here it is implemented as a deliberately CPU-heavy prime summation so the
/// example is complete while preserving the original lesson: do not run CPU-heavy
/// work directly on Tokio's async worker threads.
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let max = 50_000_u64;

    println!("Starting CPU-heavy work on Tokio's blocking thread pool...");

    let result = task::spawn_blocking(move || do_expensive_thing(max)).await?;

    println!("Sum of primes up to {max}: {result}");
    Ok(())
}

fn do_expensive_thing(max: u64) -> u64 {
    (2..=max).filter(|&n| is_prime(n)).sum()
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let limit = (n as f64).sqrt() as u64;
    for divisor in 2..=limit {
        if n % divisor == 0 {
            return false;
        }
    }
    true
}
