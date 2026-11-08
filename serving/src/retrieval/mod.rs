//! ANN candidate retrieval module.
//!
//! In-process HNSW traversal with predicate pushdown, < 10 ms budget.

use anyhow::Result;

/// Targeting predicate filters applied during HNSW traversal.
#[derive(Debug, Clone, Default)]
pub struct TargetingFilters {
    pub geo_ids: Vec<u64>,
    pub segment_ids: Vec<u64>,
    pub excluded_ad_ids: Vec<u64>,
    // TODO: expand with frequency caps, dayparting, etc.
}

/// A candidate ad returned from the ANN index.
#[derive(Debug, Clone)]
pub struct CandidateAd {
    pub ad_id: u64,
    pub campaign_id: u64,
    pub embedding: Vec<f32>,
    pub bid: f64,
    // TODO: expand with creative metadata
}

/// Wraps an HNSW index for fast approximate nearest-neighbour retrieval.
pub struct CandidateRetriever {
    // TODO: hold the actual HNSW index (e.g. instant_distance::HnswMap)
    _index_path: String,
}

impl CandidateRetriever {
    /// Load a pre-built HNSW index from disk.
    pub fn load_index(path: &str) -> Result<Self> {
        // TODO: implement — deserialize HNSW graph from `path`
        tracing::info!(path, "loading HNSW index");
        Ok(Self {
            _index_path: path.to_owned(),
        })
    }

    /// Retrieve the top-k nearest candidates for the given query embedding,
    /// applying targeting filters via predicate pushdown during traversal.
    ///
    /// Budget: < 10 ms for top-500.
    pub fn retrieve(
        &self,
        _query_embedding: &[f32],
        _top_k: usize,
        _filters: &TargetingFilters,
    ) -> Vec<CandidateAd> {
        // TODO: implement — HNSW search with inline filter predicate
        Vec::new()
    }
}
