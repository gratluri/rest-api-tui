# Variable Substitution - Current Implementation Status

## Overview
The REST API TUI currently has a **basic template variable substitution system** implemented, but it is **NOT exposed in the TUI interface**. Variables can only be used programmatically through the API, not through the user interface.

## What Has Been Implemented

### 1. Template Engine (`src/template.rs`)
A complete template engine with the following features:

#### Syntax
- Variables use double curly braces: `{{variable_name}}`
- Example: `https://api.example.com/users/{{user_id}}/posts/{{post_id}}`

#### Functions Available
1. **`find_variables(template: &str) -> Vec<String>`**
   - Finds all variable names in a template string
   - Example: `"Hello {{name}}, age {{age}}"` → `["name", "age"]`

2. **`substitute(template: &str, variables: &HashMap<String, String>) -> Result<String>`**
   - Substitutes variables with values
   - Returns error if any variable is missing
   - Example: `"Hello {{name}}"` with `{"name": "Alice"}` → `"Hello Alice"`

3. **`substitute_lenient(template: &str, variables: &HashMap<String, String>) -> String`**
   - Substitutes variables, uses empty string for missing variables
   - Never returns error
   - Example: `"Hello {{name}}"` with `{}` → `"Hello "`

4. **`has_variables(template: &str) -> bool`**
   - Checks if a string contains any template variables
   - Example: `"Hello {{name}}"` → `true`

#### Features
✅ Handles whitespace in variable names: `{{ name }}` → `name`  
✅ Supports multiple variables in one string  
✅ Works with URLs, headers, body templates  
✅ Handles nested braces correctly: `{not a var}` vs `{{var}}`  
✅ Validates template syntax (detects unclosed variables)  
✅ Comprehensive test coverage  

### 2. HTTP Client Integration (`src/http.rs`)
The HTTP client fully supports variable substitution in:

#### Request Components
1. **URLs**: `https://api.example.com/users/{{user_id}}`
2. **Headers**: `Authorization: Bearer {{token}}`
3. **Body Templates**: `{"name": "{{name}}", "age": {{age}}}`
4. **Authentication**:
   - Bearer tokens: `{{access_token}}`
   - Basic auth: username/password can use variables
   - API keys: name and value can use variables

#### RequestInputs Structure
```rust
pub struct RequestInputs {
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<String>,
    pub variables: HashMap<String, String>,  // ← Variables stored here
}
```

#### Substitution Order
1. URL is substituted first
2. Authentication is applied (with variable substitution)
3. Headers are merged and substituted
4. Body template is substituted
5. Request is executed

### 3. Examples and Demos
Several example files demonstrate variable substitution:

- **`examples/http_demo.rs`**: Shows URL and body template variables
- **`examples/template_demo.rs`**: Demonstrates template engine features
- **`examples/full_app_demo.rs`**: Shows complete workflow with variables
- **`examples/storage_demo.rs`**: Shows variables in stored endpoints

## What Has NOT Been Implemented

### ❌ TUI Interface for Variables
**CRITICAL MISSING FEATURE**: There is NO user interface to:
- Define variables
- Edit variable values
- View available variables
- Select which variables to use
- Manage variable sets/environments

### ❌ Variable Storage
Variables are NOT persisted:
- No storage in JSON files
- No environment/variable sets
- No variable history
- Variables must be provided programmatically each time

### ❌ Environment Management
No concept of environments:
- No Dev/Staging/Prod environments
- No environment switching
- No environment-specific variable sets
- No default values per environment

### ❌ Variable Discovery
No UI to show:
- Which variables are used in an endpoint
- Which variables are missing
- Variable validation before request
- Variable suggestions/autocomplete

### ❌ Variable Scope
No variable scoping:
- No global variables (shared across all endpoints)
- No collection-level variables
- No endpoint-level variables
- All variables must be provided at request time

## Current Usage (Programmatic Only)

### Example: Using Variables in Code
```rust
use rest_api_tui::http::{HttpClient, RequestInputs};
use rest_api_tui::models::ApiEndpoint;
use std::collections::HashMap;

// Create endpoint with template variables
let endpoint = ApiEndpoint {
    url: "https://api.example.com/users/{{user_id}}/posts/{{post_id}}".to_string(),
    body_template: Some(r#"{"title": "{{title}}", "body": "{{body}}"}"#.to_string()),
    // ... other fields
};

// Provide variable values
let mut inputs = RequestInputs::default();
inputs.variables.insert("user_id".to_string(), "123".to_string());
inputs.variables.insert("post_id".to_string(), "456".to_string());
inputs.variables.insert("title".to_string(), "My Post".to_string());
inputs.variables.insert("body".to_string(), "Post content".to_string());

// Execute request (variables are substituted automatically)
let client = HttpClient::new()?;
let response = client.execute(&endpoint, &inputs).await?;
```

