use rest_api_tui::models::{ApiCollection, ApiEndpoint, HttpMethod, AuthConfig, ApiKeyLocation};
use rest_api_tui::storage::StorageManager;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== REST API TUI - Storage Layer Demo ===\n");
    
    // Create storage manager with temp directory for demo
    let temp_dir = std::env::temp_dir().join("rest-api-tui-demo");
    let storage = StorageManager::new(
        temp_dir.join("collections"),
        temp_dir.join("results"),
    )?;
    
    println!("✓ Storage manager initialized");
    println!("  Collections dir: {:?}", temp_dir.join("collections"));
    println!("  Results dir: {:?}\n", temp_dir.join("results"));
    
    // Create a sample collection
    let mut collection = ApiCollection::new("GitHub API".to_string());
    
    // Add some endpoints
    let mut headers = HashMap::new();
    headers.insert("Accept".to_string(), "application/json".to_string());
    headers.insert("User-Agent".to_string(), "REST-API-TUI".to_string());
    
    let endpoint1 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "Get User".to_string(),
        method: HttpMethod::GET,
        url: "https://api.github.com/users/{{username}}".to_string(),
        headers: headers.clone(),
        body_template: None,
        auth: Some(AuthConfig::Bearer {
            token: "{{github_token}}".to_string(),
        }),
        description: Some("Fetch a GitHub user by username".to_string()),
    };
    
    let endpoint2 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "List Repos".to_string(),
        method: HttpMethod::GET,
        url: "https://api.github.com/users/{{username}}/repos".to_string(),
        headers: headers.clone(),
        body_template: None,
        auth: Some(AuthConfig::ApiKey {
            name: "X-API-Key".to_string(),
            value: "{{api_key}}".to_string(),
            location: ApiKeyLocation::Header,
        }),
        description: Some("List repositories for a user".to_string()),
    };
    
    collection.add_endpoint(endpoint1);
    collection.add_endpoint(endpoint2);
    
    println!("✓ Created collection: '{}'", collection.name);
    println!("  ID: {}", collection.id);
    println!("  Endpoints: {}", collection.endpoints.len());
    for (i, endpoint) in collection.endpoints.iter().enumerate() {
        println!("    {}. {:?} {} - {}", i + 1, endpoint.method, endpoint.name, endpoint.url);
    }
    println!();
    
    // Save the collection
    storage.save_collection(&collection)?;
    println!("✓ Collection saved to disk\n");
    
    // Load all collections
    let loaded_collections = storage.load_collections()?;
    println!("✓ Loaded {} collection(s) from disk", loaded_collections.len());
    
    for coll in &loaded_collections {
        println!("  - {} (ID: {})", coll.name, coll.id);
        println!("    Created: {}", coll.created_at);
        println!("    Updated: {}", coll.updated_at);
        println!("    Endpoints: {}", coll.endpoints.len());
    }
    println!();
    
    // Create another collection
    let mut collection2 = ApiCollection::new("JSONPlaceholder API".to_string());
    let endpoint3 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "Get Posts".to_string(),
        method: HttpMethod::GET,
        url: "https://jsonplaceholder.typicode.com/posts".to_string(),
        headers: HashMap::new(),
        body_template: None,
        auth: None,
        description: Some("Fetch all posts".to_string()),
    };
    collection2.add_endpoint(endpoint3);
    storage.save_collection(&collection2)?;
    
    println!("✓ Created and saved second collection: '{}'", collection2.name);
    println!();
    
    // Load all collections again
    let all_collections = storage.load_collections()?;
    println!("✓ Total collections in storage: {}", all_collections.len());
    for coll in &all_collections {
        println!("  - {}", coll.name);
    }
    println!();
    
    // Delete the first collection
    storage.delete_collection(&collection.id)?;
    println!("✓ Deleted collection: '{}'", collection.name);
    println!();
    
    // Load collections one more time
    let remaining_collections = storage.load_collections()?;
    println!("✓ Remaining collections: {}", remaining_collections.len());
    for coll in &remaining_collections {
        println!("  - {}", coll.name);
    }
    
    println!("\n=== Demo Complete ===");
    println!("Storage location: {:?}", temp_dir);
    println!("\nYou can inspect the saved JSON files at the location above!");
    
    Ok(())
}
