use config::{Config, File, FileFormat};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log: LogConfig,
    pub tauri: TauriConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct TauriConfig {
    pub bindings_output_path: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = Config::builder().add_source(File::new("settings", FileFormat::Toml));
        builder.build()?.try_deserialize()
    }
}

pub static CONFIG: Lazy<Settings> =
    Lazy::new(|| Settings::new().expect("Failed to initialize settings"));
