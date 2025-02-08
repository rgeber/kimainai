use std::sync::Arc;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::error::AppError;
use crate::state::AppState;

pub fn get_request_header(app_state: Arc<AppState>) -> Result<HeaderMap, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert("X-AUTH-USER", HeaderValue::from_str(app_state.app_config.api.user.as_str())?);
    headers.insert("X-AUTH-TOKEN", HeaderValue::from_str(app_state.app_config.api.key.as_str())?);
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub fn get_api_url(app_state: Arc<AppState>, endpoint: &str) -> Result<String, AppError> {
    let api_base_url = app_state.app_config.api.url.as_str();
    Ok(format!("{api_base_url}/{endpoint}"))
}