# Response Panel Scrolling

## Overview

The response panel now supports vertical scrolling, allowing you to view large responses that don't fit in the visible area.

## Features

### Automatic Scroll Indicator

When a response is larger than the visible area, a scroll indicator appears in the title bar showing the range of visible lines:

```
Response: 200 OK - 123ms - 456 bytes [25-44/150]
                                      ^^^^^^^^^^^
                                      Lines 25-44 visible out of 150 total
```

The indicator shows:
- First visible line number (1-indexed)
- Last visible line number  
- Total number of lines

### Keyboard Controls

| Key | Action | Lines Scrolled |
|-----|--------|----------------|
| **PageDown** | Scroll down | 10 lines |
| **PageUp** | Scroll up | 10 lines |
| **Home** | Jump to top | Reset to line 1 |
| **End** | Jump to bottom | Scroll to last line |

### Automatic Reset

- Scroll position resets to top when a new request is executed
- Ensures you always see the beginning of new responses

## Visual Examples

### Small Response (No Scrolling Needed)

```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 50ms - 123 bytes  [t: show traffic] │
├─────────────────────────────────────────────────────────┤
│ {                                                       │
│   "id": 1,                                              │
│   "name": "John Doe",                                   │
│   "email": "john@example.com"                           │
│ }                                                       │
│                                                         │
│ (all content visible)                                   │
└─────────────────────────────────────────────────────────┘
```

### Large Response (Scrolling Available)

**At Top:**
```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 150ms - 5KB  [1-20/150] [PgUp/PgDn] │
├─────────────────────────────────────────────────────────┤
│ {                                                       │
│   "users": [                                            │
│     {                                                   │
│       "id": 1,                                          │
│       "name": "John Doe",                               │
│       "email": "john@example.com"                       │
│     },                                                  │
│     {                                                   │
│       "id": 2,                                          │
│       ...                                               │
│                                                         │
│ (more content below - press PageDown or End)            │
└─────────────────────────────────────────────────────────┘
```

**Scrolled Down:**
```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 150ms - 5KB  [75-94/150] [PgUp/PgDn]│
├─────────────────────────────────────────────────────────┤
│       "city": "New York"                                │
│     },                                                  │
│     {                                                   │
│       "id": 50,                                         │
│       "name": "Jane Smith",                             │
│       "email": "jane@example.com",                      │
│       "city": "Los Angeles"                             │
│     },                                                  │
│     ...                                                 │
│                                                         │
│ (more content above and below)                          │
└─────────────────────────────────────────────────────────┘
```

**At Bottom:**
```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK - 150ms - 5KB  [131-150/150] [Home]   │
├─────────────────────────────────────────────────────────┤
│       "city": "Seattle"                                 │
│     }                                                   │
│   ],                                                    │
│   "total": 100,                                         │
│   "page": 1,                                            │
│   "per_page": 100                                       │
│ }                                                       │
│                                                         │
│ (end of response - press Home to go back to top)        │
└─────────────────────────────────────────────────────────┘
```

## Workflow Examples

### Example 1: Viewing Large JSON Response

1. Execute request (press 'e')
2. Response appears - notice `[1/250]` indicator
3. Press **PageDown** to scroll through the response
4. Press **PageDown** multiple times to reach the end
5. Press **Home** to jump back to the top

### Example 2: Comparing Different Parts

1. Execute request
2. Scroll to middle of response (PageDown several times)
3. Note the line number `[125/250]`
4. Execute another request
5. Scroll to same position to compare

### Example 3: With Network Traffic

When network traffic is enabled (press 't'):
- Response body takes top 50% of panel
- Scrolling works only in the response body section
- Network traffic section is separate (not scrollable)

```
┌─────────────────────────────────────────────────────────┐
│ Response: 200 OK [25/150] [t: hide traffic | PgUp/PgDn]│
├─────────────────────────────────────────────────────────┤
│ {                                                       │
│   "users": [                                            │
│     ...                                                 │
│   ]                                                     │
│ }                                                       │
├─────────────────────────────────────────────────────────┤
│ ▼ Network Traffic (Wireshark-style)                    │
│ Timing Breakdown:                                       │
│   Total: 150ms                                          │
│   ...                                                   │
└─────────────────────────────────────────────────────────┘
```

