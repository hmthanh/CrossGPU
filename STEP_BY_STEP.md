# CrossGPU - Step-by-Step Setup Guide

Follow these steps in order to set up and verify your CrossGPU project.

---

## ✅ Step 1: Verify Prerequisites

### Check Rust Installation

```bash
rustc --version
cargo --version
```

**Expected output:**

```
rustc 1.70.0 (or higher)
cargo 1.70.0 (or higher)
```

**If Rust is not installed:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## ✅ Step 2: Navigate to Project Directory

```bash
cd /Users/thanh/Workspace/CrossGPU
```

**Verify you're in the right place:**

```bash
ls -la
```

You should see:

- `Cargo.toml` (workspace manifest)
- `core/`, `backends/`, `examples/`, `wasm/` directories
- `Makefile`, `README.md`, etc.

---

## ✅ Step 3: Check Project Structure

```bash
# List all packages
ls -d */ | grep -E "core|backends|examples|wasm"
```

**Expected directories:**

- `core/`
- `backends/` (containing cpu, webgpu, vulkan, metal, dx12)
- `examples/`
- `wasm/`

**Verify Cargo workspace:**

```bash
cat Cargo.toml | grep members -A 15
```

---

## ✅ Step 4: Make Scripts Executable

```bash
chmod +x build-wasm.sh
chmod +x scripts/*.sh
```

**Verify:**

```bash
ls -la scripts/
ls -la build-wasm.sh
```

Scripts should show `-rwxr-xr-x` (executable flag).

---

## ✅ Step 5: Clean Any Previous Builds

```bash
make clean
# or: cargo clean
```

This removes the `target/` directory and any old build artifacts.

---

## ✅ Step 6: Build the Project (Debug Mode)

```bash
make build
# or: cargo build --all
```

**What this does:**

- Compiles all 9 packages
- Downloads dependencies
- Creates debug binaries in `target/debug/`

**Expected output (last line):**

```
Finished dev [unoptimized + debuginfo] target(s) in XXs
```

**If you see errors:**

- Check that all `Cargo.toml` files are present
- Ensure dependencies can be downloaded
- Review error messages for missing system libraries

---

## ✅ Step 7: Build Release Version

```bash
make release
# or: cargo build --release --all
```

**What this does:**

- Compiles with optimizations
- Takes longer but produces faster binaries
- Creates binaries in `target/release/`

**Expected output (last line):**

```
Finished release [optimized] target(s) in XXs
```

---

## ✅ Step 8: Run Tests

```bash
make test
# or: cargo test --all
```

**Expected output:**

```
running 14 tests
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**What's being tested:**

- Tensor operations
- Data type conversions (F32, F16, I8, I4)
- Quantization (8-bit, 4-bit)
- Model serialization
- Backend initialization (platform-specific)

---

## ✅ Step 9: Format Code

```bash
make fmt
# or: cargo fmt --all
```

This ensures all code follows Rust formatting standards.

---

## ✅ Step 10: Run Linter (Clippy)

```bash
make lint
# or: cargo clippy --all-targets --all-features -- -D warnings
```

**Expected output:**

```
Checking crossgpu-core...
Checking crossgpu-cpu...
... (all packages)
Finished checking
```

This catches common mistakes and suggests improvements.

---

## ✅ Step 11: Run Your First Example

```bash
make run
# or: cargo run --release --bin simple-inference
```

**Expected output:**

```
🚀 CrossGPU Transformer Inference
================================
Detecting GPU backend...
✅ Using device: Metal GPU (on macOS) or CPU (fallback)

Creating tiny transformer model...
✅ Model initialized (2M parameters, ~50MB)

