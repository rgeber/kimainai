use std::sync::Arc;
use crate::args::FileWorkdayArgs;
use crate::error::AppError;
use crate::state::AppState;


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

    // When arg sets break time to 0 disable break completely
    if args.break_duration.is_some() && &args.break_duration.unwrap() == &0 {
        return Ok(false)
    }

    let break_duration = match args.break_duration {
        Some(value) => value,
        None => app_state.app_config.workday.break_duration,
    };

    Ok(break_duration > 0)
}

pub fn get_break_duration(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> u32 {
    match args.break_duration {
        Some(value) => value,
        None => app_state.app_config.workday.break_duration
    }
}

pub fn get_workday_duration_hours(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> u32 {
    match args.duration_hours {
        Some(value) => value,
        None => app_state.app_config.workday.duration_hours
    }
}

pub fn get_workday_duration_minutes(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> u32 {
    match args.duration_minutes {
        Some(value) => value,
        None => app_state.app_config.workday.duration_minutes
    }
}