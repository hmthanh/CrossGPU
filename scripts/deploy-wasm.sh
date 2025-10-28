#!/usr/bin/env bash
# Deploy WASM package to web hosting

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
WASM_DIR="$PROJECT_ROOT/wasm"
DIST_DIR="$PROJECT_ROOT/dist"

echo "🚀 Building WASM package for production..."

# Build WASM with optimizations
cd "$WASM_DIR"
wasm-pack build --target web --release

echo "✅ WASM build complete"

# Create distribution directory
mkdir -p "$DIST_DIR"

# Copy WASM files
cp -r pkg/* "$DIST_DIR/"

# Create example HTML
cat > "$DIST_DIR/index.html" <<'EOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CrossGPU Transformer - WASM Demo</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            background: #0d1117;
            color: #c9d1d9;
        }
        h1 {
            color: #58a6ff;
        }
        .container {
            background: #161b22;
            border: 1px solid #30363d;
            border-radius: 6px;
            padding: 1.5rem;
            margin: 1rem 0;
        }
        button {
            background: #238636;
            color: white;
            border: none;
            padding: 0.75rem 1.5rem;
            border-radius: 6px;
            cursor: pointer;
            font-size: 1rem;
            margin: 0.5rem 0.5rem 0.5rem 0;
        }
        button:hover {
            background: #2ea043;
        }
        button:disabled {
            background: #6e7681;
            cursor: not-allowed;
        }
        #output {
            background: #0d1117;
            border: 1px solid #30363d;
            border-radius: 6px;
            padding: 1rem;
            margin-top: 1rem;
            font-family: 'Courier New', monospace;
            white-space: pre-wrap;
            min-height: 100px;
        }
        .status {
            color: #8b949e;
            font-style: italic;
        }
        .error {
            color: #f85149;
        }
        .success {
            color: #3fb950;
        }
    </style>
</head>
<body>
    <h1>🚀 CrossGPU Transformer Engine</h1>

    <div class="container">
        <h2>WebGPU Inference Demo</h2>
        <p>This demo runs a tiny transformer model directly in your browser using WebGPU.</p>

        <button id="initBtn" onclick="initialize()">Initialize Model</button>
        <button id="inferBtn" onclick="runInference()" disabled>Run Inference</button>
        <button id="quantizeBtn" onclick="quantizeModel()" disabled>Quantize (8-bit)</button>

        <div id="output" class="status">Click "Initialize Model" to start...</div>
    </div>

    <script type="module">
        import init, { WasmTransformer, check_webgpu_support } from './crossgpu_wasm.js';

        let transformer = null;

        async function initialize() {
            const output = document.getElementById('output');
            const initBtn = document.getElementById('initBtn');
            const inferBtn = document.getElementById('inferBtn');
            const quantizeBtn = document.getElementById('quantizeBtn');

            try {
                initBtn.disabled = true;
                output.textContent = '⏳ Initializing WASM module...';

                await init();

                output.textContent += '\n✅ WASM initialized\n⏳ Checking WebGPU support...';

                const webgpuSupported = check_webgpu_support();
                if (!webgpuSupported) {
                    throw new Error('WebGPU is not supported in this browser');
                }

                output.textContent += '\n✅ WebGPU is supported\n⏳ Creating transformer model...';

                transformer = new WasmTransformer();

                output.className = 'success';
                output.textContent += '\n✅ Model initialized successfully!\n\n' +
                    'Model Details:\n' +
                    '- Layers: 4\n' +
                    '- Hidden size: 256\n' +
                    '- Attention heads: 4\n' +
                    '- Vocabulary: 1000 tokens\n' +
                    '- Parameters: ~2M\n' +
                    '\nReady for inference!';

                inferBtn.disabled = false;
                quantizeBtn.disabled = false;
            } catch (error) {
                output.className = 'error';
                output.textContent = `❌ Error: ${error.message}`;
                initBtn.disabled = false;
            }
        }

        async function runInference() {
            const output = document.getElementById('output');
            const inferBtn = document.getElementById('inferBtn');

            try {
                inferBtn.disabled = true;
                output.className = '';
                output.textContent = '⏳ Running inference...';

                const start = performance.now();

                // Sample input tokens
                const inputTokens = new Uint32Array([1, 42, 123, 456]);

                const result = transformer.forward(inputTokens);

                const elapsed = performance.now() - start;

                output.className = 'success';
                output.textContent = `✅ Inference complete!\n\n` +
                    `Input tokens: [${Array.from(inputTokens).join(', ')}]\n` +
                    `Output shape: [${result.length}]\n` +
                    `Time: ${elapsed.toFixed(2)}ms\n` +
                    `Throughput: ${(inputTokens.length / (elapsed / 1000)).toFixed(0)} tokens/sec\n\n` +
                    `Output preview: [${result.slice(0, 5).map(x => x.toFixed(4)).join(', ')}...]`;

            } catch (error) {
                output.className = 'error';
                output.textContent = `❌ Error: ${error.message}`;
            } finally {
                inferBtn.disabled = false;
            }
        }

        async function quantizeModel() {
            const output = document.getElementById('output');
            const quantizeBtn = document.getElementById('quantizeBtn');

            try {
                quantizeBtn.disabled = true;
                output.className = '';
                output.textContent = '⏳ Quantizing model to 8-bit...';

                transformer.quantize_8bit();

                output.className = 'success';
                output.textContent = '✅ Model quantized to 8-bit!\n\n' +
                    'Memory usage reduced by ~75%\n' +
                    'Inference speed may improve on some devices.\n' +
                    'Try running inference again to see the quantized model in action.';

            } catch (error) {
                output.className = 'error';
                output.textContent = `❌ Error: ${error.message}`;
            } finally {
                quantizeBtn.disabled = false;
            }
        }

        // Expose functions globally
        window.initialize = initialize;
        window.runInference = runInference;
        window.quantizeModel = quantizeModel;
    </script>
</body>
</html>
EOF

echo "📄 Created index.html"

# Optional: Optimize WASM with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
    echo "🔧 Optimizing WASM with wasm-opt..."
    wasm-opt -Oz -o "$DIST_DIR/crossgpu_wasm_bg.wasm.opt" "$DIST_DIR/crossgpu_wasm_bg.wasm"
    mv "$DIST_DIR/crossgpu_wasm_bg.wasm.opt" "$DIST_DIR/crossgpu_wasm_bg.wasm"
    echo "✅ WASM optimized"
else
    echo "⚠️  wasm-opt not found, skipping optimization (install binaryen for smaller builds)"
fi

echo ""
echo "✅ Deployment package ready in: $DIST_DIR"
echo ""
echo "📦 Files:"
ls -lh "$DIST_DIR"
echo ""
echo "🌐 To test locally:"
echo "   cd $DIST_DIR"
echo "   python3 -m http.server 8080"
echo "   # Then open http://localhost:8080"
echo ""
echo "☁️  To deploy to hosting:"
echo "   - Netlify: netlify deploy --dir=$DIST_DIR --prod"
echo "   - Vercel: vercel --prod $DIST_DIR"
echo "   - GitHub Pages: cp -r $DIST_DIR/* docs/ && git add docs && git commit && git push"
echo "   - AWS S3: aws s3 sync $DIST_DIR s3://your-bucket-name --acl public-read"
