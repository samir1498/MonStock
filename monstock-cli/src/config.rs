use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct MonstockConfig {
    #[serde(default)]
    pub tauri: TauriConfig,
    #[serde(default)]
    pub egui: EguiConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub seed: SeedConfig,
}

#[derive(Debug, Deserialize)]
pub struct TauriConfig {
    #[serde(default = "default_dev_port")]
    pub dev_port: u16,
    #[serde(default = "default_dev_host")]
    pub dev_host: String,
    #[serde(default = "default_true")]
    pub webkit_disable_dmabuf: bool,
}

impl Default for TauriConfig {
    fn default() -> Self {
        Self {
            dev_port: 5173,
            dev_host: "127.0.0.1".to_string(),
            webkit_disable_dmabuf: true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct EguiConfig {
    #[serde(default = "default_true")]
    pub hot_reload: bool,
}

impl Default for EguiConfig {
    fn default() -> Self {
        Self { hot_reload: true }
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_path")]
    pub path: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: default_db_path(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SeedConfig {
    #[serde(default = "default_seed_file")]
    pub default_file: String,
}

impl Default for SeedConfig {
    fn default() -> Self {
        Self {
            default_file: default_seed_file(),
        }
    }
}

fn default_dev_port() -> u16 {
    5173
}

fn default_dev_host() -> String {
    "127.0.0.1".to_string()
}

fn default_true() -> bool {
    true
}

fn default_db_path() -> String {
    dirs::data_dir()
        .map(|p| p.join("monstock").join("monstock.db").to_string_lossy().to_string())
        .unwrap_or_else(|| "~/.local/share/monstock/monstock.db".to_string())
}

fn default_seed_file() -> String {
    "seeds/default.json".to_string()
}

pub fn load_from_file(path: &str) -> Result<MonstockConfig, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(Path::new(path))?;
    let config: MonstockConfig = toml::from_str(&contents)?;
    Ok(config)
}
