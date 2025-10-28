#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! crossgpu-core = { path = "../core" }
//! crossgpu-backend-cpu = { path = "../backends/cpu" }
//! crossgpu-backend-webgpu = { path = "../backends/webgpu" }
//! anyhow = "1.0"
//! tokio = { version = "1.35", features = ["rt", "rt-multi-thread"] }
//! log = "0.4"
//! env_logger = "0.11"
//! ```

//! # Complete Inference Example
//!
//! This example demonstrates the full workflow of:
//! 1. Creating a Transformer model
//! 2. Auto-detecting the best GPU backend
//! 3. Quantizing the model for compression
//! 4. Running inference
//! 5. Measuring performance
//!
//! Run with: cargo run --release --example complete-workflow

use anyhow::Result;
use crossgpu_core::{
    gpu::{DeviceType, GpuDevice, Kernel, KernelType},
    quantization::{dequantize_tensor, quantize_tensor, QuantParams},
    tensor::{DType, Tensor},
    transformer::{
        AttentionWeights, FeedForwardWeights, LayerNormWeights, TransformerConfig,
        TransformerLayerWeights, TransformerModel,
    },
};
use std::sync::Arc;
use std::time::Instant;

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
        #[cfg(target_os = "linux")]
        DeviceType::Vulkan => {
            log::info!("Using Vulkan backend");
            Arc::new(crossgpu_backend_vulkan::VulkanDevice::new()?)
        }
        #[cfg(target_os = "macos")]
        DeviceType::Metal => {
            log::info!("Using Metal backend");
            Arc::new(crossgpu_backend_metal::MetalDevice::new()?)
        }
        #[cfg(target_os = "windows")]
        DeviceType::Dx12 => {
            log::info!("Using DirectX 12 backend");
            Arc::new(crossgpu_backend_dx12::Dx12Device::new()?)
        }
        _ => {
            log::warn!("Unsupported device type, falling back to CPU");
            Arc::new(crossgpu_backend_cpu::CpuDevice::new())
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

/// Create a complete transformer model
fn create_transformer_model(config: &TransformerConfig) -> Result<TransformerModel> {
    log::info!("Creating transformer model with config:");
    log::info!("  d_model: {}", config.d_model);
    log::info!("  n_heads: {}", config.n_heads);
    log::info!("  n_layers: {}", config.n_layers);
    log::info!("  vocab_size: {}", config.vocab_size);

    // Create embeddings
    let token_embedding = Tensor::new(vec![config.vocab_size, config.d_model], DType::F32);
    let position_embedding = Tensor::new(vec![config.max_seq_len, config.d_model], DType::F32);

    // Create layers
    let mut layers = Vec::new();
    for i in 0..config.n_layers {
        log::debug!("Initializing layer {}/{}", i + 1, config.n_layers);

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

    let model = TransformerModel::new(
        config.clone(),
        token_embedding,
        position_embedding,
        layers,
        final_layer_norm,
    );

    log::info!("Model created successfully");
    log::info!(
        "Estimated model size: ~{} MB",
        model.config.estimate_size() / 1_000_000
    );

    Ok(model)
}

/// Demonstrate quantization workflow
fn demonstrate_quantization() -> Result<()> {
    log::info!("\n=== Quantization Demo ===");

    // Create sample tensor
    let data: Vec<f32> = (0..100).map(|x| x as f32 * 0.1).collect();
    let tensor = Tensor::from_f32(vec![10, 10], data.clone())?;

    log::info!("Original tensor size: {} bytes", tensor.data.len());

    // INT8 quantization
    let quant_params = QuantParams::int8_symmetric(0.1);
    let quantized = quantize_tensor(&tensor, &quant_params)?;
    log::info!("INT8 quantized size: {} bytes", quantized.data.len());
    log::info!(
        "Compression ratio: {:.2}x",
        tensor.data.len() as f32 / quantized.data.len() as f32
    );

    // Dequantize and check error
    let dequantized = dequantize_tensor(&quantized, &quant_params)?;
    let original = tensor.as_f32_slice()?;
    let restored = dequantized.as_f32_slice()?;

    let max_error = original
        .iter()
        .zip(restored.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);

    log::info!("Max quantization error: {:.4}", max_error);

    // INT4 quantization (extreme compression)
    let quant_params_4bit = QuantParams::int4(0.2);
    let quantized_4bit = quantize_tensor(&tensor, &quant_params_4bit)?;
    log::info!("INT4 quantized size: {} bytes", quantized_4bit.data.len());
    log::info!(
        "Compression ratio: {:.2}x",
        tensor.data.len() as f32 / quantized_4bit.data.len() as f32
    );

    Ok(())
}

/// Demonstrate kernel execution
fn demonstrate_kernels(device: &Arc<dyn GpuDevice>) -> Result<()> {
    log::info!("\n=== Kernel Execution Demo ===");

    // Create test tensors
    let input = Tensor::from_f32(vec![2, 4], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])?;

    log::info!("Input tensor shape: {:?}", input.shape);

    // Upload to GPU
    let start = Instant::now();
    let gpu_input = device.upload_tensor(&input)?;
    log::info!("Tensor upload time: {:?}", start.elapsed());

    // Test different kernels
    let kernels = vec![
        ("MatMul", Kernel::new(KernelType::MatMul)),
        (
            "LayerNorm",
            Kernel::with_params(KernelType::LayerNorm, vec![1e-5]),
        ),
        ("Softmax", Kernel::new(KernelType::Softmax)),
        ("GELU", Kernel::new(KernelType::Gelu)),
    ];

    for (name, kernel) in kernels {
        let start = Instant::now();
        let _output = device.run_kernel(kernel, &[gpu_input.clone()])?;
        log::info!("{} kernel time: {:?}", name, start.elapsed());
    }

    // Download result
    let start = Instant::now();
    let _output = device.download_tensor(&gpu_input)?;
    log::info!("Tensor download time: {:?}", start.elapsed());

    // Synchronize
    device.synchronize()?;

    Ok(())
}

/// Run a complete inference workflow
fn run_inference_workflow(model: &TransformerModel, device: &Arc<dyn GpuDevice>) -> Result<()> {
    log::info!("\n=== Inference Workflow ===");

    // Create input sequence
    let input_ids = vec![1, 5, 10, 15, 20, 25, 30, 35, 40, 45]; // 10 tokens
    log::info!("Input sequence length: {}", input_ids.len());

    let input_data: Vec<f32> = input_ids.iter().map(|&x| x as f32).collect();
    let input = Tensor::from_f32(vec![1, input_ids.len()], input_data)?;

    // Measure total inference time
    let start = Instant::now();

    // 1. Token embedding lookup (placeholder)
    log::debug!("Step 1: Token embedding lookup");
    let embedded = input.clone(); // Placeholder

    // 2. Upload to GPU
    log::debug!("Step 2: Upload to GPU");
    let mut gpu_tensor = device.upload_tensor(&embedded)?;

    // 3. Process through transformer layers
    log::debug!(
        "Step 3: Processing through {} layers",
        model.config.n_layers
    );
    for i in 0..model.config.n_layers {
        log::debug!("  Processing layer {}/{}", i + 1, model.config.n_layers);

        // Placeholder: In real implementation, run actual transformer operations
        let kernel = Kernel::new(KernelType::Attention);
        gpu_tensor = device.run_kernel(kernel, &[gpu_tensor])?;
    }

    // 4. Download result
    log::debug!("Step 4: Download result");
    let output = device.download_tensor(&gpu_tensor)?;

    // 5. Synchronize
    device.synchronize()?;

    let total_time = start.elapsed();
    log::info!("Total inference time: {:?}", total_time);
    log::info!(
        "Throughput: {:.2} tokens/sec",
        input_ids.len() as f64 / total_time.as_secs_f64()
    );
    log::info!("Output shape: {:?}", output.shape);

    Ok(())
}

/// Performance benchmarking
fn benchmark_performance(device: &Arc<dyn GpuDevice>) -> Result<()> {
    log::info!("\n=== Performance Benchmark ===");

    let sequence_lengths = vec![10, 50, 100, 256, 512];

    for seq_len in sequence_lengths {
        let input_data = vec![1.0; seq_len];
        let input = Tensor::from_f32(vec![1, seq_len], input_data)?;

        let start = Instant::now();

        // Upload
        let gpu_input = device.upload_tensor(&input)?;

        // Simple kernel execution
        let kernel = Kernel::new(KernelType::Gelu);
        let _output = device.run_kernel(kernel, &[gpu_input.clone()])?;

        // Download
        let _result = device.download_tensor(&gpu_input)?;
        device.synchronize()?;

        let elapsed = start.elapsed();
        log::info!(
            "Seq len {:4}: {:?} ({:.2} tokens/ms)",
            seq_len,
            elapsed,
            seq_len as f64 / elapsed.as_millis() as f64
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("╔═══════════════════════════════════════════════╗");
    log::info!("║   CrossGPU Complete Workflow Example         ║");
    log::info!("╚═══════════════════════════════════════════════╝");

    // 1. Auto-detect best device
    log::info!("\n=== Device Detection ===");
    let device = auto_detect_device()?;
    log::info!("Selected device: {}", device.device_name());
    log::info!("Device available: {}", device.is_available());

    // 2. Create transformer model
    let config = TransformerConfig::tiny();
    let model = create_transformer_model(&config)?;

    // 3. Demonstrate quantization
    demonstrate_quantization()?;

    // 4. Demonstrate kernel execution
    demonstrate_kernels(&device)?;

    // 5. Run inference workflow
    run_inference_workflow(&model, &device)?;

    // 6. Performance benchmarking
    benchmark_performance(&device)?;

    // 7. Model persistence (optional)
    log::info!("\n=== Model Persistence ===");
    let model_path = "tiny_transformer_demo.bin";
    log::info!("Saving model to {}", model_path);
    // model.save_to_file(model_path)?;
    // log::info!("Model saved successfully");

    log::info!("\n╔═══════════════════════════════════════════════╗");
    log::info!("║   Workflow completed successfully!           ║");
    log::info!("╚═══════════════════════════════════════════════╝");

    Ok(())
}
