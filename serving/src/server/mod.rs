//! gRPC service handler for the ADAPT serving engine.
//!
//! Request pipeline:
//!   1. Feature fetch   — hydrate user + context features from Redis
//!   2. ANN retrieval   — HNSW traversal with predicate pushdown
//!   3. DLRM rank       — ONNX Runtime FP16 inference over candidates
//!   4. Auction          — GSP / VCG resolution with reserve price
//!   5. Pacing           — PID bid-multiplier adjustment
//!   6. Response          — serialized winner + clearing price

use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use crate::features::FeatureStore;
use crate::retrieval::CandidateRetriever;
use crate::ranking::Ranker;
use crate::auction::AuctionEngine;
use crate::pacing::PacingController;

/// Core service struct wired into the tonic gRPC server.
pub struct AdaptService {
    pub feature_store: Arc<FeatureStore>,
    pub retrieval_engine: Arc<CandidateRetriever>,
    pub ranker: Arc<Ranker>,
    pub auction_engine: Arc<AuctionEngine>,
    pub pacer: Arc<Mutex<PacingController>>,
}

/// Placeholder request / response types until proto codegen is wired up.
pub struct DecideRequest {
    pub user_id: String,
    pub context: Vec<u8>,
}

pub struct DecideResponse {
    pub winner_id: String,
    pub clearing_price: f64,
}

// --------------------------------------------------------------------------
// gRPC trait implementation (stub)
// --------------------------------------------------------------------------

#[tonic::async_trait]
pub trait AdaptRpc: Send + Sync + 'static {
    /// Single decision RPC: takes a request context and returns the winning ad.
    ///
    /// Pipeline: feature fetch → ANN retrieval → DLRM rank → auction → pacing → response
    async fn decide(&self, request: DecideRequest) -> Result<DecideResponse, Status>;
}

#[tonic::async_trait]
impl AdaptRpc for AdaptService {
    async fn decide(&self, _request: DecideRequest) -> Result<DecideResponse, Status> {
        // TODO: implement
        //
        // 1. Fetch user features   → self.feature_store.get_user_features(...)
        // 2. ANN candidate retrieval → self.retrieval_engine.retrieve(...)
        // 3. Score candidates        → self.ranker.score_candidates(...)
        // 4. Run auction             → self.auction_engine.resolve(...)
        // 5. Apply pacing multiplier → self.pacer.lock().await.compute_bid_multiplier(...)
        // 6. Build response

        Err(Status::unimplemented("decide RPC not yet implemented"))
    }
}
