use std::sync::Arc;
use chrono::{DateTime, Local};
use reqwest::Client;
use crate::args::FileWorkdayArgs;
use crate::error::AppError;
use crate::error::AppError::TimeSheetTransmissionError;
use crate::kimai::api::{get_api_url, get_request_header};
use crate::kimai::timesheet::{TimeSheetResponse, TimeSheetPostable};
use crate::kimai::workday::datetime::{get_post_break_time_start_time, get_workday_break_time, get_workday_end_time, get_workday_start_time};
use crate::kimai::workday::util::{get_activity_id, get_project_id, has_break};
use crate::state::AppState;

pub async fn post_timesheet(app_state: Arc<AppState>, args: &FileWorkdayArgs, time_sheet: &TimeSheetPostable) -> Result<TimeSheetResponse, AppError> {

    let client = Client::new();

    let req_url = get_api_url(app_state.clone(), "timesheets")?;
    let headers = get_request_header(app_state.clone())?;

    let response = client
        .post(&req_url)
        .headers(headers)
        .json(&time_sheet)
        .send()
        .await?;

    if response.status().is_success() {
        let response_timesheet: TimeSheetResponse = response.json().await?;
        Ok(response_timesheet)
    } else {
        let msg = format!("Timesheet transmission status code: `{}`. Url: {}", response.status().to_string(), &req_url);
        Err(TimeSheetTransmissionError(msg))
    }

}

pub async fn file_work_day(app_state: Arc<AppState>, args: FileWorkdayArgs) -> Result<(), AppError> {

    let day_has_break = has_break(app_state.clone(), &args)?;
    let day_start_time = get_workday_start_time(app_state.clone(), &args)?;

    let break_time: Option<DateTime<Local>> = match day_has_break {
        true => Some(get_workday_break_time(app_state.clone(), &args)?),
        false => None
    };

    let post_break_start_time = match day_has_break {
        true => Some(get_post_break_time_start_time(app_state.clone(), &args, break_time.unwrap().clone())?),
        false => None
    };

    let day_end_time = get_workday_end_time(
        app_state.clone(),
        &args,
        day_start_time.clone(),
        break_time.clone(),
        post_break_start_time.clone()
    )?;

    let project = get_project_id(app_state.clone(), &args)?;
    let activity = get_activity_id(app_state.clone(), &args)?;

let tags:String = "".to_string();

    if day_has_break {

        let description = args.description.clone().unwrap_or("".to_string());
        let mut timesheet = TimeSheetPostable {
            begin: day_start_time,
            end: break_time.unwrap(),
            project,
            activity,
            description,
            tags,
        };

        let _ = post_timesheet(app_state.clone(), &args, &timesheet).await?;

        timesheet.begin = post_break_start_time.unwrap();
        timesheet.end = day_end_time;

        let _ = post_timesheet(app_state.clone(), &args, &timesheet).await?;

    } else {
        let description = args.description.clone().unwrap_or("".to_string()).to_owned();
        let timesheet = TimeSheetPostable {
            begin: day_start_time,
            end: day_end_time,
            project,
            activity,
            description,
            tags,
        };

        let _ = post_timesheet(app_state.clone(), &args, &timesheet).await?;
    }


    Ok(())

}