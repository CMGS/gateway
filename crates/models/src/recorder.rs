//! Timing recorder and security-block result.
//!
//! `Recorder` tracks per-request latency bookkeeping; `Block` represents a
//! content-safety verdict as a trimmed, object-safe value type.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Per-request latency recorder (subset of the full timing surface).
///
/// The full timing surface has around a dozen methods; only the ones the
/// engine layer and post-processing actually consume are implemented here.
/// Object-safe so it can be held as `Arc<dyn Recorder>` on a `GatewayResponse`.
pub trait Recorder: Send + Sync + std::fmt::Debug {
    fn start_time(&self) -> DateTime<Utc>;
    fn report_to_map(&self) -> HashMap<String, serde_json::Value>;
}

/// A recorder that measures from construction and reports nothing extra.
/// Handy default when full timing granularity isn't needed.
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

/// Full timing recorder: start time, first-message cost, last-message cost,
/// cost-since-get-req, and a report map. Interior mutability so engines can
/// stamp times through `&self`.
#[derive(Debug, Default)]
pub struct TimingRecorder {
    start: std::sync::OnceLock<DateTime<Utc>>,
    first_msg_ms: std::sync::atomic::AtomicI64,
    last_msg_ms: std::sync::atomic::AtomicI64,
    since_get_req_ms: std::sync::atomic::AtomicI64,
}

impl TimingRecorder {
    pub fn started_now() -> Self {
        let r = Self::default();
        let _ = r.start.set(Utc::now());
        r
    }

    pub fn set_first_msg_cost_ms(&self, ms: i64) {
        self.first_msg_ms
            .store(ms, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_last_msg_cost_ms(&self, ms: i64) {
        self.last_msg_ms
            .store(ms, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_cost_since_get_req_ms(&self, ms: i64) {
        self.since_get_req_ms
            .store(ms, std::sync::atomic::Ordering::Relaxed);
    }

    /// Returns (time to first chunk, time to last chunk, time since request received).
    pub fn result_ms(&self) -> (i64, i64, i64) {
        use std::sync::atomic::Ordering::Relaxed;
        (
            self.first_msg_ms.load(Relaxed),
            self.last_msg_ms.load(Relaxed),
            self.since_get_req_ms.load(Relaxed),
        )
    }
}

impl Recorder for TimingRecorder {
    fn start_time(&self) -> DateTime<Utc> {
        *self.start.get_or_init(Utc::now)
    }

    fn report_to_map(&self) -> HashMap<String, serde_json::Value> {
        let (first, last, since) = self.result_ms();
        HashMap::from([
            ("first_msg_cost_ms".to_owned(), first.into()),
            ("last_msg_cost_ms".to_owned(), last.into()),
            ("cost_since_get_req_ms".to_owned(), since.into()),
        ])
    }
}

/// Content-safety verdict.
///
/// A plain value type since the fields fully describe the verdict.
/// Invariant: a blocked verdict is always a hit; a hit is not necessarily a
/// block.
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
