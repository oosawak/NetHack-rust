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
if [ ! -f "$PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm_bg.wasm" ]; then
    echo "❌ WASM binary not found at:"
    echo "   $PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm_bg.wasm"
    echo ""
    echo "   Please build WASM first:"
    echo "   $ cd $PROJECT_ROOT"
    echo "   $ wasm-pack build crates/nethack-wasm --target web --release"
    echo ""
    echo "   Then run this script again."
    exit 1
fi

echo "✅ WASM binary found"
echo ""

# Determine which server to use
if command -v python3 &> /dev/null; then
    echo "🌐 Starting Python 3 HTTP server on port 8000..."
    echo ""
    echo "📍 Open your browser:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "📁 Project files will be served from:"
    echo "   $PROJECT_ROOT"
    echo ""
    echo "Press Ctrl+C to stop the server."
    echo ""
    cd "$PROJECT_ROOT"
    python3 -m http.server 8000
elif command -v python &> /dev/null; then
    echo "🌐 Starting Python 2 HTTP server on port 8000..."
    echo ""
    echo "📍 Open your browser:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "📁 Project files will be served from:"
    echo "   $PROJECT_ROOT"
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
        echo "📍 Open your browser:"
        echo "   http://localhost:8080/examples/wasm.html"
        echo ""
        echo "📁 Project files will be served from:"
        echo "   $PROJECT_ROOT"
        echo ""
        echo "Press Ctrl+C to stop the server."
        echo ""
        cd "$PROJECT_ROOT"
        http-server -p 8080
    else
        echo "⚠️  http-server not installed globally"
        echo ""
        echo "Install with:"
        echo "   npm install -g http-server"
        echo ""
        echo "Or use Python instead:"
        echo "   python3 -m http.server 8000"
        exit 1
    fi
elif command -v ruby &> /dev/null; then
    echo "🌐 Starting Ruby HTTP server on port 8000..."
    echo ""
    echo "📍 Open your browser:"
    echo "   http://localhost:8000/examples/wasm.html"
    echo ""
    echo "📁 Project files will be served from:"
    echo "   $PROJECT_ROOT"
    echo ""
    echo "Press Ctrl+C to stop the server."
    echo ""
    cd "$PROJECT_ROOT"
    ruby -run -ehttpd . -p8000
else
    echo "❌ No HTTP server found!"
    echo ""
    echo "Please install one of:"
    echo "  • Python 3 (recommended): Usually pre-installed"
    echo "  • Node.js: apt install nodejs npm (if needed)"
    echo "  • Ruby: apt install ruby (if needed)"
    echo ""
    echo "On most systems, you can just use Python:"
    echo "   cd $PROJECT_ROOT"
    echo "   python3 -m http.server 8000"
    exit 1
fi
