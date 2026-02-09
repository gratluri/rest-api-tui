# Demo Materials for REST API TUI

This directory contains everything you need to create an impressive demo of REST API TUI for your team.

## 📁 Demo Files

### 1. **DEMO_SCRIPT.md** - Detailed Demo Script
Complete step-by-step guide for a 5-7 minute demo. Includes:
- Pre-demo setup instructions
- Detailed flow with timing
- What to say at each step
- Recording tips and best practices

**Use this for:** Planning and practicing your demo

### 2. **DEMO_QUICK_REFERENCE.md** - Quick Reference Card
One-page cheat sheet for the demo. Includes:
- Condensed demo flow
- Essential keyboard shortcuts
- Talking points
- Troubleshooting tips

**Use this for:** Keep visible during recording/presentation

### 3. **DEMO_PRESENTATION.md** - Presentation Slides
20 slides in markdown format covering:
- Problem and solution
- Feature highlights
- Technical details
- Use cases and comparisons
- Roadmap and call to action

**Use this for:** Team presentations or converting to PowerPoint/Google Slides

### 4. **DEMO_VHS.tape** - Automated Recording Script
VHS script that automatically generates a demo GIF. Includes:
- All demo steps scripted
- Proper timing and pauses
- Professional output

**Use this for:** Creating polished GIFs without manual recording

### 5. **record-demo.sh** - Recording Helper Script
Bash script that helps you record with asciinema. Includes:
- Dependency checks
- Build verification
- Recording workflow
- Next steps guidance

**Use this for:** Manual recording with asciinema

## 🎬 Recording Options

### Option 1: Asciinema (Recommended for Sharing)

**Best for:** Shareable terminal recordings, GitHub README embeds

```bash
# Install asciinema
brew install asciinema  # macOS
apt-get install asciinema  # Linux

# Record using helper script
./record-demo.sh

# Or record manually
asciinema rec rest-api-tui-demo.cast
# Follow DEMO_SCRIPT.md
# Press Ctrl+D when done

# Play back locally
asciinema play rest-api-tui-demo.cast

# Upload and share
asciinema upload rest-api-tui-demo.cast
# Returns a shareable URL like: https://asciinema.org/a/xxxxx
```

**Pros:**
- Lightweight (text-based)
- Embeddable in GitHub README
- Searchable and copyable text
- Fast loading

**Cons:**
- Requires asciinema player to view
- No audio narration

### Option 2: VHS (Recommended for GIFs)

**Best for:** Polished, automated GIF generation

```bash
# Install VHS
brew install vhs

# Generate demo GIF automatically
vhs DEMO_VHS.tape

# Output: demo.gif (ready to share!)
```

**Pros:**
- Fully automated (no manual recording)
- Professional output
- GIF works everywhere
- Consistent results

**Cons:**
- Slower to generate
- Larger file size
- No interactivity

### Option 3: Screen Recording (Recommended for Presentations)

**Best for:** Presentations with audio narration

**macOS:**
```bash
# Use QuickTime Player
# File → New Screen Recording
# Select terminal window
# Record while following DEMO_SCRIPT.md
```

**Linux:**
```bash
# Use OBS Studio or SimpleScreenRecorder
sudo apt-get install simplescreenrecorder
```

**Windows:**
```bash
# Use OBS Studio or Windows Game Bar (Win+G)
```

**Pros:**
- Can add audio narration
- Familiar format (MP4)
- Easy to edit
- Works in presentations

**Cons:**
- Larger file size
- Requires video editing for polish
- Not embeddable in GitHub

## 🎯 Quick Start Guide

### For a 5-Minute Demo

1. **Prepare:**
   ```bash
   cd rest-api-tui
   cargo build --release
   ```

2. **Practice:**
   - Read DEMO_SCRIPT.md
   - Run through the flow 2-3 times
   - Keep DEMO_QUICK_REFERENCE.md visible

3. **Record:**
   ```bash
   # Option A: Asciinema
   ./record-demo.sh
   
   # Option B: VHS (automated)
   vhs DEMO_VHS.tape
   
   # Option C: Screen recording
   # Start QuickTime/OBS, then:
   cargo run --release
   ```

4. **Share:**
   - Upload to asciinema.org or YouTube
   - Share link with team
   - Add to GitHub README

### For a Team Presentation

1. **Create Slides:**
   - Convert DEMO_PRESENTATION.md to PowerPoint/Google Slides
   - Or use markdown presentation tool (Marp, reveal.js)

2. **Record Demo:**
   - Use screen recording with audio
   - Follow DEMO_SCRIPT.md
   - Keep it under 7 minutes

3. **Present:**
   - Show slides first (context and features)
   - Play demo video (or do live demo)
   - Q&A and discussion

## 📋 Pre-Demo Checklist

Before recording or presenting:

