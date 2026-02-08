// Demo showing collection management features
// This demonstrates creating, editing, and deleting collections programmatically

use rest_api_tui::models::{ApiCollection, ApiEndpoint, HttpMethod};
use rest_api_tui::storage::StorageManager;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Collection Management Demo ===\n");
    
    let storage = StorageManager::with_defaults()?;
    
    // 1. Create a new collection
    println!("1. Creating new collection...");
    let mut collection = ApiCollection::new("Demo API".to_string());
    println!("   ✓ Created collection: {}", collection.name);
    println!("   ID: {}", collection.id);
    
    // 2. Add endpoints to the collection
    println!("\n2. Adding endpoints...");
    
    let endpoint1 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "List Users".to_string(),
        method: HttpMethod::GET,
        url: "https://jsonplaceholder.typicode.com/users".to_string(),
        description: Some("Get all users".to_string()),
        headers: HashMap::new(),
        body_template: None,
        auth: None,
    };
    collection.add_endpoint(endpoint1);
    println!("   ✓ Added endpoint: List Users (GET)");
    
    let endpoint2 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "Create Post".to_string(),
        method: HttpMethod::POST,
        url: "https://jsonplaceholder.typicode.com/posts".to_string(),
        description: Some("Create a new post".to_string()),
        headers: HashMap::new(),
        body_template: Some(r#"{"title": "{{title}}", "body": "{{body}}", "userId": {{userId}}}"#.to_string()),
        auth: None,
    };
    collection.add_endpoint(endpoint2);
    println!("   ✓ Added endpoint: Create Post (POST)");
    
    let endpoint3 = ApiEndpoint {
        id: uuid::Uuid::new_v4(),
        name: "Delete Post".to_string(),
        method: HttpMethod::DELETE,
        url: "https://jsonplaceholder.typicode.com/posts/{{postId}}".to_string(),
        description: Some("Delete a post by ID".to_string()),
        headers: HashMap::new(),
        body_template: None,
        auth: None,
    };
    collection.add_endpoint(endpoint3);
    println!("   ✓ Added endpoint: Delete Post (DELETE)");
    
    println!("\n   Total endpoints: {}", collection.endpoints.len());
    
    // 3. Save the collection
    println!("\n3. Saving collection to storage...");
    storage.save_collection(&collection)?;
    println!("   ✓ Collection saved successfully");
    
    // 4. Load all collections
    println!("\n4. Loading all collections...");
    let collections = storage.load_collections()?;
    println!("   ✓ Loaded {} collection(s)", collections.len());
    
    for (i, coll) in collections.iter().enumerate() {
        println!("\n   Collection {}: {}", i + 1, coll.name);
        println!("   ID: {}", coll.id);
        println!("   Endpoints: {}", coll.endpoints.len());
        for (j, ep) in coll.endpoints.iter().enumerate() {
            println!("     {}. {:?} {} - {}", j + 1, ep.method, ep.name, ep.url);
        }
    }
    
    // 5. Edit the collection
    println!("\n5. Editing collection name...");
    let mut updated_collection = collection.clone();
    updated_collection.name = "Updated Demo API".to_string();
    storage.save_collection(&updated_collection)?;
    println!("   ✓ Collection name updated to: {}", updated_collection.name);
    
    // 6. Remove an endpoint
    println!("\n6. Removing an endpoint...");
    let endpoint_to_remove = updated_collection.endpoints[1].id;
    updated_collection.remove_endpoint(&endpoint_to_remove);
    storage.save_collection(&updated_collection)?;
    println!("   ✓ Endpoint removed");
    println!("   Remaining endpoints: {}", updated_collection.endpoints.len());
    
    // 7. Delete the collection
    println!("\n7. Deleting collection...");
    storage.delete_collection(&updated_collection.id)?;
    println!("   ✓ Collection deleted");
    
    // 8. Verify deletion
    println!("\n8. Verifying deletion...");
    let final_collections = storage.load_collections()?;
    println!("   ✓ Total collections now: {}", final_collections.len());
    
    println!("\n=== Demo Complete ===");
    println!("\nAll collection management operations work correctly!");
    println!("You can now use these features in the TUI:");
    println!("  - Press 'n' to create new collections/endpoints");
    println!("  - Press 'e' to edit selected items");
    println!("  - Press 'd' to delete selected items");
    
    Ok(())
}
