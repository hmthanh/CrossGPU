#!/usr/bin/env bash
# Generate a new GPU backend from template

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BACKENDS_DIR="$PROJECT_ROOT/backends"

# Check arguments
if [ $# -lt 1 ]; then
    echo "Usage: $0 <backend-name> [shader-language]"
    echo ""
    echo "Examples:"
    echo "  $0 opencl OPENCL"
    echo "  $0 cuda CUDA"
    echo "  $0 rocm HIP"
    echo ""
    echo "Available backends:"
    ls -1 "$BACKENDS_DIR"
    exit 1
fi

BACKEND_NAME=$1
SHADER_LANG=${2:-"GENERIC"}

# Convert to lowercase for directory name
BACKEND_DIR_NAME=$(echo "$BACKEND_NAME" | tr '[:upper:]' '[:lower:]')
BACKEND_DIR="$BACKENDS_DIR/$BACKEND_DIR_NAME"

# Convert to title case for struct name (e.g., OpenCL -> OpenCl, CUDA -> Cuda)
BACKEND_STRUCT=$(echo "$BACKEND_NAME" | sed 's/.*/\u&/' | sed 's/\(.\)\([A-Z]\)/\1\L\2/g')

echo "🚀 Creating new GPU backend: $BACKEND_NAME"
echo "   Directory: $BACKEND_DIR"
echo "   Struct: ${BACKEND_STRUCT}Device"
echo ""

# Check if already exists
if [ -d "$BACKEND_DIR" ]; then
    echo "❌ Backend already exists: $BACKEND_DIR"
    exit 1
fi

# Create directory structure
mkdir -p "$BACKEND_DIR/src"

# Create Cargo.toml
cat > "$BACKEND_DIR/Cargo.toml" <<EOF
[package]
name = "crossgpu-$BACKEND_DIR_NAME"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"
description = "$BACKEND_NAME backend for CrossGPU transformer engine"
repository = "https://github.com/yourusername/crossgpu"
keywords = ["gpu", "transformer", "inference", "$BACKEND_DIR_NAME"]
categories = ["science", "hardware-support"]

[dependencies]
crossgpu-core = { path = "../../core" }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

# Add backend-specific dependencies here
# For example:
# $BACKEND_DIR_NAME = "0.1.0"

[dev-dependencies]
approx = "0.5"
EOF

# Create lib.rs
cat > "$BACKEND_DIR/src/lib.rs" <<EOF
//! $BACKEND_NAME GPU backend for CrossGPU
//!
//! This backend provides GPU acceleration using $BACKEND_NAME for transformer inference.
//!
//! # Platform Support
//!
//! - **Operating Systems**: TODO: List supported OS
//! - **Hardware**: TODO: List supported hardware
//!
//! # Example
//!
//! \`\`\`no_run
//! use crossgpu_$BACKEND_DIR_NAME::${BACKEND_STRUCT}Device;
//! use crossgpu_core::GpuDevice;
//!
//! let device = ${BACKEND_STRUCT}Device::new().unwrap();
//! // Use device for inference...
//! \`\`\`

use crossgpu_core::{
    error::CoreError, gpu::{GpuDevice, GpuTensor, Kernel, KernelType}, tensor::Tensor,
};

/// $BACKEND_NAME GPU device implementation
pub struct ${BACKEND_STRUCT}Device {
    // TODO: Add backend-specific fields
    // For example:
    // context: $BACKEND_NAME::Context,
    // queue: $BACKEND_NAME::Queue,
}

impl ${BACKEND_STRUCT}Device {
    /// Create a new $BACKEND_NAME device
    pub fn new() -> Result<Self, CoreError> {
        // TODO: Initialize $BACKEND_NAME device

        // Check platform support
        #[cfg(not(target_os = "linux"))]
        {
            return Err(CoreError::UnsupportedPlatform(
                "$BACKEND_NAME is currently only supported on Linux".to_string(),
            ));
        }

        Ok(Self {
            // Initialize fields
        })
    }
}

impl GpuDevice for ${BACKEND_STRUCT}Device {
    fn name(&self) -> &str {
        "$BACKEND_NAME GPU"
    }

    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor, CoreError> {
        // TODO: Upload tensor to GPU memory

        Ok(GpuTensor {
            shape: tensor.shape.clone(),
            dtype: tensor.dtype,
            device_ptr: 0 as *mut u8, // TODO: Use actual GPU pointer
        })
    }

    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor, CoreError> {
        // TODO: Download tensor from GPU memory

        Ok(Tensor::zeros(&gpu_tensor.shape, gpu_tensor.dtype))
    }

    fn run_kernel(&self, kernel: &Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement kernel execution

        match kernel.kernel_type {
            KernelType::MatMul => self.run_matmul(inputs),
            KernelType::LayerNorm => self.run_layernorm(inputs),
            KernelType::Softmax => self.run_softmax(inputs),
            KernelType::GELU => self.run_gelu(inputs),
            KernelType::Embedding => self.run_embedding(inputs),
            KernelType::Attention => self.run_attention(inputs),
            KernelType::FeedForward => self.run_feedforward(inputs),
        }
    }

    fn compile_kernel(&self, kernel_type: KernelType, params: &[usize]) -> Result<Kernel, CoreError> {
        // TODO: Compile kernel for $BACKEND_NAME

        let shader_source = match kernel_type {
            KernelType::MatMul => Self::matmul_shader(params),
            KernelType::LayerNorm => Self::layernorm_shader(params),
            KernelType::Softmax => Self::softmax_shader(params),
            KernelType::GELU => Self::gelu_shader(params),
            KernelType::Embedding => Self::embedding_shader(params),
            KernelType::Attention => Self::attention_shader(params),
            KernelType::FeedForward => Self::feedforward_shader(params),
        };

        Ok(Kernel {
            kernel_type,
            compiled_code: shader_source.into_bytes(),
        })
    }

    fn synchronize(&self) -> Result<(), CoreError> {
        // TODO: Synchronize GPU execution
        Ok(())
    }
}

impl ${BACKEND_STRUCT}Device {
    // Kernel execution helpers

    fn run_matmul(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement matrix multiplication
        Ok(inputs[0].clone())
    }

    fn run_layernorm(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement layer normalization
        Ok(inputs[0].clone())
    }

    fn run_softmax(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement softmax
        Ok(inputs[0].clone())
    }

    fn run_gelu(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement GELU activation
        Ok(inputs[0].clone())
    }

    fn run_embedding(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement embedding lookup
        Ok(inputs[0].clone())
    }

    fn run_attention(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement multi-head attention
        Ok(inputs[0].clone())
    }

    fn run_feedforward(&self, inputs: &[GpuTensor]) -> Result<GpuTensor, CoreError> {
        // TODO: Implement feed-forward network
        Ok(inputs[0].clone())
    }

    // Shader generation

    fn matmul_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for matrix multiplication
        r#"
        // $SHADER_LANG Matrix Multiplication Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn layernorm_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for layer normalization
        r#"
        // $SHADER_LANG Layer Normalization Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn softmax_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for softmax
        r#"
        // $SHADER_LANG Softmax Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn gelu_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for GELU
        r#"
        // $SHADER_LANG GELU Activation Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn embedding_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for embedding
        r#"
        // $SHADER_LANG Embedding Lookup Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn attention_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for attention
        r#"
        // $SHADER_LANG Multi-Head Attention Kernel
        // TODO: Implement kernel
        "#.to_string()
    }

    fn feedforward_shader(_params: &[usize]) -> String {
        // TODO: Return $SHADER_LANG shader code for feed-forward
        r#"
        // $SHADER_LANG Feed-Forward Network Kernel
        // TODO: Implement kernel
        "#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Remove this once implementation is complete
    fn test_device_creation() {
        let device = ${BACKEND_STRUCT}Device::new();
        // Platform-specific test
        #[cfg(target_os = "linux")]
        assert!(device.is_ok());

        #[cfg(not(target_os = "linux"))]
        assert!(device.is_err());
    }

    #[test]
    #[ignore]
    fn test_tensor_upload_download() {
        let device = ${BACKEND_STRUCT}Device::new().unwrap();
        let tensor = Tensor::zeros(&[2, 3], crossgpu_core::tensor::DType::F32);

        let gpu_tensor = device.upload_tensor(&tensor).unwrap();
        let downloaded = device.download_tensor(&gpu_tensor).unwrap();

        assert_eq!(tensor.shape, downloaded.shape);
        assert_eq!(tensor.dtype, downloaded.dtype);
    }
}
EOF

# Add to workspace
if ! grep -q "\"backends/$BACKEND_DIR_NAME\"" "$PROJECT_ROOT/Cargo.toml"; then
    # Find the members array and add new backend
    if [ "$(uname)" == "Darwin" ]; then
        # macOS sed
        sed -i '' "/members = \[/a\\
    \"backends/$BACKEND_DIR_NAME\",
" "$PROJECT_ROOT/Cargo.toml"
    else
        # Linux sed
        sed -i "/members = \[/a\\    \"backends/$BACKEND_DIR_NAME\"," "$PROJECT_ROOT/Cargo.toml"
    fi
fi

echo "✅ Backend created successfully!"
echo ""
echo "📁 Files created:"
echo "   - $BACKEND_DIR/Cargo.toml"
echo "   - $BACKEND_DIR/src/lib.rs"
echo ""
echo "📝 Next steps:"
echo "   1. Add backend-specific dependencies to Cargo.toml"
echo "   2. Implement device initialization in ${BACKEND_STRUCT}Device::new()"
echo "   3. Implement tensor upload/download"
echo "   4. Implement kernel execution"
echo "   5. Write shader code for each kernel type"
echo "   6. Add tests and examples"
echo "   7. Update documentation"
echo ""
echo "🔧 Build with:"
echo "   cargo build -p crossgpu-$BACKEND_DIR_NAME"
echo ""
echo "🧪 Test with:"
echo "   cargo test -p crossgpu-$BACKEND_DIR_NAME"
