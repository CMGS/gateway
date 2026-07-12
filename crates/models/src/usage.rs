//! Unified usage view.
//!
//! `CommonUsage` is the normalized cross-vendor usage view. It is populated
//! from each engine's raw usage payload by a post-processing step; when
//! extraction misses or fails it stays `None` and callers fall back to the
//! top-level token fields on `GatewayResponse`.

/// Normalized token accounting across vendors.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CommonUsage {
    /// input tokens excluding cache.
    pub platform_input: i64,
    /// cache-read tokens.
    pub read_cache: i64,
    /// cache-write tokens.
    pub write_cache: i64,
    /// completion tokens, excluding reasoning.
    pub completion: i64,
    /// reasoning tokens.
    pub reason: i64,
    /// total of all tokens.
    pub platform_total: i64,
}
