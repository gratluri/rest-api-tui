# Ergonomic Improvements & UX Suggestions

This document outlines potential improvements to make the REST API TUI more user-friendly and efficient.

## Priority 1: High-Impact Improvements

### 1. Environment Variables Management

**Current State**: Variables in templates (`{{variable}}`) are substituted with empty values.

**Proposed Improvement**:
- Add an Environments panel (Ctrl+e to toggle)
- Create/edit/delete environments (Dev, Staging, Prod)
- Define variables per environment (baseUrl, apiKey, userId, etc.)
- Switch active environment with dropdown or hotkey
- Visual indicator showing current environment

**Benefits**:
- Reuse endpoints across environments
- No need to edit URLs when switching environments
- Safer testing (clear visual indicator of which environment is active)

**Implementation Complexity**: Medium

---

### 2. Request History

**Current State**: No history of executed requests.

**Proposed Improvement**:
- Store last 100 requests in memory/disk
- Add History panel (Ctrl+r to toggle)
- Show: timestamp, method, URL, status code, duration
- Replay request from history with one key
- Filter history by collection, method, or status
- Export history to file

**Benefits**:
- Quickly retry failed requests
- Compare responses over time
- Debug intermittent issues
- Track API behavior changes

**Implementation Complexity**: Medium

---

### 3. Search & Filter

**Current State**: Must manually scroll through collections/endpoints.

**Proposed Improvement**:
- Press '/' to open search bar
- Search across: collection names, endpoint names, URLs, descriptions
- Fuzzy matching (e.g., "getus" matches "Get Users")
- Show results in overlay with keyboard navigation
- Press Enter to jump to result
- ESC to close search

**Benefits**:
- Find endpoints quickly in large collections
- No need to remember which collection contains an endpoint
- Faster workflow for power users

**Implementation Complexity**: Low-Medium

---

### 4. Inline Response Comparison

**Current State**: Can only view one response at a time.

**Proposed Improvement**:
- Press 'c' to enter comparison mode
- Execute request, then execute again (or select from history)
- Split response panel vertically: left = previous, right = current
- Highlight differences (added/removed/changed lines)
- Useful for testing API changes

**Benefits**:
- Verify API behavior changes
- Debug regression issues
- Compare responses across environments

**Implementation Complexity**: High

---

### 5. Quick Execute from Collections Panel

**Current State**: Must select endpoint, then press 'e' to execute.

**Proposed Improvement**:
- Press 'x' on any endpoint in the list to execute immediately
- Response appears in bottom panel
- No need to enter detail view
- Faster workflow for repeated testing

**Benefits**:
- Fewer keystrokes for common workflow
- Faster iteration when testing multiple endpoints
- Less context switching

**Implementation Complexity**: Low

---

## Priority 2: Quality of Life Improvements

### 6. Resizable Panels

**Current State**: Fixed panel sizes (65%/35% split).

**Proposed Improvement**:
- Press Ctrl+[ to shrink left panel
- Press Ctrl+] to grow left panel
- Save panel sizes to config file
- Presets: 50/50, 60/40, 70/30, 80/20

**Benefits**:
- Customize layout for different workflows
- More space for long URLs or large responses
- Accommodate different terminal sizes

**Implementation Complexity**: Medium

---

### 7. Response Tabs

**Current State**: Only one response visible at a time.

**Proposed Improvement**:
- Execute multiple requests, each opens a new tab
- Show tabs above response panel: [1: Get Users] [2: Create User*]
- Switch tabs with Ctrl+1, Ctrl+2, etc. or Ctrl+Tab
- Close tab with Ctrl+w
- Asterisk (*) indicates unsaved/modified

**Benefits**:
- Compare multiple responses side-by-side
- Keep context when testing related endpoints
- Reduce need to re-execute requests

**Implementation Complexity**: High

---

### 8. Collapsible Sections

**Current State**: All sections always visible.

**Proposed Improvement**:
- Press Space to collapse/expand sections:
  - Headers section
  - Body section
  - Network traffic section
