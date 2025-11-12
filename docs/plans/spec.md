# tiny-terminal (Rust) — scaffold with Matrix rain default

Below is a ready‑to‑copy scaffold you can paste into a fresh repo (or drop into `tiny-terminal/`). It ships with a default **Matrix rain** effect, project‑level overrides via `./.tiny-terminal.toml`, and a small config loader. Press `q` or `Esc` to quit.

---

## Repo layout

```
tiny-terminal/
├─ Cargo.toml
├─ README.md
├─ .gitignore
├─ .cargo/
│  └─ config.toml
├─ src/
│  ├─ main.rs
│  ├─ config.rs
│  ├─ effects/
│  │  ├─ mod.rs
│  │  └─ matrix.rs
├─ examples/
│  └─ project-config/.tiny-terminal.toml
└─ .tiny-terminal.toml        # repo-level defaults (optional)
```

---

## Cargo.toml

```toml
[package]
name = "tiny-terminal"
version = "0.1.0"
edition = "2021"
description = "Tiny, customizable terminal with a Matrix rain default effect"
license = "MIT OR Apache-2.0"
authors = ["Trevor Bowman <you@example.com>"]
repository = "https://github.com/tbowman01/tiny-terminal"

[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
crossterm = "0.27"
directories = "5"
rand = "0.8"
serde = { version = "1", features = ["derive"] }
toml = "0.8"

[features]
# Default to the Matrix effect. You can add new effects as gated features later.
default = ["effect-matrix"]

effect-matrix = []
```

---

## .gitignore

```gitignore
target/
**/*.rs.bk
.DS_Store
.idea/
.vscode/
```

---

## .cargo/config.toml (faster builds & sensible defaults)

```toml
[build]
target-dir = "target"

[term]
color = "auto"

[profile.release]
codegen-units = 1
lto = true
strip = "symbols"
```

---

## src/config.rs

```rust
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Frames per second target
    pub fps: u32,
    /// Minimum column width for characters
    pub column_width: u16,
    /// Density (0.1..2.0): larger = more drops
    pub density: f32,
    /// Character set to draw with
    pub charset: String,
    /// Use truecolor green tint when supported
    pub green: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 60,
            column_width: 2,
            density: 1.0,
            charset: "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿ012345789@#$%&*".into(),
            green: true,
        }
    }
}

impl Config {
    pub fn load(override_path: Option<PathBuf>) -> Self {
        // Priority: explicit path > project file (cwd up) > user config > defaults
        if let Some(p) = override_path {
            if let Ok(cfg) = Self::from_file(&p) { return cfg; }
        }
        if let Some(p) = find_project_file() {
            if let Ok(cfg) = Self::from_file(&p) { return cfg; }
        }
        if let Some(p) = user_config_path() {
            if let Ok(cfg) = Self::from_file(&p) { return cfg; }
        }
        Self::default()
    }

    fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let raw = fs::read_to_string(path)?;
        let cfg: Config = toml::from_str(&raw)?;
        Ok(cfg)
    }
}

fn user_config_path() -> Option<PathBuf> {
    if let Some(proj) = ProjectDirs::from("io", "ArcQubit", "tiny-terminal") {
        let p = proj.config_dir().join("config.toml");
        if p.exists() { return Some(p); }
    }
    None
}

fn find_project_file() -> Option<PathBuf> {
    // Walk up from CWD looking for .tiny-terminal.toml
    let mut cur = std::env::current_dir().ok()?;
    loop {
        let cand = cur.join(".tiny-terminal.toml");
        if cand.exists() { return Some(cand); }
        if !(cur.pop()) { break; }
    }
    None
}
```

---

## src/effects/mod.rs

```rust
#[cfg(feature = "effect-matrix")]
pub mod matrix;
```

---

## src/effects/matrix.rs

