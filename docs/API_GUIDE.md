# CrossGPU API Usage Guide

This guide provides examples and best practices for using the CrossGPU library in your projects.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Creating and Managing Tensors](#creating-and-managing-tensors)
- [GPU Device Selection](#gpu-device-selection)
- [Model Loading and Inference](#model-loading-and-inference)
- [Quantization](#quantization)
- [Custom Backends](#custom-backends)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Installation

### As a Library Dependency

Add CrossGPU to your `Cargo.toml`:

```toml
[dependencies]
crossgpu-core = { path = "path/to/CrossGPU/core" }
crossgpu-backend-cpu = { path = "path/to/CrossGPU/backends/cpu" }
crossgpu-backend-webgpu = { path = "path/to/CrossGPU/backends/webgpu" }

# Optional: Add other backends as needed
# crossgpu-backend-vulkan = { path = "path/to/CrossGPU/backends/vulkan" }
# crossgpu-backend-metal = { path = "path/to/CrossGPU/backends/metal" }
# crossgpu-backend-dx12 = { path = "path/to/CrossGPU/backends/dx12" }

tokio = { version = "1.35", features = ["rt", "rt-multi-thread"] }
anyhow = "1.0"
log = "0.4"
env_logger = "0.11"
```

### For WASM Projects

```toml
[dependencies]
crossgpu-wasm = { path = "path/to/CrossGPU/wasm" }
wasm-bindgen = "0.2"
```

## Quick Start

### Basic CPU Inference

```rust
use crossgpu_core::{
    tensor::{Tensor, DType},
    gpu::GpuDevice,
};
use crossgpu_backend_cpu::CpuDevice;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Create CPU device
    let device = CpuDevice::new();
    println!("Using device: {}", device.device_name());
    
    // Create input tensor
    let input = Tensor::from_f32(
        vec![1, 4],  // Shape: [batch_size, features]
        vec![1.0, 2.0, 3.0, 4.0]
    )?;
    
    // Upload to device
    let gpu_input = device.upload_tensor(&input)?;
    
    // Run computation (placeholder)
    // let gpu_output = device.run_kernel(kernel, &[gpu_input])?;
    
    // Download result
    let output = device.download_tensor(&gpu_input)?;
    println!("Output shape: {:?}", output.shape);
    
    Ok(())
}
```

### GPU Inference with Auto-Detection

```rust
use crossgpu_core::{
    gpu::{DeviceType, GpuDevice},
    tensor::Tensor,
};
use std::sync::Arc;
use anyhow::Result;

fn create_device() -> Result<Arc<dyn GpuDevice>> {
    let device_type = DeviceType::default_for_platform();
    
    match device_type {
        DeviceType::WebGpu => {
            let device = tokio::runtime::Runtime::new()?
                .block_on(crossgpu_backend_webgpu::WebGpuDevice::new())?;
            Ok(Arc::new(device))
        }
        DeviceType::Vulkan => {
            Ok(Arc::new(crossgpu_backend_vulkan::VulkanDevice::new()?))
        }
        DeviceType::Metal => {
            Ok(Arc::new(crossgpu_backend_metal::MetalDevice::new()?))
        }
        DeviceType::Dx12 => {
            Ok(Arc::new(crossgpu_backend_dx12::Dx12Device::new()?))
        }
        DeviceType::Cpu => {
            Ok(Arc::new(crossgpu_backend_cpu::CpuDevice::new()))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let device = create_device()?;
    println!("Using GPU backend: {}", device.device_name());
    
    // Your inference code here...
    
    Ok(())
}
```

## Core Concepts

### Tensors

Tensors are n-dimensional arrays that store model weights and activations.

```rust
use crossgpu_core::tensor::{Tensor, DType};

// Create an empty tensor
let tensor = Tensor::new(vec![2, 3, 4], DType::F32);

// Create from f32 data
let data = vec![1.0, 2.0, 3.0, 4.0];
let tensor = Tensor::from_f32(vec![2, 2], data)?;

// Get tensor properties
println!("Shape: {:?}", tensor.shape);
println!("Number of elements: {}", tensor.numel());
println!("Dimensions: {}", tensor.ndim());

// Reshape
let reshaped = tensor.reshape(vec![4, 1])?;
```

### GPU Device Trait

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

### Kernels

Kernels are GPU compute operations:

```rust
use crossgpu_core::gpu::{Kernel, KernelType};

// Create a kernel
let matmul_kernel = Kernel::new(KernelType::MatMul);

// Kernel with parameters (e.g., LayerNorm epsilon)
let layernorm_kernel = Kernel::with_params(
    KernelType::LayerNorm,
    vec![1e-5]  // epsilon parameter
);

// Run kernel on device
let output = device.run_kernel(matmul_kernel, &[input1, input2])?;
```

## Creating and Managing Tensors

### From Raw Data

```rust
use crossgpu_core::tensor::{Tensor, DType};

// From f32 vector
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
let tensor = Tensor::from_f32(vec![2, 3], data)?;

// From raw bytes
let bytes = vec![0u8; 24];  // 6 floats × 4 bytes
let tensor = Tensor::from_data(vec![2, 3], DType::F32, bytes)?;
```

### Accessing Tensor Data

```rust
// Get data as f32 slice (read-only)
let slice = tensor.as_f32_slice()?;
for value in slice {
    println!("{}", value);
}

// Get mutable slice
let mut tensor = Tensor::new(vec![2, 2], DType::F32);
let slice_mut = tensor.as_f32_slice_mut()?;
slice_mut[0] = 42.0;
```

### Tensor Operations

```rust
// Reshape (must preserve number of elements)
let tensor = Tensor::new(vec![2, 3, 4], DType::F32);
let flat = tensor.reshape(vec![24])?;
let reshaped = flat.reshape(vec![4, 6])?;

// Clone
let tensor_copy = tensor.clone();
```

## GPU Device Selection

### Platform-Based Auto-Detection

```rust
use crossgpu_core::gpu::DeviceType;

let default_type = DeviceType::default_for_platform();
match default_type {
    DeviceType::WebGpu => println!("WASM/Browser detected"),
    DeviceType::Metal => println!("macOS detected"),
    DeviceType::Dx12 => println!("Windows detected"),
    DeviceType::Vulkan => println!("Linux detected"),
    DeviceType::Cpu => println!("No GPU detected"),
}
```

### Manual Backend Selection

```rust
use std::sync::Arc;
use crossgpu_core::gpu::GpuDevice;

fn create_specific_backend(backend_name: &str) -> Result<Arc<dyn GpuDevice>> {
    match backend_name {
        "cpu" => Ok(Arc::new(crossgpu_backend_cpu::CpuDevice::new())),
        "vulkan" => Ok(Arc::new(crossgpu_backend_vulkan::VulkanDevice::new()?)),
        "metal" => Ok(Arc::new(crossgpu_backend_metal::MetalDevice::new()?)),
        "dx12" => Ok(Arc::new(crossgpu_backend_dx12::Dx12Device::new()?)),
        _ => Err(anyhow::anyhow!("Unknown backend: {}", backend_name)),
    }
}
```

### Fallback Chain

```rust
fn create_device_with_fallback() -> Arc<dyn GpuDevice> {
    // Try GPU backends first
    if let Ok(device) = crossgpu_backend_webgpu::WebGpuDevice::new().await {
        return Arc::new(device);
    }
    
    #[cfg(target_os = "linux")]
    if let Ok(device) = crossgpu_backend_vulkan::VulkanDevice::new() {
        return Arc::new(device);
    }
    
    #[cfg(target_os = "macos")]
    if let Ok(device) = crossgpu_backend_metal::MetalDevice::new() {
        return Arc::new(device);
    }
    
    #[cfg(target_os = "windows")]
    if let Ok(device) = crossgpu_backend_dx12::Dx12Device::new() {
        return Arc::new(device);
    }
    
    // Fallback to CPU
    Arc::new(crossgpu_backend_cpu::CpuDevice::new())
}
```

## Model Loading and Inference

### Creating a Transformer Model

```rust
use crossgpu_core::transformer::{
    TransformerConfig,
    TransformerModel,
    TransformerLayerWeights,
    AttentionWeights,
    FeedForwardWeights,
    LayerNormWeights,
};
use crossgpu_core::tensor::{Tensor, DType};

fn create_model() -> Result<TransformerModel> {
    // Define configuration
    let config = TransformerConfig::tiny();
    
    // Create embeddings
    let token_embedding = Tensor::new(
        vec![config.vocab_size, config.d_model],
        DType::F32
    );
    let position_embedding = Tensor::new(
        vec![config.max_seq_len, config.d_model],
        DType::F32
    );
    
    // Create layer weights
    let mut layers = Vec::new();
    for _ in 0..config.n_layers {
        let attention = AttentionWeights {
            wq: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wk: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wv: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wo: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
        };
        
        let feed_forward = FeedForwardWeights {
            w1: Tensor::new(vec![config.d_model, config.d_ff], DType::F32),
            w2: Tensor::new(vec![config.d_ff, config.d_model], DType::F32),
        };
        
        let ln1 = LayerNormWeights {
            gamma: Tensor::new(vec![config.d_model], DType::F32),
            beta: Tensor::new(vec![config.d_model], DType::F32),
        };
        
        let ln2 = LayerNormWeights {
            gamma: Tensor::new(vec![config.d_model], DType::F32),
            beta: Tensor::new(vec![config.d_model], DType::F32),
        };
        
        layers.push(TransformerLayerWeights {
            attention,
            feed_forward,
            ln1,
            ln2,
        });
    }
    
    let final_layer_norm = LayerNormWeights {
        gamma: Tensor::new(vec![config.d_model], DType::F32),
        beta: Tensor::new(vec![config.d_model], DType::F32),
    };
    
    Ok(TransformerModel::new(
        config,
        token_embedding,
        position_embedding,
        layers,
        final_layer_norm,
    ))
}
```

### Saving and Loading Models

```rust
// Save model to file
let model = create_model()?;
model.save_to_file("model.bin")?;
println!("Model saved to model.bin");

// Load model from file
let loaded_model = TransformerModel::load_from_file("model.bin")?;
println!("Model loaded successfully");
```

### Running Inference

```rust
use crossgpu_core::{
    transformer::TransformerLayer,
    gpu::GpuDevice,
};

fn run_inference(
    model: &TransformerModel,
    input_ids: Vec<usize>,
    device: &Arc<dyn GpuDevice>
) -> Result<Tensor> {
    // Convert input IDs to tensor
    let input_data: Vec<f32> = input_ids.iter().map(|&x| x as f32).collect();
    let input = Tensor::from_f32(vec![1, input_ids.len()], input_data)?;
    
    // Upload to GPU
    let mut gpu_tensor = device.upload_tensor(&input)?;
    
    // Process through layers
    for (i, layer_weights) in model.layers.iter().enumerate() {
        log::info!("Processing layer {}", i);
        let layer = TransformerLayer::new(model.config.clone(), layer_weights.clone());
        gpu_tensor = layer.forward_gpu(&gpu_tensor, device)?;
    }
    
    // Download result
    let output = device.download_tensor(&gpu_tensor)?;
    device.synchronize()?;
    
    Ok(output)
}
```

## Quantization

### Quantizing a Model

```rust
use crossgpu_core::quantization::{quantize_tensor, QuantParams};

// Create quantization parameters
let params = QuantParams::int8_symmetric(0.1);  // scale = 0.1

// Quantize a tensor
let fp32_tensor = Tensor::from_f32(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;
let quantized = quantize_tensor(&fp32_tensor, &params)?;

println!("Original size: {} bytes", fp32_tensor.data.len());
println!("Quantized size: {} bytes", quantized.data.len());
println!("Compression ratio: {}x", fp32_tensor.data.len() / quantized.data.len());
```

### Dequantizing for Inference

```rust
use crossgpu_core::quantization::dequantize_tensor;

// Dequantize back to FP32
let dequantized = dequantize_tensor(&quantized, &params)?;

// Verify values are close to original
let original = fp32_tensor.as_f32_slice()?;
let restored = dequantized.as_f32_slice()?;

for (i, (&orig, &rest)) in original.iter().zip(restored.iter()).enumerate() {
    println!("Element {}: {:.2} -> {:.2}", i, orig, rest);
}
```

### Quantization Schemes

```rust
// 8-bit symmetric (zero-point = 0)
let symmetric = QuantParams::int8_symmetric(0.05);

// 8-bit asymmetric (with zero-point)
let asymmetric = QuantParams::int8_asymmetric(0.05, 128);

// 4-bit quantization (extreme compression)
let int4 = QuantParams::int4(0.1);
```

## Error Handling

### Using Result Types

```rust
use crossgpu_core::{CoreError, Result};

fn process_tensor(tensor: &Tensor) -> Result<Tensor> {
    // All operations return Result
    let reshaped = tensor.reshape(vec![4, 6])?;
    let quantized = quantize_tensor(&reshaped, &params)?;
    Ok(quantized)
}

// Handle errors
match process_tensor(&my_tensor) {
    Ok(result) => println!("Success: {:?}", result.shape),
    Err(CoreError::ShapeMismatch { expected, actual }) => {
        eprintln!("Shape mismatch: expected {:?}, got {:?}", expected, actual);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Error Types

```rust
use crossgpu_core::CoreError;

// Different error variants
match error {
    CoreError::ShapeMismatch { expected, actual } => { /* ... */ }
    CoreError::InvalidDimension(msg) => { /* ... */ }
    CoreError::GpuError(msg) => { /* ... */ }
    CoreError::QuantizationError(msg) => { /* ... */ }
    CoreError::ModelLoadError(msg) => { /* ... */ }
    CoreError::IoError(e) => { /* ... */ }
    CoreError::SerializationError(msg) => { /* ... */ }
    CoreError::Other(msg) => { /* ... */ }
}
```

## Best Practices

### 1. Resource Management

```rust
// Use Arc for shared device access
let device = Arc::new(CpuDevice::new());
let device_clone = Arc::clone(&device);

// Explicitly synchronize after GPU operations
device.synchronize()?;
```

### 2. Batch Processing

```rust
// Process multiple inputs together
let batch_input = Tensor::from_f32(
    vec![batch_size, seq_len],
    input_data
)?;
```

### 3. Logging

```rust
use log::{info, debug, error};

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    info!("Starting inference");
    debug!("Device: {:?}", device.device_name());
}
```

### 4. Error Propagation

```rust
// Use ? operator for cleaner error handling
fn full_pipeline() -> Result<Tensor> {
    let model = load_model()?;
    let device = create_device()?;
    let output = run_inference(&model, &device)?;
    Ok(output)
}
```

### 5. Type Safety

```rust
// Use type system to prevent errors
fn process_f32_tensor(tensor: &Tensor) -> Result<()> {
    // Verify data type at runtime if needed
    if tensor.dtype != DType::F32 {
        return Err(CoreError::Other("Expected F32 tensor".to_string()));
    }
    Ok(())
}
```

## Advanced Usage

### Custom Kernel Execution

```rust
use crossgpu_core::gpu::{Kernel, KernelType};

// Run custom kernel with parameters
let kernel = Kernel::with_params(
    KernelType::LayerNorm,
    vec![1e-5, 512.0]  // epsilon, normalized_shape
);

let output = device.run_kernel(kernel, &[input])?;
```

### Async GPU Operations (WebGPU)

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let device = crossgpu_backend_webgpu::WebGpuDevice::new().await?;
    
    // GPU operations are async
    let gpu_tensor = device.upload_tensor(&input)?;
    let output = device.run_kernel(kernel, &[gpu_tensor])?;
    
    // Synchronize
    device.synchronize()?;
    
    Ok(())
}
```

## Examples

See the [examples directory](../examples/) for complete working examples:

- `simple-inference`: Basic transformer inference
- More examples coming soon...

## API Reference

For detailed API documentation, run:

```bash
cargo doc --no-deps --open
```

## Further Reading

- [ARCHITECTURE.md](../ARCHITECTURE.md) - System architecture
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - Build instructions
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines

---

**Questions?** Open an issue on [GitHub](https://github.com/hmthanh/CrossGPU/issues).
