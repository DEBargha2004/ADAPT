//! Prometheus metrics registration for ADAPT pipeline stages.
//!
//! Uses the `metrics` crate (with `metrics-exporter-prometheus` backend)
//! to record latency histograms for each stage of the serving pipeline.

use metrics::describe_histogram;

/// Register all pipeline-stage latency histograms with the metrics registry.
///
/// Call once during server startup, after installing the Prometheus exporter.
pub fn register_metrics() {
    describe_histogram!(
        "adapt.feature_fetch_duration_seconds",
        "Latency of user feature hydration from Redis"
    );

    describe_histogram!(
        "adapt.ann_retrieval_duration_seconds",
        "Latency of ANN / HNSW candidate retrieval"
    );

    describe_histogram!(
        "adapt.ranking_duration_seconds",
        "Latency of DLRM ONNX ranking inference"
    );

    describe_histogram!(
        "adapt.auction_duration_seconds",
        "Latency of auction resolution (GSP/VCG)"
    );

    describe_histogram!(
        "adapt.total_request_duration_seconds",
        "End-to-end latency of a single decide RPC"
    );
}

// ---------------------------------------------------------------------------
// Convenience helpers for recording durations at each stage
// ---------------------------------------------------------------------------

/// Record a feature-fetch duration sample.
pub fn record_feature_fetch(duration_secs: f64) {
    metrics::histogram!("adapt.feature_fetch_duration_seconds").record(duration_secs);
}

/// Record an ANN retrieval duration sample.
pub fn record_ann_retrieval(duration_secs: f64) {
    metrics::histogram!("adapt.ann_retrieval_duration_seconds").record(duration_secs);
}

/// Record a ranking inference duration sample.
pub fn record_ranking(duration_secs: f64) {
    metrics::histogram!("adapt.ranking_duration_seconds").record(duration_secs);
}

/// Record an auction resolution duration sample.
pub fn record_auction(duration_secs: f64) {
    metrics::histogram!("adapt.auction_duration_seconds").record(duration_secs);
}

/// Record a total request duration sample.
pub fn record_total_request(duration_secs: f64) {
    metrics::histogram!("adapt.total_request_duration_seconds").record(duration_secs);
}
