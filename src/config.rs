use std::path::PathBuf;
use serde::Deserialize;
use crate::error::AppError;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub url: String,
    pub user: String,
    pub key: String,
}

pub fn parse_config_file(config_file_path: &PathBuf) -> Result<AppConfig, AppError> {
    let app_config_toml = std::fs::read_to_string(config_file_path)?;
    let app_config = toml::from_str(app_config_toml.as_str())?;
    Ok(app_config)
}