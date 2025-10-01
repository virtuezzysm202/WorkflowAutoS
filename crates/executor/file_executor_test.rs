use local_automation_common::Task;
use local_automation_executor::{FileExecutor, Executor};
use serde_json::json;
use std::path::PathBuf;
use tokio::fs;

#[tokio::test]
async fn test_file_read_write() {
    // Setup temp directory
    let temp_dir = std::env::temp_dir().join("local-automation-test");
    fs::create_dir_all(&temp_dir).await.unwrap();
    
    let executor = FileExecutor::new(temp_dir.clone());
    
    // Test write
    let write_task = Task::new(
        "file".to_string(),
        "write".to_string(),
        json!({
            "path": "test.txt",
            "content": "Hello, World!"
        }),
    );
    
    let result = executor.execute(&write_task).await.unwrap();
    assert!(result.success);
    
    // Test read
    let read_task = Task::new(
        "file".to_string(),
        "read".to_string(),
        json!({
            "path": "test.txt"
        }),
    );
    
    let result = executor.execute(&read_task).await.unwrap();
    assert!(result.success);
    assert_eq!(
        result.output.unwrap()["content"],
        "Hello, World!"
    );
    
    // Cleanup
    fs::remove_dir_all(&temp_dir).await.unwrap();
}