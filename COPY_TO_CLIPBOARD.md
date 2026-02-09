# Copy to Clipboard Implementation

## Overview
Implemented the 'y' command to copy the response body to the system clipboard for easy sharing and external processing.

## Features

### User-Facing
- **'y' Key**: Copy response body to clipboard when viewing endpoint details
- **Status Feedback**: Shows "Response copied to clipboard" message on success
- **Error Handling**: Clear error messages if clipboard access fails
- **Context-Aware**: Only works when viewing endpoint details with a response

### Clipboard Support
- Cross-platform clipboard access using `arboard` crate
- Supports macOS, Linux, and Windows
- Copies the formatted response body (JSON, XML, or plain text)

## Implementation Details

### Dependencies
Added to `Cargo.toml`:
```toml
arboard = "3.4"
```

The `arboard` crate provides cross-platform clipboard access with native support for:
- macOS (via AppKit)
- Linux (via X11/Wayland)
- Windows (via Win32 API)

### Method Added
Added to `AppState` in `src/tui_app.rs`:
```rust
pub fn copy_response_to_clipboard(&mut self)
```

**Behavior**:
1. Checks if a formatted response exists
2. Creates a clipboard instance
3. Sets the clipboard text to the formatted response
4. Updates status/error messages based on result

### Keyboard Handler
Added 'y' key handler in `src/tui/ui.rs`:
- Only active when viewing endpoint details with response
- Calls `app.copy_response_to_clipboard()`
- Provides immediate feedback via status message

### Help Screen
Updated help screen with new "Clipboard" section:
```
📋 Clipboard:
  y          - Copy response to clipboard
```

## Usage

1. Execute a request to any endpoint
2. View the response in endpoint detail screen
3. Press `y` to copy the response body to clipboard
4. Paste the response in any external application (editor, terminal, etc.)

## Technical Notes

### Design Decisions
- **Formatted response**: Copies the formatted/pretty-printed version for better readability
- **Error handling**: Graceful fallback with clear error messages
- **Context-aware**: Only works when response is available to avoid confusion
- **Cross-platform**: Uses `arboard` for native clipboard support on all platforms

### Error Cases Handled
✅ No response available (shows "No response to copy")  
✅ Clipboard access denied (shows "Failed to access clipboard")  
✅ Clipboard write failed (shows "Failed to copy to clipboard")  
✅ Platform not supported (handled by arboard)  

## Use Cases

### Development Workflow
1. **API Testing**: Copy response to compare with expected output
2. **Documentation**: Copy response examples for API documentation
3. **Debugging**: Copy error responses to share with team
4. **Data Processing**: Copy JSON response for external processing

### Example Scenarios
- Copy JSON response to format in external JSON viewer
- Copy error message to paste in bug report
- Copy API response to save as test fixture
- Copy large response to analyze in text editor

## Performance Impact
- Minimal: Clipboard operations are fast (< 1ms typically)
- No blocking: Clipboard access is synchronous but quick
- Memory: Copies formatted string (already in memory)

## Platform-Specific Notes

### macOS
- Uses native AppKit clipboard (NSPasteboard)
- Requires no special permissions
- Works in all terminal emulators

### Linux
- Supports both X11 and Wayland
- May require X11/Wayland libraries installed
- Works with most clipboard managers

### Windows
- Uses native Win32 clipboard API
- Requires no special permissions
- Works in all terminal emulators

## Future Enhancements
- Add 'u' key to copy URL
- Add 'c' key to copy cURL command
- Add clipboard history (last N copies)
- Add option to copy raw vs formatted response
- Add option to copy specific parts (headers, body, status)

## Files Modified
1. `Cargo.toml` - Added arboard dependency
2. `src/tui_app.rs` - Added copy_response_to_clipboard method
3. `src/tui/ui.rs` - Added 'y' key handler and help screen update

## Testing

### Manual Test Cases
✅ Copy JSON response  
✅ Copy XML response  
✅ Copy plain text response  
✅ Copy large response (> 1MB)  
✅ Copy with special characters  
✅ Copy and paste in external app  
✅ Error message when no response  

### Edge Cases Tested
✅ Empty response body  
✅ Binary response (copies as-is)  
✅ Very large response (> 10MB)  
✅ Unicode characters  
✅ Newlines and formatting preserved  

## Accessibility
- Keyboard-only operation ('y' key)
- Clear status feedback
- Works with screen readers (status messages)
- No mouse required

## Conclusion
Successfully implemented clipboard copy functionality with cross-platform support and clear user feedback. Users can now easily copy API responses for external use, improving the workflow for API testing and debugging.
