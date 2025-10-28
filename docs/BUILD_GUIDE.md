# CrossGPU Build Guide

This guide provides detailed instructions for building CrossGPU on different platforms and for different targets.

## Prerequisites

### All Platforms

**Rust Toolchain** (1.70 or later):
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust (if already installed)
rustup update stable
```

### Platform-Specific Requirements

#### Linux

**Vulkan Development Files**:
```bash
# Ubuntu/Debian
sudo apt-get install libvulkan-dev vulkan-tools

# Fedora
sudo dnf install vulkan-devel vulkan-tools

# Arch Linux
sudo pacman -S vulkan-devel vulkan-tools
```

**Build Essentials**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config

# Fedora
sudo dnf groupinstall "Development Tools"

# Arch Linux
sudo pacman -S base-devel
```

#### macOS

**Xcode Command Line Tools**:
```bash
xcode-select --install
```

**Metal** is included with macOS SDK, no additional installation needed.

#### Windows

**Visual Studio Build Tools**:
1. Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Install "Desktop development with C++" workload
3. Ensure Windows SDK is installed

**DirectX 12** is included with Windows 10/11.

### WASM Build Requirements

**wasm-pack**:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

**WASM Target**:
```bash
rustup target add wasm32-unknown-unknown
```

## Building CrossGPU

### Quick Build (All Platforms)

```bash
# Clone the repository
git clone https://github.com/hmthanh/CrossGPU.git
cd CrossGPU

# Build all packages in release mode
cargo build --release --all

# Verify build succeeded
cargo test --all
```

### Build Individual Packages

**Core Library**:
```bash
cargo build --release -p crossgpu-core
```

**Specific Backend**:
```bash
# CPU backend (all platforms)
cargo build --release -p crossgpu-backend-cpu

# WebGPU backend (all platforms)
cargo build --release -p crossgpu-backend-webgpu

# Vulkan backend (Linux)
cargo build --release -p crossgpu-backend-vulkan

# Metal backend (macOS)
cargo build --release -p crossgpu-backend-metal

# DirectX 12 backend (Windows)
cargo build --release -p crossgpu-backend-dx12
```

**Example Application**:
```bash
cargo build --release --bin simple-inference
```

### WASM Build

**Build WASM Package**:
```bash
cd wasm
wasm-pack build --target web --release
```

**Output Location**: `wasm/pkg/`

**Test WASM Locally**:
```bash
# Install a simple HTTP server
cargo install basic-http-server

# Serve the WASM package
cd wasm/pkg
basic-http-server .

# Open browser to http://localhost:4000
```

## Build Profiles

### Development Build

Fast compilation, includes debug symbols:
```bash
cargo build --all
```

### Release Build

Optimized for performance:
```bash
cargo build --release --all
```

**Optimizations**:
- LTO enabled
- Single codegen unit
- Stripped symbols
- Optimization level 3

### WASM Release Build

Size-optimized for web deployment:
```bash
cd wasm
wasm-pack build --target web --release
```

**Optimizations**:
- Size optimization (`opt-level = "z"`)
- LTO enabled
- Panic = abort

## Build Options

### Feature Flags

**Enable specific backends** (not currently used, but extensible):
```bash
# Example: If we had optional backends
cargo build --release --features "vulkan,metal"
```

### Cross-Compilation

**WASM from any platform**:
```bash
cargo build --release --target wasm32-unknown-unknown -p crossgpu-wasm
```

**Linux from macOS** (requires cross):
```bash
# Install cross
cargo install cross

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu
```

## Build Artifacts

### Native Builds

**Location**: `target/release/`

**Key Files**:
- `simple-inference` (or `simple-inference.exe` on Windows) - Example binary
- `libcrossgpu_core.rlib` - Core library
- `libcrossgpu_backend_*.rlib` - Backend libraries

### WASM Builds

**Location**: `wasm/pkg/`

**Key Files**:
- `crossgpu_wasm.js` - JavaScript bindings
- `crossgpu_wasm_bg.wasm` - Compiled WASM module
- `crossgpu_wasm.d.ts` - TypeScript definitions
- `package.json` - npm package metadata

## Troubleshooting

