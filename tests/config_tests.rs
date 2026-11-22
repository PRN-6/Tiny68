use std::path::PathBuf;
use tiny68_config::{AppConfig, ConfigManager};
use tiny68_core::types::PerformanceProfile;

#[test]
fn test_config_atomic_save_and_load() {
    let temp_dir = std::env::temp_dir().join("tiny68_test_cfg");
    let cfg_file = temp_dir.join("test_config.json");
    let mgr = ConfigManager::with_path(cfg_file.clone());

    let mut cfg = AppConfig::default();
    cfg.performance_profile = PerformanceProfile::Smooth;
    cfg.auto_start_with_windows = true;

    assert!(mgr.save(&cfg).is_ok());

    let loaded = mgr.load();
    assert_eq!(loaded.performance_profile, PerformanceProfile::Smooth);
    assert!(loaded.auto_start_with_windows);

    // Cleanup
    let _ = std::fs::remove_dir_all(temp_dir);
}

#[test]
fn test_config_fallback_on_corrupt_file() {
    let temp_dir = std::env::temp_dir().join("tiny68_corrupt_cfg");
    let cfg_file = temp_dir.join("corrupt.json");
    let _ = std::fs::create_dir_all(&temp_dir);
    let _ = std::fs::write(&cfg_file, "{ invalid json content ]");

    let mgr = ConfigManager::with_path(cfg_file);
    let loaded = mgr.load();

    assert_eq!(loaded.schema_version, 1, "Should gracefully recover with default config");

    let _ = std::fs::remove_dir_all(temp_dir);
}

