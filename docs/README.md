# CrossGPU Documentation

Welcome to the CrossGPU documentation! This directory contains comprehensive guides and references for using the CrossGPU Tiny Transformer inference engine.

## 📚 Documentation Index

### Getting Started

- **[../README.md](../README.md)** - Project overview and quick start guide
- **[BUILD_GUIDE.md](BUILD_GUIDE.md)** - Detailed build instructions for all platforms
- **[API_GUIDE.md](API_GUIDE.md)** - Complete API usage guide with examples

### Advanced Topics

- **[WASM_GUIDE.md](WASM_GUIDE.md)** - WebAssembly deployment and browser usage
- **[../ARCHITECTURE.md](../ARCHITECTURE.md)** - System architecture and design decisions
- **[../CONTRIBUTING.md](../CONTRIBUTING.md)** - Contribution guidelines

### Quick References

- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick reference for common tasks
- **[../PROJECT_SUMMARY.md](../PROJECT_SUMMARY.md)** - Project statistics and overview

## 🚀 Quick Start

### 1. Installation

```bash
git clone https://github.com/hmthanh/CrossGPU.git
cd CrossGPU
cargo build --release
```

### 2. Run Example

```bash
cargo run --release --bin simple-inference
```

### 3. Build WASM

```bash
cd wasm
wasm-pack build --target web --release
```

## 📖 Documentation by Use Case

### I want to...

#### Use CrossGPU in my Rust project
→ Read [API_GUIDE.md](API_GUIDE.md) for integration examples

#### Build for a specific platform
→ See [BUILD_GUIDE.md](BUILD_GUIDE.md) for platform-specific instructions

#### Deploy to browsers
→ Follow [WASM_GUIDE.md](WASM_GUIDE.md) for web deployment

#### Understand the architecture
→ Read [ARCHITECTURE.md](../ARCHITECTURE.md) for design details

#### Contribute to the project
→ Check [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines

#### Find a specific API
→ Run `cargo doc --open` for API documentation

## 🎯 Learning Path

### Beginner
1. Read the [README](../README.md) for project overview
2. Follow [BUILD_GUIDE.md](BUILD_GUIDE.md) to set up your environment
3. Run the `simple-inference` example
4. Explore [API_GUIDE.md](API_GUIDE.md) for basic usage

### Intermediate
1. Review [ARCHITECTURE.md](../ARCHITECTURE.md) to understand the design
2. Study the [quantization examples](API_GUIDE.md#quantization)
3. Experiment with different GPU backends
4. Try the `complete-workflow` example

### Advanced
1. Deep dive into [ARCHITECTURE.md](../ARCHITECTURE.md)
2. Implement custom kernels
3. Optimize for your specific use case
4. Contribute improvements via [CONTRIBUTING.md](../CONTRIBUTING.md)

## 📝 Code Examples

### Basic Inference

```rust
use crossgpu_core::{gpu::GpuDevice, tensor::Tensor};
use crossgpu_backend_cpu::CpuDevice;

let device = CpuDevice::new();
let input = Tensor::from_f32(vec![1, 10], vec![1.0; 10])?;
let gpu_input = device.upload_tensor(&input)?;
let output = device.download_tensor(&gpu_input)?;
```

### Auto-Detect Backend

```rust
use crossgpu_core::gpu::DeviceType;

let device_type = DeviceType::default_for_platform();
let device = create_device(device_type)?;
println!("Using: {}", device.device_name());
```

### Quantization

```rust
use crossgpu_core::quantization::{quantize_tensor, QuantParams};

let params = QuantParams::int8_symmetric(0.1);
let quantized = quantize_tensor(&tensor, &params)?;
println!("Compressed {}x", original_size / quantized_size);
```

## 🔧 Tools & Commands

### Build Commands

```bash
# Build all packages
cargo build --release --all

# Build specific backend
cargo build -p crossgpu-backend-webgpu --release

# Build WASM
cd wasm && wasm-pack build --target web --release
```

### Testing Commands

```bash
# Run all tests
cargo test --all

# Run with logging
RUST_LOG=debug cargo test --all

# Run specific test
cargo test -p crossgpu-core test_tensor_creation
```

### Quality Commands

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features

# Generate documentation
cargo doc --no-deps --open
```

## 🌐 External Resources

### Rust & WebAssembly
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Rust WASM Book](https://rustwasm.github.io/book/)

### GPU Programming
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [Metal Programming Guide](https://developer.apple.com/metal/)
- [DirectX 12 Documentation](https://docs.microsoft.com/en-us/windows/win32/direct3d12/)

### Machine Learning
- [Attention Is All You Need](https://arxiv.org/abs/1706.03762) - Original Transformer paper
- [GELU Paper](https://arxiv.org/abs/1606.08415)
- [Quantization Overview](https://arxiv.org/abs/2103.13630)

## 🤝 Community

- **Issues**: [GitHub Issues](https://github.com/hmthanh/CrossGPU/issues)
- **Discussions**: [GitHub Discussions](https://github.com/hmthanh/CrossGPU/discussions)
- **Pull Requests**: [Contributing Guide](../CONTRIBUTING.md)

## 📊 Project Structure

```
CrossGPU/
├── core/                    # Core library
├── backends/                # GPU backend implementations
│   ├── cpu/                # CPU fallback
│   ├── webgpu/             # WebGPU support
│   ├── vulkan/             # Vulkan (Linux)
│   ├── metal/              # Metal (macOS)
│   └── dx12/               # DirectX 12 (Windows)
├── examples/               # Example applications
│   ├── simple-inference/   # Basic usage
│   └── complete-workflow.rs # Comprehensive example
├── wasm/                   # WebAssembly build
├── tests/                  # Integration tests
└── docs/                   # Documentation (you are here!)
```

## 🔍 Troubleshooting

### Common Issues

**Build Errors**: See [BUILD_GUIDE.md](BUILD_GUIDE.md#troubleshooting)

**WASM Issues**: Check [WASM_GUIDE.md](WASM_GUIDE.md#troubleshooting)

**Runtime Errors**: Enable logging with `RUST_LOG=debug`

**GPU Not Detected**: Verify drivers are installed (see [BUILD_GUIDE.md](BUILD_GUIDE.md))

## 📄 License

This project is dual-licensed under MIT OR Apache-2.0.

See [LICENSE-MIT](../LICENSE-MIT) and [LICENSE-APACHE](../LICENSE-APACHE) for details.

## 🙏 Acknowledgments

Built with amazing Rust ecosystem tools:
- `wgpu` for WebGPU support
- `vulkano` for Vulkan bindings  
- `metal-rs` for Metal support
- `ndarray` for CPU linear algebra
- And many more!

---

**Last Updated**: October 2025

**Documentation Version**: 0.1.0

For the latest updates, visit [GitHub](https://github.com/hmthanh/CrossGPU).
