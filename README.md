# CrossGPU - Tiny Transformer Inference Engine

[![CI/CD](https://github.com/hmthanh/CrossGPU/workflows/CI%2FCD/badge.svg)](https://github.com/hmthanh/CrossGPU/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A **production-ready Rust project template** for building a lightweight Transformer inference engine (~50MB model size) with GPU abstraction supporting multiple backends. Designed for portability, modularity, and Rust best practices.

## 🌟 Features

- **Lightweight Transformer Engine**: Efficient inference-only implementation
- **Multi-Backend GPU Support**:
  - 🌐 **WebGPU**: Browser and native (via `wgpu`)
  - 🐧 **Vulkan**: Linux, Windows, Android
  - 🍎 **Metal**: macOS, iOS
  - 🪟 **DirectX 12**: Windows
  - 💻 **CPU Fallback**: SIMD/BLAS optimized
- **Quantization Support**: 8-bit and 4-bit quantized kernels
- **WASM Ready**: Compile to WebAssembly for browser deployment
- **Cross-Platform**: Build for Linux, macOS, Windows, and web

## 📦 Project Structure

```
CrossGPU/
├── core/                      # Core tensor operations and transformer layers
│   ├── src/
│   │   ├── tensor.rs         # Tensor data structure
│   │   ├── transformer.rs    # Transformer layer definitions
│   │   ├── gpu.rs            # GpuDevice trait
│   │   ├── quantization.rs   # Quantization utilities
│   │   └── error.rs          # Error types
│   └── Cargo.toml
├── backends/                  # GPU backend implementations
│   ├── cpu/                  # CPU fallback (SIMD/BLAS)
│   ├── webgpu/               # WebGPU backend (browser + native)
│   ├── vulkan/               # Vulkan backend (Linux)
│   ├── metal/                # Metal backend (macOS)
│   └── dx12/                 # DirectX 12 backend (Windows)
├── examples/
│   └── simple-inference/     # Example transformer inference
├── wasm/                      # WASM build for browser
├── tests/                     # Integration tests
├── .github/workflows/         # CI/CD configuration
└── Cargo.toml                 # Workspace configuration
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- For WASM builds: `wasm-pack` ([Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/))

### Building

```bash
# Build all packages
cargo build --release

# Build specific backend
cargo build -p crossgpu-backend-cpu --release
cargo build -p crossgpu-backend-webgpu --release

# Run tests
cargo test --all

# Run example
cargo run --release --bin simple-inference
```

### WASM Build

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM package
cd wasm
wasm-pack build --target web --release

# The compiled WASM will be in wasm/pkg/
```

### Running the Example

```bash
# Run the simple inference example
RUST_LOG=info cargo run --release --bin simple-inference
```

## 💡 Usage Example

```rust
use crossgpu_core::{
    gpu::{DeviceType, GpuDevice},
    tensor::Tensor,
    transformer::TransformerConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Auto-detect best GPU backend
    let device = auto_detect_device()?;
    println!("Using device: {}", device.device_name());

    // Create a tiny transformer model
    let config = TransformerConfig::tiny();
    let model = create_model(config)?;

    // Prepare input
    let input = Tensor::from_f32(vec![1, 10], vec![1.0; 10])?;

    // Upload to GPU
    let gpu_input = device.upload_tensor(&input)?;

    // Run inference
    let gpu_output = run_transformer(&model, &gpu_input, &device)?;

    // Download result
    let output = device.download_tensor(&gpu_output)?;
    
    println!("Output shape: {:?}", output.shape);
    Ok(())
}
```

## 🎯 GPU Device Abstraction

All backends implement the `GpuDevice` trait:

```rust
pub trait GpuDevice: Send + Sync {
    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor>;
    fn run_kernel(&self, kernel: Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor>;
    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor>;
    fn synchronize(&self) -> Result<()>;
    fn device_name(&self) -> &str;
    fn is_available(&self) -> bool;
}
```

Backends automatically selected based on platform:
- **Linux**: Vulkan → CPU
- **macOS**: Metal → CPU  
- **Windows**: DirectX 12 → CPU
- **WASM**: WebGPU → CPU

## 🔧 Development

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with coverage
cargo test --all --verbose
```

### Platform-Specific Builds

```bash
# Linux (Vulkan)
cargo build --release

# macOS (Metal)
cargo build --release

# Windows (DirectX 12)
cargo build --release

# WASM (WebGPU)
cd wasm && wasm-pack build --target web
```

## 📊 Model Size Estimation

The tiny transformer configuration (~50MB):
- **Embedding**: 32K vocab × 512 dim = 65M params
- **6 Layers**: Each with attention + FFN = 4M params/layer
- **Total**: ~90M parameters × 4 bytes (F32) ≈ 360MB
- **Quantized (8-bit)**: ~90MB
- **Quantized (4-bit)**: ~45MB

## 🧪 Testing

```bash
# Run all tests
cargo test --all

# Run specific package tests
cargo test -p crossgpu-core
cargo test -p crossgpu-backend-cpu

# Run with logging
RUST_LOG=debug cargo test --all
```

## 📚 Documentation

Generate and view documentation:

```bash
cargo doc --all --no-deps --open
```

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Guidelines

- Follow Rust best practices and idioms
- Add tests for new features
- Ensure `cargo fmt` and `cargo clippy` pass
- Update documentation for public APIs

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

Built with:
- [wgpu](https://github.com/gfx-rs/wgpu) - Cross-platform graphics API
- [vulkano](https://github.com/vulkano-rs/vulkano) - Vulkan bindings
- [metal-rs](https://github.com/gfx-rs/metal-rs) - Metal bindings
- [ndarray](https://github.com/rust-ndarray/ndarray) - N-dimensional arrays

## 🔗 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [Transformer Architecture](https://arxiv.org/abs/1706.03762)

---

**Note**: This is a template project with placeholder implementations. Actual kernel implementations and model weights need to be added for production use.
