# REST API TUI

A powerful terminal-based REST API testing tool with a modern split-panel interface, network traffic analysis, and comprehensive request management.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)

## Features

### 🎨 Modern Split-Panel Interface
- **Three-panel layout** inspired by Postman
- Collections & Endpoints panel (right)
- API Definition panel (top left)
- Response panel with scrolling (bottom left)
- Always-visible response for efficient workflow

### 🔍 Network Traffic Analysis
- **Wireshark-style traffic tracking** (optional)
- Timing breakdown (Request, Waiting/TTFB, Download)
- Request/response details (headers, body sizes)
- Total transfer size calculation
- Toggle with 't' key

### 📊 Response Management
- **Automatic formatting** (JSON, XML, plain text)
- **Vertical scrolling** for large responses
- Scroll indicator showing visible range
- PageUp/PageDown/Home/End navigation
- Response persists until next request

### 🗂️ Collection Management
- Create, edit, delete collections
- Organize endpoints by collection
- Persistent storage (JSON files)
- CRUD operations from TUI

### 🌐 HTTP Request Features
- All HTTP methods (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS)
- Custom headers with inline editor
- Request body templates with variables
- Authentication (Bearer, Basic, API Key)
- Template variable substitution `{{variable}}`

### ⚡ Load Testing
- Concurrent request execution
- Real-time metrics (RPS, success/failure rates)
- Configurable duration and concurrency
- Visual progress and statistics

### ⌨️ Keyboard-Driven
- Vim-style navigation (j/k)
- Ctrl+h/l for panel switching
- No mouse required
- Fast, efficient workflow

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rest-api-tui.git
cd rest-api-tui

# Build release version
cargo build --release

# Run
./target/release/rest-api-tui
```

### Requirements

- Rust 1.70 or higher
- Modern terminal with Unicode support
- macOS, Linux, or Windows

## Quick Start

### 1. Launch the Application

```bash
cargo run
# or
./target/release/rest-api-tui
```

### 2. Create Your First Collection

1. Press **'n'** to create a new collection
2. Type the collection name (e.g., "My API")
3. Press **Enter** to save

### 3. Add an Endpoint

1. Press **Ctrl+l** to switch to Endpoints panel
2. Press **'n'** to create a new endpoint
3. Fill in the details:
   - **Name**: "Get Users"
   - **Method**: Press 'm' to cycle to GET
   - **URL**: `https://jsonplaceholder.typicode.com/users`
4. Press **Enter** to save

### 4. Execute the Request

1. Press **Enter** to select the endpoint
2. Press **'e'** to execute the request
3. View the response in the bottom panel
4. Press **'t'** to toggle network traffic view

### 5. Scroll Through Response

- **PageDown**: Scroll down
- **PageUp**: Scroll up
- **Home**: Jump to top
- **End**: Jump to bottom

## User Interface

### Layout Overview

```
┌─────────────────────────────────────────────────────────────┐
│ 🚀 REST API TUI - Terminal API Testing Tool                │
├──────────────────────────────┬──────────────────────────────┤
│ API Definition               │ 📁 Collections               │
│                              │ ┌──────────────────────────┐ │
│ Method: GET                  │ │ ▶ My API Collection      │ │
│ URL: https://api.example.com │ │   → Get Users            │ │
│                              │ │     Create User          │ │
│ Headers: (2)                 │ │     Update User          │ │
│   Content-Type: app/json     │ │                          │ │
│   Authorization: Bearer...   │ │ [n: new | e: edit]       │ │
│                              │ └──────────────────────────┘ │
│ [e: execute | l: load test]  │                              │
├──────────────────────────────┴──────────────────────────────┤
│ Response: 200 OK - 123ms [1-20/150] [PgUp/PgDn | t: traffic]│
├──────────────────────────────────────────────────────────────┤
│ {                                                            │
│   "users": [                                                 │
│     { "id": 1, "name": "John" },                             │
│     ...                                                      │
│   ]                                                          │
│ }                                                            │
├──────────────────────────────────────────────────────────────┤
│ Ctrl+h/l: panels | Ctrl+j/k: nav | PgUp/PgDn: scroll | ?: help│
└──────────────────────────────────────────────────────────────┘
```

### Panel Focus

- **Cyan border**: Currently focused panel
- **Gray border**: Unfocused panels
- **Yellow text**: Selected item in focused panel
- **White text**: Selected item in unfocused panel

