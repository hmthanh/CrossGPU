# CrossGPU Architecture

This document provides an overview of the CrossGPU architecture, design decisions, and implementation details.

## Overview

CrossGPU is a modular Rust framework for running Tiny Transformer inference across multiple GPU backends (WebGPU, Vulkan, Metal, DirectX 12) with CPU fallback support.

## Design Goals

1. **Cross-Platform**: Support all major platforms (Linux, macOS, Windows, Web)
2. **Modular**: Clean separation between core logic and backend implementations
3. **Efficient**: Minimize overhead and maximize GPU utilization
4. **Portable**: Compile to native and WebAssembly
5. **Type-Safe**: Leverage Rust's type system for correctness
6. **Extensible**: Easy to add new backends and operations

## Architecture Layers

### Layer 1: Core (`crossgpu-core`)

The core layer provides platform-agnostic abstractions:

```
┌─────────────────────────────────────┐
│        crossgpu-core                │
├─────────────────────────────────────┤
│ • Tensor (data structure)           │
│ • GpuDevice (trait)                 │
│ • Transformer (model definition)    │
│ • Quantization (8-bit, 4-bit)       │
│ • Error handling                    │
└─────────────────────────────────────┘
```

**Key Components**:

- **Tensor**: N-dimensional array with support for F32, F16, I8, I4 dtypes
- **GpuDevice**: Abstract interface for GPU operations
- **TransformerModel**: Model definition with layers, weights, and configuration
- **Quantization**: Utilities for model compression

### Layer 2: Backends (`backends/*`)

Each backend implements the `GpuDevice` trait:

```
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│  WebGPU  │  │  Vulkan  │  │  Metal   │  │   DX12   │  │   CPU    │
└──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘
     │             │              │              │              │
     └─────────────┴──────────────┴──────────────┴──────────────┘
                            GpuDevice trait
```

**Backend Responsibilities**:
- Tensor upload/download
- Kernel execution
- Memory management
- Synchronization

### Layer 3: Applications

Applications use the core API with backend selection:

```
┌─────────────────────────────────────┐
│     Application Layer               │
│  • Simple Inference Example         │
│  • WASM Browser Demo                │
│  • Custom Applications              │
└─────────────────────────────────────┘
          │
          ▼
     Auto-detect or
     Manual Backend Selection
```

## Data Flow

### Inference Pipeline

```
Input Tokens
    │
    ▼
┌─────────────────┐
│ Token Embedding │
│ + Positional    │
└─────────────────┘
    │
    ▼
┌─────────────────┐
│ Transformer     │◄── GPU Device
│ Layers (x6)     │
│ • Attention     │
│ • Feed-Forward  │
│ • Layer Norm    │
└─────────────────┘
    │
    ▼
┌─────────────────┐
│ Final Layer     │
│ Normalization   │
└─────────────────┘
    │
    ▼
Output Logits
```

### GPU Kernel Workflow

```
CPU Tensor
    │
    ├──► upload_tensor() ──► GPU Tensor
    │
    ├──► run_kernel() ─────► GPU Computation
    │                         │
    │                         ▼
    │                    GPU Tensor (result)
    │
    └──► download_tensor() ◄─┘
         │
         ▼
    CPU Tensor (result)
```

## Memory Management

### Tensor Lifecycle

1. **Creation**: Allocate on CPU
2. **Upload**: Copy to GPU memory
3. **Computation**: GPU kernels operate on GPU memory
4. **Download**: Copy back to CPU (if needed)
5. **Cleanup**: Rust's RAII handles deallocation

### Quantization

```
F32 Tensor (4 bytes/element)
    │
    ├─► Quantize ─────► I8 Tensor (1 byte/element)
    │                   • 4x compression
    │                   • Minimal accuracy loss
    │
    └─► Quantize ─────► I4 Tensor (0.5 bytes/element)
                        • 8x compression
                        • Some accuracy loss
```

## Backend-Specific Details

### WebGPU Backend

- **Shaders**: WGSL (WebGPU Shading Language)
- **Platform**: Browser (WASM) + Native
- **Advantages**: Widest compatibility
- **Use Case**: Browser deployment, prototyping

### Vulkan Backend

- **Shaders**: SPIR-V
- **Platform**: Linux, Windows, Android
- **Advantages**: Explicit control, good performance
- **Use Case**: Linux servers, cross-platform apps

### Metal Backend

