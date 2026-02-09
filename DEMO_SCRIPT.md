# REST API TUI - Demo Script

## Pre-Demo Setup

Before recording, prepare the following:

1. **Clear your terminal history**: `clear`
2. **Set terminal size**: Resize to 120x30 for optimal viewing
3. **Start the app**: `cargo run --release`
4. **Have a test API ready**: We'll use JSONPlaceholder (https://jsonplaceholder.typicode.com)

## Demo Flow (5-7 minutes)

### Part 1: Introduction (30 seconds)
```
"Welcome to REST API TUI - a powerful terminal-based API testing tool"
"Let me show you what makes it special..."
```

### Part 2: Basic Workflow (1 minute)
1. **Create a Collection**
   - Press `n` → Type "Demo API" → Enter
   - Show: "Collections organize your endpoints"

2. **Add an Endpoint**
   - Press `Ctrl+l` (switch to endpoints)
   - Press `n` (new endpoint)
   - Name: "Get User"
   - Method: Press `m` to cycle to GET
   - URL: `https://jsonplaceholder.typicode.com/users/1`
   - Press Enter to save

3. **Execute Request**
   - Press Enter to select endpoint
   - Press `e` to execute
   - Show: Response appears instantly with formatting

### Part 3: Variable Management (1.5 minutes)
1. **Open Variable Manager**
   - Press `v`
   - Show: "Variables let you reuse values across requests"

2. **Create Variables**
   - Press `n`
   - Key: "BASE_URL"
   - Value: "https://jsonplaceholder.typicode.com"
   - Press Enter
   
   - Press `n` again
   - Key: "USER_ID"
   - Value: "5"
   - Press Enter

3. **Use Variables in Endpoint**
   - Press Esc to go back
   - Press `Ctrl+l` → Navigate to endpoint
   - Press `e` to edit
   - Change URL to: `{{BASE_URL}}/users/{{USER_ID}}`
   - Press Enter to save

4. **Execute with Variables**
   - Press Enter to select
   - Press `e` to execute (shows variable input screen)
   - Press Enter to execute with current values
   - Show: "Variables are substituted automatically"

### Part 4: Faker Integration (1.5 minutes)
1. **Create POST Endpoint**
   - Press `n` (new endpoint)
   - Name: "Create User"
   - Method: Press `m` to POST
   - URL: `{{BASE_URL}}/users`
   - Tab to Body field
   - Enter:
   ```json
   {
     "name": "{{f:fullname}}",
     "email": "{{f:email}}",
     "phone": "{{f:phone}}",
     "company": "{{f:company}}",
     "address": {
       "city": "{{f:city}}",
       "zipcode": "{{f:zipcode}}"
     }
   }
   ```
   - Press Enter to save

2. **Quick Execute**
   - Press `x` (quick execute)
   - Show: "Notice the faker variables generated realistic data"
   - Press `x` again
   - Show: "Each execution generates NEW data - perfect for testing!"

### Part 5: Advanced Features (1.5 minutes)
1. **Network Traffic Analysis**
   - Press `t` to toggle traffic view
   - Show: "See timing breakdown like Wireshark"
   - Point out: Request time, TTFB, Download time

2. **Response Headers**
   - Press `H` to toggle headers
   - Show: "Full response headers available"
   - Press `Shift+PageDown` to scroll headers

3. **Collapsible Sections**
   - Press `Space` to collapse headers
   - Show: "Collapse sections to focus on what matters"
   - Press `Space` again to expand

4. **Copy to Clipboard**
   - Press `y` to copy response
   - Show: "Response copied - paste anywhere!"

### Part 6: Load Testing (1 minute)
1. **Start Load Test**
   - Navigate to "Get User" endpoint
   - Press `l` (load test)
   - Show config: 10 concurrent, 30 seconds
   - Press Enter to start

2. **View Metrics**
   - Show: Real-time RPS, success/failure counts
   - Show: Response time statistics
   - Press Esc to stop

### Part 7: Keyboard Efficiency (30 seconds)
1. **Show Navigation**
   - Press `Ctrl+h` (collections panel)
   - Press `Ctrl+l` (endpoints panel)
   - Press `Ctrl+j/k` (navigate without arrow keys)
   - Show: "Vim-style navigation - hands never leave home row"

2. **Show Help**
   - Press `?` to show help screen
   - Show: "All shortcuts available anytime"

### Closing (30 seconds)
```
"REST API TUI brings the power of Postman to your terminal with:
- Variable management for reusable values
- Faker integration for realistic test data
- Quick execute for rapid testing
- Network traffic analysis
- Load testing capabilities
- All keyboard-driven for maximum efficiency

Check it out at: github.com/gratluri/rest-api-tui
Star the repo if you like it!"
```

## Recording Tips

1. **Slow down**: Pause 2-3 seconds between actions
2. **Narrate**: Explain what you're doing as you do it
3. **Show, don't tell**: Let the UI speak for itself
4. **Highlight key features**: Variables, Faker, Quick Execute
5. **Keep it under 7 minutes**: Attention spans are short

## Asciinema Commands

### Install Asciinema
```bash
# macOS
brew install asciinema

# Linux
apt-get install asciinema  # Debian/Ubuntu
yum install asciinema      # RHEL/CentOS
```

### Record Demo
```bash
# Start recording
asciinema rec rest-api-tui-demo.cast

# Run your demo following the script above

# Stop recording with Ctrl+D
```

### Upload and Share
```bash
# Upload to asciinema.org (creates shareable link)
asciinema upload rest-api-tui-demo.cast

# Or play locally
asciinema play rest-api-tui-demo.cast
```

### Convert to GIF (optional)
```bash
# Install agg (asciinema gif generator)
cargo install agg

# Convert to GIF
agg rest-api-tui-demo.cast rest-api-tui-demo.gif
```

## Alternative: VHS (Recommended for Polished Demos)

VHS creates beautiful terminal GIFs from scripts:

```bash
# Install VHS
brew install vhs

# Create demo.tape file (see DEMO_VHS.tape)
vhs DEMO_VHS.tape
```

This generates a polished GIF automatically!
