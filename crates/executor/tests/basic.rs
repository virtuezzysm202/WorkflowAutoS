use local_automation_common::Task;
use local_automation_executor::file::FileExecutor;
use local_automation_executor::Executor; 
use serde_json::json;

#[tokio::test]
async fn test_read_file_executor() {
    let task = Task::new(
        "file".to_string(),           
        "read".to_string(),           
        json!({ "path": "hello.txt" }), 
    );

    let executor = FileExecutor::new(std::path::PathBuf::from("tests/data"));

    let result = executor.execute(&task).await;
    if let Err(e) = &result {
        println!("Error saat read_file: {:?}", e);
    }

    assert!(result.is_ok());
    let content = result.unwrap();


    println!("Isi file: {}", content.output.unwrap()["content"]);
}
