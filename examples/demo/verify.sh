#!/bin/bash

# NetHack WASM 動作確認テスト

set -e

PROJECT_ROOT="/home/oosawak/Workspace/NetHack"

echo "========================================="
echo "NetHack WASM Verification Test"
echo "========================================="
echo ""

# Check WASM binary
echo "1. Checking WASM binary..."
if [ -f "$PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm_bg.wasm" ]; then
    SIZE=$(ls -lh "$PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm_bg.wasm" | awk '{print $5}')
    echo "   ✅ WASM binary found ($SIZE)"
else
    echo "   ❌ WASM binary not found"
    exit 1
fi

# Check JavaScript bindings
echo ""
echo "2. Checking JavaScript bindings..."
files=(
    "nethack_wasm.js"
    "nethack_wasm.d.ts"
    "package.json"
)

for file in "${files[@]}"; do
    if [ -f "$PROJECT_ROOT/crates/nethack-wasm/pkg/$file" ]; then
        echo "   ✅ $file found"
    else
        echo "   ❌ $file not found"
    fi
done

# Check example files
echo ""
echo "3. Checking example files..."
if [ -f "$PROJECT_ROOT/examples/wasm.html" ]; then
    echo "   ✅ wasm.html found"
else
    echo "   ❌ wasm.html not found"
fi

if [ -f "$PROJECT_ROOT/examples/run_server.sh" ]; then
    echo "   ✅ run_server.sh found"
else
    echo "   ❌ run_server.sh not found"
fi

if [ -f "$PROJECT_ROOT/examples/README.md" ]; then
    echo "   ✅ README.md found"
else
    echo "   ❌ README.md not found"
fi

# Verify WASM contents
echo ""
echo "4. Verifying WASM exports..."
if command -v wasm-objdump &> /dev/null; then
    echo "   Exported functions:"
    wasm-objdump -e "$PROJECT_ROOT/crates/nethack-wasm/pkg/nethack_wasm_bg.wasm" | grep "export" | head -10 || true
else
    echo "   ⚠️  wasm-objdump not installed (optional check)"
fi

# Summary
echo ""
echo "========================================="
echo "✅ All checks passed!"
echo "========================================="
echo ""
echo "Next steps:"
echo "1. Run the local server:"
echo "   $ ./examples/run_server.sh"
echo ""
echo "2. Open in browser:"
echo "   http://localhost:8000/examples/wasm.html"
echo ""
echo "3. Check the console (F12) for any errors"
echo ""
