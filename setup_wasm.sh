#!/bin/bash
set -e

echo "========================================="
echo "NetHack WASM Build Environment Setup"
echo "========================================="
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed"
echo "   Version: $(rustc --version)"
echo ""

# Install wasm32-unknown-unknown target
echo "📦 Installing wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown
echo "✅ wasm32-unknown-unknown target installed"
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh
    echo "✅ wasm-pack installed"
else
    echo "✅ wasm-pack is already installed"
    echo "   Version: $(wasm-pack --version)"
fi
echo ""

# Check if Node.js is installed (optional but recommended)
if ! command -v node &> /dev/null; then
    echo "⚠️  Node.js is not installed (optional, but recommended for testing WASM)"
    echo "   Install from: https://nodejs.org/ or use: sudo apt install nodejs npm"
else
    echo "✅ Node.js is installed"
    echo "   Version: $(node --version)"
fi
echo ""

# Check if npm is installed (optional but recommended)
if ! command -v npm &> /dev/null; then
    echo "⚠️  npm is not installed (optional, but recommended for testing WASM)"
else
    echo "✅ npm is installed"
    echo "   Version: $(npm --version)"
fi
echo ""

echo "========================================="
echo "Setup Complete!"
echo "========================================="
echo ""
echo "You can now build the WASM module with:"
echo "  cd /home/oosawak/Workspace/NetHack"
echo "  wasm-pack build crates/nethack-wasm --target web"
echo ""
echo "Or use Cargo directly:"
echo "  cargo build -p nethack-wasm --target wasm32-unknown-unknown --release"
echo ""
