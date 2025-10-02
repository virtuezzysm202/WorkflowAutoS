use local_automation_common::Task;
use local_automation_executor::file::FileExecutor;
use local_automation_executor::Executor;
use serde_json::json;
use tempfile::tempdir;

#[tokio::test]
async fn test_all_file_operations() {
    let dir = tempdir().unwrap();
    let executor = FileExecutor::new(dir.path().to_path_buf());

    // 1. Write file
    let write_task = Task::new(
        "file".to_string(),
        "write".to_string(),
        json!({ "path": "test.txt", "content": "Hello World" }),
    );
    let result = executor.execute(&write_task).await.unwrap();
    assert!(result.success);
    println!("Write file test passed");

    // 2. Read file
    let read_task = Task::new(
        "file".to_string(),
        "read".to_string(),
        json!({ "path": "test.txt" }),
    );
    let read_result = executor.execute(&read_task).await.unwrap();
    assert_eq!(read_result.output.unwrap()["content"], "Hello World");
    println!("Read file test passed");

    // 3. Copy file
    let copy_task = Task::new(
        "file".to_string(),
        "copy".to_string(),
        json!({ "from": "test.txt", "to": "test_copy.txt" }),
    );
    executor.execute(&copy_task).await.unwrap();
    println!("Copy file test passed");

    // 4. Move file
    let move_task = Task::new(
        "file".to_string(),
        "move".to_string(),
        json!({ "from": "test_copy.txt", "to": "test_moved.txt" }),
    );
    executor.execute(&move_task).await.unwrap();
    println!("Move file test passed");

    // 5. List dir
    let list_task = Task::new(
        "file".to_string(),
        "list".to_string(),
        json!({ "path": "." }),
    );
    let list_result = executor.execute(&list_task).await.unwrap();
    let output = list_result.output.unwrap();
    let files = output["files"].as_array().unwrap();
    assert!(files.iter().any(|f| f.as_str().unwrap() == "test.txt"));
    assert!(files.iter().any(|f| f.as_str().unwrap() == "test_moved.txt"));
    println!("List directory test passed");

    // 6. Delete file
    let delete_task = Task::new(
        "file".to_string(),
        "delete".to_string(),
        json!({ "path": "test.txt" }),
    );
    executor.execute(&delete_task).await.unwrap();

    let delete_moved = Task::new(
        "file".to_string(),
        "delete".to_string(),
        json!({ "path": "test_moved.txt" }),
    );
    executor.execute(&delete_moved).await.unwrap();
    println!("Delete file test passed");

    // 7. Write JSON file
    let json_task = Task::new(
        "file".to_string(),
        "write".to_string(),
        json!({ "path": "data.json", "content": r#"{ "key": "value" }"# }),
    );
    executor.execute(&json_task).await.unwrap();
    println!("Write JSON file test passed");

    // 8. Read JSON file
    let read_json_task = Task::new(
        "file".to_string(),
        "read_json".to_string(),
        json!({ "path": "data.json" }),
    );
    let json_result = executor.execute(&read_json_task).await.unwrap();
    assert_eq!(json_result.output.unwrap()["key"], "value");
    println!("Read JSON file test passed");

    // 9. Write CSV file
    let csv_content = "name,age\nAlice,30\nBob,25";
    let csv_task = Task::new(
        "file".to_string(),
        "write".to_string(),
        json!({ "path": "data.csv", "content": csv_content }),
    );
    executor.execute(&csv_task).await.unwrap();
    println!("Write CSV file test passed");

    // 10. Read CSV file
    let read_csv_task = Task::new(
        "file".to_string(),
        "read_csv".to_string(),
        json!({ "path": "data.csv" }),
    );
    let csv_result = executor.execute(&read_csv_task).await.unwrap();
    assert!(csv_result.success);

    let output = csv_result.output.unwrap();

    let headers = output["headers"].as_array().unwrap();
    let rows = output["rows"].as_array().unwrap();

    assert_eq!(headers.len(), 2);
    assert_eq!(headers[0], "name");
    assert_eq!(headers[1], "age");

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], json!(["Alice", "30"]));
    assert_eq!(rows[1], json!(["Bob", "25"]));

    println!("Read CSV file test passed");
    println!("Headers: {:?}", headers);
    println!("Rows: {:?}", rows);
}
