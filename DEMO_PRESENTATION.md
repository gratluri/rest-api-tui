# REST API TUI - Team Demo Presentation

## Slide 1: Title
**REST API TUI**  
*A Powerful Terminal-Based API Testing Tool*

🚀 Built with Rust + Ratatui  
⌨️ 100% Keyboard-Driven  
🎨 Modern Split-Panel Interface

---

## Slide 2: The Problem
**Current API Testing Challenges:**
- Postman/Insomnia are heavy GUI applications
- Slow to start, resource-intensive
- Not scriptable or automation-friendly
- Can't use in SSH/remote environments
- Mouse-dependent workflows

**What if we could test APIs as fast as we code?**

---

## Slide 3: The Solution
**REST API TUI brings Postman power to the terminal:**

✅ Lightning-fast startup (< 1 second)  
✅ Keyboard-driven (Vim-style navigation)  
✅ Works over SSH  
✅ Scriptable and automation-ready  
✅ Minimal resource usage  
✅ Modern, intuitive interface  

---

## Slide 4: Core Features

### 🗂️ Collection Management
- Organize endpoints by collection
- Persistent storage (JSON files)
- Full CRUD operations from TUI

### 🌐 HTTP Request Support
- All methods: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- Custom headers with inline editor
- Request body templates
- Authentication (Bearer, Basic, API Key)

### 📊 Response Management
- Automatic formatting (JSON, XML, plain text)
- Syntax highlighting
- Vertical scrolling for large responses
- Network traffic analysis (Wireshark-style)

---

## Slide 5: NEW - Variable Management

**Problem:** Repeating the same values across multiple requests

**Solution:** User-defined variables with persistent storage

```
Press 'v' → Create variables → Use anywhere
```

**Syntax:** `{{VARIABLE_NAME}}`

**Example:**
```
URL: {{BASE_URL}}/users/{{USER_ID}}
Headers: Authorization: Bearer {{AUTH_TOKEN}}
Body: {"email": "{{USER_EMAIL}}"}
```

**Benefits:**
- Define once, use everywhere
- Stored in `~/.rest-api-tui/variables.json`
- Persists across sessions
- Easy to update (change once, affects all endpoints)

---

## Slide 6: NEW - Faker Integration

**Problem:** Creating realistic test data is tedious

**Solution:** 50+ dynamic data generators built-in

**Syntax:** `{{f:variablename}}`

**Example:**
```json
{
  "name": "{{f:fullname}}",
  "email": "{{f:email}}",
  "phone": "{{f:phone}}",
  "company": "{{f:company}}",
  "address": {
    "city": "{{f:city}}",
    "zipcode": "{{f:zipcode}}"
  }
}
```

**Magic:** Each execution generates NEW realistic data!

**Available Generators:**
- Names: firstname, lastname, fullname
- Internet: email, username, url, ipv4
- Phone: phone, cellnumber
- Address: street, city, state, zipcode
- Company: company, industry
- IDs: uuid, number
- Dates: date, datetime, time
- Text: word, sentence, paragraph

---

## Slide 7: NEW - Quick Execute Mode

**Problem:** Testing requires too many keystrokes

**Solution:** Two execution modes

### Traditional Execute (`e` key)
- Shows variable input screen
- Review/edit values before execution
- Good for: Careful testing, changing values

### Quick Execute (`x` key)
- Instant execution, no prompts
- Uses saved variable values
- Faker variables generate fresh data
- Good for: Rapid testing, iteration

**Result:** 3x faster testing workflow!

---

## Slide 8: UX Improvements

### Collapsible Sections (Space key)
- Collapse/expand headers and traffic
- Focus on what matters
- Visual indicators (▶ collapsed, ▼ expanded)

### Scrollable Headers (Shift+PageUp/PageDown)
- Navigate through long header lists
- Scroll indicator shows position
- Independent from response scrolling

### Clipboard Support (`y` key)
- Copy response with one keystroke
- Cross-platform (macOS, Linux, Windows)
- Share results instantly

---

## Slide 9: Load Testing

**Built-in load testing capabilities:**

- Configure concurrency and duration
- Real-time metrics (RPS, success/failure rates)
- Response time statistics
- Visual progress indicators
- Ramp-up support for gradual load increase

**Perfect for:**
- Quick performance checks
- Stress testing endpoints
- Validating rate limits

---

## Slide 10: Keyboard Efficiency

**Vim-Style Navigation:**
- `Ctrl+h/l` - Switch panels
- `Ctrl+j/k` - Navigate items
- `j/k` or `↑/↓` - Move up/down
- No mouse required!

**Quick Actions:**
- `n` - New collection/endpoint
- `e` - Edit or execute
- `d` - Delete (with confirmation)
- `v` - Variable manager
- `x` - Quick execute
- `y` - Copy response
- `t` - Toggle traffic view
- `?` - Show help

**Hands never leave home row!**

---

## Slide 11: Live Demo

