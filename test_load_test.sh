#!/bin/bash

# Test script for load test functionality

echo "🧪 Testing Load Test Fix"
echo "========================"
echo ""

echo "1. Building project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi
echo "✅ Build successful"
echo ""

echo "2. Running tests..."
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "❌ Tests failed"
    exit 1
fi
echo "✅ All 68 tests passed"
echo ""

echo "3. Ready to test load test feature!"
echo ""
echo "Manual test steps:"
echo "  1. Run: ./target/release/rest-api-tui"
echo "  2. Press 'n' to create a collection"
echo "  3. Name it 'Test API' and press Enter"
echo "  4. Press Ctrl+l to switch to endpoints panel"
echo "  5. Press 'n' to create an endpoint"
echo "  6. Fill in:"
echo "     - Name: Get Users"
echo "     - Method: GET (press 'm' to cycle)"
echo "     - URL: https://jsonplaceholder.typicode.com/users"
echo "  7. Press Enter to save"
echo "  8. Press Enter to select the endpoint"
echo "  9. Press 'l' to start load test"
echo "  10. Watch the metrics update!"
echo "  11. Press Esc to stop (or wait 30 seconds)"
echo ""
echo "Expected results:"
echo "  - Total requests: ~300"
echo "  - Success rate: 100%"
echo "  - RPS: ~10"
echo "  - Duration: 30 seconds"
echo ""
echo "🚀 Run the app now: ./target/release/rest-api-tui"
