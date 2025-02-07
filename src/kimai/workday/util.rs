use std::sync::Arc;
use crate::args::FileWorkdayArgs;
use crate::error::AppError;
use crate::kimai::user::me::get_my_user_id;
use crate::state::AppState;

pub async fn get_user_id(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<u32, AppError> {
    let user_id: u32 = match args.user_id {
        Some(id) => id,
        None => get_my_user_id(app_state.clone()).await?
    };

    Ok(user_id)
}

pub fn get_project_id(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<u32, AppError> {
    let arg_id = args.project_id.unwrap_or(0);
    let config_id = app_state.app_config.defaults.project_id;

    if arg_id == 0 && config_id == 0 {
        return Err(AppError::ConfigError("Project ID cannot be 0".to_string()))
    }

    if arg_id > 0 {
        Ok(arg_id)
    } else {
        Ok(config_id)
    }
}

pub fn get_customer_id(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<u32, AppError> {
    let arg_id = args.customer_id.unwrap_or(0);
    let config_id = app_state.app_config.defaults.customer_id;

    if arg_id == 0 && config_id == 0 {
        return Err(AppError::ConfigError("Customer ID cannot be 0".to_string()))
    }

    if arg_id > 0 {
        Ok(arg_id)
    } else {
        Ok(config_id)
    }
}

pub fn get_activity_id(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<u32, AppError> {
    let arg_id = args.activity_id.unwrap_or(0);
    let config_id = app_state.app_config.defaults.activity_id;

    if arg_id == 0 && config_id == 0 {
        return Err(AppError::ConfigError("Activity ID cannot be 0".to_string()))
    }

    if arg_id > 0 {
        Ok(arg_id)
    } else {
        Ok(config_id)
    }
}

pub fn has_break(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<bool, AppError> {
    let has_break_arg = args.break_start_hour < 24 && args.break_start_minute < 60 && args.break_length > 0;
    let has_break_config = app_state.app_config.workday.break_hour < 24 && app_state.app_config.workday.break_minute < 60 && app_state.app_config.workday.break_duration > 0;

    let has_break = has_break_arg || has_break_config;
    Ok(has_break)
}