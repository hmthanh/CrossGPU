#!/usr/bin/env bash
# Comprehensive project verification and health check

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." &> /dev/null && pwd)"
cd "$PROJECT_ROOT"

echo "🔍 CrossGPU Project Verification"
echo "=================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0
WARNINGS=0

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    ((ERRORS++))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

# 1. Check Rust installation
echo "1. Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    check_pass "Rust installed: $RUST_VERSION"
else
    check_fail "Rust not installed"
fi

if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    check_pass "Cargo installed: $CARGO_VERSION"
else
    check_fail "Cargo not installed"
fi
echo ""

# 2. Check project structure
echo "2. Checking project structure..."
REQUIRED_DIRS=("core" "backends" "examples" "docs" "wasm" "scripts")
for dir in "${REQUIRED_DIRS[@]}"; do
    if [ -d "$dir" ]; then
        check_pass "Directory exists: $dir/"
    else
        check_fail "Missing directory: $dir/"
    fi
done
echo ""

# 3. Check Cargo.toml files
echo "3. Checking Cargo.toml files..."
CARGO_TOMLS=(
    "Cargo.toml"
    "core/Cargo.toml"
    "backends/cpu/Cargo.toml"
    "backends/webgpu/Cargo.toml"
    "backends/vulkan/Cargo.toml"
    "backends/metal/Cargo.toml"
    "backends/dx12/Cargo.toml"
    "wasm/Cargo.toml"
    "examples/simple-inference/Cargo.toml"
)
for toml in "${CARGO_TOMLS[@]}"; do
    if [ -f "$toml" ]; then
        check_pass "Found: $toml"
    else
        check_fail "Missing: $toml"
    fi
done
echo ""

# 4. Check documentation
echo "4. Checking documentation..."
DOCS=(
    "README.md"
    "ARCHITECTURE.md"
    "CONTRIBUTING.md"
    "CHANGELOG.md"
    "LICENSE-MIT"
    "LICENSE-APACHE"
)
for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        check_pass "Found: $doc"
    else
        check_warn "Missing: $doc"
    fi
done
echo ""

# 5. Test compilation
echo "5. Testing compilation..."
if cargo check --all --quiet 2>&1; then
    check_pass "cargo check --all passed"
else
    check_fail "cargo check --all failed"
fi
echo ""

# 6. Run tests
echo "6. Running tests..."
if cargo test --all --quiet 2>&1; then
    check_pass "cargo test --all passed"
else
    check_fail "cargo test --all failed"
fi
echo ""

# 7. Check formatting
echo "7. Checking code formatting..."
if cargo fmt --all -- --check 2>&1; then
    check_pass "Code is formatted correctly"
else
    check_warn "Code needs formatting (run: cargo fmt --all)"
fi
echo ""

# 8. Run clippy
echo "8. Running clippy lints..."
if cargo clippy --all-targets --all-features -- -D warnings 2>&1; then
    check_pass "Clippy passed with no warnings"
else
    check_warn "Clippy found issues (run: cargo clippy --all-targets --all-features)"
fi
echo ""

# 9. Check CI configuration
echo "9. Checking CI/CD configuration..."
if [ -f ".github/workflows/ci.yml" ]; then
    check_pass "GitHub Actions CI configured"
else
    check_warn "Missing CI configuration"
fi
echo ""

# 10. Check development tools
echo "10. Checking development tools..."
if [ -f "Makefile" ]; then
    check_pass "Makefile present"
else
    check_warn "Makefile missing"
fi

if [ -f "Dockerfile.dev" ]; then
    check_pass "Development Dockerfile present"
else
    check_warn "Development Dockerfile missing"
fi

if [ -d ".vscode" ]; then
    check_pass "VS Code configuration present"
else
    check_warn "VS Code configuration missing"
fi
echo ""

# 11. Check WASM support
echo "11. Checking WASM support..."
if command -v wasm-pack &> /dev/null; then
    check_pass "wasm-pack installed"
else
    check_warn "wasm-pack not installed (needed for WASM builds)"
fi

if [ -f "build-wasm.sh" ]; then
    check_pass "WASM build script present"
else
    check_warn "WASM build script missing"
fi
echo ""

# 12. Check scripts
echo "12. Checking utility scripts..."
SCRIPTS=(
    "scripts/deploy-wasm.sh"
    "scripts/deploy-native.sh"
    "scripts/benchmark.sh"
    "scripts/new-backend.sh"
)
for script in "${SCRIPTS[@]}"; do
    if [ -f "$script" ]; then
        if [ -x "$script" ]; then
            check_pass "Found (executable): $script"
        else
            check_warn "Found (not executable): $script"
        fi
    else
        check_warn "Missing: $script"
    fi
done
echo ""

# Summary
echo "=================================="
echo "📊 Verification Summary"
echo "=================================="
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ All checks passed!${NC}"
    echo ""
    echo "🚀 Your project is ready for development!"
    EXIT_CODE=0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  Passed with $WARNINGS warning(s)${NC}"
    echo ""
    echo "💡 Your project is functional but has minor issues."
    EXIT_CODE=0
else
    echo -e "${RED}❌ Failed with $ERRORS error(s) and $WARNINGS warning(s)${NC}"
    echo ""
    echo "🔧 Please fix the errors above before proceeding."
    EXIT_CODE=1
fi

echo ""
echo "📈 Project Statistics:"
echo "   - Rust files: $(find . -name '*.rs' -not -path './target/*' | wc -l)"
echo "   - Lines of code: $(find . -name '*.rs' -not -path './target/*' -exec cat {} \; | wc -l)"
echo "   - Packages: $(find . -name 'Cargo.toml' | wc -l)"
echo "   - Documentation: $(find docs -name '*.md' 2>/dev/null | wc -l) guides"
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    echo "🎯 Next steps:"
    echo "   - Run examples: make run"
    echo "   - Build for release: make release"
    echo "   - Run benchmarks: make benchmark"
    echo "   - Build WASM: make wasm"
    echo "   - View docs: make doc"
fi

exit $EXIT_CODE
