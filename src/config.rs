use std::path::PathBuf;
use serde::Deserialize;
use crate::error::AppError;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub api: AppConfigApi,
    pub workday: AppConfigWorkday,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfigApi {
    pub url: String,
    pub user: String,
    pub key: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfigWorkday {
    pub break_hour: u32,
    pub break_minute: u32,
    pub break_duration: u32,
}

pub fn parse_config_file(config_file_path: &PathBuf) -> Result<AppConfig, AppError> {
    let app_config_toml = std::fs::read_to_string(config_file_path)?;
    let app_config = toml::from_str(app_config_toml.as_str())?;
    Ok(app_config)
}