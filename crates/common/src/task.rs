use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub type TaskId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub executor: String,
    pub operation: String, 
    pub params: serde_json::Value,
    pub status: TaskStatus, 
    pub created_at: DateTime<Utc>, 
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

pub enum TaskStatus { 
    Pending,
    Running, 
    Completed, 
    Failed, 
    Cancelled,
}

impl Task { 
    pub fn new (executor: String, operation: String, params: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            executor,
            operation,
            params,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None, 
        }
    }
}
