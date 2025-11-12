# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of tiny-terminal application
- Matrix rain effect as default terminal effect
- Hierarchical configuration system (CLI args → project config → user config → defaults)
- Cross-platform terminal handling via crossterm
- Command-line interface with clap for effect selection and config overrides
- Customizable FPS, density, column width, and character sets
- Keyboard controls: q/Esc/Ctrl+C to quit
- Extensible effect system with feature flags
- CI/CD pipeline with GitHub Actions
- Cross-platform builds (Linux, macOS, Windows)
- Automated releases with semantic versioning

## [0.1.0] - 2025-11-12

### Added
- Initial release
- Matrix rain terminal effect
- Configuration system with TOML support
- Project-level and user-level config files
- CLI argument parsing
- README and documentation
