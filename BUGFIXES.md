# Bug Fixes

## Endpoint Edit Mode Issues (Fixed)

### Bug 1: 'm' Key Not Cycling HTTP Methods (Fixed - Updated)

**Problem:**
When in endpoint edit mode, pressing `m` to cycle between HTTP methods (GET, POST, PUT, etc.) was not working. The 'm' character was being added to the text field instead.

**Root Cause:**
The `m` key handler was placed inside the `else` block for non-edit screens. When in edit mode, ALL characters were being treated as text input, including `m`.

**Initial Fix:**
Moved the `m` key handling to check for endpoint edit mode BEFORE checking if we're in a general edit screen.

**Bug in Initial Fix:**
The initial fix made `m` cycle the method in ALL fields, so typing "m" in the URL field (like ".com") would cycle the method instead of adding "m" to the URL.

**Final Solution:**
Added a check for the current field. Now `m` only cycles the method when you're on field 1 (the Method field). In all other fields, `m` is treated as regular text input.

**Code Change:**
```rust
// Special handling for 'm' in endpoint edit - cycle method ONLY on method field
if c == 'm' && matches!(app.current_screen, Screen::EndpointEdit(_, _)) {
    if let Some(form) = &app.endpoint_form {
        // Only cycle method if we're on the method field (field 1)
        if form.current_field == 1 {
            app.cycle_http_method();
        } else {
            // Otherwise, treat 'm' as regular text input
            // ... add 'm' to current field
        }
    }
}
```

**Testing:**
1. Create or edit an endpoint
2. On the Method field (field 1), press `m` → method cycles
3. Tab to URL field, type "http://www.google.com" → 'm' in "com" is added correctly
4. Tab to Name field, type "my-method" → 'm' is added correctly
5. Verify method only cycles when on the Method field

---

### Bug 2: Shift+Tab Not Moving Cursor in Reverse

**Problem:**
In endpoint edit mode, `Tab` moves forward through fields, but `Shift+Tab` was not moving backward through fields.

**Root Cause:**
The `BackTab` key code (which is what Shift+Tab generates) was not being handled. Only forward `Tab` was implemented.

**Solution:**
Added a new `KeyCode::BackTab` handler that:
1. Checks if we're in endpoint edit mode
2. Decrements the current field index
3. Wraps around from field 0 to field 5 (last field)

**Code Change:**
```rust
KeyCode::BackTab => {
    // Move to previous field in endpoint edit (Shift+Tab)
    if let Screen::EndpointEdit(_, _) = app.current_screen {
        if let Some(form) = &mut app.endpoint_form {
            form.current_field = if form.current_field == 0 {
                5  // Wrap to last field
            } else {
                form.current_field - 1
            };
        }
    }
}
```

**Field Order:**
- 0: Name
- 1: Method (use `m` to cycle, not editable directly)
- 2: URL
- 3: Description
- 4: Headers (not yet implemented)
- 5: Body Template

**Testing:**
1. Create or edit an endpoint
2. Press `Tab` to move forward through fields
3. Press `Shift+Tab` to move backward
4. Verify cursor moves in reverse order
5. Verify wrapping: from Name (field 0) → Body Template (field 5)

---

## Summary of Fixes

| Bug | Status | Impact |
|-----|--------|--------|
| `m` key not cycling methods | ✅ Fixed | High - Core functionality |
| `m` key cycling in wrong fields | ✅ Fixed | High - Data corruption |
| Shift+Tab not working | ✅ Fixed | Medium - UX improvement |

## Related Files

- `rest-api-tui/src/tui/ui.rs` - Keyboard handling
- `rest-api-tui/src/tui_app.rs` - Method cycling logic
- `rest-api-tui/COLLECTION_MANAGEMENT.md` - Updated documentation

## Testing Checklist

- [x] `m` key cycles HTTP methods ONLY when on Method field (field 1)
- [x] `m` key works as text input in Name field
- [x] `m` key works as text input in URL field (e.g., ".com")
- [x] `m` key works as text input in Description field
- [x] `m` key works as text input in Body Template field
- [x] `Tab` moves forward through fields
- [x] `Shift+Tab` moves backward through fields
- [x] Field wrapping works in both directions
- [x] All other keyboard shortcuts still work

## Future Improvements

Possible enhancements:
- Visual indicator showing current HTTP method is selected
- Arrow keys to cycle through methods
- Direct number keys to select method (1=GET, 2=POST, etc.)
- Field 4 (Headers) implementation for key-value pairs
- Better visual feedback when cycling methods


## Bug #3: 'h' Key Activates Header Mode from Any Field (2024-02-07)

**Issue**: When typing in URL field (e.g., "http://example.com"), pressing 'h' would activate header edit mode instead of adding 'h' to the URL. This made it impossible to type URLs containing 'h'.

**Root Cause**: The keyboard handler checked if the user pressed 'h' in endpoint edit mode, but didn't verify that the current field was the Headers field (field 4). It would activate header mode from any field.

**Fix**: Added field check to only activate header mode when:
1. User presses 'h'
2. Currently on the Headers field (field 4)
3. Not already in header edit mode

**Code Changes**:
- `src/tui/ui.rs`: Updated 'h' key handler to check `form.current_field == 4`
- `src/tui/ui.rs`: Updated UI to show "[Tab to this field, then 'h' to add]" when not on Headers field
- `src/tui/ui.rs`: Added BackTab support in header edit mode for reverse navigation

**Testing**: Manual testing confirmed:
- Can now type 'h' in Name, URL, Description, and Body fields
- 'h' only activates header mode when on Headers field
- Tab/Shift+Tab work correctly in both normal and header edit modes
- All 68 unit tests still pass


## Bug #4: Last Line Not Visible When Scrolling (2024-02-08)

**Issue**: When scrolling through a large response using PageDown, the very last line of the response was not reachable. Users could scroll close to the end but couldn't see the final line.

**Root Cause**: Off-by-one error in the `max_scroll` calculation. The code used `saturating_sub()` which could produce incorrect results in edge cases. The calculation didn't properly account for ensuring the last line is always reachable.

**Example**:
- Total lines: 100
- Visible height: 20
- Old max_scroll: `100.saturating_sub(20) = 80`
- Problem: When scrolled to offset 80, shows lines 80-99 (20 lines), but line 100 is never visible

**Fix**: Changed the max_scroll calculation to explicitly handle the case:
```rust
// Before (buggy)
let max_scroll = total_lines.saturating_sub(visible_height);

// After (fixed)
let max_scroll = if total_lines > visible_height {
    total_lines - visible_height
} else {
    0
};
```

**Why This Works**:
- When `total_lines = 100` and `visible_height = 20`:
  - `max_scroll = 100 - 20 = 80`
  - At offset 80: shows lines 80-99 (indices 80-99, which is 20 lines)
  - This correctly shows the last line (line 99, index 99)
- When `total_lines <= visible_height`:
  - `max_scroll = 0` (no scrolling needed)
  - All lines fit in visible area

**Testing**: Manual testing confirmed:
- Can now scroll to see the very last line of any response
- Scroll indicator correctly shows `[81/100]` when at the end
- No off-by-one errors in scroll position
- Works correctly with both network traffic enabled and disabled

**Code Changes**:
- `src/tui/ui.rs`: Fixed max_scroll calculation in both scroll locations (with and without network traffic)
