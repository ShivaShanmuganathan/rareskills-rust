use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("Testing Panic in Panic Handler vulnerability...");
    let client = reqwest::Client::new();
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