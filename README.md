# tiny-terminal

A tiny, hackable terminal toy written in Rust. Defaults to a Matrix-style rain effect.

## Quickstart

```bash
# build & run
cargo run --release

# tweak density and fps
cargo run --release -- --density 1.5 --fps 90

# point to a specific config file
cargo run --release -- --config ./examples/project-config/.tiny-terminal.toml
```

### Keys

* `q` / `Esc`: quit
* `Ctrl+C`: quit

### Project-level overrides

Put a `.tiny-terminal.toml` in your repo or any parent directory to override defaults. You can also place a user config at `~/.config/tiny-terminal/config.toml`.

### Create your own effects

Add a module in `src/effects/`, export it in `mod.rs`, add a `feature` in `Cargo.toml`, and extend the `Effect` enum in `main.rs`.

### Packaging

```bash
# release binary
cargo build --release

# strip + tiny: handled via .cargo/config.toml (LTO/strip)
```

### License

Dual-licensed under Apache-2.0 or MIT.