```rust
use crossterm::{cursor, queue, style::{Color, Print, ResetColor, SetForegroundColor}, terminal::{Clear, ClearType}};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crate::config::Config;

#[derive(Clone)]
struct Drop {
    x: u16,
    y: i16, // allow negative to spawn above
    speed: u16,
}

pub fn run(cfg: &Config, cancel_key: Option<char>) -> anyhow::Result<()> {
    use crossterm::terminal;
    let mut stdout = stdout();
    crossterm::execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut rng = StdRng::from_entropy();
    let mut last_resize = (0, 0);
    let mut drops: Vec<Drop> = Vec::new();

    let frame = Duration::from_millis((1000 / cfg.fps.max(1)) as u64);
    let mut last = Instant::now();

    loop {
        // Resize-aware
        let (w, h) = terminal::size()?;
        if (w as usize, h as usize) != (last_resize.0 as usize, last_resize.1 as usize) {
            last_resize = (w, h);
            drops.clear();
            queue!(stdout, Clear(ClearType::All))?;
        }

        // Seed new drops based on density
        let columns = (w / cfg.column_width.max(1)).max(1);
        let add = ((columns as f32) * cfg.density).ceil() as usize;
        for _ in 0..add {
            let col = rng.gen_range(0..columns) * cfg.column_width.max(1);
            drops.push(Drop { x: col, y: -(rng.gen_range(0..(h as u16)) as i16), speed: rng.gen_range(1..=3) });
        }

        // Draw frame
        queue!(stdout, cursor::Hide)?;
        queue!(stdout, Clear(ClearType::All))?;
        for d in drops.iter_mut() {
            d.y += d.speed as i16;
            if d.y >= 0 && (d.y as u16) < h {
                let ch = pick_char(&cfg, &mut rng);
                if cfg.green { queue!(stdout, SetForegroundColor(Color::Green))?; }
                queue!(stdout, cursor::MoveTo(d.x, d.y as u16), Print(ch))?;
                if cfg.green { queue!(stdout, ResetColor)?; }
            }
        }
        stdout.flush()?;

        // Cull off-screen
        drops.retain(|d| (d.y as u16) < h + 5);

        // Key handling (non-blocking)
        if crossterm::event::poll(Duration::from_millis(0))? {
            if let crossterm::event::Event::Key(k) = crossterm::event::read()? {
                use crossterm::event::{KeyCode, KeyModifiers};
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        if let Some(ck) = cancel_key { if c == ck { break; } }
                    }
                    KeyCode::Char('c') if k.modifiers.contains(KeyModifiers::CONTROL) => break,
                    _ => {}
                }
            }
        }

        // Frame timing
        let now = Instant::now();
        let elapsed = now - last;
        if elapsed < frame { std::thread::sleep(frame - elapsed); }
        last = now;
    }

    // Cleanup
    crossterm::execute!(stdout, cursor::Show)?;
    crossterm::execute!(stdout, terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn pick_char(cfg: &Config, rng: &mut StdRng) -> char {
    let bytes = cfg.charset.as_bytes();
    if bytes.is_empty() { return '.'; }
    let i = rng.gen_range(0..bytes.len());
    bytes[i] as char
}
```

---

## src/main.rs

```rust
mod config;
mod effects;

use clap::{Parser, ValueEnum};
use config::Config;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum Effect {
    #[cfg(feature = "effect-matrix")]
    Matrix,
}

#[derive(Parser, Debug)]
#[command(name = "tiny-terminal", version, about = "Tiny terminal with Matrix default")]
struct Args {
    /// Effect to run (defaults to Matrix)
    #[arg(long, value_enum)]
    effect: Option<Effect>,

    /// Path to a config TOML (overrides discovery)
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Override FPS
    #[arg(long)]
    fps: Option<u32>,

    /// Override density
    #[arg(long)]
    density: Option<f32>,

    /// Quit when this key is pressed
    #[arg(long)]
    cancel_key: Option<char>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut cfg = Config::load(args.config);
    if let Some(fps) = args.fps { cfg.fps = fps; }
    if let Some(d) = args.density { cfg.density = d; }

    match args.effect.unwrap_or(default_effect()) {
        #[cfg(feature = "effect-matrix")]
        Effect::Matrix => {
            effects::matrix::run(&cfg, args.cancel_key)?;
        }
    }
    Ok(())
}

#[cfg(feature = "effect-matrix")]
fn default_effect() -> Effect { Effect::Matrix }
```

---

## Root `.tiny-terminal.toml` (optional)

```toml
# Tiny Terminal defaults for this repo
fps = 75
column_width = 2
# 0.5 = sparse, 1.0 = default, 2.0 = dense
density = 1.2
charset = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉ0123456789@#$%&*"
green = true
```

---

## User config example (~/.config/tiny-terminal/config.toml)

```toml
# Applies globally unless overridden by a project .tiny-terminal.toml or --config
fps = 60
column_width = 2
density = 0.9
charset = "01┃╱╲╳*+><"
green = true
```

---

## examples/project-config/.tiny-terminal.toml

```toml
# Example: make it super dense for demos
fps = 90
column_width = 2
density = 1.75
charset = "ﾐﾅｾﾛｸｹ012345789"
```

---

## README.md (starter)

````markdown
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
````

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

```

---

## Notes for customization
- **Per-project profiles**: `.tiny-terminal.toml` discovery walks up from the current directory, so you can commit one per repo.
- **CI**: add a GitHub Action later for lint (`clippy`), audit, and release builds.
- **Cross-platform**: `crossterm` handles Windows/macOS/Linux. Truecolor may fall back to basic green if a terminal is limited.
- **Future effects**: starfield, pixel snow, clock, dashboard (via `ratatui`)—each behind feature flags.

---

Happy hacking. Press `q` to escape the Matrix. 🟩

```
