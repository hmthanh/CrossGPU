//! Integration tests for CrossGPU core functionality

use crossgpu_core::{
    quantization::{quantize_tensor, dequantize_tensor, QuantParams},
    tensor::{DType, Tensor},
    transformer::TransformerConfig,
};
use approx::assert_relative_eq;

#[test]
fn test_tensor_operations() {
    let tensor = Tensor::new(vec![2, 3, 4], DType::F32);
    assert_eq!(tensor.numel(), 24);
    assert_eq!(tensor.ndim(), 3);

    let reshaped = tensor.reshape(vec![4, 6]).unwrap();
    assert_eq!(reshaped.shape, vec![4, 6]);
}

#[test]
fn test_quantization_roundtrip() {
    let data = vec![1.0, 2.0, 3.0, 4.0, -1.0, -2.0];
    let tensor = Tensor::from_f32(vec![2, 3], data.clone()).unwrap();
    
    let params = QuantParams::int8_symmetric(0.05);
    let quantized = quantize_tensor(&tensor, &params).unwrap();
    let dequantized = dequantize_tensor(&quantized, &params).unwrap();
    
    let deq_data = dequantized.as_f32_slice().unwrap();
    for (i, &original) in data.iter().enumerate() {
        assert_relative_eq!(deq_data[i], original, epsilon = 0.1);
    }
}

#[test]
fn test_transformer_config() {
    let config = TransformerConfig::tiny();
    assert_eq!(config.d_model, 512);
    assert_eq!(config.n_layers, 6);
    
    let size = config.estimate_size();
    // Should be around 50MB range
    assert!(size > 40_000_000 && size < 100_000_000);
}