## Technical Details

### Line-Based Scrolling

- Response is split into lines (by `\n`)
- Scroll offset tracks the first visible line
- Visible height calculated from panel size
- Lines are rendered from `offset` to `offset + visible_height`

### Scroll Clamping

- Scroll offset is automatically clamped to valid range
- Maximum scroll = `total_lines - visible_height`
- Prevents scrolling past the end
- Prevents negative scroll values

### Performance

- **Efficient**: Only visible lines are rendered
- **No lag**: Even with 10,000+ line responses
- **Memory**: Full response kept in memory (already loaded)

### Limitations

1. **Horizontal scrolling**: Not currently supported
   - Long lines wrap to next line
   - Use formatted JSON/XML for better readability

2. **Network traffic section**: Not scrollable
   - Shows fixed summary information
   - Designed to fit in available space

3. **Mouse wheel**: Not currently supported
   - Use PageUp/PageDown instead
   - Could be added in future

## Use Cases

### 1. Large JSON Arrays

```json
{
  "users": [
    { "id": 1, ... },
    { "id": 2, ... },
    ...
    { "id": 1000, ... }  // Scroll to see all
  ]
}
```

### 2. Detailed Error Messages

```json
{
  "error": "Validation failed",
  "details": [
    "Field 'email' is required",
    "Field 'password' must be at least 8 characters",
    ...
    // Many validation errors
  ]
}
```

### 3. API Documentation Responses

```json
{
  "endpoints": [
    {
      "path": "/api/users",
      "methods": ["GET", "POST"],
      "description": "...",
      ...
    },
    // Many endpoints
  ]
}
```

### 4. Log Entries

```json
{
  "logs": [
    { "timestamp": "2026-02-08T10:00:00Z", "message": "..." },
    { "timestamp": "2026-02-08T10:00:01Z", "message": "..." },
    ...
    // Hundreds of log entries
  ]
}
```

## Tips

### Tip 1: Use Formatted Responses

- JSON and XML are automatically formatted
- Formatted responses are easier to scroll through
- Each object/array element on its own line

### Tip 2: Check Line Count

- Look at the scroll indicator `[current/total]`
- Helps estimate response size
- Useful for pagination decisions

### Tip 3: Use End Key for Quick Jump

- Press **End** to instantly jump to the bottom
- Press **Home** to instantly jump to the top
- Faster than pressing PageDown multiple times

### Tip 4: Combine with Network Traffic

- Use 't' to toggle traffic view
- Scroll through response body
- Check timing and sizes in traffic section

## Keyboard Shortcuts Summary

| Key | Action |
|-----|--------|
| **PageDown** | Scroll response down (10 lines) |
| **PageUp** | Scroll response up (10 lines) |
| **Home** | Jump to top of response |
| **End** | Jump to bottom of response |
| **t** | Toggle network traffic view |
| **e** | Execute request (resets scroll) |

## Future Enhancements

Potential improvements:
- [ ] Horizontal scrolling for long lines
- [ ] Mouse wheel support
- [ ] Smooth scrolling animation
- [ ] Search within response (Ctrl+F)
- [ ] Jump to line number
- [ ] Scroll to specific JSON path
- [ ] Bookmarks for large responses
- [ ] Split view (compare two responses)

## Troubleshooting

### Scroll indicator not showing?

- Response might fit entirely in visible area
- Try making terminal window smaller
- Or execute request with larger response

### Can't scroll?

- Make sure you're not in edit mode
- PageUp/PageDown only work in main view
- Check that response has been loaded

### Scroll position jumps?

- New request execution resets scroll to top
- This is intentional behavior
- Ensures you see the beginning of new responses

## Feedback

Scrolling makes it easy to work with large API responses. If you have suggestions for improvements or additional navigation features, let me know!
