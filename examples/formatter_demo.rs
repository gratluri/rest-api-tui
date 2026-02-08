use rest_api_tui::formatter::{format_json, format_xml, format_plain_text, format_auto, is_json, is_xml};

fn main() {
    println!("=== REST API TUI - Response Formatter Demo ===\n");
    
    // Example 1: Format JSON
    println!("📝 Example 1: JSON Formatting");
    let json_compact = r#"{"name":"Alice","age":30,"address":{"city":"NYC","zip":"10001"},"hobbies":["reading","coding"]}"#;
    println!("Original (compact):");
    println!("{}\n", json_compact);
    
    let json_formatted = format_json(json_compact.as_bytes()).unwrap();
    println!("Formatted (pretty):");
    println!("{}\n", json_formatted);
    
    // Example 2: JSON idempotence
    println!("📝 Example 2: JSON Formatting Idempotence");
    let json_formatted_again = format_json(json_formatted.as_bytes()).unwrap();
    println!("Formatted twice produces same result: {}\n", json_formatted == json_formatted_again);
    
    // Example 3: Format XML
    println!("📝 Example 3: XML Formatting");
    let xml_compact = r#"<root><user><name>Bob</name><email>bob@example.com</email></user><user><name>Alice</name><email>alice@example.com</email></user></root>"#;
    println!("Original (compact):");
    println!("{}\n", xml_compact);
    
    let xml_formatted = format_xml(xml_compact.as_bytes()).unwrap();
    println!("Formatted (indented):");
    println!("{}\n", xml_formatted);
    
    // Example 4: XML with self-closing tags
    println!("📝 Example 4: XML with Self-Closing Tags");
    let xml_self_closing = r#"<root><item id="1"/><item id="2"/><item id="3"/></root>"#;
    println!("Original:");
    println!("{}\n", xml_self_closing);
    
    let xml_sc_formatted = format_xml(xml_self_closing.as_bytes()).unwrap();
    println!("Formatted:");
    println!("{}\n", xml_sc_formatted);
    
    // Example 5: Plain text
    println!("📝 Example 5: Plain Text (Pass-Through)");
    let plain_text = "Hello, World!\nThis is plain text.\nNo formatting needed.";
    println!("Original:");
    println!("{}\n", plain_text);
    
    let plain_formatted = format_plain_text(plain_text.as_bytes()).unwrap();
    println!("Formatted (unchanged):");
    println!("{}\n", plain_formatted);
    
    // Example 6: Auto-detection
    println!("📝 Example 6: Auto-Detection and Formatting");
    
    let test_cases = vec![
        (r#"{"status":"ok","count":42}"#, "JSON"),
        (r#"<response><status>ok</status></response>"#, "XML"),
        ("Just plain text response", "Plain Text"),
    ];
    
    for (content, expected_type) in test_cases {
        println!("\nContent: {}", content);
        println!("Detected as: {}", expected_type);
        
        let formatted = format_auto(content.as_bytes()).unwrap();
        println!("Formatted:");
        println!("{}", formatted);
    }
    println!();
    
    // Example 7: Content type detection
    println!("📝 Example 7: Content Type Detection");
    
    let test_contents = vec![
        (r#"{"key":"value"}"#, "JSON"),
        (r#"[1,2,3]"#, "JSON"),
        (r#"true"#, "JSON"),
        (r#"<root></root>"#, "XML"),
        (r#"<?xml version="1.0"?><data/>"#, "XML"),
        ("Plain text", "Plain Text"),
        ("Not JSON or XML", "Plain Text"),
    ];
    
    for (content, expected) in test_contents {
        let is_j = is_json(content.as_bytes());
        let is_x = is_xml(content.as_bytes());
        
        let detected = if is_j {
            "JSON"
        } else if is_x {
            "XML"
        } else {
            "Plain Text"
        };
        
        println!("  '{}' → {} (expected: {})", 
            if content.len() > 30 { &content[..30] } else { content },
            detected,
            expected
        );
    }
    println!();
    
    // Example 8: Real API response formatting
    println!("📝 Example 8: Real API Response Formatting");
    
    let api_response = r#"{"userId":1,"id":1,"title":"delectus aut autem","completed":false}"#;
    println!("API Response (compact):");
    println!("{}\n", api_response);
    
    let formatted_response = format_json(api_response.as_bytes()).unwrap();
    println!("Formatted for display:");
    println!("{}\n", formatted_response);
    
    // Example 9: Nested JSON
    println!("📝 Example 9: Complex Nested JSON");
    
    let complex_json = r#"{"user":{"id":123,"profile":{"name":"John Doe","settings":{"theme":"dark","notifications":true}}},"posts":[{"id":1,"title":"First Post"},{"id":2,"title":"Second Post"}]}"#;
    println!("Complex JSON (compact):");
    println!("{}\n", complex_json);
    
    let complex_formatted = format_json(complex_json.as_bytes()).unwrap();
    println!("Formatted:");
    println!("{}\n", complex_formatted);
    
    // Example 10: Error handling
    println!("📝 Example 10: Error Handling");
    
    let invalid_json = r#"{"invalid": json syntax}"#;
    println!("Invalid JSON: {}", invalid_json);
    
    match format_json(invalid_json.as_bytes()) {
        Ok(_) => println!("  ✓ Formatted successfully"),
        Err(e) => println!("  ✗ Error: {}", e),
    }
    println!();
    
    println!("=== Demo Complete ===");
    println!("\n✨ Key Features:");
    println!("  • JSON pretty printing with proper indentation");
    println!("  • XML formatting with nested tag support");
    println!("  • Plain text pass-through");
    println!("  • Auto-detection of content type");
    println!("  • Idempotent formatting (format twice = same result)");
    println!("  • Error handling for invalid content");
}