- Visual indicator: ▶ collapsed, ▼ expanded
- Remember collapsed state per endpoint

**Benefits**:
- Focus on relevant information
- More space for response
- Cleaner interface

**Implementation Complexity**: Low

---

### 9. Copy to Clipboard

**Current State**: No way to copy data from TUI.

**Proposed Improvement**:
- Press 'y' to copy current response to clipboard
- Press 'Y' to copy formatted response
- Press 'u' to copy current URL
- Visual confirmation: "Copied to clipboard"
- Works with system clipboard

**Benefits**:
- Share responses with team
- Paste into documentation
- Debug in external tools

**Implementation Complexity**: Medium (requires clipboard library)

---

### 10. Syntax Highlighting

**Current State**: Plain text response display.

**Proposed Improvement**:
- Color-code JSON responses:
  - Keys in cyan
  - Strings in green
  - Numbers in yellow
  - Booleans in magenta
  - Null in red
- Similar highlighting for XML
- Toggle with Ctrl+s

**Benefits**:
- Easier to read complex responses
- Quickly identify data types
- More visually appealing

**Implementation Complexity**: Medium

---

## Priority 3: Advanced Features

### 11. Request Chaining

**Current State**: Each request is independent.

**Proposed Improvement**:
- Extract values from response: `{{response.data.userId}}`
- Use in subsequent requests
- Define extraction paths in endpoint config
- Visual indicator showing chained requests

**Benefits**:
- Test complex workflows (create → update → delete)
- Realistic integration testing
- Reduce manual data entry

**Implementation Complexity**: High

---

### 12. GraphQL Support

**Current State**: Only REST APIs supported.

**Proposed Improvement**:
- Add GraphQL endpoint type
- Query editor with syntax highlighting
- Variable editor
- Schema introspection
- Query history

**Benefits**:
- Support modern APIs
- Unified tool for REST and GraphQL
- No need for separate tools

**Implementation Complexity**: Very High

---

### 13. WebSocket Support

**Current State**: Only HTTP requests.

**Proposed Improvement**:
- Add WebSocket endpoint type
- Connect/disconnect controls
- Send messages
- View message stream
- Filter messages

**Benefits**:
- Test real-time APIs
- Debug WebSocket connections
- Complete API testing solution

**Implementation Complexity**: Very High

---

### 14. Authentication UI

**Current State**: Must edit JSON files to configure auth.

**Proposed Improvement**:
- Add Auth tab in endpoint editor
- Dropdown: None, Bearer, Basic, API Key, OAuth2
- Form fields based on auth type
- Test auth button
- Save credentials securely

**Benefits**:
- No need to edit JSON manually
- Easier for non-technical users
- Secure credential storage

**Implementation Complexity**: Medium

---

### 15. Import/Export

**Current State**: No import/export functionality.

**Proposed Improvement**:
- Import from:
  - cURL commands
  - Postman collections (JSON)
  - HTTPie commands
  - OpenAPI/Swagger specs
- Export to:
  - Postman format
  - cURL commands
  - Code snippets (Python, JavaScript, etc.)

**Benefits**:
- Migrate from other tools
- Share collections with team
- Generate code from requests

**Implementation Complexity**: High

---

## Priority 4: Polish & Refinements

### 16. Keyboard Shortcut Customization

**Current State**: Fixed keyboard shortcuts.

**Proposed Improvement**:
- Config file: `~/.rest-api-tui/keybindings.toml`
- Remap any action to any key
- Preset profiles: Vim, Emacs, Default
- Show current bindings in help screen

**Benefits**:
- Accommodate different preferences
- Avoid conflicts with terminal shortcuts
- Muscle memory from other tools

**Implementation Complexity**: Medium

---

### 17. Themes

**Current State**: Fixed color scheme.

**Proposed Improvement**:
- Config file: `~/.rest-api-tui/theme.toml`
- Built-in themes: Dark, Light, Solarized, Dracula, Nord
- Customize all colors
- Preview themes before applying

**Benefits**:
- Personal preference
- Better visibility in different terminals
- Accessibility (high contrast themes)

