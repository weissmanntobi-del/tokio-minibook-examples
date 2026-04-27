## Rust Complete Material: https://tobiweissmann.gumroad.com/l/gnuvxu
# Tokio Code Examples

This ZIP contains runnable Rust examples extracted from **Tokio, Explained**.

Each folder is an independent Cargo project. Most examples can be run with:

```bash
cd example-folder-name
cargo run
```

Some examples are small servers and keep running until you press `Ctrl+C`. The testing example also includes `cargo test`.

## Examples

| Folder | PDF source idea | What it demonstrates | Run command |
|---|---|---|---|
| `example-01-tokio-echo-server` | Section 5: "Echo, but right" | Tokio TCP echo server with per-connection tasks, a semaphore concurrency cap, and read timeouts | `cargo run` |
| `example-02-spawn-blocking` | Section 6.1: `tokio::task::spawn_blocking` | Moving CPU-heavy work away from async runtime worker threads | `cargo run` |
| `example-03-graceful-shutdown-signal` | Section 7.1: shutdown signal handler | Waiting for `Ctrl+C` / `SIGTERM` using Tokio signal support | `cargo run` |
| `example-04-select-cancellation` | Section 7.2: `tokio::select!` cancellation | Racing normal work against shutdown/cancellation | `cargo run` |
| `example-05-semaphore-fanout` | Section 7.3: semaphore-capped fan-out | Limiting concurrent spawned jobs with `tokio::sync::Semaphore` | `cargo run` |
| `example-06-axum-production-template` | Sections 8.2-8.4 | Split `axum` + `tower-http` service with tracing, timeout, concurrency limit, and graceful shutdown | `cargo run` |
| `example-07-body-limit-timeout` | Section 8.5 | Request body limits and request timeout middleware | `cargo run` |
| `example-08-app-error` | Section 9 | `AppError` mapped to stable JSON HTTP responses with `IntoResponse` | `cargo run` |
| `example-09-broadcast-shutdown` | Section 10.1 | Broadcasting a shutdown notification to multiple background tasks | `cargo run` |
| `example-10-bounded-mpsc-channel` | Section 11.1 | Bounded `mpsc` channel for backpressure | `cargo run` |
| `example-11-tokio-tests` | Section 12 | `#[tokio::test]`, paused time, and deterministic timeout tests | `cargo run` and `cargo test` |
| `example-12-one-file-axum-starter` | Section 15.2 | Compact one-file production-style starter | `cargo run` |

## Notes about incomplete snippets

The PDF contains several short teaching snippets that are intentionally partial, for example:

- `do_expensive_thing()` in the `spawn_blocking` snippet
- `do_work()` in the `tokio::select!` cancellation snippet
- `items` and `process(item)` in the semaphore fan-out snippet
- `Job`, `job`, and `handle(job)` in the bounded channel snippet
- the `tokio::test` examples without a surrounding test crate
- middleware fragments such as request body limits

Those snippets were converted into minimal working examples while preserving the original learning purpose.

## Dependency choices

- Tokio uses `version = "1"`, so Cargo resolves to the latest compatible stable Tokio 1.x release.
- Examples use minimal Tokio feature flags instead of `features = ["full"]` where practical, matching the mini-book's guidance.
- The `axum` examples use current stable-style dependencies: `axum = "0.8"`, `tower = "0.5"`, and `tower-http = "0.6"`.
- For current `tower-http`, timeout examples use `TimeoutLayer::with_status_code(...)` instead of the older/deprecated `TimeoutLayer::new(...)` form.

## Quick smoke test

From the ZIP root after extracting:

```bash
for dir in example-*; do
  echo "Checking $dir"
  (cd "$dir" && cargo check)
done
```


