# Developer Guide

Welcome to the REST API TUI development guide! This document will help you get started with contributing to the project.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Project Structure](#project-structure)
4. [Building and Running](#building-and-running)
5. [Testing](#testing)
6. [Code Style](#code-style)
7. [Adding Features](#adding-features)
8. [Debugging](#debugging)
9. [Contributing](#contributing)

---

## Getting Started

### Prerequisites

- **Rust**: 1.70 or higher
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Git**: For version control
  ```bash
  git --version
  ```

- **Terminal**: Modern terminal with Unicode support
  - macOS: iTerm2, Terminal.app
  - Linux: GNOME Terminal, Alacritty, Kitty
  - Windows: Windows Terminal, Alacritty

### Clone the Repository

```bash
git clone https://github.com/yourusername/rest-api-tui.git
cd rest-api-tui
```

---

## Development Setup

### 1. Install Dependencies

```bash
# Update Rust
rustup update

# Install cargo-watch for auto-recompilation
cargo install cargo-watch

# Install cargo-edit for dependency management
cargo install cargo-edit
```

### 2. Build the Project

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### 3. Run the Application

```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release

# Run with logging
RUST_LOG=debug cargo run
```

### 4. Set Up Sample Data

```bash
# Create sample collections
./setup_sample_data.sh
```

---

## Project Structure

```
rest-api-tui/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library exports
│   ├── models.rs            # Data structures
│   ├── storage.rs           # Persistence
│   ├── http.rs              # HTTP client
│   ├── formatter.rs         # Response formatting
│   ├── template.rs          # Variable substitution
│   ├── load_test.rs         # Load testing
│   ├── tui_app.rs           # App state & logic
│   ├── tui.rs               # TUI module
│   └── tui/
│       ├── app.rs           # TUI initialization
│       └── ui.rs            # UI rendering
├── examples/                # Demo applications
├── tests/                   # Integration tests
├── Cargo.toml               # Dependencies
├── README.md                # User documentation
├── ARCHITECTURE.md          # Architecture docs
├── DEVELOPER.md             # This file
└── ERGONOMIC_IMPROVEMENTS.md # Feature ideas
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

---

## Building and Running

### Development Workflow

```bash
# Watch for changes and auto-rebuild
cargo watch -x run

# Watch and run tests
cargo watch -x test

# Watch and check (no build)
cargo watch -x check
```

### Build Profiles

```bash
# Debug (fast compile, slow runtime)
cargo build

# Release (slow compile, fast runtime)
cargo build --release

# Custom profile (in Cargo.toml)
cargo build --profile custom
```

### Running Examples

```bash
# List all examples
cargo run --example

# Run specific example
cargo run --example full_app_demo
cargo run --example http_demo
cargo run --example storage_demo
```

---

## Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_collection_creation

# Run tests with output
cargo test -- --nocapture

# Run tests in specific module
cargo test models::tests
```

### Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test storage_integration
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

### Writing Tests

**Unit Test Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_creation() {
        let endpoint = ApiEndpoint {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
            method: HttpMethod::GET,
            url: "https://example.com".to_string(),
            headers: HashMap::new(),
            body_template: None,
            auth: None,
            description: None,
        };
        
        assert_eq!(endpoint.name, "Test");
        assert_eq!(endpoint.method, HttpMethod::GET);
    }
}
```

**Async Test Example**:
```rust
#[tokio::test]
async fn test_http_request() {
    let client = HttpClient::new().unwrap();
    let endpoint = create_test_endpoint();
    let inputs = RequestInputs::default();
    
    let result = client.execute(&endpoint, &inputs).await;
    assert!(result.is_ok());
}
```

---

## Code Style

### Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### Naming Conventions

- **Types**: `PascalCase` (e.g., `ApiEndpoint`, `HttpMethod`)
- **Functions**: `snake_case` (e.g., `execute_request`, `save_collection`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_TIMEOUT`)
- **Modules**: `snake_case` (e.g., `tui_app`, `load_test`)

### Documentation

```rust
/// Executes an HTTP request with the given endpoint and inputs.
///
/// # Arguments
///
/// * `endpoint` - The API endpoint to execute
/// * `inputs` - User-provided values for the request
///
/// # Returns
///
/// Returns `Ok(HttpResponse)` on success, or `Err(HttpError)` on failure.
///
/// # Examples
///
/// ```
/// let client = HttpClient::new()?;
/// let endpoint = ApiEndpoint { /* ... */ };
/// let inputs = RequestInputs::default();
/// let response = client.execute(&endpoint, &inputs).await?;
/// ```
pub async fn execute(
    &self,
    endpoint: &ApiEndpoint,
    inputs: &RequestInputs,
) -> Result<HttpResponse> {
    // Implementation
}
```

---

## Adding Features

### Step-by-Step Guide

#### 1. Plan the Feature

- Read [ERGONOMIC_IMPROVEMENTS.md](ERGONOMIC_IMPROVEMENTS.md) for ideas
- Create a GitHub issue describing the feature
- Discuss design with maintainers

#### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

#### 3. Implement the Feature

**Example: Adding a "Duplicate Endpoint" feature**

**Step 1: Add to models.rs (if needed)**
```rust
// No changes needed - using existing types
```

**Step 2: Add business logic to tui_app.rs**
```rust
impl AppState {
    pub fn duplicate_endpoint(&mut self, coll_idx: usize, ep_idx: usize) {
        if let Some(collection) = self.collections.get_mut(coll_idx) {
            if let Some(endpoint) = collection.endpoints.get(ep_idx) {
                let mut new_endpoint = endpoint.clone();
                new_endpoint.id = Uuid::new_v4();
                new_endpoint.name = format!("{} (Copy)", endpoint.name);
                
                collection.add_endpoint(new_endpoint);
                
                match self.storage.save_collection(collection) {
                    Ok(_) => {
                        self.status_message = Some("Endpoint duplicated".to_string());
                    }
                    Err(e) => {
                        self.error_message = Some(format!("Failed: {}", e));
                    }
                }
            }
        }
    }
}
```

**Step 3: Add keyboard shortcut in tui/ui.rs**
```rust
fn handle_input(app: &mut AppState, key: KeyEvent) -> io::Result<bool> {
    match app.current_screen {
        Screen::EndpointList(_) => {
            match key.code {
                KeyCode::Char('D') => {
                    // Duplicate endpoint
                    let coll_idx = app.selected_collection_index;
                    let ep_idx = app.selected_endpoint_index;
                    app.duplicate_endpoint(coll_idx, ep_idx);
                }
                // ... other keys
            }
        }
        // ... other screens
    }
}
```

**Step 4: Update UI to show shortcut**
```rust
fn draw_endpoint_list(f: &mut Frame, app: &AppState, area: Rect) {
    // ... existing code
    
    let help_text = "n: new | e: edit | d: delete | D: duplicate | Enter: select";
    // ... render help text
}
```

**Step 5: Write tests**
```rust
#[test]
fn test_duplicate_endpoint() {
    let mut app = AppState::new().unwrap();
    // Create collection and endpoint
    // Call duplicate_endpoint
    // Assert new endpoint exists
}
```

**Step 6: Update documentation**
- Add to README.md keyboard shortcuts table
- Add to FEATURE_SUMMARY.md
- Update CHANGELOG.md

#### 4. Test Your Changes

```bash
# Run tests
cargo test

# Run the app
cargo run

# Test manually
# - Create a collection
# - Add an endpoint
# - Press 'D' to duplicate
# - Verify duplicate appears
```

#### 5. Commit Your Changes

```bash
git add .
git commit -m "feat: add duplicate endpoint feature"
```

#### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

---

## Debugging

### 1. Enable Logging

```rust
// Add to Cargo.toml
[dependencies]
env_logger = "0.11"
log = "0.4"

// Add to main.rs
fn main() {
    env_logger::init();
    // ... rest of main
}

// Use in code
log::debug!("Current screen: {:?}", app.current_screen);
log::info!("Request executed successfully");
log::error!("Failed to save collection: {}", e);
```

Run with logging:
```bash
RUST_LOG=debug cargo run
RUST_LOG=rest_api_tui=debug cargo run  # Only this crate
```

### 2. Debug Prints

```rust
// Temporary debug prints
dbg!(&app.current_screen);
eprintln!("Debug: {:?}", some_value);
```

### 3. Terminal Debugging

If terminal gets corrupted:
```bash
reset
# or
stty sane
```

### 4. Inspect Storage

```bash
# View collections
cat ~/.rest-api-tui/collections/*.json | jq .

# Pretty print
jq . ~/.rest-api-tui/collections/some-uuid.json

# List all
ls -la ~/.rest-api-tui/collections/
```

### 5. Use Examples for Testing

```bash
# Test HTTP client
cargo run --example http_demo

# Test storage
cargo run --example storage_demo

# Test full app
cargo run --example full_app_demo
```

### 6. Debugger (LLDB/GDB)

```bash
# Install rust-lldb
rustup component add lldb-preview

# Debug
rust-lldb target/debug/rest-api-tui

# Set breakpoint
(lldb) b main.rs:42
(lldb) run
```

---

## Contributing

### Contribution Workflow

1. **Fork** the repository
2. **Clone** your fork
3. **Create** a feature branch
4. **Make** your changes
5. **Test** thoroughly
6. **Commit** with clear messages
7. **Push** to your fork
8. **Create** a Pull Request

### Commit Message Format

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(http): add request timeout configuration

Add timeout field to ApiEndpoint and implement in HttpClient.
Defaults to 30 seconds.

Closes #123
```

```
fix(ui): resolve scrolling bug in response panel

The last line of responses was not visible due to off-by-one
error in scroll calculation.

Fixes #456
```

### Pull Request Guidelines

**PR Title**: Same format as commit messages

**PR Description**:
```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- Added X feature
- Fixed Y bug
- Updated Z documentation

## Testing
- [ ] Unit tests pass
- [ ] Manual testing completed
- [ ] Documentation updated

## Screenshots (if UI changes)
[Add screenshots here]

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### Code Review Process

1. **Automated checks** run (tests, clippy, fmt)
2. **Maintainer review** (1-2 business days)
3. **Address feedback** (if any)
4. **Approval** and merge

### What Makes a Good PR?

✅ **Good**:
- Small, focused changes
- Clear description
- Tests included
- Documentation updated
- Follows code style

❌ **Bad**:
- Large, unfocused changes
- No description
- No tests
- Breaking changes without discussion
- Doesn't follow code style

---

## Common Tasks

### Adding a New Keyboard Shortcut

1. Update `handle_input()` in `tui/ui.rs`
2. Add logic in `tui_app.rs`
3. Update help screen in `draw_help_screen()`
4. Update README.md keyboard shortcuts table

### Adding a New Screen

1. Add variant to `Screen` enum in `tui_app.rs`
2. Add rendering function in `tui/ui.rs` (e.g., `draw_my_screen()`)
3. Add input handling in `handle_input()`
4. Add navigation logic in `AppState`

### Adding a New Data Field

1. Update struct in `models.rs`
2. Update serialization (usually automatic with Serde)
3. Update forms in `tui_app.rs`
4. Update UI rendering in `tui/ui.rs`
5. Update tests

### Adding a New HTTP Feature

1. Update `HttpClient` in `http.rs`
2. Update `ApiEndpoint` in `models.rs` (if needed)
3. Add tests
4. Update examples

---

## Performance Tips

### 1. Profile Your Code

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph
```

### 2. Benchmark

```bash
# Add to Cargo.toml
[dev-dependencies]
criterion = "0.5"

# Create benchmark
# benches/my_benchmark.rs

# Run benchmarks
cargo bench
```

### 3. Optimize Builds

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## Troubleshooting

### Build Errors

**Error**: `error: linker 'cc' not found`
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc
```

**Error**: `error: failed to run custom build command for 'openssl-sys'`
```bash
# macOS
brew install openssl

# Ubuntu/Debian
sudo apt install libssl-dev pkg-config

# Fedora
sudo dnf install openssl-devel
```

### Runtime Errors

**Error**: Terminal not restored after crash
```bash
reset
```

**Error**: Collections not loading
```bash
# Check permissions
ls -la ~/.rest-api-tui/collections/

# Check JSON validity
jq . ~/.rest-api-tui/collections/*.json
```

---

## Resources

### Documentation

- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Tutorial](https://ratatui.rs/tutorial/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Reqwest Docs](https://docs.rs/reqwest/)

### Tools

- [Rust Analyzer](https://rust-analyzer.github.io/) - IDE support
- [Cargo Watch](https://github.com/watchexec/cargo-watch) - Auto-rebuild
- [Cargo Edit](https://github.com/killercup/cargo-edit) - Dependency management

### Community

- [Rust Discord](https://discord.gg/rust-lang)
- [Ratatui Discord](https://discord.gg/pMCEU9hNEj)
- [GitHub Discussions](https://github.com/yourusername/rest-api-tui/discussions)

---

## Getting Help

- **Bug Reports**: [GitHub Issues](https://github.com/yourusername/rest-api-tui/issues)
- **Feature Requests**: [GitHub Discussions](https://github.com/yourusername/rest-api-tui/discussions)
- **Questions**: [GitHub Discussions](https://github.com/yourusername/rest-api-tui/discussions)
- **Chat**: [Discord Server](https://discord.gg/your-server)

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Happy coding! 🦀**