- [ ] App builds successfully: `cargo build --release`
- [ ] Terminal size set to 120x30 (or larger)
- [ ] Terminal history cleared: `Ctrl+L` or `clear`
- [ ] Demo script reviewed: Read DEMO_SCRIPT.md
- [ ] Quick reference printed/visible: DEMO_QUICK_REFERENCE.md
- [ ] Faker JSON copied to clipboard (see DEMO_QUICK_REFERENCE.md)
- [ ] Internet connection stable (for API calls to jsonplaceholder.typicode.com)
- [ ] Recording software tested
- [ ] Practiced demo flow at least once

## 🎤 Demo Flow Summary

1. **Introduction** (30s) - Show app startup
2. **Basic Workflow** (1m) - Create collection and endpoint
3. **Variables** (1m) - Create and use variables
4. **Faker Magic** (1.5m) - Show dynamic data generation
5. **Features** (1m) - Traffic, headers, clipboard
6. **Load Testing** (1m) - Quick performance test
7. **Closing** (30s) - Show help and GitHub repo

**Total Time:** 5-7 minutes

## 💡 Tips for Success

### Before Recording
1. **Practice 2-3 times** - Get comfortable with the flow
2. **Clear distractions** - Close other apps, notifications
3. **Check lighting** - If recording video with camera
4. **Test audio** - If adding narration

### During Recording
1. **Slow down** - Pause 2-3 seconds between actions
2. **Narrate** - Explain what you're doing
3. **Show mistakes** - Demonstrates real usage
4. **Highlight wow moments** - Pause on faker and quick execute

### After Recording
1. **Review** - Watch the recording before sharing
2. **Edit if needed** - Trim dead time, add captions
3. **Share widely** - Team chat, email, GitHub
4. **Collect feedback** - Ask for questions and suggestions

## 🎁 Bonus Ideas

### Interactive Demo
- Do a live demo instead of recording
- Take questions during the demo
- Show how to handle edge cases

### Comparison Demo
- Show Postman side-by-side
- Highlight speed differences
- Compare memory usage

### Advanced Features
- Show load testing in detail
- Demonstrate network traffic analysis
- Create complex request chains

### Team Workshop
- Have team members follow along
- Create a shared collection
- Build endpoints together

## 📊 Key Metrics to Highlight

During your demo, emphasize these impressive stats:

- **Startup Time:** < 1 second (vs Postman's 5-10 seconds)
- **Memory Usage:** ~10MB (vs Postman's ~500MB)
- **Faker Variables:** 50+ realistic data generators
- **Keyboard Shortcuts:** 20+ commands for efficiency
- **Cross-Platform:** Works on macOS, Linux, Windows
- **SSH-Friendly:** Works in remote environments

## 🔗 Resources

### Documentation
- **README.md** - Full documentation
- **CHEATSHEET.md** - Keyboard shortcuts
- **FAKER_FEATURE.md** - All faker variables
- **QUICK_EXECUTE_FEATURE.md** - Quick execute details

### External Tools
- **Asciinema:** https://asciinema.org
- **VHS:** https://github.com/charmbracelet/vhs
- **OBS Studio:** https://obsproject.com
- **Marp (Markdown Presentations):** https://marp.app

### Example Demos
- Search GitHub for "TUI demo" for inspiration
- Check out other Rust TUI projects
- Look at asciinema.org for terminal recording examples

## 🤝 Sharing Your Demo

After creating your demo:

1. **Add to GitHub README:**
   ```markdown
   ## Demo
   
   [![asciicast](https://asciinema.org/a/xxxxx.svg)](https://asciinema.org/a/xxxxx)
   
   Or watch the [full demo video](link-to-video)
   ```

2. **Share on Social Media:**
   - Twitter/X with #RustLang #TUI #API
   - LinkedIn with technical details
   - Reddit on r/rust, r/commandline

3. **Team Channels:**
   - Slack/Teams with recording link
   - Email with presentation slides
   - Wiki with setup instructions

4. **Collect Feedback:**
   - Create a feedback form
   - Ask for feature requests
   - Encourage contributions

## 🎓 Learning from Your Demo

After presenting:

1. **Note questions** - What did people ask about?
2. **Identify confusion** - What wasn't clear?
3. **Gather ideas** - What features do they want?
4. **Improve docs** - Update based on feedback
5. **Plan next demo** - What to show next time?

## 🚀 Next Steps

After a successful demo:

1. **Share the repo:** https://github.com/gratluri/rest-api-tui
2. **Encourage stars:** Help the project grow
3. **Invite contributions:** Issues, PRs, feedback
4. **Plan improvements:** Based on team feedback
5. **Schedule follow-up:** Show new features later

---

**Good luck with your demo! 🎬**

If you have questions or need help, open an issue on GitHub.
