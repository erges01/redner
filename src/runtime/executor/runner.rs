use std::collections::HashMap;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin; // 👈 NEW: For pinning the boxed future
use crate::runtime::graph::node::{RuntimeNode, NodeType};
use crate::runtime::context::execution_context::ExecutionContext;

// 👈 NEW: A clean type alias so we don't have to type this nightmare everywhere
pub type RunnerFuture<'a> = Pin<Box<dyn Future<Output = Result<HashMap<String, Value>, String>> + Send + 'a>>;

/// The universal contract for all execution nodes.
pub trait NodeRunner: Send + Sync {
    /// Returns the type of node this runner knows how to execute
    fn handles(&self) -> NodeType;

    /// The actual execution logic (Returns a Dyn-compatible Boxed Future)
    fn execute<'a>(
        &'a self,
        node: &'a RuntimeNode,
        context: &'a ExecutionContext,
    ) -> RunnerFuture<'a>;
}