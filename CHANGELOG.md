# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project template setup
- Core tensor operations and GPU abstraction layer
- 5 GPU backend implementations (WebGPU, Vulkan, Metal, DX12, CPU)
- Transformer model architecture with tiny configuration
- 8-bit and 4-bit quantization support
- WASM package for browser deployment
- Comprehensive documentation (10+ guides)
- CI/CD pipeline with GitHub Actions
- Example applications (simple-inference, complete-workflow)
- Development tooling (Makefile, Docker, VS Code settings)
- Deployment scripts for WASM and native builds
- Benchmarking infrastructure

### Changed

- N/A (initial release)

### Deprecated

- N/A

### Removed

- N/A

### Fixed

- N/A

### Security

- N/A

## [0.1.0] - TBD

### Added

- Initial release
- Multi-backend GPU support
- Transformer inference engine
- Model quantization
- Cross-platform support

---

## Release Process

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md` with release date
3. Commit changes: `git commit -am "Release v0.1.0"`
4. Create tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
5. Push: `git push && git push --tags`
6. Create GitHub release from tag
7. Publish crates: `cargo publish -p crossgpu-core`

## Version Guidelines

- **Major (X.0.0)**: Breaking API changes
- **Minor (0.X.0)**: New features, backward compatible
- **Patch (0.0.X)**: Bug fixes, backward compatible

## Categories

- **Added**: New features
- **Changed**: Changes to existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security fixes
