use serde_json::Value;
use std::sync::Arc;
use crate::platform::sdk::tools::ToolRegistry;

/// A single step in an automation sequence
pub struct WorkflowStep {
    pub tool_name: String,
    pub args: Value,
}

/// A complete automation macro
pub struct Workflow {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
}

/// The engine that runs automations using the Universal Tool Registry
pub struct WorkflowEngine {
    registry: Arc<ToolRegistry>,
}

impl WorkflowEngine {
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self { registry }
    }

    /// Executes every step in the workflow sequentially.
    /// In the future, we can pass the output of Step 1 into the arguments of Step 2!
    pub async fn run_workflow(&self, workflow: &Workflow) -> Result<Vec<Value>, String> {
        println!("🤖 [WORKFLOW] Starting automation: {}", workflow.name);
        
        let mut results = Vec::new();
        
        for (i, step) in workflow.steps.iter().enumerate() {
            println!("   ⚡ [STEP {}] Executing tool: {}", i + 1, step.tool_name);
            
            match self.registry.execute_tool(&step.tool_name, step.args.clone()).await {
                Ok(result) => results.push(result),
                Err(e) => return Err(format!("Workflow failed at step '{}': {}", step.tool_name, e)),
            }
        }
        
        println!("✅ [WORKFLOW] Automation complete!");
        Ok(results)
    }
}