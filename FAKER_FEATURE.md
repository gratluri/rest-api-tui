# Faker Library Integration

## Overview

The REST API TUI now supports dynamic fake data generation using the `{{f:variablename}}` syntax. This allows you to generate realistic test data without manually creating variables.

## Syntax

- **User Variables**: `{{VARIABLE_NAME}}` - Uses saved variable values
- **Faker Variables**: `{{f:firstname}}` - Generates fake data dynamically

## Supported Faker Variables

### Names
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:firstname}}` | John | First name |
| `{{f:lastname}}` | Doe | Last name |
| `{{f:fullname}}` | John Doe | Full name |
| `{{f:name}}` | John Doe | Same as fullname |
| `{{f:title}}` | Mr. | Name title |
| `{{f:suffix}}` | Jr. | Name suffix |

### Internet
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:email}}` | john.doe@example.com | Safe email address |
| `{{f:username}}` | johndoe123 | Username |
| `{{f:password}}` | aB3$xY9z | Random password (8-16 chars) |
| `{{f:domain}}` | example.com | Domain name |
| `{{f:ipv4}}` | 192.168.1.1 | IPv4 address |
| `{{f:ipv6}}` | 2001:0db8::1 | IPv6 address |
| `{{f:useragent}}` | Mozilla/5.0... | User agent string |
| `{{f:url}}` | https://example.com | URL |

### Phone
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:phone}}` | (555) 123-4567 | Phone number |
| `{{f:cellnumber}}` | (555) 987-6543 | Cell number |

### Address
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:street}}` | Main Street | Street name |
| `{{f:city}}` | New York | City name |
| `{{f:state}}` | California | State name |
| `{{f:stateabbr}}` | CA | State abbreviation |
| `{{f:zipcode}}` | 12345 | ZIP code |
| `{{f:country}}` | United States | Country name |
| `{{f:countrycode}}` | US | Country code |
| `{{f:latitude}}` | 40.7128 | Latitude |
| `{{f:longitude}}` | -74.0060 | Longitude |

### Company
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:company}}` | Acme Corp | Company name |
| `{{f:companysuffix}}` | Inc. | Company suffix |
| `{{f:industry}}` | Technology | Industry |
| `{{f:profession}}` | Software Engineer | Profession |

### Lorem Ipsum
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:word}}` | lorem | Single word |
| `{{f:words}}` | lorem ipsum dolor | 3-5 words |
| `{{f:sentence}}` | Lorem ipsum dolor sit amet. | Sentence |
| `{{f:sentences}}` | Lorem ipsum... Dolor sit... | 2-4 sentences |
| `{{f:paragraph}}` | Lorem ipsum dolor... | Paragraph |
| `{{f:paragraphs}}` | Lorem ipsum...\n\nDolor sit... | 2-4 paragraphs |

### Numbers
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:number}}` | 42 | Random integer (1-1000) |
| `{{f:float}}` | 123.45 | Random float (1.0-1000.0) |
| `{{f:digit}}` | 7 | Single digit (0-9) |
| `{{f:boolean}}` | true | Boolean value |

### Date/Time
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:date}}` | 2024-03-15 | Date (YYYY-MM-DD) |
| `{{f:datetime}}` | 2024-03-15 14:30:00 | Date and time |
| `{{f:timestamp}}` | 2024-03-15 14:30:00 | Same as datetime |
| `{{f:time}}` | 14:30:00 | Time (HH:MM:SS) |

### Other
| Variable | Example Output | Description |
|----------|---------------|-------------|
| `{{f:uuid}}` | 550e8400-e29b-41d4-a716-446655440000 | UUID v4 |
| `{{f:color}}` | blue | Color name |
| `{{f:hexcolor}}` | #3A5FCD | Hex color code |

## Usage Examples

### Example 1: Create User Endpoint
```
POST https://api.example.com/users
Content-Type: application/json

{
  "firstName": "{{f:firstname}}",
  "lastName": "{{f:lastname}}",
  "email": "{{f:email}}",
  "phone": "{{f:phone}}",
  "address": {
    "street": "{{f:street}}",
    "city": "{{f:city}}",
    "state": "{{f:state}}",
    "zip": "{{f:zipcode}}"
  }
}
```

Each time you execute this request, it generates new random data!

### Example 2: Mixed Variables
```
POST https://api.example.com/{{API_VERSION}}/users
Authorization: Bearer {{AUTH_TOKEN}}
Content-Type: application/json

{
  "name": "{{f:fullname}}",
  "email": "{{f:email}}",
  "company": "{{f:company}}",
  "userId": "{{USER_ID}}"
}
```

This mixes:
- User variables: `{{API_VERSION}}`, `{{AUTH_TOKEN}}`, `{{USER_ID}}`
- Faker variables: `{{f:fullname}}`, `{{f:email}}`, `{{f:company}}`

### Example 3: Load Testing with Unique Data
```
POST https://api.example.com/orders
Content-Type: application/json

{
  "orderId": "{{f:uuid}}",
  "customerName": "{{f:fullname}}",
  "email": "{{f:email}}",
  "amount": {{f:float}},
  "timestamp": "{{f:datetime}}"
}
```

Perfect for load testing - each request has unique data!

## How It Works

1. **Detection**: The template engine detects `{{f:variablename}}` syntax
2. **Generation**: Calls the faker library to generate appropriate fake data
3. **Substitution**: Replaces the variable with the generated value during request execution
4. **Execution**: Sends the request with the substituted data

### Quick Execute ('x') Mode

Faker variables work seamlessly with quick execute:
- Faker variables (`{{f:firstname}}`) are automatically detected and skipped during variable validation
- They are generated fresh during request execution
- No need to define them in the variable manager
- Each execution generates new random data

### Traditional Execute ('e') Mode

Faker variables also work in traditional execute mode:
- Shows the variable input screen for user variables only
- Faker variables are generated automatically
- You can review user variables before execution

## Benefits

1. **No Manual Setup**: No need to create variables for test data
2. **Dynamic Data**: Each execution generates new data
3. **Realistic Testing**: Uses realistic-looking data
4. **Load Testing**: Perfect for generating unique data in load tests
5. **Quick Prototyping**: Rapidly test APIs without preparing test data

## Error Handling

If you use an unknown faker variable:
```
{{f:unknowntype}}
```

You'll get an error:
```
Unknown faker variable: unknowntype
```

Check the supported variables list above.

## Case Insensitive

Faker variables are case-insensitive:
- `{{f:FirstName}}` ✅
- `{{f:firstname}}` ✅
- `{{f:FIRSTNAME}}` ✅

All work the same!

## Tips

1. **Use for Testing**: Great for creating test users, orders, etc.
2. **Load Testing**: Generates unique data for each request
3. **Mix with User Variables**: Combine faker and user variables
4. **Check Output**: First execution shows what data looks like
5. **Consistent Format**: Faker generates data in consistent formats

## Implementation

- **Library**: Uses the `fake` crate (v2.9)
- **Module**: `src/faker.rs`
- **Integration**: `src/template.rs` handles substitution
- **Features**: Names, internet, phone, address, company, lorem, numbers, dates, UUIDs

## Future Enhancements

Potential additions:
- Custom faker patterns (e.g., `{{f:number:1-100}}`)
- Locale support (e.g., `{{f:name:fr}}` for French names)
- Seed support for reproducible data
- More data types (credit cards, IBANs, etc.)
