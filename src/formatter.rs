use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatterError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("XML parse error: {0}")]
    XmlParse(String),
    
    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, FormatterError>;

/// Format JSON with pretty printing and syntax highlighting markers
pub fn format_json(json_bytes: &[u8]) -> Result<String> {
    // Try to parse as JSON
    let json_str = String::from_utf8(json_bytes.to_vec())?;
    let value: serde_json::Value = serde_json::from_str(&json_str)?;
    
    // Pretty print with 2-space indentation
    let formatted = serde_json::to_string_pretty(&value)?;
    
    Ok(formatted)
}

/// Format JSON with syntax highlighting using ANSI color codes
pub fn format_json_with_colors(json_bytes: &[u8]) -> Result<String> {
    let formatted = format_json(json_bytes)?;
    
    // Add ANSI color codes for syntax highlighting
    // This is a simple implementation - a full implementation would use a proper JSON parser
    let mut result = String::new();
    let mut in_string = false;
    let mut chars = formatted.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_string = !in_string;
                if in_string {
                    // Start of string - check if it's a key or value
                    let mut lookahead = String::new();
                    let mut temp_chars = chars.clone();
                    while let Some(next) = temp_chars.next() {
                        if next == '"' {
                            break;
                        }
                        lookahead.push(next);
                    }
                    
                    // Check if followed by colon (it's a key)
                    let mut after_quote = chars.clone();
                    after_quote.next(); // skip the closing quote
                    while let Some(next) = after_quote.peek() {
                        if *next == ':' {
                            // It's a key - use cyan
                            result.push_str("\x1b[36m\"");
                            continue;
                        } else if !next.is_whitespace() {
                            break;
                        }
                        after_quote.next();
                    }
                    
                    // It's a value - use green
                    result.push_str("\x1b[32m\"");
                } else {
                    result.push('"');
                    result.push_str("\x1b[0m"); // Reset color
                }
            }
            't' | 'f' if !in_string => {
                // Boolean values
                if formatted[result.len()..].starts_with("true") || formatted[result.len()..].starts_with("false") {
                    result.push_str("\x1b[33m"); // Yellow for booleans
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
            'n' if !in_string => {
                // null value
                if formatted[result.len()..].starts_with("null") {
                    result.push_str("\x1b[90m"); // Gray for null
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
            '0'..='9' | '-' if !in_string => {
                // Numbers
                result.push_str("\x1b[35m"); // Magenta for numbers
                result.push(c);
                
                // Continue with the rest of the number
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' || next == 'e' || next == 'E' || next == '-' || next == '+' {
                        result.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                result.push_str("\x1b[0m"); // Reset color
            }
            _ => {
                result.push(c);
            }
        }
    }
    
    Ok(result)
}

/// Check if content is valid JSON
pub fn is_json(content: &[u8]) -> bool {
    if let Ok(s) = std::str::from_utf8(content) {
        serde_json::from_str::<serde_json::Value>(s).is_ok()
    } else {
        false
    }
}

/// Format XML with proper indentation
pub fn format_xml(xml_bytes: &[u8]) -> Result<String> {
    let xml_str = String::from_utf8(xml_bytes.to_vec())?;
    
    // Simple XML formatter - adds indentation
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let mut in_tag = false;
    let mut is_closing_tag = false;
    let mut is_self_closing = false;
    let mut chars = xml_str.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '<' => {
                // Check if it's a closing tag
                if let Some(&'/') = chars.peek() {
                    is_closing_tag = true;
                    indent_level = indent_level.saturating_sub(1);
                    
                    // Add newline and indentation before closing tag
                    if !formatted.is_empty() && !formatted.ends_with('\n') {
                        formatted.push('\n');
                        formatted.push_str(&"  ".repeat(indent_level));
                    }
                } else if !formatted.is_empty() && !formatted.ends_with('\n') {
                    // Add newline and indentation before opening tag
                    formatted.push('\n');
                    formatted.push_str(&"  ".repeat(indent_level));
                }
                
                in_tag = true;
                formatted.push(c);
            }
            '>' => {
                formatted.push(c);
                in_tag = false;
                
                // Check if it was a self-closing tag
                if is_self_closing {
                    is_self_closing = false;
                } else if !is_closing_tag {
                    indent_level += 1;
                }
                
                is_closing_tag = false;
            }
            '/' if in_tag => {
                // Check if it's a self-closing tag
                if let Some(&'>') = chars.peek() {
                    is_self_closing = true;
                }
                formatted.push(c);
            }
            '\n' | '\r' if !in_tag => {
                // Skip existing newlines outside tags
                continue;
            }
            ' ' if !in_tag && formatted.ends_with('>') => {
                // Skip leading spaces after tags
                continue;
            }
            _ => {
                formatted.push(c);
            }
        }
    }
    
    Ok(formatted)
}

/// Check if content is likely XML
pub fn is_xml(content: &[u8]) -> bool {
    if let Ok(s) = std::str::from_utf8(content) {
        let trimmed = s.trim();
        trimmed.starts_with('<') && trimmed.contains('>')
    } else {
        false
    }
}

/// Format plain text (pass through as-is)
pub fn format_plain_text(text_bytes: &[u8]) -> Result<String> {
    Ok(String::from_utf8(text_bytes.to_vec())?)
}

