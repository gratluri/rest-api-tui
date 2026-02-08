# New Split-Panel Layout (Option B)

## Overview

The TUI now uses a split-panel layout inspired by Postman and other API testing tools. This provides a more efficient workflow with all information visible at once.

## Layout Structure

```
┌─────────────────────────────────────────────────────────────────────┐
│ 🚀 REST API TUI - Terminal API Testing Tool                        │
├─────────────────────────────────┬───────────────────────────────────┤
│ Endpoint: Get User              │ 📁 Collections                    │
│                                 │                                   │
│ Method: [GET]                   │ ▶ 📁 API Collection 1             │
│ URL: http://api.example.com/... │   → Get User (selected)           │
│                                 │     Get Posts                     │
│ Headers: (2)                    │     Create Post                   │
│   Content-Type: application/json│                                   │
│   Authorization: Bearer xyz...  │ ▶ 📁 API Collection 2             │
│                                 │     Get Products                  │
│ Body: [empty]                   │     Create Product                │
│                                 │                                   │
│ [e: execute] [l: load test]     │ [n: new] [e: edit] [d: delete]    │
├─────────────────────────────────┴───────────────────────────────────┤
│ Response: 200 OK - 123ms - 456 bytes                               │
├─────────────────────────────────────────────────────────────────────┤
│ {                                                                   │
│   "id": 1,                                                          │
│   "name": "John Doe",                                               │
│   "email": "john@example.com"                                       │
│ }                                                                   │
│                                                                     │
│ (scrollable)                                                        │
├─────────────────────────────────────────────────────────────────────┤
│ Ctrl+h/l: switch panels | Ctrl+j/k: navigate | '?': help | 'q': quit│
└─────────────────────────────────────────────────────────────────────┘
```

## Three Main Panels

### 1. API Definition Panel (Top Left - 65% width, 50% height)
- Shows the selected endpoint's details
- Method, URL, headers, body, authentication
- Actions: Execute request, Start load test
- When no endpoint is selected, shows helpful instructions

### 2. Response Panel (Bottom Left - 65% width, 50% height)
- **Always visible** (as requested)
- Shows response status, duration, size
- Displays formatted response body (JSON, XML, or plain text)
- When no response yet, shows "No response yet" message
- Scrollable for long responses

### 3. Collections & Endpoints Panel (Right - 35% width, 100% height)
- **Top section (40%)**: Collections list
  - Shows all collections with endpoint count
  - Collapsible tree view
- **Bottom section (60%)**: Endpoints list
  - Shows endpoints for the selected collection
  - Color-coded by HTTP method (GET=green, POST=blue, etc.)
  - Indicates currently selected endpoint

## New Keyboard Shortcuts

### Panel Navigation (Ctrl+ijkl)
- **Ctrl+h**: Switch to Collections panel (left)
- **Ctrl+l**: Switch to Endpoints panel (right)
- **Ctrl+k**: Navigate up in current panel
- **Ctrl+j**: Navigate down in current panel
- **Ctrl+i**: Toggle between panels (alternative to Ctrl+h/l)

### Within Panels
- **↑/k**: Navigate up (when not in edit mode)
- **↓/j**: Navigate down (when not in edit mode)
- **Enter**: Select endpoint to view details
- **n**: New collection/endpoint (based on focused panel)
- **e**: Edit collection/endpoint OR execute request
- **d**: Delete collection/endpoint (with confirmation)
- **l**: Start load test (when endpoint is selected)

### Other
- **?**: Show help
- **q**: Quit (from main screen)
- **Esc**: Go back / Cancel

## Panel Focus Indicators

- **Focused panel**: Cyan border
- **Unfocused panels**: Dark gray border
- **Selected item in focused panel**: Yellow and bold
- **Selected item in unfocused panel**: White and bold

## Workflow Examples

### Example 1: Execute a Request

1. Start app - Collections panel is focused by default
2. Use **Ctrl+j/k** to navigate collections
3. Press **Enter** or **Ctrl+l** to switch to Endpoints panel
4. Use **Ctrl+j/k** to navigate endpoints
5. Press **Enter** to view endpoint details (left panel updates)
6. Press **e** to execute the request
7. Response appears in the bottom panel automatically

### Example 2: Create New Endpoint

1. Navigate to desired collection (Ctrl+h, then Ctrl+j/k)
2. Press **Ctrl+l** to switch to Endpoints panel
3. Press **n** to create new endpoint
4. Full-screen edit form appears (old behavior)
5. Fill in details, press Enter to save
6. Returns to split view with new endpoint visible

### Example 3: Switch Between Endpoints

1. Use **Ctrl+l** to focus Endpoints panel
2. Use **Ctrl+j/k** to navigate between endpoints
3. Press **Enter** to view each endpoint's details
4. Definition panel updates immediately
5. Previous response stays visible until new request is executed

## Full-Screen Modes

Some screens still use the old full-screen layout:
- **Collection Edit**: Full-screen form
- **Endpoint Edit**: Full-screen form with all fields
- **Load Test Running**: Full-screen with metrics and charts
- **Help Screen**: Full-screen help text
- **Delete Confirmation**: Centered dialog overlay

This keeps complex editing workflows simple while providing the split view for browsing and testing.

## Benefits

1. **Always see context**: Collections, endpoint details, and response all visible
2. **Faster navigation**: Ctrl+ijkl for quick panel switching
3. **No screen switching**: Stay in one view while testing multiple endpoints
4. **Response history**: Previous response stays visible until you execute a new request
5. **Clear focus**: Visual indicators show which panel is active

## Reverting to Old Layout

If you need to revert to the previous layout:

```bash
git checkout v0.1.0-headers-working
```

This tag contains the working version before the layout change.

## Technical Details

### New AppState Fields
- `selected_collection_index`: Tracks selected collection
- `selected_endpoint_index`: Tracks selected endpoint within collection
- `panel_focus`: Enum tracking which panel is focused (Collections or Endpoints)

### Panel Focus Enum
```rust
pub enum PanelFocus {
    Collections,
    Endpoints,
}
```

### Layout Constraints
- Horizontal split: 65% left (definition + response), 35% right (collections)
- Vertical split (left): 50% top (definition), 50% bottom (response)
- Vertical split (right): 40% top (collections), 60% bottom (endpoints)

These percentages can be adjusted in `src/tui/ui.rs` if needed.

## Future Enhancements

Potential improvements for the new layout:
- Resizable panels (drag borders)
- Collapsible response panel
- Tabs for multiple requests
- Request history sidebar
- Environment variables panel
- Test results panel

## Feedback

The layout is designed to be intuitive and efficient. If you have suggestions for improvements, please let me know!
