//! Persistent configuration management with atomic file saving and schema validation.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tiny68_core::types::{PerformanceProfile, PersonalityType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConfig {
    pub instance_id: u64,
    pub pack_id: String,
    pub display_name: String,
    pub personality: PersonalityType,
    pub scale: f32,
    pub initial_x: Option<f32>,
    pub initial_y: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub schema_version: u32,
    pub performance_profile: PerformanceProfile,
    pub sound_enabled: bool,
    pub auto_start_with_windows: bool,
    pub start_minimized: bool,
    pub pause_on_fullscreen_game: bool,
    pub characters: Vec<CharacterConfig>,
    pub target_fps_override: Option<u32>,
    pub click_through_enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            performance_profile: PerformanceProfile::Balanced,
            sound_enabled: false,
            auto_start_with_windows: false,
            start_minimized: false,
            pause_on_fullscreen_game: true,
            characters: vec![CharacterConfig {
                instance_id: 1,
                pack_id: "mochi".to_string(),
                display_name: "Mochi".to_string(),
                personality: PersonalityType::Playful,
                scale: 1.0,
                initial_x: None,
                initial_y: None,
            }],
            target_fps_override: None,
            click_through_enabled: true,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(app_name: &str) -> Self {
        let base_dir = dirs_fallback(app_name);
        Self {
            config_path: base_dir.join("config.json"),
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    pub fn load(&self) -> AppConfig {
        if !self.config_path.exists() {
            let default_cfg = AppConfig::default();
            let _ = self.save(&default_cfg);
            return default_cfg;
        }

        match std::fs::read_to_string(&self.config_path) {
            Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
                Ok(cfg) => cfg,
                Err(e) => {
                    tracing::warn!("Failed to parse config, falling back to default: {}", e);
                    AppConfig::default()
                }
            },
            Err(e) => {
                tracing::warn!("Failed to read config file: {}", e);
                AppConfig::default()
            }
        }
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), std::io::Error> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let tmp_path = self.config_path.with_extension("tmp");
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        std::fs::write(&tmp_path, json)?;
        std::fs::rename(&tmp_path, &self.config_path)?;
        Ok(())
    }
}

fn dirs_fallback(app_name: &str) -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local_app_data).join(app_name)
    } else {
        PathBuf::from(".").join(".config").join(app_name)
    }
}
