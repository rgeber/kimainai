use crate::args::Args;
use crate::config::AppConfig;

pub struct AppState {
    pub args: Args,
    pub app_config: AppConfig,
}