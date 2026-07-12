//! Local background tasks.
//!
//! Currently just one pure in-process task: periodic AK daily quota reset.
//! Batch job execution lives in ap-handler::offline (spawned on submit) and
//! needs no separate poller.

use std::sync::Arc;
use std::time::Duration;

use ap_state::GatewayState;

/// Spawn the daily quota reset loop. Returns the join handle (abort to stop).
/// `period` is configurable so tests don't wait 24h.
pub fn spawn_quota_reset(
    state: Arc<GatewayState>,
    period: Duration,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(period);
        tick.tick().await; // first tick fires immediately; skip it
        loop {
            tick.tick().await;
            state.quota.reset_all();
            tracing::info!(target: "task", "quota_reset: all AK daily counters cleared");
        }
    })
}

/// The production period: once a day.
pub const DAILY: Duration = Duration::from_secs(24 * 60 * 60);

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn quota_reset_clears_counters() {
        let state = Arc::new(GatewayState::default());
        state.quota.consume("ak-x", 42);
        assert_eq!(state.quota.used("ak-x"), 42);
        let handle = spawn_quota_reset(state.clone(), Duration::from_millis(20));
        // wait for at least one reset tick
        for _ in 0..50 {
            if state.quota.used("ak-x") == 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        assert_eq!(state.quota.used("ak-x"), 0);
        handle.abort();
    }
}
