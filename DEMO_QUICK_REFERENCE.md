# Demo Quick Reference Card

Print this out or keep it visible during your demo!

## 🎯 Demo Flow (5 minutes)

### 1. Introduction (30s)
- Start app: `cargo run --release`
- "Terminal-based API testing tool"

### 2. Basic Workflow (1m)
```
n → "Demo API" → Enter                    [Create collection]
Ctrl+l → n                                [New endpoint]
"Get User" → Tab → m → Tab               [Name + GET method]
https://jsonplaceholder.typicode.com/users/1
Enter → Enter → e                         [Save + Execute]
```

### 3. Variables (1m)
```
v → n                                     [Variable manager]
"BASE_URL" → Tab → "https://jsonplaceholder.typicode.com"
Enter → Esc                               [Save + Back]
Ctrl+l → e                                [Edit endpoint]
Change URL to: {{BASE_URL}}/users/1
Enter → Enter → e → Enter                 [Save + Execute]
```

### 4. Faker Magic (1.5m)
```
n → "Create User" → Tab → mm → Tab       [New POST endpoint]
{{BASE_URL}}/users → Tab → Tab → Tab     [URL + skip to body]
Paste JSON with faker variables (see below)
Enter → x                                 [Save + Quick execute]
x → x → x                                 [Execute multiple times]
```

**Faker JSON:**
```json
{"name":"{{f:fullname}}","email":"{{f:email}}","phone":"{{f:phone}}","company":"{{f:company}}"}
```

### 5. Features Showcase (1m)
```
t                                         [Network traffic]
H                                         [Response headers]
Space                                     [Collapse section]
y                                         [Copy response]
```

### 6. Load Test (1m)
```
Navigate to Get User endpoint
l → Enter                                 [Start load test]
Wait 10 seconds
Esc                                       [Stop]
```

### 7. Closing (30s)
```
? → Esc                                   [Show help]
q                                         [Quit]
"Check it out: github.com/gratluri/rest-api-tui"
```

---

## 🎤 Talking Points

### Opening
> "This is REST API TUI - think Postman, but in your terminal. It's fast, keyboard-driven, and perfect for developers who live in the terminal."

### Variables
> "Variables let you define values once and reuse them everywhere. No more copy-pasting URLs or tokens."

### Faker
> "Here's the magic - faker variables generate realistic test data automatically. Watch what happens when I execute this multiple times..."

### Quick Execute
> "Notice how fast that was? The 'x' key executes instantly without prompts. Perfect for rapid testing."

### Features
> "We've got network traffic analysis like Wireshark, collapsible sections, and one-key clipboard copy."

### Load Testing
> "Built-in load testing means you can stress test right from the same tool."

### Closing
> "It's open source, built with Rust, and works anywhere - even over SSH. Star it on GitHub if you like it!"

---

## ⌨️ Essential Shortcuts

| Key | Action | When to Use |
|-----|--------|-------------|
| `n` | New | Creating collections/endpoints |
| `e` | Edit/Execute | Modifying or running requests |
| `x` | Quick Execute | Fast testing without prompts |
| `v` | Variables | Managing reusable values |
| `t` | Traffic | Showing network analysis |
| `H` | Headers | Toggling response headers |
| `y` | Copy | Copying response to clipboard |
| `l` | Load Test | Starting performance test |
| `Space` | Collapse | Hiding/showing sections |
| `?` | Help | Showing all shortcuts |
| `Ctrl+h/l` | Switch Panels | Moving between collections/endpoints |

---

## 🎬 Recording Options

### Option 1: Asciinema (Recommended)
```bash
./record-demo.sh
```
- Creates shareable terminal recording
- Can be embedded in GitHub README
- Lightweight and fast

### Option 2: VHS (Automated)
```bash
brew install vhs
vhs DEMO_VHS.tape
```
- Generates polished GIF automatically
- No manual recording needed
- Perfect for presentations

### Option 3: Screen Recording
- Use macOS QuickTime or OBS
- Record terminal window
- Export as MP4
- Good for presentations with audio

---

## 📊 Key Metrics to Highlight

- **Startup Time:** < 1 second
- **Memory Usage:** ~10MB (vs Postman's ~500MB)
- **Faker Variables:** 50+ generators
- **Keyboard Shortcuts:** 20+ commands
- **Lines of Code:** ~3,000 (compact and maintainable)

---

## 🐛 Troubleshooting

### App won't start
```bash
cargo clean
cargo build --release
```

### Terminal too small
- Resize to at least 120x30
- Or use `Cmd+Plus` to zoom out

### Recording issues
```bash
# Test asciinema
asciinema rec test.cast
# Press Ctrl+D to stop
asciinema play test.cast
```

---

## 💡 Pro Tips

1. **Practice First:** Run through the demo 2-3 times before recording
2. **Slow Down:** Pause 2-3 seconds between actions
3. **Narrate:** Explain what you're doing as you do it
4. **Show Mistakes:** If you make a mistake, show how to fix it (demonstrates real usage)
5. **Highlight Wow Moments:** Pause on faker data generation and quick execute
6. **Keep It Short:** 5-7 minutes max for attention span
7. **End Strong:** Show the GitHub repo and encourage stars

---

## 📝 Pre-Demo Checklist

- [ ] App built in release mode
- [ ] Terminal size set to 120x30
- [ ] Terminal history cleared
- [ ] Demo script reviewed
- [ ] Faker JSON copied to clipboard
- [ ] Recording software tested
- [ ] Internet connection stable (for API calls)
- [ ] Practiced demo flow at least once

---

## 🎁 Bonus Content

### Show These If Time Permits

**Custom Headers:**
```
Tab to Headers field → h
"Authorization" → Tab → "Bearer token123"
Enter → Esc
```

**Scrolling:**
```
PageDown → PageUp → Home → End
Shift+PageDown (for headers)
```

**Panel Navigation:**
```
Ctrl+h → Ctrl+l → Ctrl+j → Ctrl+k
```

---

## 📧 Follow-Up

After the demo, share:
1. Recording link (asciinema or YouTube)
2. GitHub repo: https://github.com/gratluri/rest-api-tui
3. CHEATSHEET.md for keyboard shortcuts
4. Offer to help with setup

---

**Good luck with your demo! 🚀**
