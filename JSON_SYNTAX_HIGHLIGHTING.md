# JSON Syntax Highlighting Feature

## Overview
Added comprehensive JSON syntax highlighting with rainbow bracket matching to the REST API TUI application. When the response Content-Type is `application/json`, the response body is automatically colorized for better readability.

## Features Implemented

### 1. Rainbow Bracket Matching
- Matching braces `{}` and brackets `[]` are colored with the same color
- Uses 8 different colors that cycle based on nesting depth:
  - Cyan → Yellow → Magenta → Green → Blue → Light Cyan → Light Yellow → Light Magenta
- Brackets are rendered in **bold** for better visibility

### 2. Syntax Element Coloring

| Element | Color | Example |
|---------|-------|---------|
| Keys | Light Blue | `"name"` |
| String Values | Green | `"John Doe"` |
| Numbers | Magenta | `42`, `3.14` |
| Booleans | Yellow | `true`, `false` |
| Null | Red | `null` |
| Punctuation | Dark Gray | `:`, `,` |

### 3. Automatic Detection
- Checks the `Content-Type` header for `application/json`
- Falls back to plain text rendering for non-JSON responses
- Works seamlessly with existing scrolling and network traffic features

## Example

```json
{
  "user": {
    "id": 123,
    "name": "John Doe",
    "active": true,
    "email": null,
    "scores": [95, 87, 92]
  }
}
```

In the TUI, this would render with:
- `{` and `}` in cyan (outer level)
- `{` and `}` in yellow (nested level)
- `[` and `]` in magenta (array level)
- `"user"`, `"id"`, `"name"`, etc. in light blue (keys)
- `"John Doe"` in green (string value)
- `123`, `95`, `87`, `92` in magenta (numbers)
- `true` in yellow (boolean)
- `null` in red (null value)
- `:` and `,` in dark gray (punctuation)

## Implementation Details

### Function: `colorize_json()`
Located in `src/tui/ui.rs`, this function:
1. Parses JSON text character by character
2. Maintains a stack of brace colors for matching
3. Detects string boundaries and distinguishes keys from values
4. Handles escape sequences properly
5. Returns a `Vec<Line<'_>>` with styled spans

### Integration
- Modified `draw_response_panel()` to detect JSON content type
- Applies colorization before scrolling logic
- Maintains compatibility with existing features (scrolling, network traffic toggle)

## Benefits
1. **Improved Readability**: Color-coded syntax makes JSON structure immediately visible
2. **Bracket Matching**: Rainbow colors help identify matching braces at a glance
3. **Visual Hierarchy**: Different colors for different data types aid comprehension
4. **Professional Look**: Matches the enhanced UI with icons and rounded borders

## Future Enhancements (Optional)
- Add syntax highlighting for XML responses
- Support for YAML responses
- Configurable color schemes
- Toggle syntax highlighting on/off with a keyboard shortcut
