//! Redpanda / Kafka event stream consumers for real-time feature updates.
//!
//! Ingests impression, click, and conversion events, updating rolling aggregate
//! windows in Redis atomically to prevent training-serving skew.

// TODO: Implement EventConsumer
// - Connect to Redpanda topics (adapt.impressions, adapt.clicks, adapt.conversions)
// - Maintain consumer group offsets and handle message rebalancing
// - Route incoming events to rolling aggregators

