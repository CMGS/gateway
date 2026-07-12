//! DAG executor.
//!
//! Four fixed layers run in order; within a layer, nodes run in dependency
//! (topological) order via Kahn's algorithm. Running independent nodes
//! concurrently is a possible future optimization (Phase-4); execution here
//! is sequential-in-topo-order, which is behavior-compatible for these nodes.
//! Layer topologies are declared in code for now.

use ap_models::{GResult, GatewayError};

use crate::context::DagContext;

#[async_trait::async_trait]
pub trait DagNode: Send + Sync {
    fn name(&self) -> &'static str;
    /// names of same-layer nodes that must run first.
    fn deps(&self) -> &'static [&'static str] {
        &[]
    }
    async fn execute(&self, ctx: &mut DagContext) -> GResult<()>;
}

pub struct Layer {
    pub name: &'static str,
    pub nodes: Vec<Box<dyn DagNode>>,
}

/// Kahn topological order over one layer; stable for ties (declaration order).
fn topo_order(layer: &Layer) -> GResult<Vec<usize>> {
    let n = layer.nodes.len();
    let idx_of = |name: &str| layer.nodes.iter().position(|x| x.name() == name);
    let mut indegree = vec![0usize; n];
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n]; // dep -> dependents
    for (i, node) in layer.nodes.iter().enumerate() {
        for d in node.deps() {
            let Some(j) = idx_of(d) else {
                return Err(GatewayError::internal(format!(
                    "dag layer `{}`: node `{}` depends on unknown node `{d}`",
                    layer.name,
                    node.name()
                )));
            };
            edges[j].push(i);
            indegree[i] += 1;
        }
    }
    let mut queue: Vec<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    let mut head = 0;
    while head < queue.len() {
        let i = queue[head];
        head += 1;
        order.push(i);
        for &k in &edges[i] {
            indegree[k] -= 1;
            if indegree[k] == 0 {
                queue.push(k);
            }
        }
    }
    if order.len() != n {
        return Err(GatewayError::internal(format!(
            "dag layer `{}` contains a dependency cycle",
            layer.name
        )));
    }
    Ok(order)
}

/// Run all layers in order; a node error aborts the whole run (fail-fast
/// for online requests).
pub async fn run(layers: &[Layer], ctx: &mut DagContext) -> GResult<()> {
    for layer in layers {
        let order = topo_order(layer)?;
        for i in order {
            let node = &layer.nodes[i];
            tracing::debug!(layer = layer.name, node = node.name(), "dag node start");
            node.execute(ctx).await?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rec(&'static str, &'static [&'static str]);

    #[async_trait::async_trait]
    impl DagNode for Rec {
        fn name(&self) -> &'static str {
            self.0
        }
        fn deps(&self) -> &'static [&'static str] {
            self.1
        }
        async fn execute(&self, ctx: &mut DagContext) -> GResult<()> {
            ctx.decide(self.0, "ran");
            Ok(())
        }
    }

    fn test_ctx() -> DagContext {
        use std::sync::Arc;
        let cfg = Arc::new(ap_config::GatewayConfig::embedded_default().unwrap());
        let state = Arc::new(ap_state::GatewayState::from_config(&cfg));
        DagContext::new(
            cfg,
            state,
            Arc::new(ap_engines::MockTransport),
            Default::default(),
            ap_state::AkInfo {
                ak: "t".into(),
                product: "demo".into(),
                qps: 10.0,
                daily_token_quota: 1000,
                tokens_per_minute: None,
            },
        )
    }

    #[tokio::test]
    async fn topo_respects_deps() {
        let layer = Layer {
            name: "t",
            nodes: vec![
                Box::new(Rec("b", &["a"])),
                Box::new(Rec("c", &["b"])),
                Box::new(Rec("a", &[])),
            ],
        };
        let mut ctx = test_ctx();
        run(&[layer], &mut ctx).await.unwrap();
        assert_eq!(ctx.decisions, vec!["a: ran", "b: ran", "c: ran"]);
    }

    #[tokio::test]
    async fn cycle_is_an_error() {
        let layer = Layer {
            name: "t",
            nodes: vec![Box::new(Rec("a", &["b"])), Box::new(Rec("b", &["a"]))],
        };
        let mut ctx = test_ctx();
        assert!(run(&[layer], &mut ctx).await.is_err());
    }
}
