# CrossGPU Quick Reference

A cheat sheet for common CrossGPU operations and patterns.

## Installation

```bash
git clone https://github.com/hmthanh/CrossGPU.git
cd CrossGPU
cargo build --release
```

## Basic Usage

### Create CPU Device

```rust
use crossgpu_backend_cpu::CpuDevice;
let device = CpuDevice::new();
```

### Create Tensor

```rust
use crossgpu_core::tensor::{Tensor, DType};

// Empty tensor
let tensor = Tensor::new(vec![2, 3], DType::F32);

// From f32 data
let tensor = Tensor::from_f32(
    vec![2, 2],
    vec![1.0, 2.0, 3.0, 4.0]
)?;
```

### Upload/Download Tensors

```rust
// Upload to GPU
let gpu_tensor = device.upload_tensor(&cpu_tensor)?;

// Download from GPU
let cpu_tensor = device.download_tensor(&gpu_tensor)?;

// Synchronize
device.synchronize()?;
```

### Run Kernels

```rust
use crossgpu_core::gpu::{Kernel, KernelType};

let kernel = Kernel::new(KernelType::MatMul);
let output = device.run_kernel(kernel, &[input1, input2])?;
```

## Device Selection

### Auto-detect Platform

```rust
use crossgpu_core::gpu::DeviceType;

let device_type = DeviceType::default_for_platform();
// Linux → Vulkan, macOS → Metal, Windows → DX12, WASM → WebGPU
```

### Manual Selection

```rust
use crossgpu_backend_cpu::CpuDevice;
use crossgpu_backend_webgpu::WebGpuDevice;

// CPU
let device = CpuDevice::new();

// WebGPU (async)
let device = WebGpuDevice::new().await?;
```

## Transformer Model

### Create Config

```rust
use crossgpu_core::transformer::TransformerConfig;

// Tiny config (~50MB)
let config = TransformerConfig::tiny();

// Custom config
let config = TransformerConfig {
    d_model: 512,
    n_heads: 8,
    n_layers: 6,
    d_ff: 2048,
    vocab_size: 32000,
    max_seq_len: 512,
    dropout: 0.1,
    layer_norm_eps: 1e-5,
};
```

### Save/Load Model

```rust
// Save
model.save_to_file("model.bin")?;

// Load
let model = TransformerModel::load_from_file("model.bin")?;
```

## Quantization

### 8-bit Quantization

```rust
use crossgpu_core::quantization::{quantize_tensor, QuantParams};

let params = QuantParams::int8_symmetric(0.1);
let quantized = quantize_tensor(&tensor, &params)?;
```

### 4-bit Quantization

```rust
let params = QuantParams::int4(0.2);
let quantized = quantize_tensor(&tensor, &params)?;
```

### Dequantization

```rust
use crossgpu_core::quantization::dequantize_tensor;

let restored = dequantize_tensor(&quantized, &params)?;
```

## Kernel Types

| Kernel | Description | Parameters |
|--------|-------------|------------|
| `MatMul` | Matrix multiplication | - |
| `LayerNorm` | Layer normalization | `[epsilon]` |
| `Softmax` | Softmax activation | - |
| `Gelu` | GELU activation | - |
| `FusedGemmGelu` | GEMM + GELU | - |
| `FusedGemmLayerNorm` | GEMM + LayerNorm | `[epsilon]` |
| `Attention` | Multi-head attention | - |

### Example with Parameters

```rust
let kernel = Kernel::with_params(
    KernelType::LayerNorm,
    vec![1e-5]  // epsilon
);
```

## Error Handling

### Using Results

```rust
use crossgpu_core::{CoreError, Result};

fn process() -> Result<Tensor> {
    let tensor = Tensor::new(vec![2, 2], DType::F32);
    let reshaped = tensor.reshape(vec![4])?;
    Ok(reshaped)
}
```

### Pattern Matching

