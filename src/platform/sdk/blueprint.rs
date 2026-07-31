use serde_json::{Value, json};
use std::collections::HashMap;
use async_trait::async_trait;

/// 1. THE BLUEPRINT CONTRACT
/// A Blueprint is a reusable template that takes dynamic variables 
/// and generates a valid Redner project/timeline structure.
#[async_trait]
pub trait BlueprintProvider: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    
    /// The expected JSON schema for variables (e.g., text, colors, pacing)
    fn variable_schema(&self) -> Value;
    
    /// Compiles the variables into a final Redner timeline JSON
    async fn generate(&self, variables: Value) -> Result<Value, String>;
}

/// 2. THE BLUEPRINT REGISTRY
pub struct BlueprintRegistry {
    blueprints: HashMap<String, Box<dyn BlueprintProvider>>,
}

impl BlueprintRegistry {
    pub fn new() -> Self {
        Self { blueprints: HashMap::new() }
    }

    pub fn register(&mut self, blueprint: Box<dyn BlueprintProvider>) {
        self.blueprints.insert(blueprint.id().to_string(), blueprint);
    }

    pub async fn execute(&self, id: &str, variables: Value) -> Result<Value, String> {
        if let Some(blueprint) = self.blueprints.get(id) {
            blueprint.generate(variables).await
        } else {
            Err(format!("Blueprint '{}' not found.", id))
        }
    }
}