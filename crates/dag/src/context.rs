//! Per-request DAG context.
//!
//! One mutable value threaded through every node of the four layers. Nodes read
//! what upstream nodes produced and write what downstream nodes need.

use std::sync::Arc;

use ap_config::GatewayConfig;
use ap_engines::{EngineOutcome, SharedTransport};
use ap_models::GatewayRequest;
use ap_state::{AkInfo, GatewayState};

pub struct DagContext {
    // --- environment (read-only) ---
    pub cfg: Arc<GatewayConfig>,
    pub state: Arc<GatewayState>,
    pub transport: SharedTransport,

    // --- the request being served ---
    pub request: GatewayRequest,
    pub ak: AkInfo,

    // --- produced along the way ---
    /// engine result, set by the model_access layer.
    pub outcome: Option<EngineOutcome>,
    /// decision trail as (stage, detail); joined only when read, so the hot
    /// path allocates the detail once instead of a second joined string.
    pub decisions: Vec<(&'static str, String)>,
    /// Request-level cache hit (downstream nodes short-circuit on this and skip
    /// account/engine/billing).
    pub cache_hit: bool,
    /// This request's cache key (computed by cache_lookup, reused by cache_store).
    pub cache_key: Option<String>,
}

impl DagContext {
    pub fn new(
        cfg: Arc<GatewayConfig>,
        state: Arc<GatewayState>,
        transport: SharedTransport,
        request: GatewayRequest,
        ak: AkInfo,
    ) -> Self {
        Self {
            cfg,
            state,
            transport,
            request,
            ak,
            outcome: None,
            decisions: Vec::new(),
            cache_hit: false,
            cache_key: None,
        }
    }

    pub fn decide(&mut self, node: &'static str, what: impl Into<String>) {
        self.decisions.push((node, what.into()));
    }

    /// The decision trail as `"stage: detail"` lines.
    pub fn decision_lines(&self) -> impl Iterator<Item = String> + '_ {
        self.decisions.iter().map(|(n, w)| format!("{n}: {w}"))
    }
}
