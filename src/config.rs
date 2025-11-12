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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert_eq!(cfg.fps, 60);
        assert_eq!(cfg.column_width, 2);
        assert_eq!(cfg.density, 1.0);
        assert_eq!(cfg.charset, "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿ012345789@#$%&*");
        assert!(cfg.green);
    }

    #[test]
    fn test_config_serialization() {
        let cfg = Config::default();
        let toml_str = toml::to_string(&cfg).unwrap();
        assert!(toml_str.contains("fps = 60"));
        assert!(toml_str.contains("density = 1.0"));
        assert!(toml_str.contains("green = true"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            fps = 30
            column_width = 3
            density = 0.5
            charset = "01"
            green = false
        "#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.fps, 30);
        assert_eq!(cfg.column_width, 3);
        assert_eq!(cfg.density, 0.5);
        assert_eq!(cfg.charset, "01");
        assert!(!cfg.green);
    }

    #[test]
    fn test_config_partial_deserialization() {
        // Test that partial config with missing fields uses serde defaults
        let toml_str = r#"
            fps = 90
        "#;
        // This will fail without #[serde(default)] attributes
        // For now, we expect all fields to be present or use serde defaults
        let result: Result<Config, _> = toml::from_str(toml_str);
        assert!(result.is_err(), "Partial config should fail without serde defaults");
    }

    #[test]
    fn test_from_file_valid() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_config_valid.toml");

        let toml_content = r#"
fps = 45
column_width = 4
density = 1.5
charset = "ABC123"
green = true
"#;
        let mut file = fs::File::create(&temp_file).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        let cfg = Config::from_file(&temp_file).unwrap();
        assert_eq!(cfg.fps, 45);
        assert_eq!(cfg.column_width, 4);
        assert_eq!(cfg.density, 1.5);
        assert_eq!(cfg.charset, "ABC123");
        assert!(cfg.green);

        // Cleanup
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_from_file_invalid_toml() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_config_invalid.toml");

        let invalid_content = "this is not valid toml {{[";
        let mut file = fs::File::create(&temp_file).unwrap();
        file.write_all(invalid_content.as_bytes()).unwrap();

        let result = Config::from_file(&temp_file);
        assert!(result.is_err(), "Invalid TOML should return an error");

        // Cleanup
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_from_file_nonexistent() {
        let nonexistent = PathBuf::from("/tmp/this_file_does_not_exist_12345.toml");
        let result = Config::from_file(&nonexistent);
        assert!(result.is_err(), "Nonexistent file should return an error");
    }

    #[test]
    fn test_load_with_explicit_path() {
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("test_explicit.toml");

        let toml_content = r#"
fps = 120
column_width = 1
density = 2.0
charset = "XYZ"
green = false
"#;
        let mut file = fs::File::create(&temp_file).unwrap();
        file.write_all(toml_content.as_bytes()).unwrap();

        let cfg = Config::load(Some(temp_file.clone()));
        assert_eq!(cfg.fps, 120);
        assert_eq!(cfg.density, 2.0);
        assert_eq!(cfg.charset, "XYZ");
        assert!(!cfg.green);

        // Cleanup
        fs::remove_file(&temp_file).ok();
    }

    #[test]
    fn test_load_with_invalid_path_falls_back() {
        let nonexistent = PathBuf::from("/tmp/nonexistent_config_xyz_99999.toml");
        let cfg = Config::load(Some(nonexistent));

        // Should fall back to project config or defaults
        // The config should be valid even if we can't predict exact values
        assert!(cfg.fps > 0);
        assert!(cfg.column_width > 0);
        assert!(cfg.density > 0.0);
        assert!(!cfg.charset.is_empty());
    }

    #[test]
    fn test_load_without_explicit_path_uses_default() {
        // This test assumes no project or user config exists
        // In a real environment, this might find actual configs
        let cfg = Config::load(None);

        // At minimum, the config should be valid
        assert!(cfg.fps > 0);
        assert!(cfg.column_width > 0);
        assert!(cfg.density > 0.0);
        assert!(!cfg.charset.is_empty());
    }

    #[test]
    fn test_config_clone() {
        let cfg1 = Config::default();
        let cfg2 = cfg1.clone();
        assert_eq!(cfg1.fps, cfg2.fps);
        assert_eq!(cfg1.density, cfg2.density);
        assert_eq!(cfg1.charset, cfg2.charset);
    }

    #[test]
    fn test_config_debug() {
        let cfg = Config::default();
        let debug_str = format!("{:?}", cfg);
        assert!(debug_str.contains("fps"));
        assert!(debug_str.contains("density"));
    }
}
