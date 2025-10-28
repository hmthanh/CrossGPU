//! WebGPU backend for CrossGPU - Works in browser and native

#![deny(warnings)]
#![deny(missing_docs)]

use crossgpu_core::{
    error::{CoreError, Result},
    gpu::{GpuDevice, GpuTensor, Kernel},
    tensor::Tensor,
};
use std::sync::Arc;
use wgpu::{Device, Queue};

/// WebGPU device implementation
pub struct WebGpuDevice {
    device: Arc<Device>,
    queue: Arc<Queue>,
    name: String,
}

impl WebGpuDevice {
    /// Create a new WebGPU device
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| CoreError::GpuError("Failed to find GPU adapter".to_string()))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("CrossGPU WebGPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| CoreError::GpuError(format!("Failed to create device: {}", e)))?;

        let name = format!("WebGPU ({})", adapter.get_info().name);

        Ok(Self {
            device: Arc::new(device),
            queue: Arc::new(queue),
            name,
        })
    }

    /// Get the underlying wgpu device
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Get the underlying wgpu queue
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}

impl GpuDevice for WebGpuDevice {
    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor> {
        use wgpu::util::DeviceExt;

        // Create a GPU buffer and upload tensor data
        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Tensor Buffer"),
                contents: &tensor.data,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            });

        Ok(GpuTensor {
            shape: tensor.shape.clone(),
            handle: Arc::new(buffer),
        })
    }

    fn run_kernel(&self, kernel: Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor> {
        // Placeholder: Implement WebGPU compute shaders
        log::info!("Running {:?} kernel on WebGPU", kernel.kernel_type);

        if inputs.is_empty() {
            return Err(CoreError::GpuError("No input tensors".to_string()));
        }

        // For now, return the first input as placeholder
        // In real implementation:
        // 1. Create compute pipeline with shader
        // 2. Set up bind groups
        // 3. Dispatch compute shader
        // 4. Read back results

        Ok(inputs[0].clone())
    }

    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor> {
        // Placeholder: Download buffer from GPU
        // In real implementation:
        // 1. Create staging buffer
        // 2. Copy from GPU buffer to staging buffer
        // 3. Map staging buffer and read data

        log::debug!("Downloading tensor from WebGPU");

        // For now, create empty tensor with correct shape
        Ok(Tensor::new(
            gpu_tensor.shape.clone(),
            crossgpu_core::tensor::DType::F32,
        ))
    }

    fn synchronize(&self) -> Result<()> {
        // WebGPU operations are asynchronous, but we can poll the device
        self.device.poll(wgpu::Maintain::Wait);
        Ok(())
    }

    fn device_name(&self) -> &str {
        &self.name
    }

    fn is_available(&self) -> bool {
        true
    }
}

/// WGSL shader templates for common operations
pub mod shaders {
    /// Matrix multiplication shader template
    pub const MATMUL_SHADER: &str = r#"
        @group(0) @binding(0) var<storage, read> input_a: array<f32>;
        @group(0) @binding(1) var<storage, read> input_b: array<f32>;
        @group(0) @binding(2) var<storage, read_write> output: array<f32>;
        
        @compute @workgroup_size(8, 8)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            // Placeholder: Implement matrix multiplication
            let index = global_id.y * 256u + global_id.x;
            output[index] = input_a[index] + input_b[index];
        }
    "#;

    /// Layer normalization shader template
    pub const LAYER_NORM_SHADER: &str = r#"
        @group(0) @binding(0) var<storage, read> input: array<f32>;
        @group(0) @binding(1) var<storage, read_write> output: array<f32>;
        
        @compute @workgroup_size(256)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            // Placeholder: Implement layer normalization
            let index = global_id.x;
            output[index] = input[index];
        }
    "#;

    /// GELU activation shader template
    pub const GELU_SHADER: &str = r#"
        @group(0) @binding(0) var<storage, read> input: array<f32>;
        @group(0) @binding(1) var<storage, read_write> output: array<f32>;
        
        @compute @workgroup_size(256)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            let index = global_id.x;
            let x = input[index];
            // GELU approximation: x * 0.5 * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3)))
            output[index] = x * 0.5 * (1.0 + tanh(0.797885 * (x + 0.044715 * x * x * x)));
        }
    "#;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webgpu_device_creation() {
        // Note: This test may fail in environments without GPU support
        if let Ok(device) = WebGpuDevice::new().await {
            assert!(device.is_available());
            assert!(device.device_name().contains("WebGPU"));
        }
    }
}
