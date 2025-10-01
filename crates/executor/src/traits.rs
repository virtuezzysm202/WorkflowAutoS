use async_trait::async_trait;
use local_automation_common::{Result, Task};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: Option<Value>,
    pub error: Option<String>,
}

#[async_trait]
pub trait Executor: Send + Sync {
    fn name(&self) -> &str;
    
    
    async fn execute(&self, task: &Task) -> Result<ExecutionResult>;
    
    
    fn validate(&self, task: &Task) -> Result<()>;
}