/// Auto-detect content type and format accordingly
pub fn format_auto(content: &[u8]) -> Result<String> {
    if is_json(content) {
        format_json(content)
    } else if is_xml(content) {
        format_xml(content)
    } else {
        format_plain_text(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_simple() {
        let json = r#"{"name":"Alice","age":30}"#;
        let formatted = format_json(json.as_bytes()).unwrap();
        
        assert!(formatted.contains("\"name\""));
        assert!(formatted.contains("\"Alice\""));
        assert!(formatted.contains("\"age\""));
        assert!(formatted.contains("30"));
        assert!(formatted.contains('\n')); // Should have newlines
    }

    #[test]
    fn test_format_json_nested() {
        let json = r#"{"user":{"name":"Bob","address":{"city":"NYC"}}}"#;
        let formatted = format_json(json.as_bytes()).unwrap();
        
        assert!(formatted.contains("\"user\""));
        assert!(formatted.contains("\"name\""));
        assert!(formatted.contains("\"address\""));
        assert!(formatted.contains("\"city\""));
    }

    #[test]
    fn test_format_json_array() {
        let json = r#"[1,2,3]"#;
        let formatted = format_json(json.as_bytes()).unwrap();
        
        assert!(formatted.contains("1"));
        assert!(formatted.contains("2"));
        assert!(formatted.contains("3"));
    }

    #[test]
    fn test_format_json_invalid() {
        let json = r#"{"invalid": json}"#;
        let result = format_json(json.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn test_format_json_idempotent() {
        let json = r#"{"name":"Alice","age":30}"#;
        let formatted1 = format_json(json.as_bytes()).unwrap();
        let formatted2 = format_json(formatted1.as_bytes()).unwrap();
        
        // Formatting twice should produce the same result
        assert_eq!(formatted1, formatted2);
    }

    #[test]
    fn test_is_json_valid() {
        assert!(is_json(r#"{"key":"value"}"#.as_bytes()));
        assert!(is_json(r#"[1,2,3]"#.as_bytes()));
        assert!(is_json(r#"true"#.as_bytes()));
        assert!(is_json(r#"null"#.as_bytes()));
        assert!(is_json(r#"123"#.as_bytes()));
    }

    #[test]
    fn test_is_json_invalid() {
        assert!(!is_json(b"not json"));
        assert!(!is_json(b"{invalid}"));
        assert!(!is_json(b"<xml>"));
    }

    #[test]
    fn test_format_json_with_special_chars() {
        let json = r#"{"message":"Hello\nWorld","emoji":"😀"}"#;
        let formatted = format_json(json.as_bytes()).unwrap();
        
        assert!(formatted.contains("Hello\\nWorld"));
        assert!(formatted.contains("😀"));
    }
    
    #[test]
    fn test_format_xml_simple() {
        let xml = r#"<root><name>Alice</name><age>30</age></root>"#;
        let formatted = format_xml(xml.as_bytes()).unwrap();
        
        assert!(formatted.contains("<root>"));
        assert!(formatted.contains("<name>"));
        assert!(formatted.contains("</name>"));
        assert!(formatted.contains('\n')); // Should have newlines
    }
    
    #[test]
    fn test_format_xml_nested() {
        let xml = r#"<root><user><name>Bob</name></user></root>"#;
        let formatted = format_xml(xml.as_bytes()).unwrap();
        
        assert!(formatted.contains("<root>"));
        assert!(formatted.contains("<user>"));
        assert!(formatted.contains("<name>"));
    }
    
    #[test]
    fn test_format_xml_self_closing() {
        let xml = r#"<root><item/><item/></root>"#;
        let formatted = format_xml(xml.as_bytes()).unwrap();
        
        assert!(formatted.contains("<item/>"));
    }
    
    #[test]
    fn test_is_xml_valid() {
        assert!(is_xml(b"<root></root>"));
        assert!(is_xml(b"<item/>"));
        assert!(is_xml(b"<?xml version=\"1.0\"?><root/>"));
    }
    
    #[test]
    fn test_is_xml_invalid() {
        assert!(!is_xml(b"not xml"));
        assert!(!is_xml(b"{\"json\":true}"));
    }
    
    #[test]
    fn test_format_plain_text() {
        let text = b"Hello, World!\nThis is plain text.";
        let formatted = format_plain_text(text).unwrap();
        
        assert_eq!(formatted, "Hello, World!\nThis is plain text.");
    }
    
    #[test]
    fn test_format_auto_json() {
        let json = r#"{"key":"value"}"#;
        let formatted = format_auto(json.as_bytes()).unwrap();
        
        assert!(formatted.contains("\"key\""));
        assert!(formatted.contains('\n')); // Should be formatted
    }
    
    #[test]
    fn test_format_auto_xml() {
        let xml = b"<root><item>test</item></root>";
        let formatted = format_auto(xml).unwrap();
        
        assert!(formatted.contains("<root>"));
        assert!(formatted.contains('\n')); // Should be formatted
    }
    
    #[test]
    fn test_format_auto_plain() {
        let text = b"Just plain text";
        let formatted = format_auto(text).unwrap();
        
        assert_eq!(formatted, "Just plain text");
    }
}
