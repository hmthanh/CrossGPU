//! CPU backend for CrossGPU - SIMD/BLAS fallback implementation

#![deny(warnings)]
#![deny(missing_docs)]

use crossgpu_core::{
    error::{CoreError, Result},
    gpu::{GpuDevice, GpuTensor, Kernel, KernelType},
    tensor::Tensor,
};
use std::sync::Arc;

/// CPU device implementation
pub struct CpuDevice {
    name: String,
}

impl CpuDevice {
    /// Create a new CPU device
    pub fn new() -> Self {
        Self {
            name: "CPU".to_string(),
        }
    }
}

impl Default for CpuDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuDevice for CpuDevice {
    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor> {
        // For CPU backend, "upload" just wraps the tensor
        Ok(GpuTensor {
            shape: tensor.shape.clone(),
            handle: Arc::new(tensor.clone()),
        })
    }

    fn run_kernel(&self, kernel: Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement CPU kernels using ndarray/SIMD
        log::info!("Running {:?} kernel on CPU", kernel.kernel_type);

        if inputs.is_empty() {
            return Err(CoreError::GpuError("No input tensors".to_string()));
        }

        match kernel.kernel_type {
            KernelType::MatMul => self.run_matmul(inputs),
            KernelType::LayerNorm => self.run_layer_norm(inputs, &kernel.params),
            KernelType::Softmax => self.run_softmax(inputs),
            KernelType::Gelu => self.run_gelu(inputs),
            KernelType::FusedGemmGelu => self.run_fused_gemm_gelu(inputs),
            KernelType::FusedGemmLayerNorm => {
                self.run_fused_gemm_layer_norm(inputs, &kernel.params)
            }
            KernelType::Attention => self.run_attention(inputs),
        }
    }

    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor> {
        // For CPU backend, "download" just extracts the tensor
        let tensor = gpu_tensor
            .handle
            .downcast_ref::<Tensor>()
            .ok_or_else(|| CoreError::GpuError("Invalid tensor handle".to_string()))?;
        Ok(tensor.clone())
    }

    fn synchronize(&self) -> Result<()> {
        // CPU operations are synchronous
        Ok(())
    }

    fn device_name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        true // CPU is always available
    }
}

impl CpuDevice {
    fn run_matmul(&self, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement matrix multiplication using ndarray
        log::debug!("MatMul on CPU");
        Ok(inputs[0].clone())
    }

    fn run_layer_norm(&self, inputs: &[GpuTensor], params: &[f32]) -> Result<GpuTensor> {
        // Placeholder: Implement layer normalization
        log::debug!("LayerNorm on CPU with epsilon: {:?}", params.first());
        Ok(inputs[0].clone())
    }

    fn run_softmax(&self, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement softmax
        log::debug!("Softmax on CPU");
        Ok(inputs[0].clone())
    }

    fn run_gelu(&self, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement GELU activation
        log::debug!("GELU on CPU");
        Ok(inputs[0].clone())
    }

    fn run_fused_gemm_gelu(&self, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement fused GEMM + GELU
        log::debug!("Fused GEMM+GELU on CPU");
        Ok(inputs[0].clone())
    }

    fn run_fused_gemm_layer_norm(&self, inputs: &[GpuTensor], params: &[f32]) -> Result<GpuTensor> {
        // Placeholder: Implement fused GEMM + LayerNorm
        log::debug!(
            "Fused GEMM+LayerNorm on CPU with epsilon: {:?}",
            params.first()
        );
        Ok(inputs[0].clone())
    }

    fn run_attention(&self, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement multi-head attention
        log::debug!("Attention on CPU");
        Ok(inputs[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossgpu_core::tensor::DType;

    #[test]
    fn test_cpu_device_creation() {
        let device = CpuDevice::new();
        assert_eq!(device.device_name(), "CPU");
        assert!(device.is_available());
    }

    #[test]
    fn test_tensor_upload_download() {
        let device = CpuDevice::new();
        let tensor = Tensor::new(vec![2, 3], DType::F32);

        let gpu_tensor = device.upload_tensor(&tensor).unwrap();
        assert_eq!(gpu_tensor.shape, vec![2, 3]);

        let downloaded = device.download_tensor(&gpu_tensor).unwrap();
        assert_eq!(downloaded.shape, tensor.shape);
    }
}