Running inference...
Input tokens: [1, 42, 123, 456]
✅ Inference complete!
Output shape: [4, 1000]
```

---

## ✅ Step 12: Build WASM Package (Optional)

**First, install wasm-pack:**

```bash
cargo install wasm-pack
```

**Then build WASM:**

```bash
make wasm
# or: ./build-wasm.sh
```

**Expected output:**

```
[INFO]: Checking for wasm-pack...
[INFO]: Compiling to WebAssembly...
[INFO]: Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries...
[INFO]: ✅ WASM package created in wasm/pkg/
```

**Files created:**

- `wasm/pkg/crossgpu_wasm.js`
- `wasm/pkg/crossgpu_wasm_bg.wasm`
- `wasm/pkg/package.json`

---

## ✅ Step 13: Deploy WASM to Browser (Optional)

```bash
./scripts/deploy-wasm.sh
```

**What this does:**

- Builds optimized WASM
- Creates `dist/` folder with example HTML
- Copies all necessary files

**Test locally:**

```bash
cd dist
python3 -m http.server 8080
# Open browser to http://localhost:8080
```

---

## ✅ Step 14: Run Benchmarks (Optional)

```bash
./scripts/benchmark.sh
```

**What this does:**

- Runs tests 5 times each
- Measures build times, test times, inference speed
- Saves results to `benchmark-results/`

**Output files:**

- `benchmark_YYYYMMDD_HHMMSS.json` (raw data)
- `benchmark_YYYYMMDD_HHMMSS.md` (formatted report)

---

## ✅ Step 15: Generate Documentation

```bash
make doc
# or: cargo doc --all --no-deps --open
```

**What this does:**

- Generates HTML documentation
- Opens in browser automatically
- Documentation at `target/doc/crossgpu_core/index.html`

---

## ✅ Step 16: Verify Project Health

```bash
./scripts/verify.sh
```

**This comprehensive script checks:**

- ✅ Rust installation
- ✅ Project structure
- ✅ All Cargo.toml files
- ✅ Documentation files
- ✅ Code compilation
- ✅ Tests pass
- ✅ Code formatting
- ✅ Clippy lints
- ✅ CI configuration
- ✅ Development tools
- ✅ WASM support
- ✅ Utility scripts

**Expected output:**

```
✓ All checks passed!
🚀 Your project is ready for development!
```

---

## ✅ Step 17: Run Complete Workflow Example

```bash
cargo run --release --example complete-workflow
```

**This demonstrates:**

- Model creation
- Device auto-detection
- Inference with different backends
- Quantization (8-bit and 4-bit)
- Benchmarking
- Model saving/loading

---

## 🐳 Optional: Docker Setup

### Build Development Container

```bash
docker-compose build
```

### Start Container

```bash
docker-compose up -d
```

### Enter Container

```bash
docker-compose exec dev bash
```

### Build Inside Container

```bash
cargo build --all
cargo test --all
```

### Stop Container

```bash
docker-compose down
```

---

## 🎯 Quick Command Reference

### Daily Development

```bash
make build          # Quick debug build
make test           # Run all tests
make run            # Run simple example
make fmt            # Format code
make lint           # Check with clippy
```

### Release & Deploy

```bash
make release        # Optimized build
make wasm           # Build for browser
make benchmark      # Performance tests
./scripts/deploy-wasm.sh      # Deploy to web
./scripts/deploy-native.sh    # Package binaries
```

### Utilities

```bash
make clean          # Remove build artifacts
make doc            # Generate documentation
make ci             # Full CI checks
./scripts/verify.sh           # Health check
./scripts/new-backend.sh      # Create new backend
```

---

## 🎓 What to Read Next

### Beginners

1. `GETTING_STARTED.md` - Quick start guide
2. `docs/QUICK_REFERENCE.md` - Cheat sheet
3. Run examples in `examples/`

### Developers

1. `docs/API_GUIDE.md` - Complete API reference
2. `ARCHITECTURE.md` - System design
3. `docs/BUILD_GUIDE.md` - Platform-specific builds

### Contributors

1. `CONTRIBUTING.md` - Contribution guidelines
2. Implement GPU kernels in `backends/*/src/lib.rs`
3. Add tests and documentation

---

## ✅ Success Checklist

After completing all steps, you should have:

- [ ] Rust 1.70+ installed and verified
- [ ] All 9 packages built successfully
- [ ] All 14 tests passing
- [ ] `simple-inference` example runs
- [ ] Code formatted with `cargo fmt`
- [ ] Clippy checks pass
- [ ] WASM package built (if doing browser work)
- [ ] Documentation generated
- [ ] `./scripts/verify.sh` shows all green ✓

---

## 🚨 Troubleshooting

### Build Fails

```bash
# Clean and retry
make clean
cargo update
make build
```

### Tests Fail

```bash
# Run specific test for details
cargo test --package crossgpu-core -- --nocapture
```

### WASM Build Fails

```bash
# Ensure wasm-pack is installed
cargo install wasm-pack --force
# Try again
./build-wasm.sh
```

### Permission Denied on Scripts

```bash
# Make scripts executable
chmod +x scripts/*.sh build-wasm.sh
```

---

## 🎉 You're Done!

If you've completed all steps successfully, your CrossGPU project is fully set up and ready for development!

**Next steps:**

- Explore the examples
- Read the API documentation
- Start implementing GPU kernels
- Build your transformer models!

For questions, see:

- `GETTING_STARTED.md`
- `docs/` directory
- GitHub Issues

**Happy coding! 🚀**
