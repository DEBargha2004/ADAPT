//! Deep ranking inference module.
//!
//! ONNX Runtime FP16 inference, < 20 ms budget for 500 candidates.

use anyhow::Result;

use crate::features::UserFeatures;
use crate::retrieval::CandidateAd;

/// A scored candidate after DLRM inference.
#[derive(Debug, Clone)]
pub struct ScoredCandidate {
    pub candidate_id: u64,
    /// Predicted click-through rate.
    pub p_ctr: f64,
    /// Predicted conversion rate.
    pub p_cvr: f64,
    /// Expected CPM (= bid × p_ctr × calibration).
    pub ecpm: f64,
}

/// Wraps an ONNX Runtime session for deep-learning–based ranking (DLRM).
pub struct Ranker {
    // TODO: hold an ort::Session
    _model_path: String,
}

impl Ranker {
    /// Load an ONNX model from disk and create an inference session.
    pub fn load_model(onnx_path: &str) -> Result<Self> {
        // TODO: implement — ort::Session::builder()...commit_from_file(onnx_path)?
        tracing::info!(onnx_path, "loading ONNX ranking model");
        Ok(Self {
            _model_path: onnx_path.to_owned(),
        })
    }

    /// Score a batch of candidate ads given user features.
    ///
    /// Returns candidates sorted descending by eCPM.
    /// Budget: < 20 ms for 500 candidates (FP16 inference).
    pub fn score_candidates(
        &self,
        _candidates: &[CandidateAd],
        _user_features: &UserFeatures,
    ) -> Result<Vec<ScoredCandidate>> {
        // TODO: implement
        //   1. Build input tensor from candidate embeddings + user features
        //   2. Run ONNX session
        //   3. Parse output tensor into ScoredCandidate vec
        Ok(Vec::new())
    }
}
