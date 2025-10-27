//! Tensor data structure and operations

use crate::error::{CoreError, Result};
use serde::{Deserialize, Serialize};

/// Data type for tensor elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DType {
    /// 32-bit floating point
    F32,
    /// 16-bit floating point
    F16,
    /// 8-bit integer (quantized)
    I8,
    /// 4-bit integer (quantized)
    I4,
}

/// Tensor data structure for n-dimensional arrays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    /// Shape of the tensor
    pub shape: Vec<usize>,
    /// Data type
    pub dtype: DType,
    /// Raw data buffer
    pub data: Vec<u8>,
}

impl Tensor {
    /// Create a new tensor with the given shape and data type
    pub fn new(shape: Vec<usize>, dtype: DType) -> Self {
        let size = shape.iter().product::<usize>() * dtype.size_bytes();
        Self {
            shape,
            dtype,
            data: vec![0u8; size],
        }
    }

    /// Create a tensor from raw data
    pub fn from_data(shape: Vec<usize>, dtype: DType, data: Vec<u8>) -> Result<Self> {
        let expected_size = shape.iter().product::<usize>() * dtype.size_bytes();
        if data.len() != expected_size {
            return Err(CoreError::InvalidDimension(format!(
                "Data size {} does not match expected size {} for shape {:?}",
                data.len(),
                expected_size,
                shape
            )));
        }
        Ok(Self { shape, dtype, data })
    }

    /// Create a tensor from f32 data
    pub fn from_f32(shape: Vec<usize>, data: Vec<f32>) -> Result<Self> {
        let expected_size = shape.iter().product::<usize>();
        if data.len() != expected_size {
            return Err(CoreError::InvalidDimension(format!(
                "Data size {} does not match expected size {} for shape {:?}",
                data.len(),
                expected_size,
                shape
            )));
        }
        let bytes = bytemuck::cast_slice(&data).to_vec();
        Ok(Self {
            shape,
            dtype: DType::F32,
            data: bytes,
        })
    }

    /// Get the total number of elements
    pub fn numel(&self) -> usize {
        self.shape.iter().product()
    }

    /// Get the number of dimensions
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// Reshape the tensor
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self> {
        let old_size = self.numel();
        let new_size = new_shape.iter().product();
        if old_size != new_size {
            return Err(CoreError::ShapeMismatch {
                expected: vec![old_size],
                actual: vec![new_size],
            });
        }
        Ok(Self {
            shape: new_shape,
            dtype: self.dtype,
            data: self.data.clone(),
        })
    }

    /// Get data as f32 slice (assumes F32 dtype)
    pub fn as_f32_slice(&self) -> Result<&[f32]> {
        if self.dtype != DType::F32 {
            return Err(CoreError::Other("Tensor is not F32 type".to_string()));
        }
        Ok(bytemuck::cast_slice(&self.data))
    }

    /// Get mutable data as f32 slice (assumes F32 dtype)
    pub fn as_f32_slice_mut(&mut self) -> Result<&mut [f32]> {
        if self.dtype != DType::F32 {
            return Err(CoreError::Other("Tensor is not F32 type".to_string()));
        }
        Ok(bytemuck::cast_slice_mut(&mut self.data))
    }
}

impl DType {
    /// Get the size in bytes for this data type
    pub fn size_bytes(&self) -> usize {
        match self {
            DType::F32 => 4,
            DType::F16 => 2,
            DType::I8 => 1,
            DType::I4 => 1, // Packed, 2 elements per byte
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let tensor = Tensor::new(vec![2, 3], DType::F32);
        assert_eq!(tensor.shape, vec![2, 3]);
        assert_eq!(tensor.numel(), 6);
        assert_eq!(tensor.ndim(), 2);
    }

    #[test]
    fn test_tensor_from_f32() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let tensor = Tensor::from_f32(vec![2, 2], data).unwrap();
        assert_eq!(tensor.numel(), 4);
        assert_eq!(tensor.dtype, DType::F32);
    }

    #[test]
    fn test_tensor_reshape() {
        let tensor = Tensor::new(vec![2, 3], DType::F32);
        let reshaped = tensor.reshape(vec![3, 2]).unwrap();
        assert_eq!(reshaped.shape, vec![3, 2]);
        assert_eq!(reshaped.numel(), 6);
    }
}
