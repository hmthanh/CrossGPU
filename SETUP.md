# 🚀 CrossGPU - Quick Setup Guide

## Three Ways to Set Up

### 🎯 Option 1: Interactive Wizard (Recommended)

```bash
./setup-wizard.sh
```

This will guide you through each step interactively.

### ⚡ Option 2: Quick Setup (3 Commands)

```bash
# 1. Build
make build

# 2. Test
make test

# 3. Run
make run
```

### 📋 Option 3: Manual Step-by-Step

See `STEP_BY_STEP.md` for detailed instructions.

---

## Essential Commands

```bash
# Development
make build          # Build all packages
make test           # Run all tests
make run            # Run simple inference example
make fmt            # Format code
make lint           # Run clippy

# Build variants
make release        # Optimized build
make wasm           # Build for browser

# Utilities
make clean          # Remove build artifacts
make doc            # Generate documentation
make help           # Show all commands
```

---

## Project Structure

```
crossgpu/
├── core/              # Core library
├── backends/          # GPU implementations
│   ├── cpu/          # CPU fallback
│   ├── webgpu/       # WebGPU
│   ├── vulkan/       # Vulkan
│   ├── metal/        # Metal (macOS)
│   └── dx12/         # DirectX 12
├── wasm/              # Browser package
├── examples/          # Example apps
├── docs/              # Documentation
└── scripts/           # Utility scripts
```

---

## Documentation

- **STEP_BY_STEP.md** - Detailed setup instructions (this file)
- **GETTING_STARTED.md** - Quick start guide
- **docs/API_GUIDE.md** - Complete API reference
- **ARCHITECTURE.md** - System design
- **SETUP_COMPLETE.md** - Feature overview

---

## Verification

Check that everything works:

```bash
./scripts/verify.sh
```

Expected output: `✅ All checks passed!`

---

## Troubleshooting

### Build fails?

```bash
make clean
cargo update
make build
```

### Tests fail?

```bash
cargo test --all -- --nocapture
```

### Need help?

- Read `GETTING_STARTED.md`
- Check `docs/` directory
- Run `make help`

---

## Next Steps

After setup is complete:

1. ✅ Verify: `./scripts/verify.sh`
2. 📖 Read: `cat GETTING_STARTED.md`
3. 🏃 Run: `make run`
4. 🚀 Build: Start developing!

---

**Quick Start:** `./setup-wizard.sh`

**Full Guide:** `STEP_BY_STEP.md`

**Documentation:** `docs/API_GUIDE.md`
