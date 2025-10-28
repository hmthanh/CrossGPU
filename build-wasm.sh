#!/bin/bash
# Build script for WASM package

set -e

echo "Building CrossGPU WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed"
    echo "Install it with: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

# Build the WASM package
cd wasm
wasm-pack build --target web --release

echo "WASM package built successfully!"
echo "Output located in: wasm/pkg/"
