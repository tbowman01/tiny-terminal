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
#[command(
    name = "tiny-terminal",
    version,
    about = "Tiny terminal with Matrix default"
)]
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
    if let Some(fps) = args.fps {
        cfg.fps = fps;
    }
    if let Some(d) = args.density {
        cfg.density = d;
    }

    match args.effect.unwrap_or(default_effect()) {
        #[cfg(feature = "effect-matrix")]
        Effect::Matrix => {
            effects::matrix::run(&cfg, args.cancel_key)?;
        }
    }
    Ok(())
}

#[cfg(feature = "effect-matrix")]
fn default_effect() -> Effect {
    Effect::Matrix
}
