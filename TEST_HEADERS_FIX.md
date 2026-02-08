# Testing the Headers Feature Fix

## Quick Test Guide

### Test 1: Type URLs with 'h'
1. Run the app: `cargo run`
2. Press `n` to create a new collection
3. Type "Test Collection" and press Enter
4. Press `n` to create a new endpoint
5. Tab to the URL field (field 2)
6. Type: `http://example.com`
7. **Expected**: All characters including 'h' should appear in the URL
8. **Before fix**: Pressing 'h' would activate header mode
9. **After fix**: 'h' is added to the URL normally

### Test 2: Add Headers (Correct Way)
1. Continue from Test 1 (or create a new endpoint)
2. Press Tab until you reach the Headers field (field 4)
3. Notice the instruction changes to: "[press 'h' to add]"
4. Press `h` to enter header edit mode
5. Type header key: `Content-Type`
6. Press Tab to move to value field
7. Type header value: `application/json`
8. Press Enter to add the header
9. **Expected**: Header is added and shown in the list
10. Press Esc to exit header mode

### Test 3: Multiple Headers
1. Continue from Test 2
2. Make sure you're on the Headers field (Tab if needed)
3. Press `h` to enter header edit mode
4. Add header: `Authorization` = `Bearer token123`
5. Press Enter
6. Add header: `X-API-Key` = `my-key`
7. Press Enter
8. Press Esc to exit header mode
9. **Expected**: All 3 headers are shown in the list

### Test 4: Field Navigation
1. Create a new endpoint
2. Test Tab navigation:
   - Field 0 (Name) → Tab → Field 1 (Method)
   - Field 1 (Method) → Tab → Field 2 (URL)
   - Field 2 (URL) → Tab → Field 3 (Description)
   - Field 3 (Description) → Tab → Field 4 (Headers)
   - Field 4 (Headers) → Tab → Field 5 (Body)
   - Field 5 (Body) → Tab → Field 0 (Name)
3. Test Shift+Tab (reverse):
   - Field 0 (Name) → Shift+Tab → Field 5 (Body)
   - Field 5 (Body) → Shift+Tab → Field 4 (Headers)
   - etc.
4. **Expected**: Navigation works smoothly in both directions

### Test 5: Header Edit Mode Navigation
1. Tab to Headers field
2. Press `h` to enter header edit mode
3. Type some text in Key field
4. Press Tab - should move to Value field
5. Type some text in Value field
6. Press Shift+Tab - should move back to Key field
7. **Expected**: Tab and Shift+Tab work in header edit mode

### Test 6: Type 'h' in Other Fields
1. Create a new endpoint
2. In Name field, type: "Get Health Status"
3. Tab to URL field, type: "https://api.example.com"
4. Tab to Description field, type: "This checks the health"
5. Tab to Body field, type: "{ \"check\": \"health\" }"
6. **Expected**: All 'h' characters appear as text, header mode never activates

## Expected Behavior Summary

| Field | Pressing 'h' | Expected Result |
|-------|-------------|-----------------|
| Name (0) | Type 'h' | Adds 'h' to name |
| Method (1) | Type 'h' | No effect (method field) |
| URL (2) | Type 'h' | Adds 'h' to URL |
| Description (3) | Type 'h' | Adds 'h' to description |
| Headers (4) | Press 'h' | **Activates header edit mode** |
| Body (5) | Type 'h' | Adds 'h' to body |

## Common URLs to Test

Try typing these URLs to verify 'h' works correctly:
- `http://example.com`
- `https://api.github.com`
- `http://localhost:8080`
- `https://httpbin.org/get`
- `http://jsonplaceholder.typicode.com/posts`

## Visual Indicators

### When NOT on Headers field:
```
Headers: (0) [Tab to this field, then 'h' to add]
```

### When ON Headers field:
```
Headers: (0) [press 'h' to add]
```
(Notice the text is highlighted in yellow)

### When IN header edit mode:
```
Headers (Edit Mode):

  Key: Content-Type_
  Value: 

  Tab: switch field | Enter: add | Esc: cancel
```

## Regression Check

Make sure these still work:
- ✅ 'm' key cycles HTTP method (only on Method field)
- ✅ 'n' creates new collection/endpoint (from list screens)
- ✅ 'e' edits collection/endpoint (from list screens)
- ✅ 'd' deletes collection/endpoint (from list screens)
- ✅ Delete confirmation dialog works
- ✅ All text input works in edit forms
- ✅ Esc cancels edit forms
- ✅ Enter saves forms

## Success Criteria

✅ Can type 'h' in Name, URL, Description, and Body fields
✅ 'h' only activates header mode when on Headers field
✅ Tab/Shift+Tab navigation works in all modes
✅ Headers can be added successfully
✅ Multiple headers can be added
✅ All 68 unit tests pass
✅ No other functionality is broken

## Run the Tests

```bash
# Unit tests
cargo test

# Build and run the app
cargo run

# Or run the release build
cargo build --release
./target/release/rest-api-tui
```
