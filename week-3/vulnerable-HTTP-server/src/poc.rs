use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() {
    // PoC 1: Division by Zero
    println!("Testing Division by Zero vulnerability...");
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/math")
        .json(&json!({
            "a": 10,
            "b": 0,
            "operation": "division"
        }))
        .send()
        .await
        .unwrap();
    
    println!("Division by zero response status: {}", response.status());

    // PoC 2: Panic in Panic Handler
    println!("\nTesting Panic in Panic Handler vulnerability...");
    // This will cause a panic in the server's panic handler
    let response = client
        .post("http://localhost:3000/math")
        .json(&json!({
            "a": 0,
            "b": 0,
            "operation": "invalid_operation"
        }))
        .send()
        .await
        .unwrap();
    
    println!("Panic handler test response status: {}", response.status());
} 