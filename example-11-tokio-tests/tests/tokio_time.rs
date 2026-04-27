use tokio::time::{sleep, timeout, Duration};

#[tokio::test(flavor = "multi_thread")]
async fn health_is_ok() {
    assert_eq!(health().await, "ok");
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::test]
async fn timeout_triggers() {
    tokio::time::pause();

    let task = tokio::spawn(async {
        timeout(Duration::from_secs(5), async {
            sleep(Duration::from_secs(10)).await;
            "finished"
        })
        .await
    });

    // Let the spawned task register its timer before advancing time.
    tokio::task::yield_now().await;

    // Time advances instantly in the paused clock.
    tokio::time::advance(Duration::from_secs(5)).await;

    assert!(task.await.unwrap().is_err());
}
