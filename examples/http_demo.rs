use rest_api_tui::models::{ApiEndpoint, HttpMethod, AuthConfig, ApiKeyLocation};
use rest_api_tui::http::{HttpClient, RequestInputs};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== REST API TUI - HTTP Client Demo ===\n");
    
    let client = HttpClient::new()?;
    println!("✓ HTTP client initialized (timeout: {:?})\n", client.timeout());
    
    // Example 1: Simple GET request to JSONPlaceholder
    println!("📝 Example 1: Simple GET Request");
    let endpoint1 = ApiEndpoint::new(
        "Get Post".to_string(),
        HttpMethod::GET,
        "https://jsonplaceholder.typicode.com/posts/1".to_string(),
    );
    
    println!("  {} {}", format!("{:?}", endpoint1.method), endpoint1.url);
    
    let inputs1 = RequestInputs::default();
    let response1 = client.execute(&endpoint1, &inputs1).await?;
    
    println!("  Status: {}", response1.status);
    println!("  Duration: {:?}", response1.duration);
    println!("  Body size: {} bytes", response1.body.len());
    
    let body_str = String::from_utf8_lossy(&response1.body);
    if body_str.len() < 200 {
        println!("  Body: {}", body_str);
    } else {
        println!("  Body: {}...", &body_str[..200]);
    }
    println!();
    
    // Example 2: GET with query parameters
    println!("📝 Example 2: GET with Query Parameters");
    let endpoint2 = ApiEndpoint::new(
        "Search Posts".to_string(),
        HttpMethod::GET,
        "https://jsonplaceholder.typicode.com/posts".to_string(),
    );
    
    let mut inputs2 = RequestInputs::default();
    inputs2.query_params.insert("userId".to_string(), "1".to_string());
    
    println!("  {} {} (with userId=1)", format!("{:?}", endpoint2.method), endpoint2.url);
    
    let response2 = client.execute(&endpoint2, &inputs2).await?;
    
    println!("  Status: {}", response2.status);
    println!("  Duration: {:?}", response2.duration);
    println!("  Body size: {} bytes", response2.body.len());
    
    // Parse JSON to count results
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response2.body) {
        if let Some(array) = json.as_array() {
            println!("  Results: {} posts", array.len());
        }
    }
    println!();
    
    // Example 3: GET with template variables
    println!("📝 Example 3: GET with Template Variables");
    let endpoint3 = ApiEndpoint::new(
        "Get User Post".to_string(),
        HttpMethod::GET,
        "https://jsonplaceholder.typicode.com/posts/{{post_id}}".to_string(),
    );
    
    let mut inputs3 = RequestInputs::default();
    inputs3.variables.insert("post_id".to_string(), "5".to_string());
    
    println!("  Template: {}", endpoint3.url);
    println!("  Variables: post_id=5");
    
    let response3 = client.execute(&endpoint3, &inputs3).await?;
    
    println!("  Status: {}", response3.status);
    println!("  Duration: {:?}", response3.duration);
    
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response3.body) {
        if let Some(title) = json.get("title") {
            println!("  Post title: {}", title);
        }
    }
    println!();
    
    // Example 4: POST with JSON body
    println!("📝 Example 4: POST with JSON Body");
    let mut endpoint4 = ApiEndpoint::new(
        "Create Post".to_string(),
        HttpMethod::POST,
        "https://jsonplaceholder.typicode.com/posts".to_string(),
    );
    
    endpoint4.headers.insert("Content-Type".to_string(), "application/json".to_string());
    endpoint4.body_template = Some(r#"{
  "title": "{{title}}",
  "body": "{{body}}",
  "userId": {{user_id}}
}"#.to_string());
    
    let mut inputs4 = RequestInputs::default();
    inputs4.variables.insert("title".to_string(), "My Test Post".to_string());
    inputs4.variables.insert("body".to_string(), "This is a test post created via REST API TUI".to_string());
    inputs4.variables.insert("user_id".to_string(), "1".to_string());
    
    println!("  {} {}", format!("{:?}", endpoint4.method), endpoint4.url);
    println!("  Content-Type: application/json");
    
    let response4 = client.execute(&endpoint4, &inputs4).await?;
    
    println!("  Status: {}", response4.status);
    println!("  Duration: {:?}", response4.duration);
    
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response4.body) {
        println!("  Created post ID: {}", json.get("id").unwrap_or(&serde_json::Value::Null));
    }
    println!();
    
    // Example 5: Custom headers
    println!("📝 Example 5: Custom Headers");
    let mut endpoint5 = ApiEndpoint::new(
        "Get with Headers".to_string(),
        HttpMethod::GET,
        "https://httpbin.org/headers".to_string(),
    );
    
    endpoint5.headers.insert("X-Custom-Header".to_string(), "{{custom_value}}".to_string());
    endpoint5.headers.insert("User-Agent".to_string(), "REST-API-TUI/1.0".to_string());
    
    let mut inputs5 = RequestInputs::default();
    inputs5.variables.insert("custom_value".to_string(), "test-123".to_string());
    
    println!("  {} {}", format!("{:?}", endpoint5.method), endpoint5.url);
    println!("  Headers: X-Custom-Header, User-Agent");
    
    let response5 = client.execute(&endpoint5, &inputs5).await?;
    
    println!("  Status: {}", response5.status);
    println!("  Duration: {:?}", response5.duration);
    
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response5.body) {
        if let Some(headers) = json.get("headers") {
            println!("  Server received headers:");
            if let Some(custom) = headers.get("X-Custom-Header") {
                println!("    X-Custom-Header: {}", custom);
            }
            if let Some(ua) = headers.get("User-Agent") {
                println!("    User-Agent: {}", ua);
            }
        }
    }
    println!();
    
    // Example 6: Bearer token authentication
    println!("📝 Example 6: Bearer Token Authentication");
    let mut endpoint6 = ApiEndpoint::new(
        "Authenticated Request".to_string(),
        HttpMethod::GET,
        "https://httpbin.org/bearer".to_string(),
    );
    
    endpoint6.auth = Some(AuthConfig::Bearer {
        token: "{{access_token}}".to_string(),
    });
    
    let mut inputs6 = RequestInputs::default();
    inputs6.variables.insert("access_token".to_string(), "my-secret-token-12345".to_string());
    
    println!("  {} {}", format!("{:?}", endpoint6.method), endpoint6.url);
    println!("  Auth: Bearer token");
    
    let response6 = client.execute(&endpoint6, &inputs6).await?;
    
    println!("  Status: {}", response6.status);
    println!("  Duration: {:?}", response6.duration);
    
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response6.body) {
        if let Some(authenticated) = json.get("authenticated") {
            println!("  Authenticated: {}", authenticated);
        }
        if let Some(token) = json.get("token") {
            println!("  Token received: {}", token);
        }
    }
    println!();
    
    println!("=== Demo Complete ===");
    println!("\n✨ Key Features Demonstrated:");
    println!("  • Simple GET requests");
    println!("  • Query parameters with URL encoding");
    println!("  • Template variable substitution");
    println!("  • POST with JSON body");
    println!("  • Custom headers");
    println!("  • Bearer token authentication");
    println!("  • Response parsing and metadata");
    
    Ok(())
}
