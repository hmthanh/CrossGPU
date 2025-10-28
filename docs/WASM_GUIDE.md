# WASM Deployment Guide

This guide explains how to build, test, and deploy CrossGPU for WebAssembly (WASM) with WebGPU support.

## Overview

CrossGPU can be compiled to WebAssembly to run Transformer inference directly in web browsers using WebGPU for GPU acceleration.

## Prerequisites

### Required Tools

**Rust with WASM target**:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown
```

**wasm-pack**:

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Verify installation
wasm-pack --version
```

### Browser Requirements

CrossGPU WASM requires a browser with WebGPU support:

- **Chrome/Edge**: Version 113+ (stable WebGPU support)
- **Firefox**: Experimental support (enable via flags)
- **Safari**: Technology Preview builds

Check WebGPU support: [WebGPU Status](https://caniuse.com/webgpu)

## Building WASM

### Quick Build

```bash
cd wasm
wasm-pack build --target web --release
```

Output will be in `wasm/pkg/` directory.

### Build Targets

**Web** (recommended for browsers):

```bash
wasm-pack build --target web --release
```

**Node.js**:

```bash
wasm-pack build --target nodejs --release
```

**Bundler** (webpack, rollup):

```bash
wasm-pack build --target bundler --release
```

**No modules**:

```bash
wasm-pack build --target no-modules --release
```

### Build Profiles

**Development** (faster builds, larger size):

```bash
wasm-pack build --target web --dev
```

**Release** (optimized for size):

```bash
wasm-pack build --target web --release
```

The release profile uses:
- Size optimization (`opt-level = "z"`)
- Link-time optimization (LTO)
- Panic = abort (smaller binary)

## Output Files

After building, the `wasm/pkg/` directory contains:

```
wasm/pkg/
├── crossgpu_wasm.js          # JavaScript bindings
├── crossgpu_wasm_bg.wasm     # Compiled WASM module
├── crossgpu_wasm.d.ts        # TypeScript definitions
├── package.json              # npm package metadata
└── README.md                 # Package documentation
```

## Using in HTML

### Basic Usage

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>CrossGPU WASM Demo</title>
</head>
<body>
    <h1>CrossGPU Transformer Inference</h1>
    <button id="runInference">Run Inference</button>
    <div id="output"></div>
    
    <script type="module">
        import init, { greet, check_webgpu_support } from './pkg/crossgpu_wasm.js';
        
        async function run() {
            // Initialize WASM module
            await init();
            
            // Test basic functionality
            const message = greet('Browser');
            console.log(message);
            
            // Check WebGPU support
            const hasWebGPU = await check_webgpu_support();
            if (!hasWebGPU) {
                alert('WebGPU not supported in this browser');
                return;
            }
            
            console.log('WebGPU is supported!');
        }
        
        run();
        
        document.getElementById('runInference').addEventListener('click', async () => {
            // Your inference code here
            document.getElementById('output').textContent = 'Running inference...';
        });
    </script>
</body>
</html>
```

### Serving Locally

```bash
# Install a simple HTTP server
cargo install basic-http-server

# Or use Python
python3 -m http.server 8000

# Or use Node.js
npx http-server

# Serve the directory containing your HTML file
basic-http-server .
```

Open browser to `http://localhost:4000` (or appropriate port).

## Using with JavaScript Frameworks

### React

**Install package**:

```bash
npm install ./wasm/pkg
```

**Component example**:

```jsx
import { useEffect, useState } from 'react';
import init, { greet, check_webgpu_support } from 'crossgpu-wasm';

function TransformerDemo() {
    const [status, setStatus] = useState('Initializing...');
    const [output, setOutput] = useState('');
    
    useEffect(() => {
        async function initialize() {
            await init();
            const hasGPU = await check_webgpu_support();
            setStatus(hasGPU ? 'WebGPU ready' : 'WebGPU not available');
        }
        initialize();
    }, []);
    
    const runInference = async () => {
        setOutput('Running inference...');
        // Your inference code here
    };
    
    return (
        <div>
            <h1>CrossGPU Transformer</h1>
            <p>Status: {status}</p>
            <button onClick={runInference}>Run Inference</button>
            <div>{output}</div>
        </div>
    );
}
```

### Vue.js

```vue
<template>
    <div>
        <h1>CrossGPU Transformer</h1>
        <p>Status: {{ status }}</p>
        <button @click="runInference">Run Inference</button>
        <div>{{ output }}</div>
    </div>
</template>

<script>
import init, { greet, check_webgpu_support } from 'crossgpu-wasm';

export default {
    data() {
        return {
            status: 'Initializing...',
            output: ''
        };
    },
    async mounted() {
        await init();
        const hasGPU = await check_webgpu_support();
        this.status = hasGPU ? 'WebGPU ready' : 'WebGPU not available';
    },
    methods: {
        async runInference() {
            this.output = 'Running inference...';
            // Your inference code here
        }
    }
};
</script>
```

### Svelte

```svelte
<script>
    import { onMount } from 'svelte';
    import init, { greet, check_webgpu_support } from 'crossgpu-wasm';
    
    let status = 'Initializing...';
    let output = '';
    
    onMount(async () => {
        await init();
        const hasGPU = await check_webgpu_support();
        status = hasGPU ? 'WebGPU ready' : 'WebGPU not available';
    });
    
    async function runInference() {
        output = 'Running inference...';
        // Your inference code here
    }
</script>

<div>
    <h1>CrossGPU Transformer</h1>
    <p>Status: {status}</p>
    <button on:click={runInference}>Run Inference</button>
    <div>{output}</div>
</div>
```

## Deployment

### GitHub Pages

The CI/CD workflow automatically deploys to GitHub Pages when you push to `main`.

**Manual deployment**:

```bash
# Build WASM
cd wasm
wasm-pack build --target web --release

# Create deployment directory
mkdir -p deploy
cp -r pkg/* deploy/
cp your-index.html deploy/index.html

# Deploy (using gh-pages npm package)
npx gh-pages -d deploy
```

### Netlify

**netlify.toml**:

```toml
[build]
  command = "cd wasm && wasm-pack build --target web --release"
  publish = "wasm/pkg"

[[headers]]
  for = "/*"
  [headers.values]
    Cross-Origin-Embedder-Policy = "require-corp"
    Cross-Origin-Opener-Policy = "same-origin"
```

### Vercel

**vercel.json**:

```json
{
  "buildCommand": "cd wasm && wasm-pack build --target web --release",
  "outputDirectory": "wasm/pkg",
  "headers": [
    {
      "source": "/(.*)",
      "headers": [
        {
          "key": "Cross-Origin-Embedder-Policy",
          "value": "require-corp"
        },
        {
          "key": "Cross-Origin-Opener-Policy",
          "value": "same-origin"
        }
      ]
    }
  ]
}
```

## Advanced Features

### Worker Threads

Run inference in a Web Worker to avoid blocking the main thread:

**worker.js**:

```javascript
import init, { WasmTransformer } from './pkg/crossgpu_wasm.js';

let transformer = null;

self.onmessage = async (e) => {
    const { type, data } = e.data;
    
    if (type === 'init') {
        await init();
        transformer = await WasmTransformer.new();
        self.postMessage({ type: 'ready' });
    } else if (type === 'inference') {
        const output = await transformer.inference(data);
        self.postMessage({ type: 'result', output });
    }
};
```

**main.js**:

```javascript
const worker = new Worker('./worker.js', { type: 'module' });

worker.onmessage = (e) => {
    const { type, output } = e.data;
    if (type === 'ready') {
        console.log('Worker ready');
    } else if (type === 'result') {
        console.log('Result:', output);
    }
};

// Initialize
worker.postMessage({ type: 'init' });

// Run inference
worker.postMessage({ type: 'inference', data: inputTokens });
```

### SharedArrayBuffer

For better performance, enable SharedArrayBuffer:

**HTTP Headers** (required):

```
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Opener-Policy: same-origin
```

**Webpack dev server** configuration:

```javascript
devServer: {
    headers: {
        'Cross-Origin-Embedder-Policy': 'require-corp',
        'Cross-Origin-Opener-Policy': 'same-origin',
    },
},
```

## Optimization Tips

### 1. Code Size Reduction

**Use wasm-opt** (from Binaryen):

```bash
# Install wasm-opt
npm install -g wasm-opt

# Optimize WASM file
wasm-opt -Oz -o output.wasm input.wasm
```

**Rust compile flags** in `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "link-arg=-z",
    "-C", "link-arg=stack-size=65536",
    "-C", "opt-level=z",
]
```

### 2. Lazy Loading

Load the WASM module only when needed:

```javascript
let wasmModule = null;

async function loadWasm() {
    if (!wasmModule) {
        const { default: init } = await import('./pkg/crossgpu_wasm.js');
        await init();
        wasmModule = true;
    }
}

// Load on user interaction
button.addEventListener('click', async () => {
    await loadWasm();
    // Use WASM functions
});
```

### 3. Compression

Enable Brotli or gzip compression:

**nginx**:

```nginx
gzip on;
gzip_types application/wasm;

# Or Brotli (better compression)
brotli on;
brotli_types application/wasm;
```

### 4. Caching

Use appropriate cache headers:

```
Cache-Control: public, max-age=31536000, immutable
```

## Debugging

### Browser DevTools

**Enable verbose logging**:

```javascript
// In browser console
localStorage.setItem('debug', 'crossgpu:*');
```

**WebGPU inspection** (Chrome):

1. Open DevTools
2. Go to `chrome://gpu`
3. Check WebGPU status

### Console Logging

The WASM module uses `console.log` for output:

```rust
use log::{info, debug, error};

info!("Inference started");
debug!("Device: {}", device.device_name());
```

### Error Handling

Errors are propagated to JavaScript:

```javascript
try {
    const result = await transformer.inference(input);
} catch (error) {
    console.error('Inference failed:', error);
    // Display user-friendly error message
}
```

## Testing

### Unit Tests (Rust)

```bash
cd wasm
cargo test --target wasm32-unknown-unknown
```

### Browser Tests (wasm-pack)

```bash
wasm-pack test --headless --chrome
wasm-pack test --headless --firefox
```

### Manual Testing

1. Build WASM: `wasm-pack build --target web --dev`
2. Serve: `basic-http-server .`
3. Open browser DevTools
4. Check console for errors
5. Verify WebGPU initialization

## Performance Benchmarks

Typical inference times (single sequence, 512 tokens):

| Browser | Device | Time (ms) |
|---------|--------|-----------|
| Chrome | M1 Mac | ~15 ms |
| Chrome | RTX 3080 | ~10 ms |
| Chrome | Integrated GPU | ~30 ms |
| Chrome | CPU fallback | ~100 ms |

*Actual performance varies by model size and hardware.*

## Troubleshooting

### WebGPU Not Available

**Check browser support**:

```javascript
if (!navigator.gpu) {
    console.error('WebGPU not supported');
    // Show fallback message
}
```

**Enable in Firefox**:
1. Go to `about:config`
2. Set `dom.webgpu.enabled` to `true`

### CORS Errors

Serve files from HTTP server, not `file://` protocol:

```bash
basic-http-server .
# NOT: file:///path/to/index.html
```

### Large WASM File

- Use `opt-level = "z"` in Cargo.toml
- Run `wasm-opt -Oz`
- Enable Brotli compression
- Consider code splitting

### Memory Issues

Monitor memory usage:

```javascript
if (performance.memory) {
    console.log('Used:', performance.memory.usedJSHeapSize);
    console.log('Total:', performance.memory.totalJSHeapSize);
}
```

## Resources

- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [MDN WebGPU Guide](https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API)
- [Rust WASM Book](https://rustwasm.github.io/book/)

## Examples

See the CI/CD workflow for a complete example deployment: [.github/workflows/ci.yml](../.github/workflows/ci.yml)

---

**Need Help?** Open an issue on [GitHub](https://github.com/hmthanh/CrossGPU/issues).
