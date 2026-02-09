# Debugging JSON Syntax Highlighting

## How to Verify It's Working

### 1. Visual Indicators
When JSON highlighting is active, you should see:
- **🎨 JSON** indicator in the response panel title
- Example: `✓ Response: 200 OK - 45ms - 1234 bytes 🎨 JSON [t: show traffic | PgUp/PgDn: scroll]`

### 2. Color Verification
In a JSON response, you should see:
- **Braces `{}`**: Colored (cyan, yellow, magenta, green, blue, etc.)
- **Brackets `[]`**: Colored (same as braces)
- **Keys**: Light blue (e.g., `"name"`, `"id"`)
- **String values**: Green (e.g., `"John Doe"`)
- **Numbers**: Magenta (e.g., `123`, `3.14`)
- **Booleans**: Yellow (`true`, `false`)
- **Null**: Red (`null`)
- **Punctuation**: Dark gray (`:`, `,`)

### 3. Test Endpoints
Use these public APIs to test JSON responses:

#### JSONPlaceholder (Fake REST API)
```
GET https://jsonplaceholder.typicode.com/users/1
GET https://jsonplaceholder.typicode.com/posts/1
GET https://jsonplaceholder.typicode.com/todos/1
```

#### HTTPBin (HTTP Testing Service)
```
GET https://httpbin.org/json
GET https://httpbin.org/get
POST https://httpbin.org/post
```

## Troubleshooting

### Issue: No 🎨 JSON indicator appears

**Possible causes:**
1. Response Content-Type header is not set to JSON
2. Content-Type header uses different casing

**Solution:**
Check the response headers. The Content-Type must contain "json" (case-insensitive).

Valid Content-Type values:
- `application/json`
- `application/json; charset=utf-8`
- `Application/JSON`
- `text/json`

### Issue: 🎨 JSON appears but no colors

**Possible causes:**
1. Terminal doesn't support colors
2. Response body is not valid JSON
3. Response body is empty

**Solution:**
1. Verify your terminal supports 256 colors
2. Check the response body is valid JSON
3. Try a different endpoint

### Issue: Colors appear but brackets don't match

**Possible causes:**
1. Malformed JSON (missing closing braces)
2. JSON contains escaped quotes

**Solution:**
This is expected behavior for malformed JSON. The colorizer will do its best but may not match brackets correctly if the JSON is invalid.

## Testing with Network Traffic

The JSON highlighting should work both:
1. **Without network traffic** (default view)
2. **With network traffic** (press 't' to toggle)

When network traffic is displayed:
- Top panel: Response body with JSON highlighting
- Bottom panel: Network traffic details

Both views should show the 🎨 JSON indicator if the response is JSON.

## Code Changes Made

### 1. Improved JSON Detection
Changed from:
```rust
let is_json = response.headers.iter()
    .any(|(k, v)| k.to_lowercase() == "content-type" && v.contains("application/json"));
```

To:
```rust
let is_json = response.headers.iter()
    .any(|(k, v)| k.to_lowercase() == "content-type" && v.to_lowercase().contains("json"));
```

This now:
- Converts both key and value to lowercase
- Checks for "json" anywhere in the value (not just "application/json")
- Handles variations like "text/json", "Application/JSON", etc.

### 2. Added Visual Indicator
Added 🎨 JSON to the response title when JSON is detected:
```rust
let json_indicator = if is_json { " 🎨 JSON" } else { "" };
```

## Expected Behavior

### Scenario 1: JSON Response (No Network Traffic)
```
✓ Response: 200 OK - 45ms - 1234 bytes 🎨 JSON [t: show traffic | PgUp/PgDn: scroll]
┌─────────────────────────────────────────────────────────────┐
│ {                                                           │ <- Cyan
│   "user": {                                                 │ <- "user" in light blue, { in yellow
│     "id": 123,                                              │ <- "id" in light blue, 123 in magenta
│     "name": "John Doe"                                      │ <- "name" in light blue, "John Doe" in green
│   }                                                         │ <- } in yellow
│ }                                                           │ <- } in cyan
└─────────────────────────────────────────────────────────────┘
```

### Scenario 2: JSON Response (With Network Traffic)
```
✓ Response: 200 OK - 45ms - 1234 bytes 🎨 JSON [t: hide traffic | PgUp/PgDn: scroll]
┌─────────────────────────────────────────────────────────────┐
│ {                                                           │ <- Colored JSON
│   "user": { ... }                                           │
└─────────────────────────────────────────────────────────────┘

📡 Network Traffic (Wireshark-style)
┌─────────────────────────────────────────────────────────────┐
│ ⏱️  Timing Breakdown:                                       │
│   🔍 DNS Lookup: 5ms                                        │
│   ...                                                       │
└─────────────────────────────────────────────────────────────┘
```

### Scenario 3: Non-JSON Response
```
✓ Response: 200 OK - 45ms - 1234 bytes [t: show traffic | PgUp/PgDn: scroll]
┌─────────────────────────────────────────────────────────────┐
│ <html>                                                      │ <- Plain text, no colors
│   <body>Hello World</body>                                  │
│ </html>                                                     │
└─────────────────────────────────────────────────────────────┘
```

## Quick Test

1. Start the application: `cargo run --release`
2. Create a test endpoint:
   - URL: `https://jsonplaceholder.typicode.com/users/1`
   - Method: GET
3. Execute the request (press 'e')
4. Look for **🎨 JSON** in the response title
5. Verify colors in the response body
6. Press 't' to toggle network traffic
7. Verify colors still appear in the top panel

If you see the 🎨 JSON indicator and colors, it's working!
