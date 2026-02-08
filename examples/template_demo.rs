use rest_api_tui::template::{find_variables, substitute, substitute_lenient, has_variables};
use std::collections::HashMap;

fn main() {
    println!("=== REST API TUI - Template Engine Demo ===\n");
    
    // Example 1: Simple variable substitution
    println!("📝 Example 1: Simple Variable Substitution");
    let template1 = "Hello {{name}}, welcome to {{app_name}}!";
    println!("Template: {}", template1);
    
    let vars1 = find_variables(template1);
    println!("Variables found: {:?}", vars1);
    
    let mut values1 = HashMap::new();
    values1.insert("name".to_string(), "Alice".to_string());
    values1.insert("app_name".to_string(), "REST API TUI".to_string());
    
    let result1 = substitute(template1, &values1).unwrap();
    println!("Result: {}\n", result1);
    
    // Example 2: URL with path parameters
    println!("📝 Example 2: API URL with Path Parameters");
    let template2 = "https://api.github.com/repos/{{owner}}/{{repo}}/issues/{{issue_number}}";
    println!("Template: {}", template2);
    
    let vars2 = find_variables(template2);
    println!("Variables found: {:?}", vars2);
    
    let mut values2 = HashMap::new();
    values2.insert("owner".to_string(), "rust-lang".to_string());
    values2.insert("repo".to_string(), "rust".to_string());
    values2.insert("issue_number".to_string(), "12345".to_string());
    
    let result2 = substitute(template2, &values2).unwrap();
    println!("Result: {}\n", result2);
    
    // Example 3: JSON body template
    println!("📝 Example 3: JSON Request Body Template");
    let template3 = r#"{
  "username": "{{username}}",
  "email": "{{email}}",
  "role": "{{role}}",
  "active": true
}"#;
    println!("Template:\n{}", template3);
    
    let mut values3 = HashMap::new();
    values3.insert("username".to_string(), "john_doe".to_string());
    values3.insert("email".to_string(), "john@example.com".to_string());
    values3.insert("role".to_string(), "admin".to_string());
    
    let result3 = substitute(template3, &values3).unwrap();
    println!("Result:\n{}\n", result3);
    
    // Example 4: Authentication header
    println!("📝 Example 4: Authentication Header Template");
    let template4 = "Bearer {{access_token}}";
    println!("Template: {}", template4);
    
    let mut values4 = HashMap::new();
    values4.insert("access_token".to_string(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...".to_string());
    
    let result4 = substitute(template4, &values4).unwrap();
    println!("Result: {}\n", result4);
    
    // Example 5: Missing variable (error case)
    println!("📝 Example 5: Missing Variable (Error Handling)");
    let template5 = "Hello {{name}}, your ID is {{user_id}}";
    println!("Template: {}", template5);
    
    let mut values5 = HashMap::new();
    values5.insert("name".to_string(), "Bob".to_string());
    // Note: user_id is missing
    
    match substitute(template5, &values5) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("❌ Error: {}", e),
    }
    println!();
    
    // Example 6: Lenient substitution (missing variables become empty)
    println!("📝 Example 6: Lenient Substitution (Missing Variables → Empty String)");
    let template6 = "Hello {{name}}, your ID is {{user_id}}";
    println!("Template: {}", template6);
    
    let mut values6 = HashMap::new();
    values6.insert("name".to_string(), "Bob".to_string());
    
    let result6 = substitute_lenient(template6, &values6);
    println!("Result: '{}' (note: user_id is empty)\n", result6);
    
    // Example 7: Complex URL with query parameters
    println!("📝 Example 7: Complex URL with Query Parameters");
    let template7 = "https://api.example.com/search?q={{query}}&page={{page}}&limit={{limit}}&sort={{sort_by}}";
    println!("Template: {}", template7);
    
    let mut values7 = HashMap::new();
    values7.insert("query".to_string(), "rust programming".to_string());
    values7.insert("page".to_string(), "1".to_string());
    values7.insert("limit".to_string(), "20".to_string());
    values7.insert("sort_by".to_string(), "relevance".to_string());
    
    let result7 = substitute(template7, &values7).unwrap();
    println!("Result: {}\n", result7);
    
    // Example 8: Checking for variables
    println!("📝 Example 8: Checking if String Contains Variables");
    let test_strings = vec![
        "https://api.example.com/users/{{id}}",
        "https://api.example.com/users/123",
        "No variables here",
        "{{var1}} and {{var2}}",
    ];
    
    for s in test_strings {
        println!("  '{}' → has variables: {}", s, has_variables(s));
    }
    println!();
    
    // Example 9: Nested braces (not variables)
    println!("📝 Example 9: Handling Nested Braces");
    let template9 = "{{var}} and {not a var} and {{another}}";
    println!("Template: {}", template9);
    
    let mut values9 = HashMap::new();
    values9.insert("var".to_string(), "value1".to_string());
    values9.insert("another".to_string(), "value2".to_string());
    
    let result9 = substitute(template9, &values9).unwrap();
    println!("Result: {} (single braces preserved)\n", result9);
    
    println!("=== Demo Complete ===");
    println!("\n✨ Key Features:");
    println!("  • Find all variables in a template");
    println!("  • Substitute variables with values");
    println!("  • Error handling for missing variables");
    println!("  • Lenient mode for optional variables");
    println!("  • Works with URLs, JSON, headers, and more!");
}
