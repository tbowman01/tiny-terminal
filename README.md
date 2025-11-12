# tiny-terminal

[![CI](https://github.com/tbowman01/tiny-terminal/workflows/CI/badge.svg)](https://github.com/tbowman01/tiny-terminal/actions)
[![Release](https://github.com/tbowman01/tiny-terminal/workflows/Release/badge.svg)](https://github.com/tbowman01/tiny-terminal/releases)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

A tiny, hackable terminal toy written in Rust. Defaults to a Matrix-style rain effect with smooth 60 FPS animation.

![Matrix Rain Effect](https://via.placeholder.com/800x400/000000/00FF00?text=Matrix+Rain+Effect)

## Features

- 🎨 **Matrix rain visual effect** with customizable parameters
- ⚙️ **Hierarchical configuration** (CLI args → project file → user config → defaults)
- 🖥️ **Cross-platform** support (Linux, macOS, Windows) via crossterm
- 🎯 **Feature-gated** modular architecture for easy extensibility
- ⚡ **Optimized builds** with LTO and symbol stripping (~1.2MB binary)
- 🎮 **Non-blocking input** with multiple exit methods

## Installation

### From Source

```bash
git clone https://github.com/tbowman01/tiny-terminal.git
cd tiny-terminal
make install
```

### Using Cargo

```bash
cargo install --path .
```

### Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/tbowman01/tiny-terminal/releases).

## Quick Start

```bash
# Build and run with defaults
cargo run --release

# Or use make
make run

# Customize density and FPS
cargo run --release -- --density 1.5 --fps 90

# Use a specific config file
cargo run --release -- --config ./examples/project-config/.tiny-terminal.toml
```

## Usage

### Command Line Options

```
Usage: tiny-terminal [OPTIONS]

Options:
      --effect <EFFECT>          Effect to run (defaults to Matrix)
  -c, --config <CONFIG>          Path to a config TOML (overrides discovery)
      --fps <FPS>                Override FPS (default: 60)
      --density <DENSITY>        Override density (default: 1.0, range: 0.1-2.0)
      --cancel-key <CANCEL_KEY>  Custom quit key
  -h, --help                     Print help
  -V, --version                  Print version
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| `q` | Quit application |
| `Esc` | Quit application |
| `Ctrl+C` | Quit application |

### Configuration

Configuration is loaded in the following priority order (highest to lowest):

1. **CLI arguments** - Runtime overrides
2. **Explicit config file** - Via `--config` flag
3. **Project config** - `.tiny-terminal.toml` in current directory or parent directories
4. **User config** - `~/.config/tiny-terminal/config.toml` (Linux/macOS) or `%APPDATA%\ArcQubit\tiny-terminal\config.toml` (Windows)
5. **Default values** - Built-in defaults

#### Configuration Options

Create a `.tiny-terminal.toml` file with the following options:

```toml
# Frames per second (default: 60)
fps = 75

# Minimum column width for characters (default: 2)
column_width = 2

# Drop density: 0.5 = sparse, 1.0 = default, 2.0 = dense
density = 1.2

# Character set to use for drops
charset = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉ0123456789@#$%&*"

# Enable green color (default: true)
green = true
```

#### Example Configurations

**Sparse and slow:**
```toml
fps = 30
density = 0.5
charset = "01"
```

**Dense Matrix style:**
```toml
fps = 90
density = 1.75
charset = "ﾐﾅｾﾛｸｹ012345789"
```

**Binary style:**
```toml
fps = 60
density = 1.0
charset = "01"
green = false
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
# Debug build
make build

# Release build (optimized)
make release

# Run tests
make test

# Run clippy
make lint

# Format code
make fmt

# Clean build artifacts
make clean
```

### Project Structure

```
tiny-terminal/
├── .cargo/
│   └── config.toml          # Build configuration
├── .github/
│   └── workflows/
│       ├── ci.yml          # Continuous Integration
│       └── release.yml     # Release automation
├── docs/
│   └── plans/
│       ├── spec.md         # Original specification
│       └── IMPLEMENTATION_PLAN.md
├── examples/
│   └── project-config/
│       └── .tiny-terminal.toml  # Example config
├── src/
│   ├── main.rs             # CLI entry point
│   ├── config.rs           # Configuration management
│   └── effects/
│       ├── mod.rs          # Effects module
│       └── matrix.rs       # Matrix rain effect
├── Cargo.toml
├── Makefile
└── README.md
```

### Creating Custom Effects

1. Create a new module in `src/effects/` (e.g., `starfield.rs`)
2. Add feature flag to `Cargo.toml`:
   ```toml
   [features]
   default = ["effect-matrix"]
   effect-starfield = []
   ```
3. Export module in `src/effects/mod.rs`:
   ```rust
   #[cfg(feature = "effect-starfield")]
   pub mod starfield;
   ```
4. Add variant to `Effect` enum in `src/main.rs`:
   ```rust
   #[derive(Debug, Clone, ValueEnum)]
   enum Effect {
       #[cfg(feature = "effect-matrix")]
       Matrix,
       #[cfg(feature = "effect-starfield")]
       Starfield,
   }
   ```
5. Implement the `run` function matching the signature:
   ```rust
   pub fn run(cfg: &Config, cancel_key: Option<char>) -> anyhow::Result<()>
   ```

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run with make
make test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards

- Follow Rust naming conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add tests for new functionality
- Update documentation as needed

## Performance

- **Binary Size**: ~1.2MB (stripped, with LTO)
- **Memory Usage**: ~2-5MB during runtime
- **CPU Usage**: <5% on modern hardware at 60 FPS
- **Frame Rate**: Configurable, default 60 FPS

## Troubleshooting

### Terminal doesn't restore properly after crash

The application should handle cleanup automatically, but if something goes wrong:

```bash
# Reset terminal
reset

# Or
stty sane
```

### Colors not displaying correctly

Some terminals have limited color support. Try:
- Using a modern terminal emulator (iTerm2, Alacritty, Windows Terminal)
- Checking `TERM` environment variable is set correctly
- Setting `green = false` in config to disable color

### Performance issues

If the animation is choppy:
- Lower the FPS: `--fps 30`
- Reduce density: `--density 0.5`
- Close other resource-intensive applications
- Ensure you're running the release build (`cargo run --release`)

## License

Dual-licensed under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

You may choose either license for your use.

## Acknowledgments

- Inspired by the classic Matrix digital rain effect
- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation
- Uses [clap](https://github.com/clap-rs/clap) for elegant CLI argument parsing

## Roadmap

- [ ] Additional effects (starfield, snow, fire)
- [ ] Color scheme customization
- [ ] Recording/playback functionality
- [ ] Interactive mode with runtime controls
- [ ] Performance metrics display
- [ ] Plugin system for external effects

---

**Happy hacking!** Press `q` to escape the Matrix. 🟩
