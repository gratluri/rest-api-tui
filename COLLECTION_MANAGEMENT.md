# Collection Management Features

## Overview

The REST API TUI now supports full CRUD operations for collections and endpoints directly from the terminal interface!

## Features Added

### Collection Management
- **Create New Collection** - Press `n` from the collection list
- **Edit Collection** - Press `e` on a selected collection
- **Delete Collection** - Press `d` on a selected collection
- **Persistent Storage** - All changes are automatically saved to `~/.rest-api-tui/collections/`

### Endpoint Management
- **Create New Endpoint** - Press `n` from the endpoint list
- **Edit Endpoint** - Press `e` on a selected endpoint
- **Delete Endpoint** - Press `d` on a selected endpoint
- **Full Configuration** - Set name, HTTP method, URL, description, and body template

## Keyboard Shortcuts

### Navigation
- `↑/k` - Move up
- `↓/j` - Move down
- `Enter` - Select/Open item
- `Esc` - Go back
- `q` - Quit (from main screen)

### Collection List Screen
- `n` - Create new collection
- `e` - Edit selected collection
- `d` - Delete selected collection (with confirmation)
- `Enter` - Open collection

### Endpoint List Screen
- `n` - Create new endpoint
- `e` - Edit selected endpoint
- `d` - Delete selected endpoint (with confirmation)
- `Enter` - View endpoint details

### Collection Edit Screen
- Type to enter collection name (ALL characters work, including 'e', 'd', 'n', etc.)
- `Backspace` - Delete character
- `Enter` - Save collection
- `Esc` - Cancel

### Endpoint Edit Screen
- Type to enter text in current field (ALL characters work as input)
- `Tab` - Move to next field
- `Shift+Tab` - Move to previous field
- `m` - Cycle HTTP method (ONLY when on the Method field)
- `Backspace` - Delete character
- `Enter` - Save endpoint
- `Esc` - Cancel

**Note**: The `m` key only cycles the HTTP method when you're on the Method field. In all other fields (Name, URL, Description, Body), `m` is treated as a regular character.

#### Endpoint Fields
1. **Name** - Display name for the endpoint
2. **Method** - HTTP method (use `m` to cycle)
3. **URL** - Full endpoint URL (supports template variables like `{{userId}}`)
4. **Description** - Optional description
5. **Body Template** - Request body template (for POST/PUT/PATCH)

### Endpoint Detail Screen
- `e` - Execute request
- `l` - Start load test
- `Esc` - Go back

## Usage Examples

### Creating a New Collection

1. Start the TUI: `cargo run`
2. Press `n` to create a new collection
3. Type the collection name (e.g., "My API")
4. Press `Enter` to save

### Adding an Endpoint

1. Select a collection and press `Enter`
2. Press `n` to create a new endpoint
3. Fill in the fields:
   - Name: "Get User"
   - Method: Press `m` to select GET
   - URL: "https://api.example.com/users/{{userId}}"
   - Description: "Fetch user by ID"
4. Press `Tab` to move between fields
5. Press `Enter` to save

### Editing an Endpoint

1. Navigate to an endpoint in the list
2. Press `e` to edit
3. Modify any field
4. Press `Enter` to save changes

### Deleting Items

1. Select a collection or endpoint
2. Press `d` to delete
3. **Confirmation dialog appears** showing what will be deleted
4. Press `Y` to confirm or `N`/`Esc` to cancel
5. If confirmed, the item is removed and changes are saved

## Data Persistence

All collections are stored as JSON files in:
```
~/.rest-api-tui/collections/
```

Each collection is saved as a separate file named `{collection-id}.json`.

Changes are saved immediately when you:
- Create a new collection
- Edit a collection name
- Add/edit/delete endpoints
- Delete a collection

## Technical Details

### New Screens
- `CollectionEdit` - Form for creating/editing collections
- `EndpointEdit` - Multi-field form for creating/editing endpoints

### Form State
- `CollectionForm` - Tracks collection name and editing mode
- `EndpointForm` - Tracks all endpoint fields, current field, and editing mode

### Methods Added to AppState
- `start_new_collection()` - Initialize new collection form
- `start_edit_collection(index)` - Load collection for editing
- `save_collection()` - Persist collection changes
- `delete_collection(index)` - Remove collection
- `start_new_endpoint(coll_idx)` - Initialize new endpoint form
- `start_edit_endpoint(coll_idx, ep_idx)` - Load endpoint for editing
- `save_endpoint()` - Persist endpoint changes
- `delete_endpoint(coll_idx, ep_idx)` - Remove endpoint
- `cycle_http_method()` - Rotate through HTTP methods

## Demo

Run the application and try these steps:

```bash
cd rest-api-tui
cargo run
```

1. Press `n` to create a new collection called "Test API"
2. Press `Enter` to save
3. Select the new collection and press `Enter`
4. Press `n` to add a new endpoint
5. Fill in:
   - Name: "Get Posts"
   - Method: GET (default)
   - URL: "https://jsonplaceholder.typicode.com/posts"
6. Press `Enter` to save
7. Select the endpoint and press `Enter` to view details
8. Press `e` to execute the request
9. View the formatted response!

## Next Steps

Future enhancements could include:
- Authentication configuration in the UI
- Header management interface
- Template variable editor
- Import/export collections
- Duplicate collection/endpoint
- Search and filter
