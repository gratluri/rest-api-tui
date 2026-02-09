# Quick Start Guide

Get up and running with REST API TUI in 5 minutes.

## For Users

### 1. Install Rust (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and Build
```bash
git clone https://github.com/yourusername/rest-api-tui.git
cd rest-api-tui
cargo build --release
```

### 3. Run
```bash
./target/release/rest-api-tui
```

### 4. Create Your First Request

**Step 1**: Press `n` to create a collection
- Name: "JSONPlaceholder"
- Press Enter

**Step 2**: Press `Ctrl+l` to switch to endpoints panel

**Step 3**: Press `n` to create an endpoint
- Name: "Get Users"
- Method: Press `m` until GET
- URL: `https://jsonplaceholder.typicode.com/users`
- Press Enter

**Step 4**: Press Enter to select the endpoint

**Step 5**: Press `e` to execute the request

**Step 6**: View the response in the bottom panel
- Press `t` to toggle network traffic view
- Press `PageDown` to scroll through response

### 5. Essential Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Ctrl+h` | Switch to Collections panel |
| `Ctrl+l` | Switch to Endpoints panel |
| `n` | New collection/endpoint |
| `e` | Edit or Execute |
| `d` | Delete (with confirmation) |
| `t` | Toggle network traffic |
| `PageUp/PageDown` | Scroll response |
| `?` | Show help |
| `q` | Quit |

### 6. Next Steps

- Read [README.md](README.md) for full documentation
- Check [FEATURE_SUMMARY.md](FEATURE_SUMMARY.md) for recent features
- See [ERGONOMIC_IMPROVEMENTS.md](ERGONOMIC_IMPROVEMENTS.md) for upcoming features

---

## For Developers

### 1. Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install cargo-watch
cargo install cargo-edit
```

### 2. Clone and Setup
```bash
git clone https://github.com/yourusername/rest-api-tui.git
cd rest-api-tui
cargo build
```

### 3. Run Tests
```bash
cargo test
```

### 4. Run in Development Mode
```bash
# Auto-rebuild on changes
cargo watch -x run

# With logging
RUST_LOG=debug cargo run
```

### 5. Make Your First Change

**Example: Add a status message**

Edit `src/tui_app.rs`:
```rust
pub fn execute_request(&mut self, coll_idx: usize, ep_idx: usize) {
    // Add this line
    self.status_message = Some("🚀 Launching request...".to_string());
    
    // ... existing code
}
```

Run and test:
```bash
cargo run
```

### 6. Run Examples
```bash
# Full app demo
cargo run --example full_app_demo

# HTTP client demo
cargo run --example http_demo

# Storage demo
cargo run --example storage_demo
```

### 7. Code Style
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### 8. Next Steps

- Read [DEVELOPER.md](DEVELOPER.md) for detailed guide
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for technical details
- Check [QUICK_IMPROVEMENTS.md](QUICK_IMPROVEMENTS.md) for feature ideas
- Join discussions on GitHub

---

## Common Issues

### Issue: Build fails with "linker 'cc' not found"

**Solution**:
```bash
# macOS
xcode-select --install

# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc
```

### Issue: Terminal corrupted after crash

**Solution**:
```bash
reset
```

### Issue: Collections not loading

**Solution**:
```bash
# Check storage directory
ls -la ~/.rest-api-tui/collections/

# Validate JSON
jq . ~/.rest-api-tui/collections/*.json
```

---

## Quick Reference

### Project Structure
```
rest-api-tui/
├── src/
│   ├── main.rs          # Entry point
│   ├── models.rs        # Data structures
│   ├── storage.rs       # Persistence
│   ├── http.rs          # HTTP client
│   ├── tui_app.rs       # App state
│   └── tui/ui.rs        # UI rendering
├── examples/            # Demo apps
├── README.md            # User docs
├── DEVELOPER.md         # Dev guide
└── ARCHITECTURE.md      # Technical docs
```

### Key Files to Edit

**Adding a feature**:
1. `src/models.rs` - Data structures
2. `src/tui_app.rs` - Business logic
3. `src/tui/ui.rs` - UI rendering

**Adding a keyboard shortcut**:
1. `src/tui/ui.rs` - `handle_input()` function

**Adding a screen**:
1. `src/tui_app.rs` - Add to `Screen` enum
2. `src/tui/ui.rs` - Add `draw_*_screen()` function

### Testing Commands
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_collection_creation

# Run with output
cargo test -- --nocapture

# Run examples
cargo run --example full_app_demo
```

### Build Commands
```bash
# Debug build (fast compile)
cargo build

# Release build (optimized)
cargo build --release

# Check without building
cargo check

# Auto-rebuild on changes
cargo watch -x run
```

---

## Resources

### Documentation
- [README.md](README.md) - User guide
- [DEVELOPER.md](DEVELOPER.md) - Developer guide
- [ARCHITECTURE.md](ARCHITECTURE.md) - Technical architecture
- [ERGONOMIC_IMPROVEMENTS.md](ERGONOMIC_IMPROVEMENTS.md) - Feature ideas

### External Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Tutorial](https://ratatui.rs/tutorial/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Community
- GitHub Issues - Bug reports
- GitHub Discussions - Questions
- Discord - Chat (if available)

---

## What's Next?

### For Users
1. ✅ Create your first collection
2. ✅ Execute a request
3. ✅ Explore network traffic view
4. ✅ Try load testing
5. ✅ Add custom headers
6. ⏭️ Import from Postman (coming soon)
7. ⏭️ Use environment variables (coming soon)

### For Developers
1. ✅ Set up development environment
2. ✅ Run tests
3. ✅ Make a small change
4. ✅ Read architecture docs
5. ⏭️ Pick a feature from QUICK_IMPROVEMENTS.md
6. ⏭️ Submit a pull request
7. ⏭️ Become a contributor

---

**Ready to dive deeper?**

- **Users**: Read [README.md](README.md)
- **Developers**: Read [DEVELOPER.md](DEVELOPER.md)
- **Contributors**: Check [QUICK_IMPROVEMENTS.md](QUICK_IMPROVEMENTS.md)

**Happy coding! 🦀**

