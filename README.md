# tiny-terminal

[![CI](https://github.com/tbowman01/tiny-terminal/workflows/CI/badge.svg)](https://github.com/tbowman01/tiny-terminal/actions)

A tiny, hackable terminal toy written in Rust. Defaults to a Matrix-style rain effect.

## Installation

### From Release (recommended)

Download the latest binary for your platform from [Releases](https://github.com/tbowman01/tiny-terminal/releases).

### From Source

```bash
cargo install tiny-terminal
```

Or clone and build:

```bash
git clone https://github.com/tbowman01/tiny-terminal.git
cd tiny-terminal
cargo build --release
```

## Quickstart

```bash
# run with defaults
tiny-terminal

# tweak density and fps
tiny-terminal --density 1.5 --fps 90

# point to a specific config file
tiny-terminal --config ./examples/project-config/.tiny-terminal.toml
```

### Development

```bash
# build & run from source
cargo run --release

# with options
cargo run --release -- --density 1.5 --fps 90
```

### Keys

* `q` / `Esc`: quit
* `Ctrl+C`: quit

### Project-level overrides

Put a `.tiny-terminal.toml` in your repo or any parent directory to override defaults. You can also place a user config at `~/.config/tiny-terminal/config.toml`.

### Create your own effects

Add a module in `src/effects/`, export it in `mod.rs`, add a `feature` in `Cargo.toml`, and extend the `Effect` enum in `main.rs`.

## CI/CD & Releases

This project uses GitHub Actions for continuous integration and automated releases:

- **CI Pipeline**: Runs on every PR and push
  - Build and test on Linux, macOS, and Windows
  - Clippy linting
  - Rustfmt formatting checks
  - Security audit with cargo-audit

- **Release Pipeline**: Triggered by version tags (e.g., `v0.1.0`)
  - Builds optimized binaries for multiple platforms
  - Creates GitHub Release with artifacts
  - Publishes to crates.io

See [RELEASING.md](RELEASING.md) for details on the release process and semantic versioning.

## License

Dual-licensed under Apache-2.0 or MIT.
