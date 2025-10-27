//! Error types for the CrossGPU core library

use thiserror::Error;

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core error types
#[derive(Error, Debug)]
pub enum CoreError {
    /// Tensor shape mismatch error
    #[error("Tensor shape mismatch: expected {expected:?}, got {actual:?}")]
    ShapeMismatch {
        /// Expected shape
        expected: Vec<usize>,
        /// Actual shape
        actual: Vec<usize>,
    },

    /// Invalid tensor dimension
    #[error("Invalid tensor dimension: {0}")]
    InvalidDimension(String),

    /// GPU operation failed
    #[error("GPU operation failed: {0}")]
    GpuError(String),

    /// Quantization error
    #[error("Quantization error: {0}")]
    QuantizationError(String),

    /// Model loading error
    #[error("Model loading error: {0}")]
    ModelLoadError(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
