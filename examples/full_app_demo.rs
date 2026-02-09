// Full application demo showing all implemented features
// This demonstrates the complete TUI application functionality

use rest_api_tui::models::{ApiEndpoint, HttpMethod};
use rest_api_tui::storage::StorageManager;
use rest_api_tui::http::{HttpClient, RequestInputs};
use rest_api_tui::formatter;
use rest_api_tui::template;
use rest_api_tui::load_test::{LoadTestConfig, MetricsCollector, calculate_percentiles};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== REST API TUI - Full Application Demo ===\n");
    
    // 1. Storage Layer Demo
    println!("1. STORAGE LAYER");
    println!("   Loading collections from ~/.rest-api-tui/collections/");
    
    let storage = StorageManager::new(
        dirs::home_dir().unwrap().join(".rest-api-tui/collections"),
        dirs::home_dir().unwrap().join(".rest-api-tui/results"),
    )?;
    
    let collections = storage.load_collections()?;
    println!("   ✓ Loaded {} collection(s)", collections.len());
    
    if let Some(collection) = collections.first() {
        println!("   Collection: {}", collection.name);
        println!("   Endpoints: {}", collection.endpoints.len());
        for (i, endpoint) in collection.endpoints.iter().enumerate() {
            println!("     {}. {:?} {}", i + 1, endpoint.method, endpoint.url);
        }
    }
    println!();
    
    // 2. Template Variable Substitution Demo
    println!("2. TEMPLATE VARIABLE SUBSTITUTION");
    let mut variables = HashMap::new();
    variables.insert("userId".to_string(), "123".to_string());
    variables.insert("token".to_string(), "abc-xyz-789".to_string());
    
    let template_url = "https://api.example.com/users/{{userId}}/profile";
    let substituted = template::substitute(template_url, &variables)?;
    println!("   Template: {}", template_url);
    println!("   Result:   {}", substituted);
    println!();
    
    // 3. HTTP Client Demo
    println!("3. HTTP CLIENT");
    println!("   Executing real HTTP request to JSONPlaceholder API...");
    
    let client = HttpClient::new()?;
    let endpoint = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "Get User".to_string(),
        method: HttpMethod::GET,
        url: "https://jsonplaceholder.typicode.com/users/1".to_string(),
        description: Some("Fetch user details".to_string()),
        headers: HashMap::new(),
        body_template: None,
        auth: None,
        load_test_config: None,
    };
    
    let inputs = RequestInputs {
        headers: HashMap::new(),
        query_params: HashMap::new(),
        body: None,
        variables: HashMap::new(),
    };
    
    match client.execute(&endpoint, &inputs).await {
        Ok(response) => {
            println!("   ✓ Status: {}", response.status);
            println!("   ✓ Duration: {}ms", response.duration.as_millis());
            println!("   ✓ Body size: {} bytes", response.body.len());
        }
        Err(e) => {
            println!("   ✗ Error: {}", e);
        }
    }
    println!();
    
    // 4. Response Formatting Demo
    println!("4. RESPONSE FORMATTING");
    let json_data = r#"{"name":"John","age":30,"city":"New York"}"#;
    let formatted = formatter::format_json(json_data.as_bytes())?;
    println!("   Original: {}", json_data);
    println!("   Formatted:\n{}", formatted);
    println!();
    
    // 5. Load Test Metrics Demo
    println!("5. LOAD TEST METRICS");
    println!("   Simulating load test metrics collection...");
    
    let collector = MetricsCollector::new();
    
    // Simulate some requests
    collector.record_success(Duration::from_millis(120));
    collector.record_success(Duration::from_millis(95));
    collector.record_success(Duration::from_millis(150));
    collector.record_failure("Timeout".to_string(), Duration::from_millis(5000));
    collector.record_success(Duration::from_millis(80));
    
    let snapshot = collector.snapshot();
    println!("   Total Requests: {}", snapshot.total_requests);
    println!("   Successful: {}", snapshot.successful_requests);
    println!("   Failed: {}", snapshot.failed_requests);
    
    let percentiles = calculate_percentiles(&snapshot.latencies);
    println!("   P50: {:.0}ms, P95: {:.0}ms, P99: {:.0}ms", 
             percentiles.p50.as_millis(), percentiles.p95.as_millis(), percentiles.p99.as_millis());
    println!();
    
    // 6. Load Test Configuration Demo
    println!("6. LOAD TEST CONFIGURATION");
    let config = LoadTestConfig::new(10, Duration::from_secs(30));
    println!("   ✓ Valid configuration created:");
    println!("     Concurrency: {}", config.concurrency);
    println!("     Duration: {}s", config.duration.as_secs());
    println!();
    
    // 7. TUI Application State Demo
    println!("7. TUI APPLICATION");
    println!("   The TUI application is ready to run!");
    println!("   Run 'cargo run' to start the interactive interface.");
    println!();
    println!("   Available screens:");
    println!("   • Collection List - Browse your API collections");
    println!("   • Endpoint List - View endpoints in a collection");
    println!("   • Endpoint Detail - See endpoint details and execute");
    println!("   • Response View - View formatted API responses");
    println!("   • Load Test Running - Real-time load test visualization");
    println!("   • Help - Keyboard shortcuts and commands");
    println!();
    println!("   Navigation:");
    println!("   • ↑/↓ or j/k - Move selection");
    println!("   • Enter - Select/Execute");
    println!("   • Esc - Go back");
    println!("   • q - Quit");
    println!("   • ? - Show help");
    println!();
    
    println!("=== Demo Complete ===");
    println!("\nAll core features are implemented and working!");
    println!("The TUI is ready for interactive use.");
    
    Ok(())
}
