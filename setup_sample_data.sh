#!/bin/bash

# Create sample collection directory
mkdir -p ~/.rest-api-tui/collections

# Create sample collection
cat > ~/.rest-api-tui/collections/sample.json << 'EOF'
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Sample APIs",
  "created_at": "2026-02-07T00:00:00Z",
  "updated_at": "2026-02-07T00:00:00Z",
  "endpoints": [
    {
      "id": "660e8400-e29b-41d4-a716-446655440001",
      "name": "Get Post",
      "method": "GET",
      "url": "https://jsonplaceholder.typicode.com/posts/1",
      "headers": {
        "Accept": "application/json"
      },
      "body_template": null,
      "auth": null,
      "description": "Fetch a single post"
    },
    {
      "id": "770e8400-e29b-41d4-a716-446655440002",
      "name": "List Posts",
      "method": "GET",
      "url": "https://jsonplaceholder.typicode.com/posts",
      "headers": {
        "Accept": "application/json"
      },
      "body_template": null,
      "auth": null,
      "description": "Fetch all posts"
    },
    {
      "id": "880e8400-e29b-41d4-a716-446655440003",
      "name": "Create Post",
      "method": "POST",
      "url": "https://jsonplaceholder.typicode.com/posts",
      "headers": {
        "Content-Type": "application/json"
      },
      "body_template": "{\"title\": \"Test Post\", \"body\": \"This is a test\", \"userId\": 1}",
      "auth": null,
      "description": "Create a new post"
    }
  ]
}
EOF

echo "✓ Sample collection created at ~/.rest-api-tui/collections/sample.json"
