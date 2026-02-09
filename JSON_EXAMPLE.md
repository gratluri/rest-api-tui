# JSON Syntax Highlighting Example

## Before (Plain Text)
```
{
  "user": {
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com",
    "active": true,
    "balance": 1234.56,
    "address": null,
    "roles": ["admin", "user"],
    "metadata": {
      "created": "2024-01-01",
      "verified": false
    }
  }
}
```

## After (With Syntax Highlighting)

### Color Legend
- 🔵 **Cyan** - Outer braces `{ }`
- 🟡 **Yellow** - Nested braces `{ }`
- 🟣 **Magenta** - Array brackets `[ ]`
- 🔷 **Light Blue** - Keys: `"user"`, `"id"`, `"name"`, etc.
- 🟢 **Green** - String values: `"John Doe"`, `"john@example.com"`, `"admin"`, `"user"`
- 🟣 **Magenta** - Numbers: `123`, `1234.56`
- 🟡 **Yellow** - Booleans: `true`, `false`
- 🔴 **Red** - Null: `null`
- ⚫ **Dark Gray** - Punctuation: `:`, `,`

### Visual Representation
```
🔵{
  🔷"user"⚫:🔵 🟡{
    🔷"id"⚫:🔵 🟣123⚫,
    🔷"name"⚫:🔵 🟢"John Doe"⚫,
    🔷"email"⚫:🔵 🟢"john@example.com"⚫,
    🔷"active"⚫:🔵 🟡true⚫,
    🔷"balance"⚫:🔵 🟣1234.56⚫,
    🔷"address"⚫:🔵 🔴null⚫,
    🔷"roles"⚫:🔵 🟣[🟢"admin"⚫,🔵 🟢"user"🟣]⚫,
    🔷"metadata"⚫:🔵 🟢{
      🔷"created"⚫:🔵 🟢"2024-01-01"⚫,
      🔷"verified"⚫:🔵 🟡false
    🟢}
  🟡}
🔵}
```

## Benefits

### 1. Bracket Matching
Notice how matching brackets have the same color:
- Outer `{ }` are both cyan
- User object `{ }` are both yellow
- Metadata object `{ }` are both green
- Roles array `[ ]` are both magenta

This makes it easy to see structure at a glance, especially in deeply nested JSON.

### 2. Data Type Recognition
Different colors for different types help you quickly identify:
- **Keys** (light blue) vs **values**
- **Strings** (green) vs **numbers** (magenta)
- **Booleans** (yellow) for true/false
- **Null values** (red) stand out

### 3. Visual Hierarchy
The rainbow bracket colors create a natural visual hierarchy:
- Level 1: Cyan
- Level 2: Yellow
- Level 3: Magenta
- Level 4: Green
- Level 5: Blue
- And so on...

### 4. Professional Appearance
The syntax highlighting matches what developers expect from modern tools like:
- VS Code
- Postman
- Insomnia
- Browser DevTools

## Real-World Example

When testing an API that returns user data, the colorized JSON makes it immediately obvious:
- Which fields are strings vs numbers
- Where objects are nested
- Which values are null or boolean
- The overall structure of the response

This is especially helpful when:
- Debugging API responses
- Comparing responses from different endpoints
- Validating data types
- Spotting missing or null values
- Understanding complex nested structures

## Technical Notes

### Automatic Detection
The syntax highlighting is automatically applied when:
1. The response has a `Content-Type` header
2. The header contains `application/json`

For non-JSON responses (HTML, XML, plain text), the response is displayed without colorization.

### Performance
The colorization is efficient and handles:
- Large JSON responses (scrolling works normally)
- Deeply nested structures (8 colors cycle)
- Escaped characters in strings
- Unicode characters
- Malformed JSON (gracefully falls back to plain text)

### Compatibility
Works seamlessly with existing features:
- Scrolling (PgUp/PgDn, Home/End)
- Network traffic toggle (t key)
- Response panel resizing
- Split-panel layout