**Demo Flow:**
1. Create collection and endpoint
2. Execute basic request
3. Create variables (BASE_URL, USER_ID)
4. Use variables in endpoint
5. Create POST endpoint with faker variables
6. Quick execute multiple times (show new data)
7. Show network traffic analysis
8. Copy response to clipboard
9. Run quick load test

**Time:** 5-7 minutes

---

## Slide 12: Technical Highlights

**Built With:**
- **Rust** - Memory-safe, blazingly fast
- **Ratatui** - Modern terminal UI framework
- **Reqwest** - Robust HTTP client
- **Tokio** - Async runtime for concurrency
- **Fake** - Realistic data generation
- **Serde** - JSON serialization

**Architecture:**
- Clean separation of concerns
- Async request execution
- Persistent storage (JSON files)
- Cross-platform clipboard support

---

## Slide 13: Use Cases

### Development
- Test APIs during development
- Validate request/response formats
- Debug authentication issues

### Testing
- Create test data with faker
- Run load tests
- Validate edge cases

### DevOps
- Test APIs in production (SSH)
- Monitor endpoint health
- Automate API checks

### Documentation
- Share collection files with team
- Document API usage
- Create reproducible examples

---

## Slide 14: Comparison

| Feature | REST API TUI | Postman | cURL |
|---------|--------------|---------|------|
| Startup Time | < 1s | ~5-10s | < 1s |
| Memory Usage | ~10MB | ~500MB | ~5MB |
| Keyboard-Driven | ✅ | ❌ | ✅ |
| GUI | TUI | GUI | CLI |
| Collections | ✅ | ✅ | ❌ |
| Variables | ✅ | ✅ | ❌ |
| Faker Data | ✅ | ❌ | ❌ |
| Load Testing | ✅ | ✅ | ❌ |
| Works over SSH | ✅ | ❌ | ✅ |
| Scriptable | ✅ | ⚠️ | ✅ |

---

## Slide 15: Roadmap

### Recently Completed ✅
- Variable management
- Faker integration
- Quick execute mode
- Clipboard support
- Collapsible sections
- Scrollable headers

### Coming Soon 🚀
- Import/Export (cURL, Postman, HTTPie)
- Environment variables (dev, staging, prod)
- Request history
- Authentication UI
- Search across collections
- GraphQL support
- WebSocket support
- Request chaining

---

## Slide 16: Getting Started

### Installation
```bash
git clone https://github.com/gratluri/rest-api-tui.git
cd rest-api-tui
cargo build --release
./target/release/rest-api-tui
```

### Quick Start
1. Press `n` to create a collection
2. Press `Ctrl+l` then `n` to add an endpoint
3. Press `Enter` then `e` to execute
4. Press `?` for help anytime

### Documentation
- README.md - Full documentation
- CHEATSHEET.md - Keyboard shortcuts
- FAKER_FEATURE.md - All faker variables

---

## Slide 17: Demo Time! 🎬

**[Show asciinema recording or live demo]**

Key moments to highlight:
1. Fast startup
2. Creating endpoint with variables
3. Faker generating realistic data
4. Quick execute speed
5. Network traffic analysis
6. Load testing capabilities

---

## Slide 18: Why This Matters

**For Developers:**
- Faster testing workflow
- Works in any environment
- Keyboard-driven efficiency
- Lightweight and fast

**For Teams:**
- Shareable collection files
- Consistent testing approach
- Easy onboarding
- Open source and extensible

**For DevOps:**
- SSH-friendly
- Scriptable
- Minimal dependencies
- Production-ready

---

## Slide 19: Call to Action

### Try It Out
```bash
git clone https://github.com/gratluri/rest-api-tui.git
cd rest-api-tui
cargo run
```

### Contribute
- Star the repo ⭐
- Report issues 🐛
- Submit PRs 🔧
- Share feedback 💬

### Learn More
- GitHub: github.com/gratluri/rest-api-tui
- Documentation: See README.md
- Cheatsheet: See CHEATSHEET.md

---

## Slide 20: Questions?

**Thank you!**

🔗 **Repository:** github.com/gratluri/rest-api-tui  
📧 **Feedback:** Open an issue on GitHub  
⭐ **Star if you like it!**

---

## Presentation Tips

### Before Demo
1. Practice the demo flow 2-3 times
2. Have terminal ready with app built
3. Clear terminal history
4. Set terminal size to 120x30
5. Test all features work

### During Demo
1. Speak clearly and slowly
2. Explain what you're doing before doing it
3. Pause after each feature
4. Show keyboard shortcuts on screen
5. Highlight the "wow" moments (faker, quick execute)

### After Demo
1. Share the recording link
2. Share the GitHub repo
3. Offer to help with setup
4. Collect feedback
5. Follow up with interested team members

### Talking Points
- "This is like Postman, but in your terminal"
- "Notice how fast it starts - under 1 second"
- "Everything is keyboard-driven - no mouse needed"
- "Faker generates realistic data automatically"
- "Quick execute makes testing 3x faster"
- "Works perfectly over SSH"
- "Open source and built with Rust"
