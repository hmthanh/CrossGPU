//! CrossGPU Core - Tensor operations and Transformer layer definitions
//!
//! This crate provides the core abstractions for the CrossGPU Tiny Transformer engine:
//! - Tensor data structures and operations
//! - Transformer layer interfaces
//! - Quantization support (8-bit, 4-bit)
//! - GPU device abstraction trait

#![deny(warnings)]
#![deny(missing_docs)]

pub mod error;
pub mod gpu;
pub mod quantization;
pub mod tensor;
pub mod transformer;

pub use error::{CoreError, Result};
pub use gpu::{GpuDevice, GpuTensor, Kernel};
pub use tensor::Tensor;
pub use transformer::{TransformerConfig, TransformerLayer};
