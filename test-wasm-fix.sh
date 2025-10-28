#!/usr/bin/env bash
# Test script to verify WASM build fix

set -e

echo "🧪 Testing WASM Build Fix"
echo "=========================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

# Step 1: Test core library for WASM
echo -e "${BLUE}Step 1: Testing core library for WASM target${NC}"
echo "Command: cargo build --package crossgpu-core --target wasm32-unknown-unknown --release"
echo ""

if cargo build --package crossgpu-core --target wasm32-unknown-unknown --release; then
    echo -e "${GREEN}✅ Core library compiles for WASM${NC}"
else
    echo -e "${RED}❌ Core library failed for WASM${NC}"
    exit 1
fi

echo ""

# Step 2: Test WebGPU backend for WASM
echo -e "${BLUE}Step 2: Testing WebGPU backend for WASM target${NC}"
echo "Command: cargo build --package crossgpu-backend-webgpu --target wasm32-unknown-unknown --release"
echo ""

if cargo build --package crossgpu-backend-webgpu --target wasm32-unknown-unknown --release; then
    echo -e "${GREEN}✅ WebGPU backend compiles for WASM${NC}"
else
    echo -e "${RED}❌ WebGPU backend failed for WASM${NC}"
    exit 1
fi

echo ""

# Step 3: Test full WASM package
echo -e "${BLUE}Step 3: Building complete WASM package${NC}"
echo "Command: cd wasm && wasm-pack build --target web --release"
echo ""

cd wasm
if wasm-pack build --target web --release; then
    echo -e "${GREEN}✅ WASM package built successfully${NC}"
else
    echo -e "${RED}❌ WASM package build failed${NC}"
    exit 1
fi

cd ..

echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ All WASM Tests Passed! ✅         ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo "📦 WASM Package Location: wasm/pkg/"
echo ""
echo "Files created:"
ls -lh wasm/pkg/ | grep -E "\.wasm$|\.js$"
echo ""
echo "🌐 To deploy and test in browser:"
echo "   ./scripts/deploy-wasm.sh"
echo "   cd dist"
echo "   python3 -m http.server 8080"
echo "   # Open http://localhost:8080"
echo ""
