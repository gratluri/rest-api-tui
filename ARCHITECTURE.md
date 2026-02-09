# Architecture Documentation

This document explains the internal structure and design of the REST API TUI application.

## Overview

REST API TUI is built using Rust with the Ratatui framework for terminal UI rendering. The application follows a modular architecture with clear separation of concerns.

## Technology Stack

- **Language**: Rust 1.70+
- **UI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- **HTTP Client**: [Reqwest](https://github.com/seanmonstar/reqwest) - Async HTTP client
- **Async Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **Error Handling**: [Thiserror](https://github.com/dtolnay/thiserror) - Error derive macros

## Project Structure

```
rest-api-tui/
├── src/
│   ├── main.rs              # Entry point, event loop
│   ├── lib.rs               # Library exports
│   ├── models.rs            # Data structures
│   ├── storage.rs           # Persistence layer
│   ├── http.rs              # HTTP client
│   ├── formatter.rs         # Response formatting
│   ├── template.rs          # Variable substitution
│   ├── load_test.rs         # Load testing engine
│   ├── tui_app.rs           # Application state & logic
│   ├── tui.rs               # TUI module exports
│   └── tui/
│       ├── app.rs           # TUI initialization
│       └── ui.rs            # UI rendering & input handling
├── examples/                # Demo applications
├── Cargo.toml               # Dependencies
└── README.md                # User documentation
```

## Module Breakdown

### 1. `models.rs` - Data Structures

**Purpose**: Define core data types used throughout the application.

**Key Types**:
- `HttpMethod`: Enum for HTTP methods (GET, POST, PUT, etc.)
- `AuthConfig`: Authentication configuration (Bearer, Basic, API Key)
- `ApiEndpoint`: Single API endpoint definition
- `ApiCollection`: Collection of related endpoints

**Design Decisions**:
- Uses `Uuid` for unique identifiers
- Implements `Serialize`/`Deserialize` for JSON persistence
- Uses `HashMap` for flexible header storage
- Optional fields (`Option<T>`) for nullable data

**Example**:
```rust
pub struct ApiEndpoint {
    pub id: Uuid,
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body_template: Option<String>,
    pub auth: Option<AuthConfig>,
    pub description: Option<String>,
}
```

---

### 2. `storage.rs` - Persistence Layer

**Purpose**: Handle saving and loading collections from disk.

**Key Components**:
- `StorageManager`: Main storage interface
- `StorageError`: Custom error type for storage operations

**Storage Location**:
- Default: `~/.rest-api-tui/collections/`
- Each collection is a separate JSON file
- File name: `{collection_id}.json`

**Operations**:
- `save_collection()`: Write collection to disk
- `load_collections()`: Read all collections from disk
- `delete_collection()`: Remove collection file

**Design Decisions**:
- File-based storage (simple, no database needed)
- One file per collection (easy to backup/share)
- JSON format (human-readable, editable)
- Creates directories automatically

**Error Handling**:
- IO errors (file not found, permission denied)
- JSON parsing errors (corrupted files)
- Custom error types with `thiserror`

---

### 3. `http.rs` - HTTP Client

**Purpose**: Execute HTTP requests with detailed timing and traffic capture.

**Key Components**:
- `HttpClient`: Wrapper around `reqwest::Client`
- `RequestInputs`: User-provided values for request
- `HttpResponse`: Response data with timing
- `NetworkTiming`: Detailed timing breakdown
- `NetworkTraffic`: Complete traffic analysis data

**Features**:
- Async request execution
- Template variable substitution
- Authentication handling (Bearer, Basic, API Key)
- Custom headers
- Request/response body capture
- Detailed timing (request sent, waiting, download)
- Transfer size calculation

**Design Decisions**:
- Uses `reqwest` for HTTP (mature, well-tested)
- Captures timing at multiple stages
- Stores raw response bytes (for formatting later)
- Async/await for non-blocking requests

**Timing Capture**:
```rust
let start = Instant::now();
let response = client.send(request).await?;
let headers_received = Instant::now();
let body = response.bytes().await?;
let end = Instant::now();

NetworkTiming {
    request_sent: headers_received - start,
    waiting: headers_received - start,
    content_download: end - headers_received,
    total: end - start,
}
```

---

### 4. `formatter.rs` - Response Formatting

**Purpose**: Pretty-print responses based on content type.

**Supported Formats**:
- JSON: Pretty-printed with indentation
- XML: Formatted with proper structure
- Plain text: Displayed as-is

**Key Functions**:
- `format_auto()`: Auto-detect format and apply formatting
- `format_json()`: JSON-specific formatting
- `format_xml()`: XML-specific formatting

**Design Decisions**:
- Detects format from response bytes
- Falls back to plain text if parsing fails
- Uses `serde_json` for JSON formatting
- Handles UTF-8 and non-UTF-8 responses

---

### 5. `template.rs` - Variable Substitution

**Purpose**: Replace template variables in URLs, headers, and bodies.

**Syntax**: `{{variable_name}}`

**Key Functions**:
- `substitute()`: Replace all variables in a string
- `extract_variables()`: Find all variables in a template

**Design Decisions**:
- Simple regex-based substitution
- Currently substitutes with empty string (placeholder)
- Designed for future environment variable support

**Example**:
```rust
let template = "https://api.example.com/users/{{userId}}";
let variables = HashMap::from([("userId", "123")]);
let result = substitute(template, &variables);
// result: "https://api.example.com/users/123"
```

---

### 6. `load_test.rs` - Load Testing Engine

**Purpose**: Execute concurrent requests for performance testing.

**Key Components**:
- `LoadTestEngine`: Manages concurrent request execution
- `LoadTestConfig`: Configuration (concurrency, duration)
- `LoadTestMetrics`: Real-time statistics

**Features**:
- Configurable concurrency level
- Time-based duration
- Real-time metrics (RPS, success/failure counts)
- Thread-safe metric updates (Arc<Mutex>)

**Design Decisions**:
- Uses Tokio for async concurrency
- Atomic operations for thread-safe counters
- Separate thread for engine (non-blocking)
- Stop signal for graceful shutdown

**Metrics Tracked**:
- Total requests
- Successful requests
- Failed requests
- Current RPS (requests per second)
- Elapsed time

---

### 7. `tui_app.rs` - Application State

**Purpose**: Manage application state and business logic.

**Key Components**:
- `AppState`: Main application state
- `Screen`: Enum for different UI screens
- `PanelFocus`: Which panel is currently focused
- `CollectionForm`: Form state for collection editing
- `EndpointForm`: Form state for endpoint editing

**State Management**:
- Centralized state in `AppState` struct
- Screen navigation with `Screen` enum
- Panel focus tracking for split-panel layout
- Form state for editing operations

**Key Operations**:
- Navigation: `navigate_up()`, `navigate_down()`, `toggle_panel_focus()`
- CRUD: `save_collection()`, `delete_endpoint()`, etc.
- Request execution: `execute_request()`, `start_load_test()`
- Scrolling: `scroll_response_up()`, `scroll_response_down()`

**Design Decisions**:
- Single source of truth for state
- Immutable state updates (clone and replace)
- Async operations for HTTP requests
- Error messages stored in state (displayed in UI)

---

### 8. `tui/ui.rs` - UI Rendering

**Purpose**: Render the terminal UI and handle keyboard input.

**Key Functions**:
- `run_app()`: Main event loop
- `draw_ui()`: Render current screen
- `handle_input()`: Process keyboard events
- `draw_*_panel()`: Render specific panels

**UI Layout**:
```
┌─────────────────────────────────────────────────────────────┐
│ Title Bar                                                   │
├──────────────────────────────┬──────────────────────────────┤
│ API Definition Panel (65%)   │ Collections Panel (35%)      │
│                              │                              │
│ [Focused with cyan border]   │ [Unfocused with gray border] │
│                              │                              │
├──────────────────────────────┴──────────────────────────────┤
│ Response Panel (65%)                                        │
│                                                             │
│ [Scrollable with PageUp/PageDown]                          │
│                                                             │
├──────────────────────────────────────────────────────────────┤
│ Status Bar                                                  │
└──────────────────────────────────────────────────────────────┘
```

**Keyboard Handling**:
- Event-driven input processing
- Different handlers for different screens
- Modal dialogs (confirmation, help)
- Form input with field navigation

**Design Decisions**:
- Uses Ratatui's `Layout` for responsive sizing
- Color-coded borders for focus indication
- Scrolling with offset tracking
- Async request execution (non-blocking UI)

---

### 9. `main.rs` - Entry Point

**Purpose**: Initialize application and start event loop.

**Flow**:
1. Initialize terminal (raw mode, alternate screen)
2. Create `AppState`
3. Run event loop (`run_app()`)
4. Handle errors and cleanup
5. Restore terminal state

**Error Handling**:
- Catches panics and restores terminal
- Displays error messages before exit
- Ensures terminal is always restored

---

## Data Flow

### 1. Request Execution Flow

```
User Input (press 'e')
    ↓
handle_input() in ui.rs
    ↓
execute_request() in tui_app.rs
    ↓
HttpClient.execute() in http.rs
    ↓
Template substitution (template.rs)
    ↓
HTTP request (reqwest)
    ↓
Response capture with timing
    ↓
Format response (formatter.rs)
    ↓
Update AppState (last_response)
    ↓
UI re-render (draw_response_panel)
    ↓
Display to user
```

### 2. Collection Save Flow

```
User Input (press Enter in form)
    ↓
handle_input() in ui.rs
    ↓
save_collection() in tui_app.rs
    ↓
Validate form data
    ↓
Create/update ApiCollection
    ↓
StorageManager.save_collection() in storage.rs
    ↓
Serialize to JSON (serde)
    ↓
Write to file (~/.rest-api-tui/collections/)
    ↓
Update AppState.collections
    ↓
UI re-render
    ↓
Display success message
```

### 3. Load Test Flow

```
User Input (press 'l')
    ↓
start_load_test() in tui_app.rs
    ↓
Create LoadTestEngine (load_test.rs)
    ↓
Spawn background thread
    ↓
Execute concurrent requests
    ↓
Update metrics (atomic operations)
    ↓
UI polls metrics (get_load_test_metrics)
    ↓
Render metrics in UI
    ↓
User stops test (press Esc)
    ↓
Stop signal sent
    ↓
Thread cleanup
```

---

## State Management

### AppState Structure

```rust
pub struct AppState {
    // Data
    pub collections: Vec<ApiCollection>,
    pub last_response: Option<HttpResponse>,
    pub last_response_formatted: Option<String>,
    
    // UI State
    pub current_screen: Screen,
    pub panel_focus: PanelFocus,
    pub selected_collection_index: usize,
    pub selected_endpoint_index: usize,
    pub response_scroll_offset: usize,
    pub show_network_traffic: bool,
    
    // Forms
    pub collection_form: Option<CollectionForm>,
    pub endpoint_form: Option<EndpointForm>,
    
    // Messages
    pub status_message: Option<String>,
    pub error_message: Option<String>,
    
    // Services
    pub storage: StorageManager,
    pub http_client: HttpClient,
    pub load_test_engine: Option<LoadTestEngine>,
}
```

### Screen Navigation

```rust
pub enum Screen {
    CollectionList,
    CollectionEdit(Option<usize>),
    EndpointList(usize),
    EndpointEdit(usize, Option<usize>),
    EndpointDetail(usize, usize),
    ResponseView(usize, usize),
    LoadTestConfig(usize, usize),
    LoadTestRunning(usize, usize),
    ConfirmDelete(DeleteTarget),
    Help,
}
```

---

## Error Handling Strategy

### Custom Error Types

Each module defines its own error type:

```rust
// storage.rs
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// http.rs
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Template error: {0}")]
    Template(#[from] template::TemplateError),
}
```

### Error Propagation

- Errors bubble up with `?` operator
- Converted to user-friendly messages in `AppState`
- Displayed in status bar or error dialog
- Never panic (except for unrecoverable errors)

---

## Testing Strategy

### Unit Tests

Each module has unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collection_creation() {
        let collection = ApiCollection::new("Test".to_string());
        assert_eq!(collection.name, "Test");
        assert_eq!(collection.endpoints.len(), 0);
    }
}
```

### Integration Tests

Examples serve as integration tests:
- `examples/full_app_demo.rs`: Full application flow
- `examples/http_demo.rs`: HTTP client testing
- `examples/storage_demo.rs`: Storage operations

### Running Tests

```bash
# Run all unit tests
cargo test

# Run specific test
cargo test test_collection_creation

# Run with output
cargo test -- --nocapture

# Run examples
cargo run --example full_app_demo
```

---

## Performance Considerations

### 1. Async Operations

- HTTP requests are async (non-blocking)
- UI remains responsive during requests
- Load tests use concurrent tasks

### 2. Memory Management

- Collections loaded into memory (fast access)
- Responses stored until next request
- Old responses dropped (no memory leak)

### 3. File I/O

- Collections saved on every change (durability)
- Lazy loading possible for large collections
- JSON parsing is fast for typical collection sizes

### 4. UI Rendering

- Only re-render on state change
- Efficient terminal updates (Ratatui optimization)
- Scrolling uses offset (no re-rendering entire response)

---

## Security Considerations

### 1. Credential Storage

- Currently stored in plain text JSON
- **TODO**: Encrypt sensitive data (tokens, passwords)
- **TODO**: Use system keychain integration

### 2. HTTPS

- Reqwest validates SSL certificates by default
- Can be disabled for testing (not recommended)

### 3. Input Validation

- URL validation before request
- Header validation (no invalid characters)
- JSON body validation (syntax check)

---

## Future Architecture Changes

### 1. Plugin System

- Allow custom formatters
- Custom authentication methods
- Custom request processors

### 2. Database Backend

- SQLite for better query performance
- Full-text search across collections
- Request history storage

### 3. Multi-User Support

- Shared collections (team collaboration)
- User-specific settings
- Permission system

### 4. Cloud Sync

- Sync collections across devices
- Backup to cloud storage
- Version control for collections

---

## Development Guidelines

### Adding a New Feature

1. **Define data structures** in `models.rs`
2. **Add business logic** in `tui_app.rs`
3. **Implement UI** in `tui/ui.rs`
4. **Add keyboard shortcuts** in `handle_input()`
5. **Write tests** in module
6. **Update documentation** (README, ARCHITECTURE)

### Code Style

- Follow Rust conventions (rustfmt)
- Use descriptive variable names
- Add doc comments for public APIs
- Keep functions small and focused
- Prefer immutability

### Commit Messages

- Use conventional commits format
- Examples:
  - `feat: add environment variables support`
  - `fix: resolve scrolling bug in response panel`
  - `docs: update architecture documentation`
  - `refactor: simplify state management`

---

## Debugging Tips

### 1. Enable Logging

```rust
// Add to main.rs
env_logger::init();

// Use in code
log::debug!("Current state: {:?}", app.current_screen);
```

### 2. Terminal Issues

If terminal is corrupted after crash:
```bash
reset
# or
stty sane
```

### 3. Inspect Storage

```bash
# View collections
cat ~/.rest-api-tui/collections/*.json | jq .

# List all collections
ls -la ~/.rest-api-tui/collections/
```

### 4. Test HTTP Requests

```bash
# Use examples to test HTTP client
cargo run --example http_demo
```

---

## Dependencies

### Core Dependencies

```toml
[dependencies]
ratatui = "0.26"           # Terminal UI
crossterm = "0.27"         # Terminal control
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
dirs = "5.0"
```

### Why These Dependencies?

- **Ratatui**: Best-in-class terminal UI library
- **Crossterm**: Cross-platform terminal control
- **Tokio**: Industry-standard async runtime
- **Reqwest**: Most popular HTTP client
- **Serde**: De facto serialization standard
- **Uuid**: Unique identifiers for collections/endpoints
- **Chrono**: Date/time handling
- **Thiserror**: Ergonomic error handling
- **Dirs**: Cross-platform directory paths

---

## Conclusion

This architecture provides:

- **Modularity**: Clear separation of concerns
- **Testability**: Each module can be tested independently
- **Extensibility**: Easy to add new features
- **Performance**: Async operations, efficient rendering
- **Maintainability**: Clean code, good documentation

For questions or suggestions, please open an issue on GitHub.

