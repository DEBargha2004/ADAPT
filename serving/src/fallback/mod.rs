//! Circuit breaker and graceful degradation.
//!
//! Falls back to cached GBDT heuristic ranker if ANN or DLRM breaches timeout.

use std::time::Instant;

/// Circuit breaker states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation — requests flow through.
    Closed,
    /// Failures exceeded threshold — all requests short-circuit to fallback.
    Open,
    /// Cooling-off period — a single probe request is allowed through.
    HalfOpen,
}

/// A simple circuit breaker that trips after `threshold` consecutive failures.
#[derive(Debug)]
pub struct CircuitBreaker {
    pub state: CircuitState,
    pub failure_count: u64,
    pub threshold: u64,
    pub last_failure_time: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(threshold: u64) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            threshold,
            last_failure_time: None,
        }
    }

    /// Execute `f`; if the circuit is open (or `f` fails), return `fallback` instead.
    ///
    /// Falls back to cached GBDT heuristic ranker if ANN or DLRM breaches timeout.
    pub fn execute<F, T>(&mut self, f: F, fallback: T) -> T
    where
        F: FnOnce() -> Option<T>,
    {
        // TODO: implement
        //   - If state == Open, return fallback immediately
        //   - If state == HalfOpen, allow one probe; on success → Closed, on fail → Open
        //   - If state == Closed, execute f; on failure bump counter, maybe trip to Open
        match self.state {
            CircuitState::Open => fallback,
            CircuitState::HalfOpen | CircuitState::Closed => match f() {
                Some(result) => {
                    self.failure_count = 0;
                    self.state = CircuitState::Closed;
                    result
                }
                None => {
                    self.failure_count += 1;
                    self.last_failure_time = Some(Instant::now());
                    if self.failure_count >= self.threshold {
                        self.state = CircuitState::Open;
                    }
                    fallback
                }
            },
        }
    }
}
