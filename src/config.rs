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
            if let Ok(cfg) = Self::from_file(&p) {
                return cfg;
            }
        }
        if let Some(p) = find_project_file() {
            if let Ok(cfg) = Self::from_file(&p) {
                return cfg;
            }
        }
        if let Some(p) = user_config_path() {
            if let Ok(cfg) = Self::from_file(&p) {
                return cfg;
            }
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
        if p.exists() {
            return Some(p);
        }
    }
    None
}

fn find_project_file() -> Option<PathBuf> {
    // Walk up from CWD looking for .tiny-terminal.toml
    let mut cur = std::env::current_dir().ok()?;
    loop {
        let cand = cur.join(".tiny-terminal.toml");
        if cand.exists() {
            return Some(cand);
        }
        if !(cur.pop()) {
            break;
        }
    }
    None
}
