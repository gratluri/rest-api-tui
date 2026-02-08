use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Missing variable: {0}")]
    MissingVariable(String),
    
    #[error("Invalid template syntax: {0}")]
    InvalidSyntax(String),
}

pub type Result<T> = std::result::Result<T, TemplateError>;

/// Find all template variables in a string (e.g., {{variable_name}})
pub fn find_variables(template: &str) -> Vec<String> {
    let mut variables = Vec::new();
    let mut chars = template.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // consume second '{'
                    
                    // Extract variable name
                    let mut var_name = String::new();
                    let mut found_closing = false;
                    
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            if let Some(&next) = chars.peek() {
                                if next == '}' {
                                    chars.next(); // consume second '}'
                                    found_closing = true;
                                    break;
                                }
                            }
                        }
                        var_name.push(c);
                    }
                    
                    if found_closing && !var_name.is_empty() {
                        variables.push(var_name.trim().to_string());
                    }
                }
            }
        }
    }
    
    variables
}

/// Substitute template variables with values from the provided map
/// Returns an error if any variable is missing
pub fn substitute(template: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // consume second '{'
                    
                    // Extract variable name
                    let mut var_name = String::new();
                    let mut found_closing = false;
                    
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            if let Some(&next) = chars.peek() {
                                if next == '}' {
                                    chars.next(); // consume second '}'
                                    found_closing = true;
                                    break;
                                }
                            }
                        }
                        var_name.push(c);
                    }
                    
                    if found_closing {
                        let var_name = var_name.trim();
                        if let Some(value) = variables.get(var_name) {
                            result.push_str(value);
                        } else {
                            return Err(TemplateError::MissingVariable(var_name.to_string()));
                        }
                    } else {
                        // Unclosed template variable
                        return Err(TemplateError::InvalidSyntax(
                            format!("Unclosed template variable: {{{{{}", var_name)
                        ));
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    Ok(result)
}

/// Substitute template variables, using empty string for missing variables
pub fn substitute_lenient(template: &str, variables: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next(); // consume second '{'
                    
                    // Extract variable name
                    let mut var_name = String::new();
                    let mut found_closing = false;
                    
                    while let Some(c) = chars.next() {
                        if c == '}' {
                            if let Some(&next) = chars.peek() {
                                if next == '}' {
                                    chars.next(); // consume second '}'
                                    found_closing = true;
                                    break;
                                }
                            }
                        }
                        var_name.push(c);
                    }
                    
                    if found_closing {
                        let var_name = var_name.trim();
                        if let Some(value) = variables.get(var_name) {
                            result.push_str(value);
                        }
                        // If variable not found, just skip it (empty string)
                    } else {
                        // Unclosed template, keep original
                        result.push_str("{{");
                        result.push_str(&var_name);
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

/// Check if a string contains any template variables
pub fn has_variables(template: &str) -> bool {
    template.contains("{{") && template.contains("}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_variables() {
        let template = "Hello {{name}}, your age is {{age}}";
        let vars = find_variables(template);
        assert_eq!(vars, vec!["name", "age"]);
    }

    #[test]
    fn test_find_variables_with_spaces() {
        let template = "{{ name }} and {{ age }}";
        let vars = find_variables(template);
        assert_eq!(vars, vec!["name", "age"]);
    }

    #[test]
    fn test_find_variables_none() {
        let template = "No variables here";
        let vars = find_variables(template);
        assert_eq!(vars.len(), 0);
    }

    #[test]
    fn test_substitute_success() {
        let template = "Hello {{name}}, your age is {{age}}";
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("age".to_string(), "30".to_string());
        
        let result = substitute(template, &vars).unwrap();
        assert_eq!(result, "Hello Alice, your age is 30");
    }

    #[test]
    fn test_substitute_missing_variable() {
        let template = "Hello {{name}}";
        let vars = HashMap::new();
        
        let result = substitute(template, &vars);
        assert!(result.is_err());
        match result {
            Err(TemplateError::MissingVariable(var)) => assert_eq!(var, "name"),
            _ => panic!("Expected MissingVariable error"),
        }
    }

    #[test]
    fn test_substitute_with_url() {
        let template = "https://api.example.com/users/{{user_id}}/posts/{{post_id}}";
        let mut vars = HashMap::new();
        vars.insert("user_id".to_string(), "123".to_string());
        vars.insert("post_id".to_string(), "456".to_string());
        
        let result = substitute(template, &vars).unwrap();
        assert_eq!(result, "https://api.example.com/users/123/posts/456");
    }

    #[test]
    fn test_substitute_lenient() {
        let template = "Hello {{name}}, your age is {{age}}";
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        
        let result = substitute_lenient(template, &vars);
        assert_eq!(result, "Hello Alice, your age is ");
    }

    #[test]
    fn test_has_variables() {
        assert!(has_variables("Hello {{name}}"));
        assert!(!has_variables("Hello world"));
        assert!(!has_variables("Hello { name }"));
    }

    #[test]
    fn test_nested_braces() {
        let template = "{{var}} and {not a var} and {{another}}";
        let mut vars = HashMap::new();
        vars.insert("var".to_string(), "value1".to_string());
        vars.insert("another".to_string(), "value2".to_string());
        
        let result = substitute(template, &vars).unwrap();
        assert_eq!(result, "value1 and {not a var} and value2");
    }

    #[test]
    fn test_empty_template() {
        let template = "";
        let vars = HashMap::new();
        let result = substitute(template, &vars).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_variables() {
        let template = "Just plain text";
        let vars = HashMap::new();
        let result = substitute(template, &vars).unwrap();
        assert_eq!(result, "Just plain text");
    }
}
