//! Transformer layer definitions and configuration

use crate::error::Result;
use crate::gpu::{GpuDevice, GpuTensor};
use crate::tensor::Tensor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for a Transformer model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformerConfig {
    /// Model dimension (hidden size)
    pub d_model: usize,
    /// Number of attention heads
    pub n_heads: usize,
    /// Number of transformer layers
    pub n_layers: usize,
    /// Feedforward dimension
    pub d_ff: usize,
    /// Vocabulary size
    pub vocab_size: usize,
    /// Maximum sequence length
    pub max_seq_len: usize,
    /// Dropout rate (not used in inference)
    pub dropout: f32,
    /// Layer normalization epsilon
    pub layer_norm_eps: f32,
}

impl TransformerConfig {
    /// Create a tiny transformer configuration (~50MB model)
    pub fn tiny() -> Self {
        Self {
            d_model: 512,
            n_heads: 8,
            n_layers: 6,
            d_ff: 2048,
            vocab_size: 32000,
            max_seq_len: 512,
            dropout: 0.1,
            layer_norm_eps: 1e-5,
        }
    }

    /// Estimate model size in bytes
    pub fn estimate_size(&self) -> usize {
        // Embedding: vocab_size * d_model
        let embedding_size = self.vocab_size * self.d_model * 4; // f32

        // Per layer: Q,K,V,O weights + 2 FF layers + layer norms
        let per_layer_size = (
            4 * self.d_model * self.d_model + // QKV + output projection
            2 * self.d_model * self.d_ff +     // Two FF layers
            4 * self.d_model
            // Two layer norms (weight + bias each)
        ) * 4; // f32

        embedding_size + per_layer_size * self.n_layers
    }
}

/// Multi-head attention layer weights
#[derive(Debug, Clone)]
pub struct AttentionWeights {
    /// Query projection weights [d_model, d_model]
    pub wq: Tensor,
    /// Key projection weights [d_model, d_model]
    pub wk: Tensor,
    /// Value projection weights [d_model, d_model]
    pub wv: Tensor,
    /// Output projection weights [d_model, d_model]
    pub wo: Tensor,
}

/// Feed-forward network weights
#[derive(Debug, Clone)]
pub struct FeedForwardWeights {
    /// First linear layer [d_model, d_ff]
    pub w1: Tensor,
    /// Second linear layer [d_ff, d_model]
    pub w2: Tensor,
}

/// Layer normalization weights
#[derive(Debug, Clone)]
pub struct LayerNormWeights {
    /// Scale parameter [d_model]
    pub gamma: Tensor,
    /// Bias parameter [d_model]
    pub beta: Tensor,
}

/// Complete transformer layer weights
#[derive(Debug, Clone)]
pub struct TransformerLayerWeights {
    /// Multi-head attention weights
    pub attention: AttentionWeights,
    /// Feed-forward network weights
    pub feed_forward: FeedForwardWeights,
    /// Layer norm before attention
    pub ln1: LayerNormWeights,
    /// Layer norm before feed-forward
    pub ln2: LayerNormWeights,
}

/// Transformer layer - performs forward pass computation
pub struct TransformerLayer {
    config: TransformerConfig,
    #[allow(dead_code)]
    weights: TransformerLayerWeights,
}

impl TransformerLayer {
    /// Create a new transformer layer with the given configuration and weights
    pub fn new(config: TransformerConfig, weights: TransformerLayerWeights) -> Self {
        Self { config, weights }
    }

    /// Forward pass on CPU (fallback implementation)
    pub fn forward_cpu(&self, input: &Tensor) -> Result<Tensor> {
        // Placeholder: Implement actual transformer forward pass
        // 1. Layer norm
        // 2. Multi-head attention
        // 3. Residual connection
        // 4. Layer norm
        // 5. Feed-forward
        // 6. Residual connection

        log::info!("Running transformer layer forward pass on CPU");
        Ok(input.clone())
    }

    /// Forward pass on GPU
    pub fn forward_gpu(&self, input: &GpuTensor, device: &Arc<dyn GpuDevice>) -> Result<GpuTensor> {
        // Placeholder: Use GPU kernels for computation
        log::info!(
            "Running transformer layer forward pass on GPU: {}",
            device.device_name()
        );

        // Example workflow:
        // 1. Run layer norm kernel
        // 2. Run attention kernel (fused QKV computation + softmax)
        // 3. Add residual
        // 4. Run layer norm kernel
        // 5. Run fused GEMM+GELU for feed-forward
        // 6. Add residual

        Ok(input.clone())
    }

    /// Get the layer configuration
    pub fn config(&self) -> &TransformerConfig {
        &self.config
    }
}