**Implementation Complexity**: Low-Medium

---

### 18. Status Bar Improvements

**Current State**: Basic status messages.

**Proposed Improvement**:
- Left: Current mode/action
- Center: Status message
- Right: Active environment, request count, time
- Color-coded: green = success, red = error, yellow = warning
- Progress bar for long operations

**Benefits**:
- More context at a glance
- Better feedback on actions
- Professional appearance

**Implementation Complexity**: Low

---

### 19. Validation & Error Prevention

**Current State**: Errors shown after save attempt.

**Proposed Improvement**:
- Real-time validation:
  - URL format (show ✓ or ✗)
  - Required fields (highlight if empty)
  - JSON body syntax
- Prevent saving invalid data
- Helpful error messages with suggestions

**Benefits**:
- Catch errors early
- Reduce frustration
- Faster workflow

**Implementation Complexity**: Medium

---

### 20. Tooltips & Contextual Help

**Current State**: Must press '?' to see all shortcuts.

**Proposed Improvement**:
- Show relevant shortcuts in each panel
- Hover-style tooltips (if terminal supports mouse)
- Context-sensitive help (F1 key)
- Inline hints for first-time users

**Benefits**:
- Easier to learn
- Reduce need to memorize shortcuts
- Better onboarding

**Implementation Complexity**: Low

---

## Quick Wins (Easy to Implement)

1. **Confirmation on Quit** (if unsaved changes)
2. **Auto-save** (save collections on every change)
3. **Undo/Redo** (Ctrl+z / Ctrl+y)
4. **Duplicate Endpoint** (press 'D' to clone)
5. **Rename Collection/Endpoint** (press 'r')
6. **Sort Endpoints** (by name, method, recent)
7. **Endpoint Tags** (organize by feature/version)
8. **Request Timeout Configuration** (per endpoint)
9. **Follow Redirects Toggle** (per endpoint)
10. **Show Response Headers** (toggle with 'H')

---

## Accessibility Improvements

1. **Screen Reader Support** (announce state changes)
2. **High Contrast Mode** (for visual impairments)
3. **Larger Font Option** (for readability)
4. **Keyboard-Only Navigation** (no mouse required - already done!)
5. **Audio Feedback** (optional beeps for errors/success)

---

## Performance Optimizations

1. **Lazy Loading** (load collections on demand)
2. **Response Streaming** (for large responses)
3. **Background Requests** (don't block UI)
4. **Cache Responses** (optional, for faster replay)
5. **Async Collection Loading** (faster startup)

---

## Implementation Roadmap

### Phase 1 (Next Release)
- Quick Execute (Priority 1.5)
- Search & Filter (Priority 1.3)
- Copy to Clipboard (Priority 2.9)
- Show Response Headers (Quick Win)
- Duplicate Endpoint (Quick Win)

### Phase 2 (Following Release)
- Environment Variables (Priority 1.1)
- Request History (Priority 1.2)
- Authentication UI (Priority 3.14)
- Syntax Highlighting (Priority 2.10)

### Phase 3 (Future)
- Import/Export (Priority 3.15)
- Request Chaining (Priority 3.11)
- Response Tabs (Priority 2.7)
- Resizable Panels (Priority 2.6)

### Phase 4 (Long-term)
- GraphQL Support (Priority 3.12)
- WebSocket Support (Priority 3.13)
- Response Comparison (Priority 1.4)

---

## User Feedback Needed

Before implementing these improvements, gather feedback on:

1. **Most wanted features** (survey or GitHub issues)
2. **Current pain points** (what's frustrating?)
3. **Workflow patterns** (how do users actually use the tool?)
4. **Missing features** (what's not on this list?)

---

## Conclusion

This document provides a comprehensive roadmap for improving the REST API TUI. The suggestions are prioritized based on:

- **Impact**: How much value does it provide?
- **Complexity**: How hard is it to implement?
- **User Demand**: How many users want it?

Start with Priority 1 and Quick Wins for maximum impact with minimal effort.

