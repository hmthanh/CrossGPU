# Contributing to CrossGPU

Thank you for your interest in contributing to CrossGPU! This document provides guidelines and best practices for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/CrossGPU.git
   cd CrossGPU
   ```
3. **Set up the development environment**:
   ```bash
   cargo build --all
   cargo test --all
   ```

## Development Workflow

### Making Changes

1. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Test your changes**:
   ```bash
   cargo test --all
   cargo clippy --all-targets --all-features
   cargo fmt --all -- --check
   ```

4. **Commit your changes** with a descriptive message:
   ```bash
   git add .
   git commit -m "Add feature: description of your changes"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Open a Pull Request** on GitHub

## Coding Standards

### Rust Best Practices

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `#![deny(warnings)]` in library code
- Add documentation comments (`///`) for all public items
- Write unit tests for new functionality
- Use `Result<T, E>` for error handling

### Code Style

- Run `cargo fmt --all` before committing
- Run `cargo clippy --all-targets --all-features` and address warnings
- Maximum line length: 100 characters
- Use descriptive variable and function names

### Documentation

- Add doc comments for all public APIs
- Include examples in doc comments where appropriate
- Update README.md if you add new features
- Keep comments concise and meaningful

## Testing

### Running Tests

```bash
# Run all tests
cargo test --all

# Run specific package tests
cargo test -p crossgpu-core

# Run tests with logging
RUST_LOG=debug cargo test --all
```

### Writing Tests

- Add unit tests in the same file as the code (`#[cfg(test)] mod tests`)
- Add integration tests in the `tests/` directory
- Test edge cases and error conditions
- Use `approx` crate for floating-point comparisons

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt --all`)
- [ ] Clippy is happy (`cargo clippy --all-targets --all-features`)
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### PR Description

Include in your PR description:

1. **What** - What changes does this PR introduce?
2. **Why** - Why is this change necessary?
3. **How** - How does it work?
4. **Testing** - How was it tested?

### Review Process

1. Automated CI checks will run on your PR
2. A maintainer will review your code
3. Address any feedback or requested changes
4. Once approved, your PR will be merged

## Project Structure

```
CrossGPU/
├── core/                  # Core tensor and transformer operations
├── backends/              # GPU backend implementations
│   ├── cpu/              # CPU fallback
│   ├── webgpu/           # WebGPU support
│   ├── vulkan/           # Vulkan support
│   ├── metal/            # Metal support
│   └── dx12/             # DirectX 12 support
├── examples/             # Example applications
├── wasm/                 # WASM build
└── tests/                # Integration tests
```

## Adding a New Backend

If you want to add support for a new GPU backend:

1. Create a new package in `backends/your-backend/`
2. Implement the `GpuDevice` trait from `crossgpu-core`
3. Add shader templates/kernels
4. Write tests for your backend
5. Update the main README with build instructions
6. Add CI configuration for your backend

## Performance Considerations

- Minimize allocations in hot paths
- Use zero-copy operations where possible
- Profile your changes with benchmarks
- Consider SIMD optimizations for CPU code

## Security

- Never commit secrets or credentials
- Validate all user inputs
- Use safe Rust practices (avoid `unsafe` unless necessary)
- Report security issues privately to maintainers

## Communication

- **Issues**: Use GitHub Issues for bug reports and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **PRs**: Keep discussions focused and constructive

## License

By contributing to CrossGPU, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

---

Thank you for contributing to CrossGPU! 🚀
