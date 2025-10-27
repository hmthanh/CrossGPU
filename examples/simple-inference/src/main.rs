//! Simple Transformer Inference Example
//!
//! This example demonstrates how to:
//! 1. Auto-detect the best available GPU backend
//! 2. Load a tiny transformer model
//! 3. Run inference on CPU or GPU
//! 4. Output results

use anyhow::Result;
use crossgpu_core::{
    gpu::{DeviceType, GpuDevice},
    tensor::{DType, Tensor},
    transformer::{
        AttentionWeights, FeedForwardWeights, LayerNormWeights, TransformerConfig,
        TransformerLayerWeights, TransformerModel,
    },
};
use std::sync::Arc;

/// Create a device factory that returns the best available GPU backend
fn create_device(device_type: DeviceType) -> Result<Arc<dyn GpuDevice>> {
    log::info!("Creating device: {:?}", device_type);

    let device: Arc<dyn GpuDevice> = match device_type {
        DeviceType::Cpu => {
            log::info!("Using CPU backend");
            Arc::new(crossgpu_backend_cpu::CpuDevice::new())
        }
        DeviceType::WebGpu => {
            log::info!("Using WebGPU backend");
            let device = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current()
                    .block_on(crossgpu_backend_webgpu::WebGpuDevice::new())
            })?;
            Arc::new(device)
        }
        DeviceType::Vulkan => {
            log::info!("Using Vulkan backend");
            Arc::new(crossgpu_backend_vulkan::VulkanDevice::new()?)
        }
        DeviceType::Metal => {
            log::info!("Using Metal backend");
            Arc::new(crossgpu_backend_metal::MetalDevice::new()?)
        }
        DeviceType::Dx12 => {
            log::info!("Using DirectX 12 backend");
            Arc::new(crossgpu_backend_dx12::Dx12Device::new()?)
        }
    };

    Ok(device)
}

/// Auto-detect the best available device
fn auto_detect_device() -> Result<Arc<dyn GpuDevice>> {
    let preferred = DeviceType::default_for_platform();
    log::info!("Platform default device: {:?}", preferred);

    // Try preferred device first
    if let Ok(device) = create_device(preferred) {
        if device.is_available() {
            return Ok(device);
        }
    }

    // Fall back to CPU
    log::warn!("Falling back to CPU backend");
    Ok(Arc::new(crossgpu_backend_cpu::CpuDevice::new()))
}

/// Create a dummy tiny transformer model for demonstration
fn create_dummy_model() -> Result<TransformerModel> {
    let config = TransformerConfig::tiny();
    log::info!("Creating tiny transformer model with config: {:?}", config);
    log::info!(
        "Estimated model size: ~{} MB",
        config.estimate_size() / 1_000_000
    );

    // Create dummy embeddings
    let token_embedding = Tensor::new(vec![config.vocab_size, config.d_model], DType::F32);
    let position_embedding = Tensor::new(vec![config.max_seq_len, config.d_model], DType::F32);

    // Create dummy layer weights
    let mut layers = Vec::new();
    for i in 0..config.n_layers {
        log::debug!("Initializing layer {}", i);

        let attention = AttentionWeights {
            wq: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wk: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wv: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
            wo: Tensor::new(vec![config.d_model, config.d_model], DType::F32),
        };

        let feed_forward = FeedForwardWeights {
            w1: Tensor::new(vec![config.d_model, config.d_ff], DType::F32),
            w2: Tensor::new(vec![config.d_ff, config.d_model], DType::F32),
        };

        let ln1 = LayerNormWeights {
            gamma: Tensor::new(vec![config.d_model], DType::F32),
            beta: Tensor::new(vec![config.d_model], DType::F32),
        };

        let ln2 = LayerNormWeights {
            gamma: Tensor::new(vec![config.d_model], DType::F32),
            beta: Tensor::new(vec![config.d_model], DType::F32),
        };

        layers.push(TransformerLayerWeights {
            attention,
            feed_forward,
            ln1,
            ln2,
        });
    }

    let final_layer_norm = LayerNormWeights {
        gamma: Tensor::new(vec![config.d_model], DType::F32),
        beta: Tensor::new(vec![config.d_model], DType::F32),
    };

    Ok(TransformerModel::new(
        config,
        token_embedding,
        position_embedding,
        layers,
        final_layer_norm,
    ))
}

/// Run inference with the model
fn run_inference(_model: &TransformerModel, device: &Arc<dyn GpuDevice>) -> Result<()> {
    log::info!("Running inference on device: {}", device.device_name());

    // Create dummy input tensor (batch_size=1, seq_len=10)
    let input_ids = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input_tensor = Tensor::from_f32(
        vec![1, input_ids.len()],
        input_ids.iter().map(|&x| x as f32).collect(),
    )?;

    log::info!("Input shape: {:?}", input_tensor.shape);

    // Upload to GPU (if GPU backend)
    let gpu_input = device.upload_tensor(&input_tensor)?;
    log::debug!("Tensor uploaded to GPU");

    // In a real implementation, we would:
    // 1. Embed input tokens
    // 2. Add positional embeddings
    // 3. Run through transformer layers
    // 4. Apply final layer norm
    // 5. Project to vocabulary logits
    // 6. Take argmax to get predicted tokens

    log::info!("Forward pass complete (placeholder)");

    // Download result (if GPU backend)
    let output = device.download_tensor(&gpu_input)?;
    log::info!("Output shape: {:?}", output.shape);

    device.synchronize()?;
    log::info!("Inference complete!");

    Ok(())
}

fn main() -> Result<()> {
    // Create a runtime manually
    let _rt = tokio::runtime::Runtime::new()?;

    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("=== CrossGPU Tiny Transformer Inference Example ===");

    // Auto-detect best available device
    let device = auto_detect_device()?;
    log::info!("Selected device: {}", device.device_name());

    // Create or load model
    let model = create_dummy_model()?;

    // Optionally save model to file
    // model.save_to_file("tiny_transformer.bin")?;
    // log::info!("Model saved to tiny_transformer.bin");

    // Run inference
    run_inference(&model, &device)?;

    log::info!("=== Example completed successfully ===");
    Ok(())
}
