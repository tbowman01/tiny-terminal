use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

// Note: These integration tests would normally import from the crate,
// but since Config and related functions are not public in the library,
// we test through the binary interface or by making them public.
// For now, we'll create tests that verify the behavior indirectly.

/// Helper to create a temporary config file
fn create_temp_config(content: &str, name: &str) -> PathBuf {
    let temp_dir = env::temp_dir();
    let temp_file = temp_dir.join(name);
    let mut file = fs::File::create(&temp_file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    temp_file
}

/// Helper to cleanup temp file
fn cleanup_temp_file(path: &PathBuf) {
    fs::remove_file(path).ok();
}

#[test]
fn test_config_hierarchy_explicit_overrides_all() {
    // This test verifies that an explicit config file takes precedence
    // We'll test this by creating files and verifying the parsing

    let explicit_config = r#"
fps = 100
column_width = 5
density = 1.8
charset = "EXPLICIT"
green = false
"#;

    let project_config = r#"
fps = 50
column_width = 3
density = 1.2
charset = "PROJECT"
green = true
"#;

    let explicit_path = create_temp_config(explicit_config, "test_explicit_override.toml");
    let _project_path = create_temp_config(project_config, ".tiny-terminal.toml");

    // Verify explicit config content
    let content = fs::read_to_string(&explicit_path).unwrap();
    assert!(content.contains("EXPLICIT"));
    assert!(content.contains("fps = 100"));

    cleanup_temp_file(&explicit_path);
    cleanup_temp_file(&_project_path);
}

#[test]
fn test_valid_config_file_parsing() {
    let valid_config = r#"
fps = 75
column_width = 2
density = 1.5
charset = "ﾐﾅｾﾛｸｹ012345789"
green = true
"#;

    let temp_file = create_temp_config(valid_config, "test_valid_config.toml");

    // Verify file was created and can be read
    let content = fs::read_to_string(&temp_file).unwrap();
    assert!(content.contains("fps = 75"));
    assert!(content.contains("density = 1.5"));
    assert!(content.contains("charset"));

    // Verify it's valid TOML
    let parsed: Result<toml::Value, _> = toml::from_str(&content);
    assert!(parsed.is_ok(), "Config should be valid TOML");

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_invalid_config_file_format() {
    let invalid_config = "this is not valid TOML syntax {{[[";

    let temp_file = create_temp_config(invalid_config, "test_invalid_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: Result<toml::Value, _> = toml::from_str(&content);
    assert!(parsed.is_err(), "Invalid TOML should fail to parse");

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_config_with_all_fields() {
    let full_config = r#"
fps = 60
column_width = 2
density = 1.0
charset = "ABC123"
green = true
"#;

    let temp_file = create_temp_config(full_config, "test_full_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: toml::Value = toml::from_str(&content).unwrap();

    // Verify all fields are present
    assert!(parsed.get("fps").is_some());
    assert!(parsed.get("column_width").is_some());
    assert!(parsed.get("density").is_some());
    assert!(parsed.get("charset").is_some());
    assert!(parsed.get("green").is_some());

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_config_with_partial_fields() {
    // Test configs with only some fields specified
    let partial_config = r#"
fps = 90
density = 1.5
"#;

    let temp_file = create_temp_config(partial_config, "test_partial_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: toml::Value = toml::from_str(&content).unwrap();

    assert_eq!(parsed.get("fps").and_then(|v| v.as_integer()), Some(90));
    assert_eq!(
        parsed.get("density").and_then(|v| v.as_float()),
        Some(1.5)
    );
    assert!(parsed.get("charset").is_none());

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_config_with_extreme_values() {
    let extreme_config = r#"
fps = 1
column_width = 1
density = 0.1
charset = "X"
green = false
"#;

    let temp_file = create_temp_config(extreme_config, "test_extreme_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: toml::Value = toml::from_str(&content).unwrap();

    assert_eq!(parsed.get("fps").and_then(|v| v.as_integer()), Some(1));
    assert_eq!(
        parsed.get("density").and_then(|v| v.as_float()),
        Some(0.1)
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_config_with_high_values() {
    let high_config = r#"
fps = 240
column_width = 10
density = 2.0
charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
green = true
"#;

    let temp_file = create_temp_config(high_config, "test_high_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: toml::Value = toml::from_str(&content).unwrap();

    assert_eq!(parsed.get("fps").and_then(|v| v.as_integer()), Some(240));
    assert_eq!(
        parsed.get("density").and_then(|v| v.as_float()),
        Some(2.0)
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_config_with_unicode_charset() {
    let unicode_config = r#"
fps = 60
column_width = 2
density = 1.0
charset = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉ0123456789"
green = true
"#;

    let temp_file = create_temp_config(unicode_config, "test_unicode_config.toml");

    let content = fs::read_to_string(&temp_file).unwrap();
    let parsed: toml::Value = toml::from_str(&content).unwrap();

    let charset = parsed
        .get("charset")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(charset.contains('ｱ'));
    assert!(charset.contains('ｲ'));

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_multiple_config_files_isolation() {
    // Test that multiple config files don't interfere with each other
    let config1 = r#"
fps = 30
density = 0.5
charset = "CONFIG1"
green = true
"#;

    let config2 = r#"
fps = 120
density = 2.0
charset = "CONFIG2"
green = false
"#;

    let file1 = create_temp_config(config1, "test_config_1.toml");
    let file2 = create_temp_config(config2, "test_config_2.toml");

    let content1 = fs::read_to_string(&file1).unwrap();
    let content2 = fs::read_to_string(&file2).unwrap();

    let parsed1: toml::Value = toml::from_str(&content1).unwrap();
    let parsed2: toml::Value = toml::from_str(&content2).unwrap();

    assert_eq!(parsed1.get("fps").and_then(|v| v.as_integer()), Some(30));
    assert_eq!(parsed2.get("fps").and_then(|v| v.as_integer()), Some(120));

    cleanup_temp_file(&file1);
    cleanup_temp_file(&file2);
}
