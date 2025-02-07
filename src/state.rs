use crate::args::AppArgs;
use crate::config::AppConfig;

pub struct AppState {
    pub args: AppArgs,
    pub app_config: AppConfig,
}