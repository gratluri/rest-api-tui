# REST API TUI - Keyboard Shortcuts Cheatsheet

## Quick Reference Card

### 🧭 Navigation
| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` | Select item / View details |
| `Esc` | Go back / Cancel |
| `q` | Quit (from main screen) |
| `?` | Show help |

### 🎯 Panel Navigation (Main Screen)
| Key | Action |
|-----|--------|
| `Ctrl+h` | Switch to collections panel (left) |
| `Ctrl+l` | Switch to endpoints panel (right) |
| `Ctrl+j` | Navigate down in current panel |
| `Ctrl+k` | Navigate up in current panel |
| `Ctrl+i` / `Tab` | Toggle between panels |

### 📁 Collection Management
| Key | Action | Context |
|-----|--------|---------|
| `n` | New collection | Collections panel |
| `e` | Edit collection | Collections panel |
| `d` | Delete collection | Collections panel |

### 🔗 Endpoint Management
| Key | Action | Context |
|-----|--------|---------|
| `n` | New endpoint | Endpoints panel |
| `e` | Edit endpoint | Endpoints panel |
| `d` | Delete endpoint | Endpoints panel |
| `Enter` | View endpoint details | Endpoints panel |

### 🚀 Request Execution
| Key | Action | Context | Variables? |
|-----|--------|---------|-----------|
| `x` | Quick execute | Main screen | ❌ No prompt (uses saved) |
| `e` | Execute request | Endpoint detail | ✅ Shows input screen |

### 🔧 Variable Management
| Key | Action | Context |
|-----|--------|---------|
| `v` | Open variable list | Main screen |
| `n` | New variable | Variable list |
| `e` | Edit variable | Variable list |
| `d` | Delete variable | Variable list |

### 📜 Scrolling & Viewing
| Key | Action |
|-----|--------|
| `PgUp` | Scroll response up (10 lines) |
| `PgDn` | Scroll response down (10 lines) |
| `Shift+PgUp` | Scroll headers up (5 lines) |
| `Shift+PgDn` | Scroll headers down (5 lines) |
| `Home` | Scroll to top of response |
| `Shift+Home` | Scroll to top of headers |
| `End` | Scroll to bottom of response |

### 👁️ View Options
| Key | Action |
|-----|--------|
| `t` | Toggle network traffic display |
| `H` | Toggle response headers display |
| `Space` | Collapse/expand sections |

### 📋 Clipboard
| Key | Action | Context |
|-----|--------|---------|
| `y` | Copy response to clipboard | Endpoint detail with response |

### ⚡ Load Testing
| Key | Action | Context |
|-----|--------|---------|
| `l` | Start load test | Endpoints panel |
| `Esc` | Stop load test | Load test running |

### ✏️ Form Editing
| Key | Action | Context |
|-----|--------|---------|
| `Tab` | Next field | Edit forms |
| `Shift+Tab` | Previous field | Edit forms |
| `m` | Cycle HTTP method | Endpoint edit (method field) |
| `h` | Toggle header edit mode | Endpoint edit (headers field) |
| `Backspace` | Delete character | Edit forms |
| `Enter` | Save | Edit forms |

### ⚠️ Confirmation Dialogs
| Key | Action |
|-----|--------|
| `y` / `Y` | Confirm action |
| `n` / `N` / `Esc` | Cancel action |

---

## Variable Syntax

### User Variables
```
{{VARIABLE_NAME}}
```
- Uses saved values from variable manager
- Press `v` to manage variables
- Example: `{{API_URL}}`, `{{AUTH_TOKEN}}`

### Faker Variables
```
{{f:variablename}}
```
- Generates fake data dynamically
- No need to define in variable manager
- Example: `{{f:firstname}}`, `{{f:email}}`, `{{f:uuid}}`

### Popular Faker Variables
| Variable | Example Output |
|----------|---------------|
| `{{f:firstname}}` | John |
| `{{f:lastname}}` | Doe |
| `{{f:email}}` | john.doe@example.com |
| `{{f:phone}}` | (555) 123-4567 |
| `{{f:company}}` | Acme Corp |
| `{{f:uuid}}` | 550e8400-e29b-41d4-a716... |
| `{{f:number}}` | 42 |
| `{{f:date}}` | 2024-03-15 |
| `{{f:url}}` | https://example.com |

See `FAKER_FEATURE.md` for complete list of 50+ faker variables.

---

## Common Workflows

### 🎯 Quick Test an Endpoint
1. Navigate to endpoint (`Ctrl+l`, `↑/↓`)
2. Press `x` (quick execute)
3. View response in response panel

### 📝 Create New Endpoint
1. Navigate to endpoints panel (`Ctrl+l`)
2. Press `n` (new endpoint)
3. Fill in details (use `Tab` to navigate fields)
4. Press `Enter` to save

### 🔄 Execute with Variables
1. Define variables: Press `v` → `n` → enter key/value → `Enter`
2. Use in endpoint: `{{VARIABLE_NAME}}`
3. Execute: Press `e` (shows variable input) or `x` (uses saved values)

### 🎲 Use Faker for Test Data
1. Edit endpoint body:
   ```json
   {
     "name": "{{f:fullname}}",
     "email": "{{f:email}}",
     "phone": "{{f:phone}}"
   }
   ```
2. Press `x` to execute (generates new data each time)

### 📊 Run Load Test
1. Navigate to endpoint
2. Press `l` (load test)
3. Configure: concurrency, duration, ramp-up
4. Press `Enter` to start
5. Press `Esc` to stop

### 📋 Copy Response
1. Execute request (`e` or `x`)
2. Press `y` to copy response to clipboard

---

## Tips & Tricks

### 💡 Productivity Tips
- Use `x` for rapid testing (no prompts)
- Use `e` when you need to review/edit variables
- Combine user variables and faker variables
- Use `Ctrl+h/l` for fast panel switching
- Press `?` anytime to see help

### 🎨 Visual Feedback
- **Yellow highlight** = Selected item
- **Cyan borders** = Focused panel
- **Green status** = Success
- **Red status** = Error
- **JSON syntax highlighting** = Automatic

### ⚡ Performance
- Quick execute (`x`) is fastest
- Load tests run in background
- Responses are formatted automatically
- Network traffic is optional (toggle with `t`)

### 🔍 Debugging
- Toggle network traffic (`t`) to see timing details
- Toggle headers (`H`) to see response headers
- Use `Space` to collapse/expand sections
- Scroll through long responses with `PgUp/PgDn`

---

## File Locations

### Configuration
- Variables: `~/.rest-api-tui/variables.json`
- Collections: `~/.rest-api-tui/collections/`

### Documentation
- Full docs: `README.md`
- Faker variables: `FAKER_FEATURE.md`
- Quick execute: `QUICK_EXECUTE_FEATURE.md`
- Variable UI: `VARIABLE_UI_COMPLETE.md`

---

## Getting Help

- Press `?` in the app for keyboard shortcuts
- Check `README.md` for full documentation
- See `FAKER_FEATURE.md` for all faker variables
- Report issues on GitHub

---

**Version**: 0.1.0  
**Last Updated**: 2024

*Happy API Testing! 🚀*
