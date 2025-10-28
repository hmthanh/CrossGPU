#!/usr/bin/env bash
# Build native binaries for multiple platforms

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RELEASES_DIR="$PROJECT_ROOT/releases"

echo "🏗️  Building native binaries for distribution..."

# Create releases directory
mkdir -p "$RELEASES_DIR"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        PLATFORM="macos"
        ;;
    Linux)
        PLATFORM="linux"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        PLATFORM="windows"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH="x64"
        ;;
    arm64|aarch64)
        ARCH="arm64"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

TARGET_NAME="${PLATFORM}-${ARCH}"
echo "📦 Building for: $TARGET_NAME"

# Build in release mode with optimizations
cd "$PROJECT_ROOT"
echo "⚙️  Building release binaries..."
cargo build --release --all

# Create platform-specific package
PACKAGE_DIR="$RELEASES_DIR/crossgpu-$TARGET_NAME"
mkdir -p "$PACKAGE_DIR/bin"
mkdir -p "$PACKAGE_DIR/examples"
mkdir -p "$PACKAGE_DIR/docs"

echo "📋 Copying files..."

# Copy binaries
if [ "$PLATFORM" = "windows" ]; then
    cp target/release/simple-inference.exe "$PACKAGE_DIR/bin/" 2>/dev/null || true
else
    cp target/release/simple-inference "$PACKAGE_DIR/bin/" 2>/dev/null || true
fi

# Copy example models/data
echo "Sample model data" > "$PACKAGE_DIR/examples/README.md"

# Copy documentation
cp README.md "$PACKAGE_DIR/"
cp LICENSE-MIT "$PACKAGE_DIR/"
cp LICENSE-APACHE "$PACKAGE_DIR/"
cp -r docs/* "$PACKAGE_DIR/docs/" 2>/dev/null || true

# Create installation script
cat > "$PACKAGE_DIR/install.sh" <<'EOF'
#!/bin/bash
set -e

BIN_DIR="${HOME}/.local/bin"
mkdir -p "$BIN_DIR"

echo "Installing CrossGPU to $BIN_DIR..."
cp -f bin/* "$BIN_DIR/"
chmod +x "$BIN_DIR"/*

echo "✅ Installation complete!"
echo ""
echo "Make sure $BIN_DIR is in your PATH:"
echo 'export PATH="$HOME/.local/bin:$PATH"'
echo ""
echo "Run: simple-inference --help"
EOF
chmod +x "$PACKAGE_DIR/install.sh"

# Create README
cat > "$PACKAGE_DIR/README.md" <<EOF
# CrossGPU Transformer Engine - $TARGET_NAME

GPU-accelerated transformer inference engine for multiple platforms.

## Installation

### Quick Install (Unix-like)
\`\`\`bash
./install.sh
\`\`\`

### Manual Install
Copy binaries from \`bin/\` to a directory in your PATH.

## Usage

### Run Simple Inference
\`\`\`bash
simple-inference
\`\`\`

The binary will automatically detect and use the best available GPU backend:
- **macOS**: Metal
- **Linux**: Vulkan or CPU fallback
- **Windows**: DirectX 12 or CPU fallback

## Documentation

See the \`docs/\` directory for detailed guides:
- \`BUILD_GUIDE.md\` - Build from source
- \`API_GUIDE.md\` - API reference
- \`QUICK_REFERENCE.md\` - Quick start

## Platform-Specific Notes

### macOS
- Requires macOS 10.13+ for Metal support
- Apple Silicon (M1/M2) provides best performance

### Linux
- Vulkan drivers required for GPU acceleration
- Install: \`sudo apt install vulkan-tools libvulkan1\` (Debian/Ubuntu)

### Windows
- DirectX 12 supported on Windows 10+
- Older systems fall back to CPU

## License

Dual-licensed under MIT or Apache 2.0.

## Support

- GitHub: https://github.com/yourusername/crossgpu
- Docs: https://crossgpu.dev/docs
EOF

# Create archive
cd "$RELEASES_DIR"
ARCHIVE_NAME="crossgpu-${TARGET_NAME}.tar.gz"
echo "📦 Creating archive: $ARCHIVE_NAME"
tar -czf "$ARCHIVE_NAME" "crossgpu-$TARGET_NAME"

# Checksum
if command -v shasum &> /dev/null; then
    shasum -a 256 "$ARCHIVE_NAME" > "${ARCHIVE_NAME}.sha256"
    echo "✅ Checksum: $(cat ${ARCHIVE_NAME}.sha256)"
fi

echo ""
echo "✅ Build complete!"
echo ""
echo "📦 Package: $RELEASES_DIR/$ARCHIVE_NAME"
echo "📊 Size: $(du -h $ARCHIVE_NAME | cut -f1)"
echo ""
echo "🚀 Distribution options:"
echo "   - GitHub Release: gh release create v1.0.0 $ARCHIVE_NAME"
echo "   - Download server: scp $ARCHIVE_NAME user@server:/var/www/downloads/"
echo "   - Package registry: cargo publish (for library crates)"