### What Users CANNOT Do
Users cannot:
1. Define variables in the TUI
2. Edit variable values in the TUI
3. See which variables are needed
4. Save variable sets
5. Switch between environments
6. Use variables when executing requests from the TUI

## What Needs to Be Implemented

### Phase 1: Basic Variable UI (High Priority)
1. **Variable Definition Screen**
   - Add/edit/delete variables
   - Key-value pairs
   - Validation

2. **Variable Input Before Request**
   - Detect variables in endpoint
   - Prompt for values before execution
   - Remember last used values

3. **Variable Display**
   - Show variables in endpoint detail view
   - Highlight variables in URLs/bodies
   - Show which variables are missing

### Phase 2: Environment Management (Medium Priority)
1. **Environment Concept**
   - Create/edit/delete environments
   - Switch between environments
   - Environment-specific variable sets

2. **Variable Storage**
   - Save variables to JSON
   - Load variables on startup
   - Per-environment storage

3. **Variable Scope**
   - Global variables (all endpoints)
   - Collection variables (per collection)
   - Endpoint variables (per endpoint)

### Phase 3: Advanced Features (Low Priority)
1. **Variable Discovery**
   - Auto-detect variables in templates
   - Show missing variables
   - Suggest variable names

2. **Variable Validation**
   - Required vs optional
   - Type validation (string, number, etc.)
   - Format validation (URL, email, etc.)

3. **Variable History**
   - Recent values
   - Autocomplete from history
   - Clear history

4. **Variable Import/Export**
   - Import from .env files
   - Export to .env files
   - Import from Postman environments

## Comparison with Other Tools

### Postman
✅ Has: Environment management, variable UI, global/collection/environment scope  
❌ REST API TUI: None of these features in UI

### Insomnia
✅ Has: Environment management, variable UI, base environments  
❌ REST API TUI: None of these features in UI

### HTTPie
✅ Has: Session-based variable storage  
❌ REST API TUI: No session concept

## Technical Architecture

### Current Architecture
```
Template Engine (template.rs)
    ↓
HTTP Client (http.rs)
    ↓
RequestInputs { variables: HashMap }
    ↓
Substitution happens automatically
```

### Proposed Architecture (with UI)
```
Variable Storage (new: variables.rs)
    ↓
Variable Manager (new: manages environments, scopes)
    ↓
TUI Variable Screen (new: ui for editing)
    ↓
Request Execution (prompts for missing variables)
    ↓
HTTP Client (existing: substitutes variables)
```

## Files That Would Need Changes

### New Files Needed
1. `src/variables.rs` - Variable storage and management
2. `src/tui/variables_ui.rs` - Variable editing UI
3. `src/tui/environment_ui.rs` - Environment management UI

### Existing Files to Modify
1. `src/models.rs` - Add variable/environment models
2. `src/storage.rs` - Add variable persistence
3. `src/tui_app.rs` - Add variable state and methods
4. `src/tui/ui.rs` - Add variable screens and prompts

## Estimated Implementation Effort

### Phase 1: Basic Variable UI
- Variable definition screen: 8-10 hours
- Variable input before request: 4-6 hours
- Variable display: 2-3 hours
- **Total**: 14-19 hours

### Phase 2: Environment Management
- Environment CRUD: 6-8 hours
- Variable storage: 4-6 hours
- Variable scope: 6-8 hours
- **Total**: 16-22 hours

### Phase 3: Advanced Features
- Variable discovery: 4-6 hours
- Variable validation: 6-8 hours
- Variable history: 4-6 hours
- Import/export: 8-10 hours
- **Total**: 22-30 hours

### Grand Total: 52-71 hours

## Conclusion

The REST API TUI has a **solid foundation** for variable substitution with a complete template engine and HTTP client integration. However, it is **completely missing the user interface** to make this feature accessible to users.

**Current Status**: ⚠️ **Implemented but Not Usable**
- ✅ Template engine works perfectly
- ✅ HTTP client supports variables
- ❌ No UI to define/manage variables
- ❌ No storage for variables
- ❌ No environment management

**Priority**: 🔴 **HIGH** - This is a critical feature for any API testing tool. Most users expect environment/variable management as a core feature.

**Recommendation**: Implement Phase 1 (Basic Variable UI) as the next major feature to make the existing variable substitution system usable through the TUI.
