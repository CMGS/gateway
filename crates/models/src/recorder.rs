//! Timing recorder (`Recorder`) and the content-safety verdict (`Block`).

use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Per-request latency recorder — the subset of the full timing surface the
/// engine layer and post-processing consume. Object-safe so it can be held as
/// `Arc<dyn Recorder>` on a `GatewayResponse`.
pub trait Recorder: Send + Sync + std::fmt::Debug {
    fn start_time(&self) -> DateTime<Utc>;
    fn report_to_map(&self) -> HashMap<String, serde_json::Value>;
}

/// Measures from construction and reports nothing extra — the handy default.
#[derive(Debug, Clone)]
pub struct SimpleRecorder {
    start: DateTime<Utc>,
}

impl SimpleRecorder {
    pub fn new(start: DateTime<Utc>) -> Self {
        Self { start }
    }
}

impl Recorder for SimpleRecorder {
    fn start_time(&self) -> DateTime<Utc> {
        self.start
    }
    fn report_to_map(&self) -> HashMap<String, serde_json::Value> {
        HashMap::new()
    }
}

/// Content-safety verdict. Invariant: a blocked verdict is always a hit; a hit
/// is not necessarily a block.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Block {
    pub block: bool,
    pub hit: bool,
    pub message: String,
    pub err_code: i32,
}

impl Block {
    /// A clean (not hit, not blocked) verdict.
    pub fn allow() -> Self {
        Self::default()
    }

    /// A blocking verdict (implies hit, per the invariant above).
    pub fn blocked(message: impl Into<String>, err_code: i32) -> Self {
        Self {
            block: true,
            hit: true,
            message: message.into(),
            err_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_implies_hit() {
        let b = Block::blocked("nope", 4003);
        assert!(b.block && b.hit);
        assert!(!Block::allow().hit);
    }
}
