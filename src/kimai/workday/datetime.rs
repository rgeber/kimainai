use std::sync::Arc;
use crate::args::FileWorkdayArgs;
use crate::error::AppError;
use crate::state::AppState;
use chrono::prelude::*;
use chrono::TimeDelta;
use crate::kimai::workday::util::{get_break_duration, get_workday_duration_hours, get_workday_duration_minutes};

pub fn get_workday_start_time(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<DateTime<Local>, AppError> {

    let now = Local::now();

    let start_hour = match args.start_time_hour {
        Some(hour) => hour,
        None => app_state.app_config.workday.start_hour
    };

    let start_minute = match args.start_time_minute {
        Some(minute) => minute,
        None => app_state.app_config.workday.start_minute
    };

    let start_day = match args.start_time_day {
        Some(day) => day,
        None => now.day()
    };

    let start_month = match args.start_time_month {
        Some(month) => month,
        None => now.month()
    };

    let start_year = match args.start_time_year {
        Some(year) => year as i32,
        None => now.year()
    };

    let workday_start_time = Local.with_ymd_and_hms(
        start_year,
        start_month,
        start_day,
        start_hour,
        start_minute,
        0
    ).unwrap();

    Ok(workday_start_time)
}

pub fn get_workday_break_time(app_state: Arc<AppState>, args: &FileWorkdayArgs) -> Result<DateTime<Local>, AppError> {

    let now = Local::now();

    let start_hour = match args.break_start_hour {
        Some(hour) => hour,
        None => app_state.app_config.workday.break_hour
    };

    let start_minute = match args.break_start_minute {
        Some(minute) => minute,
        None => app_state.app_config.workday.break_minute
    };

    let start_day = match args.start_time_day {
        Some(day) => day,
        None => now.day()
    };

    let start_month = match args.start_time_month {
        Some(month) => month,
        None => now.month()
    };

    let start_year = match args.start_time_year {
        Some(year) => year as i32,
        None => now.year()
    };

    let workday_start_time = Local.with_ymd_and_hms(
        start_year,
        start_month,
        start_day,
        start_hour,
        start_minute,
        0
    ).unwrap();

    Ok(workday_start_time)
}

pub fn get_post_break_time_start_time(app_state: Arc<AppState>, args: &FileWorkdayArgs, break_time: DateTime<Local>) -> Result<DateTime<Local>, AppError>{

    let break_duration = get_break_duration(app_state.clone(), &args) as i64;
    let break_td = TimeDelta::try_minutes(break_duration).unwrap();

    Ok(break_time + break_td)
}

pub fn get_workday_end_time(
    app_state: Arc<AppState>,
    args: &FileWorkdayArgs,
    day_start_time: DateTime<Local>,
    break_time: Option<DateTime<Local>>,
    post_break_start_time: Option<DateTime<Local>>
) -> Result<DateTime<Local>, AppError> {
    let workday_hours = get_workday_duration_hours(app_state.clone(), &args);
    let workday_minutes = get_workday_duration_minutes(app_state.clone(), &args);

    let worktime_minutes = (workday_hours * 60) + workday_minutes;


    if break_time.is_some() {
        let pre_break_work_minutes = (break_time.unwrap() - day_start_time).num_minutes();
        let remaining_work_minutes = (worktime_minutes as i64) - pre_break_work_minutes;
        let remaining_work_td = TimeDelta::try_minutes(remaining_work_minutes).unwrap();
        Ok(post_break_start_time.unwrap() + remaining_work_td)

    } else {
        let workday_td = TimeDelta::try_minutes(worktime_minutes as i64).unwrap();
        Ok(day_start_time + workday_td)
    }
}