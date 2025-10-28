//! GPU device abstraction and kernel interface

use crate::error::Result;
use crate::tensor::Tensor;
use std::sync::Arc;

/// GPU tensor handle - represents tensor data on GPU
#[derive(Debug, Clone)]
pub struct GpuTensor {
    /// Tensor shape
    pub shape: Vec<usize>,
    /// Backend-specific handle (opaque pointer) for native platforms
    /// Includes Send + Sync bounds for thread safety in multi-threaded environments
    #[cfg(not(target_arch = "wasm32"))]
    pub handle: Arc<dyn std::any::Any + Send + Sync>,
    /// Backend-specific handle (opaque pointer) for WASM platform
    /// WASM is single-threaded so Send + Sync bounds are not required
    #[cfg(target_arch = "wasm32")]
    pub handle: Arc<dyn std::any::Any>,
}

/// Kernel type enumeration for common operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelType {
    /// Matrix multiplication (GEMM)
    MatMul,
    /// Layer normalization
    LayerNorm,
    /// Softmax activation
    Softmax,
    /// GELU activation
    Gelu,
    /// Fused GEMM + GELU
    FusedGemmGelu,
    /// Fused GEMM + LayerNorm
    FusedGemmLayerNorm,
    /// Attention kernel (fused Q, K, V computation)
    Attention,
}

/// Kernel configuration and parameters
#[derive(Debug, Clone)]
pub struct Kernel {
    /// Type of kernel
    pub kernel_type: KernelType,
    /// Additional parameters (e.g., epsilon for LayerNorm)
    pub params: Vec<f32>,
}

impl Kernel {
    /// Create a new kernel with the given type
    pub fn new(kernel_type: KernelType) -> Self {
        Self {
            kernel_type,
            params: Vec::new(),
        }
    }

    /// Create a kernel with parameters
    pub fn with_params(kernel_type: KernelType, params: Vec<f32>) -> Self {
        Self {
            kernel_type,
            params,
        }
    }
}

/// GPU device abstraction trait - common interface for all GPU backends
/// Note: On WASM, we don't require Send + Sync since it's single-threaded
#[cfg(not(target_arch = "wasm32"))]
pub trait GpuDevice: Send + Sync {
    /// Upload a tensor from CPU to GPU
    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor>;

    /// Run a kernel on GPU tensors
    fn run_kernel(&self, kernel: Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor>;

    /// Download a tensor from GPU to CPU
    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor>;

    /// Synchronize GPU operations (wait for completion)
    fn synchronize(&self) -> Result<()>;

    /// Get device name/identifier
    fn device_name(&self) -> &str;

    /// Check if device is available
    fn is_available(&self) -> bool;
}

/// GPU device abstraction trait for WASM - common interface for WebGPU backend
///
/// This is the WASM-specific version without Send + Sync bounds, since WebAssembly
/// runs in a single-threaded environment and doesn't require thread safety guarantees.
#[cfg(target_arch = "wasm32")]
pub trait GpuDevice {
    /// Upload a tensor from CPU to GPU
    fn upload_tensor(&self, tensor: &Tensor) -> Result<GpuTensor>;

    /// Run a kernel on GPU tensors
    fn run_kernel(&self, kernel: Kernel, inputs: &[GpuTensor]) -> Result<GpuTensor>;

    /// Download a tensor from GPU to CPU
    fn download_tensor(&self, gpu_tensor: &GpuTensor) -> Result<Tensor>;

    /// Synchronize GPU operations (wait for completion)
    fn synchronize(&self) -> Result<()>;

    /// Get device name/identifier
    fn device_name(&self) -> &str;

    /// Check if device is available
    fn is_available(&self) -> bool;
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// CPU fallback
    Cpu,
    /// WebGPU (browser and native)
    WebGpu,
    /// Vulkan (Linux, Windows, Android)
    Vulkan,
    /// Metal (macOS, iOS)
    Metal,
    /// DirectX 12 (Windows)
    Dx12,
}

impl DeviceType {
    /// Get the default device for the current platform
    pub fn default_for_platform() -> Self {
        #[cfg(target_arch = "wasm32")]
        return DeviceType::WebGpu;

        #[cfg(target_os = "macos")]
        return DeviceType::Metal;

        #[cfg(target_os = "windows")]
        return DeviceType::Dx12;

        #[cfg(target_os = "linux")]
        return DeviceType::Vulkan;

        #[cfg(not(any(
            target_arch = "wasm32",
            target_os = "macos",
            target_os = "windows",
            target_os = "linux"
        )))]
        return DeviceType::Cpu;
    }
}
