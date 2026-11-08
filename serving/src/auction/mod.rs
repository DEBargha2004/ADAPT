//! Auction resolution module — GSP and VCG auction implementations.

use crate::ranking::ScoredCandidate;

/// Supported auction mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuctionType {
    /// Generalised Second-Price auction.
    GSP,
    /// Vickrey–Clarke–Groves auction.
    VCG,
}

/// Result of an auction round.
#[derive(Debug, Clone)]
pub struct AuctionResult {
    /// Ad ID of the winning candidate.
    pub winner_id: u64,
    /// Price the winner actually pays (second-price or VCG price).
    pub clearing_price: f64,
    /// The second-highest eCPM (useful for logging / analytics).
    pub second_price: f64,
}

/// Stateless auction engine.
pub struct AuctionEngine;

impl AuctionEngine {
    pub fn new() -> Self {
        Self
    }

    /// Resolve the auction given a mutable slice of scored candidates,
    /// the auction mechanism, and a reserve (floor) price.
    ///
    /// Candidates are sorted in-place by eCPM descending. The winner is
    /// the top candidate whose eCPM exceeds the reserve price.
    pub fn resolve(
        &self,
        _scored: &mut [ScoredCandidate],
        _auction_type: AuctionType,
        _reserve_price: f64,
    ) -> AuctionResult {
        // TODO: implement
        //   GSP  → winner pays second-price + ε
        //   VCG  → winner pays opportunity cost of displacing others
        AuctionResult {
            winner_id: 0,
            clearing_price: 0.0,
            second_price: 0.0,
        }
    }
}

impl Default for AuctionEngine {
    fn default() -> Self {
        Self::new()
    }
}
