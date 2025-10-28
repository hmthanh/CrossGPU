# CrossGPU Development Makefile
# Convenience commands for common development tasks

.PHONY: help build test clean fmt lint check doc run run-example run-workflow wasm install-tools

# Default target
help:
	@echo "CrossGPU Development Commands"
	@echo "=============================="
	@echo ""
	@echo "Setup:"
	@echo "  make install-tools  - Install development tools (wasm-pack, etc.)"
	@echo ""
	@echo "Build:"
	@echo "  make build          - Build all packages in release mode"
	@echo "  make build-dev      - Build all packages in debug mode"
	@echo "  make wasm           - Build WASM package"
	@echo ""
	@echo "Quality:"
	@echo "  make fmt            - Format all Rust code"
	@echo "  make lint           - Run clippy linter"
	@echo "  make check          - Quick compile check"
	@echo "  make test           - Run all tests"
	@echo "  make test-verbose   - Run tests with output"
	@echo ""
	@echo "Documentation:"
	@echo "  make doc            - Generate and open documentation"
	@echo "  make doc-private    - Generate docs including private items"
	@echo ""
	@echo "Run:"
	@echo "  make run            - Run simple-inference example (alias)"
	@echo "  make run-example    - Run simple-inference example"
	@echo "  make run-workflow   - Run complete-workflow example"
	@echo ""
	@echo "Workflows:"
	@echo "  make ci             - Run all CI checks locally"
	@echo "  make dev            - Quick development checks (fmt+lint+test)"
	@echo "  make release        - Full release build"
	@echo ""
	@echo "Clean:"
	@echo "  make clean          - Remove build artifacts"
	@echo "  make clean-all      - Remove all generated files"
	@echo ""

# Install development tools
install-tools:
	@echo "Installing development tools..."
	@rustup component add rustfmt clippy
	@rustup target add wasm32-unknown-unknown
	@command -v wasm-pack >/dev/null 2>&1 || \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	@echo "✓ Tools installed"

# Build commands
build:
	@echo "Building all packages in release mode..."
	@cargo build --release --all
	@echo "✓ Build complete"

build-dev:
	@echo "Building all packages in debug mode..."
	@cargo build --all
	@echo "✓ Debug build complete"

wasm:
	@echo "Building WASM package..."
	@cd wasm && wasm-pack build --target web --release
	@echo "✓ WASM build complete (output: wasm/pkg/)"

# Quality checks
fmt:
	@echo "Formatting Rust code..."
	@cargo fmt --all
	@echo "✓ Code formatted"

lint:
	@echo "Running clippy linter..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✓ Clippy checks passed"

check:
	@echo "Running quick compile check..."
	@cargo check --all
	@echo "✓ Check complete"

# Testing
test:
	@echo "Running all tests..."
	@cargo test --all
	@echo "✓ All tests passed"

test-verbose:
	@echo "Running tests with output..."
	@cargo test --all -- --nocapture
	@echo "✓ Tests complete"

test-core:
	@echo "Testing core library..."
	@cargo test -p crossgpu-core

test-cpu:
	@echo "Testing CPU backend..."
	@cargo test -p crossgpu-backend-cpu

test-integration:
	@echo "Running integration tests..."
	@cargo test --test integration_test

# Documentation
doc:
	@echo "Generating documentation..."
	@cargo doc --no-deps --open
	@echo "✓ Documentation opened in browser"

doc-private:
	@echo "Generating documentation (including private items)..."
	@cargo doc --no-deps --document-private-items --open

# Run examples
run: run-example
	@# Alias for run-example

run-example:
	@echo "Running simple-inference example..."
	@RUST_LOG=info cargo run --release --bin simple-inference

run-workflow:
	@echo "Running complete-workflow example..."
	@RUST_LOG=info cargo run --release --example complete-workflow

# Clean commands
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@echo "✓ Clean complete"

clean-all: clean
	@echo "Removing all generated files..."
	@rm -rf wasm/pkg
	@rm -rf target
	@rm -rf */target
	@find . -name "Cargo.lock" -type f -delete
	@echo "✓ All generated files removed"

# CI simulation
ci:
	@echo "Running CI checks locally..."
	@make fmt
	@make lint
	@make test
	@make build
	@make wasm
	@echo "✓ All CI checks passed"

# Quick development workflow
dev: fmt lint test
	@echo "✓ Development checks complete"

# Release workflow
release: clean
	@make fmt
	@make lint
	@make test
	@make build
	@make wasm
	@echo "✓ Release build complete"

# Benchmarking (placeholder for future)
bench:
	@echo "Benchmarking not yet implemented"
	@echo "TODO: Add cargo-criterion benchmarks"

# Watch mode (requires cargo-watch)
watch:
	@command -v cargo-watch >/dev/null 2>&1 || \
		(echo "Installing cargo-watch..." && cargo install cargo-watch)
	@cargo watch -x check -x test

# Code coverage (requires cargo-tarpaulin)
coverage:
	@command -v cargo-tarpaulin >/dev/null 2>&1 || \
		(echo "Installing cargo-tarpaulin..." && cargo install cargo-tarpaulin)
	@cargo tarpaulin --out Html --output-dir coverage
	@echo "✓ Coverage report generated: coverage/index.html"
