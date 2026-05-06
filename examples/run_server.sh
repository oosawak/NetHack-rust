#!/bin/bash

# NetHack WASM ローカルサーバー起動スクリプト

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "========================================="
echo "NetHack WASM Local Server"
echo "========================================="
echo ""

# Check if WASM was built
if [ ! -f "$PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm.wasm" ]; then
    echo "❌ WASM binary not found!"
    echo "   Please build WASM first:"
    echo "   $ cd $PROJECT_ROOT"
    echo "   $ wasm-pack build crates/nethack-wasm --target web --release"
    exit 1
fi

echo "✅ WASM binary found"
echo ""

# Determine which server to use
if command -v python3 &> /dev/null; then
    echo "🌐 Starting Python 3 HTTP server..."
    echo ""
    echo "📍 Open your browser and navigate to:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "Press Ctrl+C to stop the server."
    echo ""
    cd "$PROJECT_ROOT"
    python3 -m http.server 8000
elif command -v python &> /dev/null; then
    echo "🌐 Starting Python 2 HTTP server..."
    echo ""
    echo "📍 Open your browser and navigate to:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "Press Ctrl+C to stop the server."
    echo ""
    cd "$PROJECT_ROOT"
    python -m SimpleHTTPServer 8000
elif command -v node &> /dev/null; then
    echo "🌐 Starting Node.js HTTP server..."
    
    # Check if http-server is installed globally
    if command -v http-server &> /dev/null; then
        echo ""
        echo "📍 Open your browser and navigate to:"
        echo "   http://localhost:8080/examples/wasm.html"
        echo ""
        echo "Press Ctrl+C to stop the server."
        echo ""
        cd "$PROJECT_ROOT"
        http-server -p 8080
    else
        echo "⚠️  http-server not installed globally"
        echo "   Install with: npm install -g http-server"
        echo ""
        echo "   Or use Python:"
        echo "   $ python3 -m http.server 8000"
        exit 1
    fi
elif command -v ruby &> /dev/null; then
    echo "🌐 Starting Ruby HTTP server..."
    echo ""
    echo "📍 Open your browser and navigate to:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "Press Ctrl+C to stop the server."
    echo ""
    cd "$PROJECT_ROOT"
    ruby -run -ehttpd . -p8000
else
    echo "❌ No HTTP server found!"
    echo ""
    echo "Please install one of:"
    echo "  • Python 3: apt install python3 (or use system Python)"
    echo "  • Node.js: apt install nodejs npm"
    echo "  • Ruby: apt install ruby"
    echo ""
    echo "Then run this script again."
    exit 1
fi