- **Shaders**: MSL (Metal Shading Language)
- **Platform**: macOS, iOS
- **Advantages**: Best performance on Apple hardware
- **Use Case**: macOS/iOS applications

### DirectX 12 Backend

- **Shaders**: HLSL
- **Platform**: Windows
- **Advantages**: Native Windows support
- **Use Case**: Windows applications

### CPU Backend

- **Implementation**: ndarray + BLAS/SIMD
- **Platform**: All
- **Advantages**: Always available
- **Use Case**: Fallback, debugging, testing

## Kernel Types

CrossGPU supports the following kernel operations:

1. **MatMul**: Matrix multiplication (GEMM)
2. **LayerNorm**: Layer normalization
3. **Softmax**: Softmax activation
4. **GELU**: GELU activation function
5. **FusedGemmGelu**: Fused matrix multiply + GELU
6. **FusedGemmLayerNorm**: Fused matrix multiply + LayerNorm
7. **Attention**: Multi-head self-attention

### Kernel Fusion

Fused kernels reduce memory bandwidth:

```
Traditional:
  MatMul → Write → Read → GELU → Write

Fused:
  MatMul + GELU → Write
```

## Error Handling

CrossGPU uses Rust's `Result` type for error handling:

```rust
pub type Result<T> = std::result::Result<T, CoreError>;

pub enum CoreError {
    ShapeMismatch { expected: Vec<usize>, actual: Vec<usize> },
    InvalidDimension(String),
    GpuError(String),
    QuantizationError(String),
    ModelLoadError(String),
    IoError(std::io::Error),
    SerializationError(String),
    Other(String),
}
```

## Model Format

Models are serialized using `bincode`:

```
┌─────────────────────────────┐
│ TransformerModel            │
├─────────────────────────────┤
│ • Config                    │
│ • Token Embeddings          │
│ • Position Embeddings       │
│ • Layer Weights [×6]        │
│   ├─ Attention (Q,K,V,O)   │
│   ├─ Feed-Forward (W1, W2) │
│   └─ Layer Norms (×2)      │
│ • Final Layer Norm          │
└─────────────────────────────┘
         │
         ▼ bincode::serialize()
     .bin file
```

## Build System

### Native Build

```
cargo build --release
    │
    ├─► Detect platform
    ├─► Select default backend
    └─► Compile with optimizations
```

### WASM Build

```
wasm-pack build --target web
    │
    ├─► Compile to WASM
    ├─► Generate JS bindings
    └─► Output to pkg/
```

## Performance Considerations

### Optimization Strategies

1. **Batch Processing**: Process multiple sequences together
2. **Kernel Fusion**: Combine operations to reduce memory I/O
3. **Quantization**: Use 8-bit/4-bit for reduced memory and faster compute
4. **Async Operations**: Overlap CPU and GPU work
5. **Memory Pooling**: Reuse GPU buffers when possible

### Benchmarking

```bash
# Profile CPU performance
cargo build --release
perf record ./target/release/simple-inference
perf report

# Profile GPU performance (backend-specific tools)
# - Metal: Xcode Instruments
# - Vulkan: RenderDoc, NSight
# - WebGPU: Chrome DevTools
```

## Future Enhancements

### Planned Features

- [ ] KV-cache for autoregressive generation
- [ ] Dynamic batching
- [ ] INT4 quantization with GPTQ
- [ ] Flash Attention implementation
- [ ] Model parallelism support
- [ ] Streaming inference
- [ ] Custom CUDA kernels (optional)

### Extension Points

1. **New Backends**: Implement `GpuDevice` trait
2. **New Kernels**: Add to `KernelType` enum
3. **New Models**: Extend `TransformerModel`
4. **New Quantization**: Add to `QuantScheme`

## Testing Strategy

### Unit Tests

- Test each module independently
- Mock GPU operations for CPU testing
- Property-based testing for tensor operations

### Integration Tests

- Test full inference pipeline
- Test backend switching
- Test model serialization

### CI/CD

- Build on all platforms (Linux, macOS, Windows)
- Run tests on CPU and GPU (when available)
- Check code formatting and linting
- Deploy WASM build to GitHub Pages

## References

- [Attention Is All You Need](https://arxiv.org/abs/1706.03762) - Original Transformer paper
- [GELU Paper](https://arxiv.org/abs/1606.08415) - GELU activation function
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [Vulkan Specification](https://www.khronos.org/vulkan/)
- [Metal Documentation](https://developer.apple.com/metal/)

---

For implementation details, see the source code documentation: `cargo doc --no-deps --open`
