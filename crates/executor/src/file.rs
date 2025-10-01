use async_trait::async_trait;
use local_automation_common::{Error, Result, Task};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::traits::{Executor, ExecutionResult};

pub struct FileExecutor {
    base_path: PathBuf,
}

impl FileExecutor {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
    
    fn resolve_path(&self, path: &str) -> Result<PathBuf> {
        let path = Path::new(path);
        
        // Security: prevent path traversal
        if path.to_string_lossy().contains("..") {
            return Err(Error::PermissionDenied(
                "Path traversal not allowed".to_string()
            ));
        }
        
        Ok(self.base_path.join(path))
    }
}

#[async_trait]
impl Executor for FileExecutor {
    fn name(&self) -> &str {
        "file"
    }
    
    fn validate(&self, task: &Task) -> Result<()> {
        if task.executor != self.name() {
            return Err(Error::InvalidConfig(
                format!("Wrong executor: expected 'file', got '{}'", task.executor)
            ));
        }
        Ok(())
    }
    
    async fn execute(&self, task: &Task) -> Result<ExecutionResult> {
        self.validate(task)?;
        
        match task.operation.as_str() {
            "read" => self.read_file(task).await,
            "write" => self.write_file(task).await,
            "delete" => self.delete_file(task).await,
            "list" => self.list_dir(task).await,
            _ => Err(Error::InvalidConfig(
                format!("Unknown operation: {}", task.operation)
            )),
        }
    }
}

// Implementasi operations
impl FileExecutor {
    async fn read_file(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let content = fs::read_to_string(&full_path).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "content": content })),
            error: None,
        })
    }
    
    async fn write_file(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
            content: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        fs::write(&full_path, params.content.as_bytes()).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "path": full_path })),
            error: None,
        })
    }
    
    async fn delete_file(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        fs::remove_file(&full_path).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: None,
            error: None,
        })
    }
    
    async fn list_dir(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let mut entries = fs::read_dir(&full_path).await?;
        
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "files": files })),
            error: None,
        })
    }
}