## Keyboard Shortcuts

### Global Navigation

| Key | Action |
|-----|--------|
| **Ctrl+h** | Switch to Collections panel |
| **Ctrl+l** | Switch to Endpoints panel |
| **Ctrl+k** | Navigate up in current panel |
| **Ctrl+j** | Navigate down in current panel |
| **Ctrl+i** | Toggle between panels |
| **↑/k** | Navigate up (when not in edit mode) |
| **↓/j** | Navigate down (when not in edit mode) |
| **Enter** | Select item / Save form |
| **Esc** | Go back / Cancel |
| **?** | Show help screen |
| **q** | Quit application |

### Collection & Endpoint Management

| Key | Action |
|-----|--------|
| **n** | New collection/endpoint (based on focused panel) |
| **e** | Edit collection/endpoint OR execute request |
| **d** | Delete collection/endpoint (with confirmation) |

### Request Execution

| Key | Action |
|-----|--------|
| **e** | Execute request (when endpoint is selected) |
| **l** | Start load test |
| **t** | Toggle network traffic view |

### Response Scrolling

| Key | Action |
|-----|--------|
| **PageDown** | Scroll down 10 lines |
| **PageUp** | Scroll up 10 lines |
| **Home** | Jump to top of response |
| **End** | Jump to bottom of response |

### Form Editing

| Key | Action |
|-----|--------|
| **Tab** | Next field |
| **Shift+Tab** | Previous field |
| **m** | Cycle HTTP method (on Method field only) |
| **h** | Enter header edit mode (on Headers field only) |
| **Backspace** | Delete character |
| **Enter** | Save form |
| **Esc** | Cancel editing |

### Header Editing

When on the Headers field, press **'h'** to enter header edit mode:

| Key | Action |
|-----|--------|
| **Tab** | Switch between Key and Value fields |
| **Shift+Tab** | Switch backwards |
| **Enter** | Add header |
| **Esc** | Exit header edit mode |

## Workflows

### Creating and Testing an API Endpoint

1. **Create Collection**:
   ```
   Press 'n' → Type "GitHub API" → Enter
   ```

2. **Add Endpoint**:
   ```
   Ctrl+l → Press 'n'
   Name: "Get Repositories"
   Method: Press 'm' until GET
   URL: https://api.github.com/users/octocat/repos
   Tab to Headers → Press 'h'
   Key: "Accept" → Tab → Value: "application/json" → Enter
   Esc → Enter (save)
   ```

3. **Execute Request**:
   ```
   Enter (select endpoint) → Press 'e'
   ```

4. **View Response**:
   ```
   Response appears in bottom panel
   Press 't' to see network traffic
   Press PageDown to scroll through response
   ```

### Adding Custom Headers

1. Create or edit an endpoint
2. Tab to the Headers field (field 4)
3. Press **'h'** to enter header edit mode
4. Type header key (e.g., "Authorization")
5. Press **Tab** to move to value field
6. Type header value (e.g., "Bearer your-token")
7. Press **Enter** to add the header
8. Repeat for more headers
9. Press **Esc** to exit header mode
10. Press **Enter** to save endpoint

### Using Template Variables

Endpoints support template variables using `{{variable}}` syntax:

**Example**:
```
URL: https://api.example.com/users/{{userId}}
Body: {"name": "{{userName}}", "email": "{{userEmail}}"}
```

**Note**: Variable substitution is currently done with empty values. Full variable management is planned for a future release.

### Load Testing an Endpoint

1. Select an endpoint
2. Press **'l'** to start load test
3. View real-time metrics:
   - Total requests
   - Successful/failed requests
   - Current RPS (requests per second)
4. Press **Esc** to stop the load test

### Viewing Network Traffic

1. Execute a request (press 'e')
2. Press **'t'** to toggle network traffic view
3. View detailed information:
   - **Timing**: Request sent, Waiting (TTFB), Download, Total
   - **Request**: Method, URL, headers, body size
   - **Response**: Status, headers, body size
   - **Transfer**: Total bytes transferred
4. Press **'t'** again to hide traffic view

## Configuration

### Storage Location

Collections are stored as JSON files in:
```
~/.rest-api-tui/collections/
```

Each collection is a separate JSON file with all its endpoints.

### Collection File Format