```rust
match result {
    Ok(tensor) => println!("Success: {:?}", tensor.shape),
    Err(CoreError::ShapeMismatch { expected, actual }) => {
        eprintln!("Shape error: expected {:?}, got {:?}", expected, actual);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## WASM

### Build WASM

```bash
cd wasm
wasm-pack build --target web --release
```

### Use in HTML

```html
<script type="module">
    import init, { greet } from './pkg/crossgpu_wasm.js';
    
    async function run() {
        await init();
        console.log(greet('World'));
    }
    
    run();
</script>
```

## Build Commands

```bash
# Build all
cargo build --release --all

# Build specific package
cargo build -p crossgpu-core --release

# Build example
cargo run --release --bin simple-inference

# Run tests
cargo test --all

# Format code
cargo fmt --all

# Lint
cargo clippy --all-targets --all-features

# Documentation
cargo doc --no-deps --open
```

## Logging

```bash
# Set log level
export RUST_LOG=debug

# In code
env_logger::init();
log::info!("Starting inference");
log::debug!("Device: {}", device.device_name());
```

## Common Patterns

### Device Factory

```rust
fn create_device() -> Result<Arc<dyn GpuDevice>> {
    let device_type = DeviceType::default_for_platform();
    
    match device_type {
        DeviceType::Cpu => Ok(Arc::new(CpuDevice::new())),
        DeviceType::WebGpu => {
            let device = WebGpuDevice::new().await?;
            Ok(Arc::new(device))
        }
        // ... other backends
    }
}
```

### Batch Processing

```rust
let batch_size = 4;
let seq_len = 10;
let input = Tensor::from_f32(
    vec![batch_size, seq_len],
    vec![1.0; batch_size * seq_len]
)?;
```

### Async Runtime (for WebGPU)

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let device = WebGpuDevice::new().await?;
    // ... use device
    Ok(())
}
```

## Tensor Operations

### Shape Manipulation

```rust
// Get properties
tensor.shape        // Vec<usize>
tensor.numel()      // Total elements
tensor.ndim()       // Number of dimensions

// Reshape
let reshaped = tensor.reshape(vec![4, 6])?;
```

### Data Access

```rust
// Read data
let slice = tensor.as_f32_slice()?;

// Modify data
let slice_mut = tensor.as_f32_slice_mut()?;
slice_mut[0] = 42.0;
```

## Performance Tips

1. **Batch operations**: Process multiple inputs together
2. **Fused kernels**: Use `FusedGemmGelu` instead of separate operations
3. **Quantization**: Use INT8/INT4 for smaller models
4. **Async GPU ops**: Don't block on synchronize unless needed
5. **Reuse buffers**: Upload tensors once, run multiple kernels

## Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable trace logging for specific module
RUST_LOG=crossgpu_core=trace cargo run

# Run with backtrace
RUST_BACKTRACE=1 cargo run
```

## Platform-Specific Notes

### Linux
- Default: Vulkan
- Requires: `libvulkan-dev`

### macOS
- Default: Metal
- Requires: Xcode Command Line Tools

### Windows
- Default: DirectX 12
- Requires: Visual Studio Build Tools

### WASM
- Default: WebGPU
- Requires: wasm-pack, Chrome 113+

## Common Issues

### Shape Mismatch
```rust
// Error: ShapeMismatch { expected: [6], actual: [8] }
// Fix: Ensure reshape preserves element count
let tensor = Tensor::new(vec![2, 3], DType::F32);  // 6 elements
let reshaped = tensor.reshape(vec![3, 2])?;        // Still 6 elements ✓
```

### GPU Not Available
```rust
// Check availability
if !device.is_available() {
    // Fall back to CPU
    let device = CpuDevice::new();
}
```

### WASM Not Loading
```html
<!-- Serve via HTTP, not file:// -->
<!-- Use: python3 -m http.server -->
```

## Resources

- [API Guide](API_GUIDE.md) - Full API documentation
- [Build Guide](BUILD_GUIDE.md) - Build instructions
- [WASM Guide](WASM_GUIDE.md) - WebAssembly deployment
- [Architecture](../ARCHITECTURE.md) - System design
- `cargo doc --open` - API reference

## Version Info

- **Rust**: 1.70+
- **CrossGPU**: 0.1.0
- **Last Updated**: October 2025

---

For more details, see the [full documentation](README.md).