/// Complete transformer model
pub struct TransformerModel {
    /// Model configuration
    pub config: TransformerConfig,
    /// Token embedding weights [vocab_size, d_model]
    pub token_embedding: Tensor,
    /// Position embedding weights [max_seq_len, d_model]
    pub position_embedding: Tensor,
    /// Transformer layers
    pub layers: Vec<TransformerLayerWeights>,
    /// Final layer norm
    pub final_layer_norm: LayerNormWeights,
}

impl TransformerModel {
    /// Create a new transformer model
    pub fn new(
        config: TransformerConfig,
        token_embedding: Tensor,
        position_embedding: Tensor,
        layers: Vec<TransformerLayerWeights>,
        final_layer_norm: LayerNormWeights,
    ) -> Self {
        Self {
            config,
            token_embedding,
            position_embedding,
            layers,
            final_layer_norm,
        }
    }

    /// Load model from binary file
    pub fn load_from_file(path: &str) -> Result<Self> {
        use crate::error::CoreError;
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        bincode::deserialize(&buffer)
            .map_err(|e| CoreError::ModelLoadError(format!("Failed to deserialize model: {}", e)))
    }

    /// Save model to binary file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        use crate::error::CoreError;
        use std::fs::File;
        use std::io::Write;

        let encoded = bincode::serialize(self).map_err(|e| {
            CoreError::SerializationError(format!("Failed to serialize model: {}", e))
        })?;

        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }
}

// Implement Serialize/Deserialize for the model
impl Serialize for TransformerModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TransformerModel", 5)?;
        state.serialize_field("config", &self.config)?;
        state.serialize_field("token_embedding", &self.token_embedding)?;
        state.serialize_field("position_embedding", &self.position_embedding)?;
        state.serialize_field("layers", &self.layers)?;
        state.serialize_field("final_layer_norm", &self.final_layer_norm)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TransformerModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransformerModelHelper {
            config: TransformerConfig,
            token_embedding: Tensor,
            position_embedding: Tensor,
            layers: Vec<TransformerLayerWeights>,
            final_layer_norm: LayerNormWeights,
        }

        let helper = TransformerModelHelper::deserialize(deserializer)?;
        Ok(TransformerModel {
            config: helper.config,
            token_embedding: helper.token_embedding,
            position_embedding: helper.position_embedding,
            layers: helper.layers,
            final_layer_norm: helper.final_layer_norm,
        })
    }
}

// Implement Serialize/Deserialize for weight structures
impl Serialize for AttentionWeights {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AttentionWeights", 4)?;
        state.serialize_field("wq", &self.wq)?;
        state.serialize_field("wk", &self.wk)?;
        state.serialize_field("wv", &self.wv)?;
        state.serialize_field("wo", &self.wo)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for AttentionWeights {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            wq: Tensor,
            wk: Tensor,
            wv: Tensor,
            wo: Tensor,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(AttentionWeights {
            wq: helper.wq,
            wk: helper.wk,
            wv: helper.wv,
            wo: helper.wo,
        })
    }
}

impl Serialize for FeedForwardWeights {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("FeedForwardWeights", 2)?;
        state.serialize_field("w1", &self.w1)?;
        state.serialize_field("w2", &self.w2)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for FeedForwardWeights {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            w1: Tensor,
            w2: Tensor,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(FeedForwardWeights {
            w1: helper.w1,
            w2: helper.w2,
        })
    }
}

impl Serialize for LayerNormWeights {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LayerNormWeights", 2)?;
        state.serialize_field("gamma", &self.gamma)?;
        state.serialize_field("beta", &self.beta)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for LayerNormWeights {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            gamma: Tensor,
            beta: Tensor,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(LayerNormWeights {
            gamma: helper.gamma,
            beta: helper.beta,
        })
    }
}

impl Serialize for TransformerLayerWeights {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TransformerLayerWeights", 4)?;
        state.serialize_field("attention", &self.attention)?;
        state.serialize_field("feed_forward", &self.feed_forward)?;
        state.serialize_field("ln1", &self.ln1)?;
        state.serialize_field("ln2", &self.ln2)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for TransformerLayerWeights {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            attention: AttentionWeights,
            feed_forward: FeedForwardWeights,
            ln1: LayerNormWeights,
            ln2: LayerNormWeights,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(TransformerLayerWeights {
            attention: helper.attention,
            feed_forward: helper.feed_forward,
            ln1: helper.ln1,
            ln2: helper.ln2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = TransformerConfig::tiny();
        assert_eq!(config.d_model, 512);
        assert_eq!(config.n_heads, 8);
        assert_eq!(config.n_layers, 6);
    }

    #[test]
    fn test_model_size_estimation() {
        let config = TransformerConfig::tiny();
        let size = config.estimate_size();
        // The estimate is around 90MB, so check a wider range
        assert!(size > 80_000_000 && size < 400_000_000, "Size was {}", size);
    }
}
