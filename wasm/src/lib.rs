//! WASM build for CrossGPU - Browser support with WebGPU

#![deny(warnings)]
#![deny(missing_docs)]
#![allow(clippy::unused_unit)]

use wasm_bindgen::prelude::*;

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("CrossGPU WASM module initialized");
}

/// Simple utility function to test WASM module
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from CrossGPU, {}!", name)
}

/// Check if WebGPU is available
#[wasm_bindgen]
pub async fn check_webgpu_support() -> Result<bool, JsValue> {
    use crossgpu_core::gpu::GpuDevice;

    match crossgpu_backend_webgpu::WebGpuDevice::new().await {
        Ok(device) => Ok(device.is_available()),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello from CrossGPU, World!");
    }
}
