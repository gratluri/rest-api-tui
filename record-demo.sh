#!/bin/bash
# Script to help record REST API TUI demo

set -e

echo "🎬 REST API TUI Demo Recording Helper"
echo "======================================"
echo ""

# Check if asciinema is installed
if ! command -v asciinema &> /dev/null; then
    echo "❌ asciinema is not installed"
    echo ""
    echo "Install it with:"
    echo "  macOS:   brew install asciinema"
    echo "  Ubuntu:  sudo apt-get install asciinema"
    echo "  Fedora:  sudo dnf install asciinema"
    echo ""
    exit 1
fi

echo "✅ asciinema is installed"
echo ""

# Check if app is built
if [ ! -f "target/release/rest-api-tui" ]; then
    echo "⚠️  Release build not found. Building now..."
    cargo build --release
    echo "✅ Build complete"
else
    echo "✅ Release build found"
fi

echo ""
echo "📋 Demo Checklist:"
echo "  1. Terminal size: 120x30 (recommended)"
echo "  2. Clear terminal history: Ctrl+L"
echo "  3. Review DEMO_SCRIPT.md for flow"
echo "  4. Practice once before recording"
echo ""

read -p "Ready to record? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled. Practice with: cargo run --release"
    exit 0
fi

echo ""
echo "🎥 Starting recording in 3 seconds..."
echo "   Press Ctrl+D when done to stop recording"
echo ""
sleep 3

# Start recording
asciinema rec -t "REST API TUI Demo" rest-api-tui-demo.cast

echo ""
echo "✅ Recording saved to: rest-api-tui-demo.cast"
echo ""
echo "Next steps:"
echo "  1. Review: asciinema play rest-api-tui-demo.cast"
echo "  2. Upload: asciinema upload rest-api-tui-demo.cast"
echo "  3. Convert to GIF (optional):"
echo "     - Install agg: cargo install agg"
echo "     - Convert: agg rest-api-tui-demo.cast demo.gif"
echo ""
echo "Or use VHS for automated recording:"
echo "  1. Install: brew install vhs"
echo "  2. Run: vhs DEMO_VHS.tape"
echo ""
