# Headers and Authentication

## Custom Headers

You can now add custom HTTP headers to your endpoints directly from the TUI!

### Adding Headers

1. Create or edit an endpoint
2. Press `h` to enter header edit mode
3. Type the header key (e.g., "Content-Type")
4. Press `Tab` to move to the value field
5. Type the header value (e.g., "application/json")
6. Press `Enter` to add the header
7. Press `Esc` to exit header edit mode

### Header Edit Mode

When you press `h`, the screen changes to show:

```
Headers (Edit Mode):

  Key: Content-Type_
  Value: 

  Tab: switch field | Enter: add | Esc: cancel
```

### Viewing Headers

When not in header edit mode, you'll see all existing headers:

```
Headers: (2) [press 'h' to add]
  Content-Type: application/json
  Authorization: Bearer token123
```

### Keyboard Shortcuts

**In Normal Mode:**
- `h` - Enter header edit mode

**In Header Edit Mode:**
- Type - Enter key or value
- `Tab` - Switch between Key and Value fields
- `Backspace` - Delete character
- `Enter` - Add header and stay in edit mode
- `Esc` - Exit header edit mode

### Example Workflow

Adding multiple headers:

1. Press `h` to enter header mode
2. Type "Content-Type", Tab, "application/json", Enter
3. Type "Authorization", Tab, "Bearer abc123", Enter
4. Type "X-API-Key", Tab, "my-key", Enter
5. Press `Esc` to exit header mode
6. Press `Enter` to save the endpoint

### Common Headers

Some commonly used headers:
- `Content-Type`: `application/json`, `application/xml`, `text/plain`
- `Authorization`: `Bearer <token>`, `Basic <credentials>`
- `Accept`: `application/json`, `*/*`
- `User-Agent`: `MyApp/1.0`
- `X-API-Key`: `your-api-key`
- `X-Request-ID`: `unique-id`

## Authentication

### Current Status

Authentication configuration is currently **only available via JSON file editing**. The TUI does not yet have a UI for configuring authentication.

### Supported Auth Types

The application supports three authentication types:

#### 1. API Key

Can be sent as a header or query parameter.

**JSON Format:**
```json
{
  "auth": {
    "type": "ApiKey",
    "name": "X-API-Key",
    "value": "your-api-key-here",
    "location": "Header"
  }
}
```

Or as query parameter:
```json
{
  "auth": {
    "type": "ApiKey",
    "name": "api_key",
    "value": "your-api-key-here",
    "location": "QueryParam"
  }
}
```

#### 2. Bearer Token

For OAuth 2.0 and similar token-based auth.

**JSON Format:**
```json
{
  "auth": {
    "type": "Bearer",
    "token": "your-bearer-token-here"
  }
}
```

#### 3. Basic Authentication

Username and password encoded in Base64.

**JSON Format:**
```json
{
  "auth": {
    "type": "Basic",
    "username": "your-username",
    "password": "your-password"
  }
}
```

### How to Add Auth (Manual)

1. Locate your collection file in `~/.rest-api-tui/collections/`
2. Open the JSON file in a text editor
3. Find the endpoint you want to add auth to
4. Add the `auth` field with one of the formats above
5. Save the file
6. Reload the TUI or restart the application

### Example Endpoint with Auth

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Get Protected Resource",
  "method": "GET",
  "url": "https://api.example.com/protected",
  "headers": {
    "Content-Type": "application/json"
  },
  "body_template": null,
  "auth": {
    "type": "Bearer",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  },
  "description": "Requires authentication"
}
```

### Future Enhancement: Auth UI

A future update will add a UI for configuring authentication directly in the TUI, similar to the header editor. This would include:

- Press `a` to enter auth configuration mode
- Select auth type (API Key, Bearer, Basic)
- Fill in required fields
- Save auth configuration

**Would you like me to implement the Auth UI now?** Let me know if this is a priority!

## Workaround: Using Headers for Auth

Until the Auth UI is implemented, you can use custom headers for many authentication scenarios:

### Bearer Token via Header
Instead of using the `auth` field, add a header:
- Key: `Authorization`
- Value: `Bearer your-token-here`

### API Key via Header
Add a header:
- Key: `X-API-Key` (or whatever your API uses)
- Value: `your-api-key`

### Basic Auth via Header
You'll need to Base64 encode `username:password` first, then add:
- Key: `Authorization`
- Value: `Basic base64-encoded-credentials`

This approach works for most APIs and can be done entirely through the TUI!

## Summary

| Feature | Status | How to Use |
|---------|--------|------------|
| Custom Headers | ✅ Available | Press `h` in endpoint edit |
| API Key Auth | ⚠️ JSON only | Edit collection file manually |
| Bearer Token Auth | ⚠️ JSON only | Edit collection file manually |
| Basic Auth | ⚠️ JSON only | Edit collection file manually |
| Auth via Headers | ✅ Workaround | Use custom headers |

## Testing

Try adding these headers to an endpoint:

1. **JSON API**:
   - Content-Type: application/json
   - Accept: application/json

2. **With API Key**:
   - X-API-Key: test-key-123
   - Content-Type: application/json

3. **With Bearer Token**:
   - Authorization: Bearer eyJhbGc...
   - Content-Type: application/json

Then execute the request and verify the headers are sent correctly!
