use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub player: PlayerCongig,
    pub spectrum: SpectrumConfig,
    pub appearance: AppearanceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player: PlayerCongig::default(),
            spectrum: SpectrumConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct PlayerCongig {
    pub volume: f32,
    pub shuffle: bool,
    pub repeat: bool,
}

impl Default for PlayerCongig {
    fn default() -> Self {
        Self {
            volume: 0.8,
            shuffle: false,
            repeat: false,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SpectrumConfig {
    pub enabled: bool,
}

impl Default for SpectrumConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub color: String,
    pub show_cover: bool,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            color: "cyan".to_string(),
            show_cover: true,
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new("config.yaml");
    println!("Path: {}", path.display());

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;

    println!("{:#?}", config);

    Ok(config)
}
