# Getting Started with CrossGPU

This guide will help you get up and running with CrossGPU in minutes.

## Prerequisites

### Required

- **Rust 1.70+**: Install from [rustup.rs](https://rustup.rs/)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Optional (for specific backends)

- **WASM**: wasm-pack for browser deployment
  ```bash
  cargo install wasm-pack
  ```
- **Vulkan** (Linux/Windows): Vulkan drivers
- **Metal** (macOS): Xcode Command Line Tools
- **DirectX 12** (Windows): Windows 10+ SDK

## Quick Start

### 1. Clone and Verify

```bash
git clone https://github.com/yourusername/crossgpu.git
cd crossgpu
./scripts/verify.sh
```

### 2. Build the Project

```bash
# Option A: Using Make (recommended)
make build

# Option B: Using Cargo directly
cargo build --release --all
```

### 3. Run Your First Example

```bash
# Option A: Using Make
make run

# Option B: Using Cargo
cargo run --release --bin simple-inference
```

You should see output like:

```
🚀 CrossGPU Transformer Inference
Using device: Metal GPU (or CPU fallback)
✅ Model initialized
Running inference...
✅ Inference complete!
```

## Development Workflow

### Using Make Commands

The project includes a `Makefile` with convenient commands:

```bash
make build          # Build all packages
make test           # Run all tests
make run            # Run simple-inference example
make fmt            # Format code
make lint           # Run clippy
make doc            # Generate documentation
make clean          # Clean build artifacts
make wasm           # Build WASM package
make benchmark      # Run benchmarks
make ci             # Run full CI checks
```

### Using VS Code

1. **Open the project** in VS Code
2. **Install recommended extensions** (prompted automatically)
3. **Press F5** to debug `simple-inference`
4. **Use tasks** (Cmd/Ctrl+Shift+B) for builds

### Using Docker

For consistent development environment:

```bash
# Start development container
docker-compose up -d

# Enter container
docker-compose exec dev bash

# Build inside container
cargo build --all
```

## Project Structure

```
crossgpu/
├── core/              # Core tensor & GPU abstractions
├── backends/          # GPU backend implementations
│   ├── cpu/          # CPU fallback
│   ├── webgpu/       # WebGPU (cross-platform)
│   ├── vulkan/       # Vulkan (Linux/Windows)
│   ├── metal/        # Metal (macOS)
│   └── dx12/         # DirectX 12 (Windows)
├── wasm/              # Browser/WASM package
├── examples/          # Example applications
├── docs/              # Comprehensive documentation
└── scripts/           # Utility scripts
```

## Your First Model

### Create a Transformer Model

```rust
use crossgpu_core::transformer::TransformerConfig;

// Create tiny transformer (2M parameters, ~50MB)
let config = TransformerConfig::tiny();
let model = TransformerModel::new(&config);
```

### Save and Load

```rust
// Save model to disk
model.save_to_file("model.bin")?;

// Load model
let loaded = TransformerModel::load_from_file("model.bin")?;
```

### Run Inference

```rust
use crossgpu_webgpu::WebGpuDevice; // or any backend
use crossgpu_core::GpuDevice;

// Initialize GPU
let device = WebGpuDevice::new()?;

// Upload model
let gpu_model = model.upload_to_device(&device)?;

// Run inference
let input_tokens = vec![1, 42, 123];
let output = gpu_model.forward(&input_tokens)?;
```

### Quantize for Efficiency

```rust
use crossgpu_core::quantization::quantize_tensor;

// Quantize to 8-bit (4x smaller, faster inference)
let quantized_model = model.quantize_8bit();

// Or 4-bit for 8x compression
let ultra_quantized = model.quantize_4bit();
```

## Platform-Specific Notes

### macOS

- **Metal** is the default and fastest backend
- Requires macOS 10.13+
- Apple Silicon (M1/M2) provides best performance

### Linux

- **Vulkan** is the default backend
- Install drivers: `sudo apt install vulkan-tools libvulkan1`
- Falls back to CPU if Vulkan unavailable

### Windows

- **DirectX 12** is the default backend
- Requires Windows 10+
- Falls back to CPU on older systems

### WASM/Browser

- **WebGPU** backend (Chrome 113+, Edge 113+)
- Build with: `make wasm`
- Deploy with: `./scripts/deploy-wasm.sh`

## Common Tasks

### Adding a New Backend

```bash
./scripts/new-backend.sh opencl OPENCL
```

This generates a complete backend template with:

- Cargo.toml configuration
- Device implementation stub
- Shader templates
- Tests

### Running Benchmarks

```bash
./scripts/benchmark.sh
```

Results saved to `benchmark-results/` with:

- JSON data for analysis
- Markdown report
- Performance metrics

### Deploying WASM

```bash
./scripts/deploy-wasm.sh
```

Creates `dist/` with:

- Compiled WASM module
- JavaScript bindings
- Example HTML page

Test locally:

```bash
cd dist
python3 -m http.server 8080
# Open http://localhost:8080
```

### Deploying Native Binaries

```bash
./scripts/deploy-native.sh
```

Creates platform-specific packages in `releases/`:

- Compiled binaries
- Documentation
- Install scripts

## Learning Resources

### Documentation

1. **[API Guide](docs/API_GUIDE.md)** - Complete API reference with examples
2. **[Build Guide](docs/BUILD_GUIDE.md)** - Platform-specific build instructions
3. **[WASM Guide](docs/WASM_GUIDE.md)** - Browser deployment guide
4. **[Architecture](ARCHITECTURE.md)** - System design overview
5. **[Quick Reference](docs/QUICK_REFERENCE.md)** - Cheat sheet

### Examples

1. **simple-inference** - Basic usage demonstration
2. **complete-workflow** - Comprehensive example with quantization

### Code Examples

See `docs/API_GUIDE.md` for:

- Tensor operations
- Custom models
- Multi-backend execution
- Quantization strategies
- Performance optimization

## Troubleshooting

### Build Errors

**Problem**: Compilation fails with dependency errors

```bash
# Solution: Clean and rebuild
make clean
cargo update
make build
```

**Problem**: GPU backend not available

```bash
# Solution: Use CPU fallback explicitly
cargo run --release --features cpu-only
```

### Runtime Errors

**Problem**: GPU initialization fails

- Check GPU drivers are installed
- Verify platform compatibility in `docs/BUILD_GUIDE.md`
- Use CPU backend as fallback

**Problem**: Out of memory on GPU

- Use quantization (8-bit or 4-bit)
- Reduce batch size
- Use CPU backend for larger models

### WASM Issues

**Problem**: WASM build fails

```bash
# Install wasm-pack
cargo install wasm-pack

# Use build script
./build-wasm.sh
```

**Problem**: WebGPU not supported in browser

- Use Chrome/Edge 113+ or Firefox Nightly
- Check `chrome://flags/#enable-webgpu`

## Next Steps

### For Users

1. ✅ Run the examples
2. 📖 Read the API Guide
3. 🚀 Build your first model
4. 📊 Run benchmarks on your hardware

### For Contributors

1. 📋 Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. 🐛 Pick an issue from GitHub
3. 🔧 Implement GPU kernels
4. ✅ Add tests and documentation

### For Researchers

1. 🧪 Experiment with quantization
2. 📈 Benchmark on different hardware
3. 🔬 Implement new model architectures
4. 📝 Share your findings

## Getting Help

- **Documentation**: See `docs/` directory
- **Examples**: See `examples/` directory
- **Issues**: GitHub issue tracker
- **Discussions**: GitHub discussions

## Quick Command Reference

```bash
# Build & Test
make build           # Build all packages
make test            # Run tests
make ci              # Full CI checks

# Development
make fmt             # Format code
make lint            # Run clippy
make doc             # Generate docs

# Examples
make run             # Run simple inference
make example         # Run complete workflow

# Deployment
make wasm            # Build WASM
make release         # Build release binaries
make benchmark       # Run benchmarks

# Utilities
make clean           # Clean build artifacts
./scripts/verify.sh  # Verify project health
```

## Success Checklist

- [ ] Rust 1.70+ installed
- [ ] Project builds successfully
- [ ] Tests pass
- [ ] Simple example runs
- [ ] GPU backend detected (or CPU fallback works)
- [ ] Documentation accessible

**Congratulations! You're ready to use CrossGPU! 🎉**

For detailed information, continue to the [API Guide](docs/API_GUIDE.md).
