//! Quantization utilities for model compression

use crate::error::{CoreError, Result};
use crate::tensor::{DType, Tensor};

/// Quantization scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantScheme {
    /// Symmetric 8-bit quantization
    Int8Symmetric,
    /// Asymmetric 8-bit quantization
    Int8Asymmetric,
    /// 4-bit quantization (for extreme compression)
    Int4,
}

/// Quantization parameters
#[derive(Debug, Clone)]
pub struct QuantParams {
    /// Scale factor
    pub scale: f32,
    /// Zero point (for asymmetric quantization)
    pub zero_point: i32,
    /// Quantization scheme
    pub scheme: QuantScheme,
}

impl QuantParams {
    /// Create symmetric 8-bit quantization parameters
    pub fn int8_symmetric(scale: f32) -> Self {
        Self {
            scale,
            zero_point: 0,
            scheme: QuantScheme::Int8Symmetric,
        }
    }

    /// Create asymmetric 8-bit quantization parameters
    pub fn int8_asymmetric(scale: f32, zero_point: i32) -> Self {
        Self {
            scale,
            zero_point,
            scheme: QuantScheme::Int8Asymmetric,
        }
    }

    /// Create 4-bit quantization parameters
    pub fn int4(scale: f32) -> Self {
        Self {
            scale,
            zero_point: 0,
            scheme: QuantScheme::Int4,
        }
    }
}

/// Quantize a tensor from F32 to a quantized format
pub fn quantize_tensor(tensor: &Tensor, params: &QuantParams) -> Result<Tensor> {
    if tensor.dtype != DType::F32 {
        return Err(CoreError::QuantizationError(
            "Can only quantize F32 tensors".to_string(),
        ));
    }

    let data = tensor.as_f32_slice()?;

    match params.scheme {
        QuantScheme::Int8Symmetric | QuantScheme::Int8Asymmetric => {
            let quantized: Vec<i8> = data
                .iter()
                .map(|&x| {
                    let q = (x / params.scale).round() as i32 + params.zero_point;
                    q.clamp(-128, 127) as i8
                })
                .collect();

            let bytes = bytemuck::cast_slice(&quantized).to_vec();
            Tensor::from_data(tensor.shape.clone(), DType::I8, bytes)
        }
        QuantScheme::Int4 => {
            // For 4-bit, pack 2 values per byte
            let mut packed = Vec::new();
            for chunk in data.chunks(2) {
                let q1 = ((chunk[0] / params.scale).round() as i32).clamp(-8, 7) as i8;
                let q2 = if chunk.len() > 1 {
                    ((chunk[1] / params.scale).round() as i32).clamp(-8, 7) as i8
                } else {
                    0
                };
                // Pack two 4-bit values into one byte
                let packed_byte = ((q1 & 0x0F) << 4) | (q2 & 0x0F);
                packed.push(packed_byte as u8);
            }
            Tensor::from_data(tensor.shape.clone(), DType::I4, packed)
        }
    }
}

/// Dequantize a tensor back to F32
pub fn dequantize_tensor(tensor: &Tensor, params: &QuantParams) -> Result<Tensor> {
    match tensor.dtype {
        DType::I8 => {
            let quantized: &[i8] = bytemuck::cast_slice(&tensor.data);
            let data: Vec<f32> = quantized
                .iter()
                .map(|&q| (q as i32 - params.zero_point) as f32 * params.scale)
                .collect();
            Tensor::from_f32(tensor.shape.clone(), data)
        }
        DType::I4 => {
            let mut data = Vec::new();
            for &byte in &tensor.data {
                let q1 = ((byte >> 4) & 0x0F) as i8;
                let q2 = (byte & 0x0F) as i8;
                // Sign extend from 4-bit to 8-bit
                let q1 = if q1 & 0x08 != 0 { q1 | (-16i8) } else { q1 };
                let q2 = if q2 & 0x08 != 0 { q2 | (-16i8) } else { q2 };
                data.push(q1 as f32 * params.scale);
                data.push(q2 as f32 * params.scale);
            }
            // Trim to actual tensor size
            data.truncate(tensor.numel());
            Tensor::from_f32(tensor.shape.clone(), data)
        }
        _ => Err(CoreError::QuantizationError(
            "Can only dequantize I8 or I4 tensors".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_int8_quantization() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let tensor = Tensor::from_f32(vec![2, 2], data.clone()).unwrap();
        let params = QuantParams::int8_symmetric(0.1);

        let quantized = quantize_tensor(&tensor, &params).unwrap();
        assert_eq!(quantized.dtype, DType::I8);

        let dequantized = dequantize_tensor(&quantized, &params).unwrap();
        let deq_data = dequantized.as_f32_slice().unwrap();

        for (i, &original) in data.iter().enumerate() {
            assert_relative_eq!(deq_data[i], original, epsilon = 0.15);
        }
    }
}
