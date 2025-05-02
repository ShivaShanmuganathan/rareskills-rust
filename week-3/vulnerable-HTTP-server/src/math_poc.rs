use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() {
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
} 