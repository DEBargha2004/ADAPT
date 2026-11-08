//! Feature store client — reads user features and rolling aggregates from Redis.

use anyhow::Result;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};

/// Hydrated user-level features (demographics, historical engagement, etc.).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFeatures {
    pub user_id: String,
    pub embedding: Vec<f32>,
    pub segment_ids: Vec<u64>,
    // TODO: expand with demographic / behavioural fields
}

/// Time-windowed rolling aggregates (impressions, clicks, spend, etc.).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RollingAggregates {
    pub impressions_1h: u64,
    pub clicks_1h: u64,
    pub spend_1h: f64,
    pub impressions_24h: u64,
    pub clicks_24h: u64,
    pub spend_24h: f64,
    // TODO: expand with additional windows / metrics
}

/// Thin wrapper around a Redis connection pool for low-latency feature lookups.
pub struct FeatureStore {
    conn: MultiplexedConnection,
}

impl FeatureStore {
    /// Create a new FeatureStore connected to the given Redis URL.
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(Self { conn })
    }

    /// Fetch pre-computed user features from Redis.
    pub async fn get_user_features(&self, _user_id: &str) -> Result<UserFeatures> {
        // TODO: implement — HGET / pipeline lookup keyed by user_id
        Ok(UserFeatures::default())
    }

    /// Fetch rolling aggregate counters from Redis.
    pub async fn get_rolling_aggregates(&self, _user_id: &str) -> Result<RollingAggregates> {
        // TODO: implement — HGET rolling-window counters
        Ok(RollingAggregates::default())
    }
}