```json
{
  "id": "uuid",
  "name": "My API Collection",
  "endpoints": [
    {
      "id": "uuid",
      "name": "Get Users",
      "method": "GET",
      "url": "https://api.example.com/users",
      "description": "Fetches all users",
      "headers": {
        "Content-Type": "application/json",
        "Authorization": "Bearer token"
      },
      "body_template": null,
      "auth": null
    }
  ]
}
```

### Authentication Configuration

Authentication is currently configured via JSON file editing:

**Bearer Token**:
```json
"auth": {
  "type": "Bearer",
  "token": "your-token-here"
}
```

**Basic Auth**:
```json
"auth": {
  "type": "Basic",
  "username": "user",
  "password": "pass"
}
```

**API Key**:
```json
"auth": {
  "type": "ApiKey",
  "name": "X-API-Key",
  "value": "your-key",
  "location": "Header"
}
```

## Tips & Tricks

### 1. Fast Navigation

- Use **Ctrl+h/l** to quickly switch between collections and endpoints
- Use **Ctrl+j/k** to navigate without leaving home row
- Press **Enter** twice to select and view an endpoint

### 2. Efficient Scrolling

- Press **End** to jump to bottom of large responses
- Press **Home** to jump back to top
- Use **PageDown/PageUp** for controlled scrolling

### 3. Network Traffic Analysis

- Enable traffic view ('t') to debug slow requests
- Check **Waiting (TTFB)** for server performance
- Check **Content Download** for network speed
- Monitor **Total Transfer** for bandwidth usage

### 4. Header Management

- Use custom headers for authentication (Authorization: Bearer token)
- Add Content-Type headers for JSON/XML requests
- Add custom headers for API keys

### 5. Keyboard Efficiency

- Learn Ctrl+h/j/k/l for navigation (no arrow keys needed)
- Use 'n', 'e', 'd' for quick CRUD operations
- Press '?' to see all shortcuts

## Troubleshooting

### Issue: Can't see last line of response

**Solution**: Press **End** key to jump to bottom. The scroll indicator shows the visible range (e.g., `[586-602/602]`).

### Issue: 'h' key not working in URL field

**Solution**: The 'h' key only activates header mode when on the Headers field. Tab to the Headers field first, then press 'h'.

### Issue: Can't type 'm' in URL

**Solution**: The 'm' key only cycles HTTP method when on the Method field. In other fields, 'm' works as normal text input.

### Issue: Response not showing

**Solution**: Make sure you've executed a request (press 'e'). The response panel shows "No response yet" until a request is executed.

### Issue: Collections not persisting

**Solution**: Collections are auto-saved to `~/.rest-api-tui/collections/`. Check file permissions on this directory.

## Advanced Features

### Load Testing

Configure load test parameters:
- **Concurrent requests**: 10 (default)
- **Duration**: 30 seconds (default)

Currently configured in code. UI configuration planned for future release.

### Response Formatting

Responses are automatically formatted based on Content-Type:
- **JSON**: Pretty-printed with indentation
- **XML**: Formatted with proper indentation
- **Plain text**: Displayed as-is

### Template Variables

Use `{{variable}}` syntax in:
- URLs
- Headers
- Body templates

Variables are substituted at request time.

## Roadmap

### Planned Features

- [ ] **Import/Export**: cURL, Postman, HTTPie
- [ ] **Environment Variables**: Manage variables per environment
- [ ] **Request History**: Track and replay previous requests
- [ ] **Authentication UI**: Configure auth from TUI
- [ ] **Search**: Find endpoints across collections
- [ ] **Tabs**: Multiple requests in tabs
- [ ] **GraphQL Support**: Query and mutation support
- [ ] **WebSocket Support**: Real-time connections
- [ ] **Request Chaining**: Use response from one request in another
- [ ] **Themes**: Customizable color schemes

### Ergonomic Improvements

See [ERGONOMIC_IMPROVEMENTS.md](ERGONOMIC_IMPROVEMENTS.md) for detailed suggestions.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Update documentation
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Credits

Built with:
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization

Inspired by:
- Postman
- Insomnia
- HTTPie
- Wireshark

## Support

- **Documentation**: See docs in the repository
- **Issues**: Report bugs on GitHub
- **Discussions**: Ask questions on GitHub Discussions

## Changelog

See [FEATURE_SUMMARY.md](FEATURE_SUMMARY.md) for recent updates and [BUGFIXES.md](BUGFIXES.md) for bug fix history.

---

**Made with ❤️ for API developers who love the terminal**
