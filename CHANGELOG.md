# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Calendar Versioning](https://calver.org/) using the format `YYYY.MM.MICRO`.

## [Unreleased]

### Added
- Comprehensive test suite with 12 unit tests and 9 integration tests
- Detailed testing documentation in README
- Calendar versioning (CalVer) using YYYY.MM.MICRO format

### Changed
- Migrated from semantic versioning to calendar versioning
- Enhanced documentation with testing guidelines and coverage information

## [2025.11.0] - 2025-11-12

### Added
- Initial release of tiny-terminal
- Matrix rain visual effect with smooth 60 FPS animation
- Hierarchical configuration system supporting multiple sources:
  - CLI arguments for runtime overrides
  - Explicit config file via `--config` flag
  - Project-level `.tiny-terminal.toml` (walks up directory tree)
  - User config at `~/.config/tiny-terminal/config.toml`
  - Built-in sensible defaults
- Cross-platform terminal manipulation using crossterm
- Feature-gated modular effects architecture for extensibility
- Non-blocking input handling with multiple exit methods:
  - `q` key
  - `Esc` key
  - `Ctrl+C`
  - Custom cancel key via CLI
- Terminal resize detection and automatic adaptation
- Optimized release builds with LTO and symbol stripping
- Configuration options:
  - `fps`: Frames per second (default: 60)
  - `column_width`: Character column width (default: 2)
  - `density`: Drop density 0.1-2.0 (default: 1.0)
  - `charset`: Customizable character set
  - `green`: Enable/disable green color tinting
- Example configurations:
  - Root-level project config
  - Dense demo configuration in examples/
- Comprehensive README with:
  - Installation instructions
  - Usage examples
  - Configuration guide
  - Development setup
  - Custom effect creation guide
  - Troubleshooting section
- Project documentation:
  - Original specification (docs/plans/spec.md)
  - Implementation plan (docs/plans/IMPLEMENTATION_PLAN.md)
- Build optimizations via .cargo/config.toml:
  - Link-time optimization (LTO)
  - Symbol stripping
  - Single codegen unit for release

### Technical Details
- Written in Rust (Edition 2021)
- Dependencies:
  - `crossterm` ^0.27 - Cross-platform terminal manipulation
  - `clap` ^4 - Command-line argument parsing
  - `rand` ^0.8 - Random number generation
  - `serde` ^1 - Serialization framework
  - `toml` ^0.8 - TOML configuration parsing
  - `directories` ^5 - Platform-specific directory paths
  - `anyhow` ^1 - Error handling
- Binary size: ~1.2MB (stripped, release build)
- Memory usage: ~2-5MB during runtime
- CPU usage: <5% on modern hardware at 60 FPS
- Clean clippy linting (no warnings with -D warnings)
- Supports Linux, macOS, and Windows

### Performance
- Efficient drop spawning and culling algorithms
- Configurable frame rate for performance tuning
- Minimal memory footprint
- Low CPU usage even at high FPS

[Unreleased]: https://github.com/tbowman01/tiny-terminal/compare/v2025.11.0...HEAD
[2025.11.0]: https://github.com/tbowman01/tiny-terminal/releases/tag/v2025.11.0
