//! ADAPT Streaming Feature Consumer Entry Point.
//!
//! Listens to impression, click, and conversion event topics from Redpanda/Kafka
//! and maintains live aggregate feature tables in Redis.

mod aggregators;
mod consumers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("ADAPT Streaming Feature Consumer starting up...");

    // TODO: Initialize Redis connection manager
    // TODO: Initialize Redpanda event consumer
    // TODO: Spawn rolling aggregate worker tasks
    // TODO: Listen for OS shutdown signals (SIGINT, SIGTERM)

    Ok(())
}

