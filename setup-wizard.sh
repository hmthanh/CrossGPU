#!/usr/bin/env bash
# Interactive setup wizard for CrossGPU

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

clear
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                                                        ║${NC}"
echo -e "${BLUE}║          🚀 CrossGPU Setup Wizard 🚀                  ║${NC}"
echo -e "${BLUE}║                                                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
cd "$PROJECT_ROOT"

# Function to run a step
run_step() {
    local step_num=$1
    local step_name=$2
    local command=$3

    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Step $step_num: $step_name${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""

    read -p "Press ENTER to run this step (or 's' to skip): " choice

    if [[ "$choice" == "s" ]]; then
        echo -e "${YELLOW}⊘ Skipped${NC}"
        return
    fi

    echo ""
    echo -e "${BLUE}Running: $command${NC}"
    echo ""

    if eval "$command"; then
        echo ""
        echo -e "${GREEN}✅ Step $step_num complete!${NC}"
    else
        echo ""
        echo -e "${RED}❌ Step $step_num failed!${NC}"
        read -p "Continue anyway? (y/N): " continue_choice
        if [[ "$continue_choice" != "y" ]]; then
            echo "Setup aborted."
            exit 1
        fi
    fi
}

# Welcome
echo "This wizard will guide you through setting up CrossGPU."
echo "You can skip steps if needed."
echo ""
read -p "Ready to begin? (y/N): " ready
if [[ "$ready" != "y" ]]; then
    echo "Setup cancelled."
    exit 0
fi

# Step 1: Check Rust
run_step 1 "Check Rust Installation" "rustc --version && cargo --version"

# Step 2: Make scripts executable
run_step 2 "Make Scripts Executable" "chmod +x build-wasm.sh scripts/*.sh"

# Step 3: Clean previous builds
run_step 3 "Clean Previous Builds" "cargo clean"

# Step 4: Build project (debug)
run_step 4 "Build Project (Debug)" "cargo build --all"

# Step 5: Run tests
run_step 5 "Run Tests" "cargo test --all"

# Step 6: Format code
run_step 6 "Format Code" "cargo fmt --all"

# Step 7: Run clippy
run_step 7 "Run Clippy Lints" "cargo clippy --all-targets --all-features"

# Step 8: Build release
run_step 8 "Build Release Version" "cargo build --release --all"

# Step 9: Run example
run_step 9 "Run Simple Inference Example" "cargo run --release --bin simple-inference"

# Ask about optional steps
echo ""
echo -e "${YELLOW}Optional steps (can be skipped):${NC}"
read -p "Build WASM package? (y/N): " build_wasm
read -p "Generate documentation? (y/N): " gen_docs
read -p "Run benchmarks? (y/N): " run_bench

if [[ "$build_wasm" == "y" ]]; then
    # Check for wasm-pack
    if ! command -v wasm-pack &> /dev/null; then
        echo ""
        echo -e "${YELLOW}wasm-pack not found. Installing...${NC}"
        cargo install wasm-pack
    fi
    run_step 10 "Build WASM Package" "./build-wasm.sh"
fi

if [[ "$gen_docs" == "y" ]]; then
    run_step 11 "Generate Documentation" "cargo doc --all --no-deps"
fi

if [[ "$run_bench" == "y" ]]; then
    run_step 12 "Run Benchmarks" "./scripts/benchmark.sh"
fi

# Final verification
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}Final Verification${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
read -p "Run comprehensive health check? (y/N): " health_check

if [[ "$health_check" == "y" ]]; then
    ./scripts/verify.sh
fi

# Summary
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                        ║${NC}"
echo -e "${GREEN}║              ✅ Setup Complete! ✅                     ║${NC}"
echo -e "${GREEN}║                                                        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Your CrossGPU project is ready!"
echo ""
echo "Next steps:"
echo "  📖 Read GETTING_STARTED.md for detailed usage"
echo "  🏃 Run: make run"
echo "  📚 View docs: make doc"
echo "  🚀 Deploy WASM: ./scripts/deploy-wasm.sh"
echo ""
echo "Quick commands:"
echo "  make build    # Build all packages"
echo "  make test     # Run tests"
echo "  make run      # Run example"
echo "  make help     # Show all commands"
echo ""
echo -e "${GREEN}Happy coding! 🚀${NC}"
