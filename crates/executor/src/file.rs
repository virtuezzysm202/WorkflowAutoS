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
            "read_csv" => self.read_csv(task).await,
            "read_json" => self.read_json(task).await,
            "write" => self.write_file(task).await,
            "delete" => self.delete_file(task).await,
            "move" => self.move_file(task).await,
            "copy" => self.copy_file(task).await,
            "list" => self.list_dir(task).await,
            "write_json" => self.write_json(task).await,
            "write_csv"  => self.write_csv(task).await,
            "create_dir" => self.create_dir(task).await,
            "exists"     => self.exists(task).await,
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

    async fn read_csv(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let content = fs::read_to_string(&full_path).await?;
        
        let mut reader = csv::Reader::from_reader(content.as_bytes());
        
        //Get headers
        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string()
            )))?
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        //Get data rows (without headers)
        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result.map_err(|e| Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string()
            )))?;
            
            let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            rows.push(row);
        }
        
        //Return both headers and rows
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({
                "headers": headers,
                "rows": rows
            })),
            error: None,
        })
    }

    async fn read_json(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let content = fs::read_to_string(&full_path).await?;
        let json: serde_json::Value = serde_json::from_str(&content)?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(json),
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

    async fn copy_file(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            from: String,
            to: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
        .map_err(|e| Error::InvalidConfig(e.to_string()))?;
    
    let from_path = self.resolve_path(&params.from)?;
    let to_path = self.resolve_path(&params.to)?;
    
    fs::copy(&from_path, &to_path).await?;
    
    Ok(ExecutionResult {
        success: true,
        output: Some(serde_json::json!({
            "from": from_path,
            "to": to_path
        })),
        error: None,
    })
    }

    async fn move_file(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            from: String,
            to: String,
        }

        let params:Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;

        let from_path = self.resolve_path(&params.from)?;
        let to_path = self.resolve_path(&params.to)?;

        fs::rename(&from_path, &to_path).await?;

        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({
                "from": from_path,
                "to": to_path
            })),
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

    async fn write_json(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
            data: serde_json::Value,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let json_string = serde_json::to_string_pretty(&params.data)?;
        fs::write(&full_path, json_string.as_bytes()).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "path": full_path })),
            error: None,
        })
    }
    
    async fn write_csv(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
            headers: Vec<String>,
            rows: Vec<Vec<String>>,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record(&params.headers)
            .map_err(|e| Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string()
            )))?;
        
        for row in params.rows {
            wtr.write_record(&row)
                .map_err(|e| Error::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string()
                )))?;
        }
        
        let data = wtr.into_inner()
            .map_err(|e| Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string()
            )))?;
        
        fs::write(&full_path, data).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "path": full_path })),
            error: None,
        })
    }
    
    async fn create_dir(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        fs::create_dir_all(&full_path).await?;
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "path": full_path })),
            error: None,
        })
    }
    
    async fn exists(&self, task: &Task) -> Result<ExecutionResult> {
        #[derive(Deserialize)]
        struct Params {
            path: String,
        }
        
        let params: Params = serde_json::from_value(task.params.clone())
            .map_err(|e| Error::InvalidConfig(e.to_string()))?;
        
        let full_path = self.resolve_path(&params.path)?;
        let exists = full_path.exists();
        
        Ok(ExecutionResult {
            success: true,
            output: Some(serde_json::json!({ "exists": exists })),
            error: None,
        })
    }
}