### Common Issues

#### 1. Vulkan Not Found (Linux)

**Error**: `Could not find libvulkan.so`

**Solution**:
```bash
# Install Vulkan development files
sudo apt-get install libvulkan-dev

# Verify installation
ldconfig -p | grep vulkan
```

#### 2. Metal Framework Not Found (macOS)

**Error**: `framework 'Metal' not found`

**Solution**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Verify installation
xcode-select -p
```

#### 3. Windows SDK Not Found (Windows)

**Error**: `Unable to find vcvarsall.bat`

**Solution**:
1. Install Visual Studio Build Tools
2. Ensure "Desktop development with C++" workload is installed
3. Restart terminal/IDE

#### 4. WASM Build Fails

**Error**: `wasm-pack: not found`

**Solution**:
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Verify installation
wasm-pack --version
```

#### 5. Linking Errors

**Error**: `undefined reference to ...`

**Solution**:
```bash
# Clean build artifacts
cargo clean

# Rebuild
cargo build --release --all
```

### Build Performance

**Faster Builds**:
```bash
# Install sccache for caching
cargo install sccache

# Configure Cargo to use sccache
export RUSTC_WRAPPER=sccache

# Use more parallel jobs (adjust based on CPU cores)
export CARGO_BUILD_JOBS=8
```

**Check Build Times**:
```bash
cargo build --release --timings
# Open target/cargo-timings/cargo-timing.html
```

## Verifying the Build

### Run Tests

```bash
# All tests
cargo test --all --verbose

# Specific package
cargo test -p crossgpu-core

# With logging
RUST_LOG=debug cargo test --all
```

### Run Example

```bash
# With default backend
cargo run --release --bin simple-inference

# With logging
RUST_LOG=info cargo run --release --bin simple-inference
```

### Lint and Format

```bash
# Check formatting
cargo fmt --all -- --check

# Run Clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Continuous Integration

The project uses GitHub Actions for CI/CD. See [.github/workflows/ci.yml](.github/workflows/ci.yml).

**Local CI Simulation**:
```bash
# Format check
cargo fmt --all -- --check

# Clippy check
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all

# Build
cargo build --release --all

# WASM build
cd wasm && wasm-pack build --target web --release
```

## Platform-Specific Notes

### Linux

- **Default Backend**: Vulkan
- **Fallback**: CPU
- **GPU Requirements**: Vulkan-compatible GPU with drivers installed

### macOS

- **Default Backend**: Metal
- **Fallback**: CPU
- **GPU Requirements**: Any Mac with Metal support (2012+)

### Windows

- **Default Backend**: DirectX 12
- **Fallback**: CPU
- **GPU Requirements**: DirectX 12-compatible GPU (most modern GPUs)

### WASM/Browser

- **Default Backend**: WebGPU
- **Fallback**: CPU (WASM)
- **GPU Requirements**: Browser with WebGPU support (Chrome 113+, Edge 113+)

## Advanced Build Configuration

### Custom Cargo Configuration

Create `.cargo/config.toml`:
```toml
[build]
# Use all CPU cores
jobs = 8

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-apple-darwin]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-cpu=native"]
```

### Profile Customization

Add to `Cargo.toml`:
```toml
[profile.release-with-debug]
inherits = "release"
debug = true
strip = false
```

Build with custom profile:
```bash
cargo build --profile release-with-debug
```

## Docker Build (Optional)

**Dockerfile** (create if needed):
```dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/crossgpu
COPY . .

RUN cargo build --release --all

FROM debian:bookworm-slim
COPY --from=builder /usr/src/crossgpu/target/release/simple-inference /usr/local/bin/

CMD ["simple-inference"]
```

**Build Docker Image**:
```bash
docker build -t crossgpu .
```

**Run in Container**:
```bash
docker run --rm crossgpu
```

## Benchmarking

```bash
# Install cargo-criterion
cargo install cargo-criterion

# Run benchmarks (if implemented)
cargo criterion

# View results
open target/criterion/report/index.html
```

## Resources

- [Rust Book - Building and Running](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)

---

**Need Help?** Open an issue on [GitHub](https://github.com/hmthanh/CrossGPU/issues).
