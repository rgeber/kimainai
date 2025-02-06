use std::sync::Arc;
use reqwest::Client;
use crate::error::AppError;
use crate::kimai::activities::Activity;
use crate::kimai::api::{get_api_url, get_request_header};
use crate::state::AppState;

pub async fn fetch_activities_from_api(app_state: Arc<AppState>) -> Result<Vec<Activity>, AppError> {
    let client = Client::new();

    let req_url = get_api_url(app_state.clone(), "activities")?;
    let headers = get_request_header(app_state.clone())?;

    let response = client
        .get(req_url)
        .headers(headers)
        .send()
        .await?;

    let mut activities: Vec<Activity> = response.json().await?;
    activities.sort_by_key(|c| c.id);

    Ok(activities)
}

pub async fn print_activities_list(app_state: Arc<AppState>, filter_project_id: Option<u32>) -> Result<(), AppError> {

    let customers = fetch_activities_from_api(app_state.clone()).await?;

    for activity in customers.iter() {
        let activity = activity.to_owned();

        if filter_project_id.is_some() && filter_project_id.unwrap() != activity.project.unwrap_or(0) {
            continue;
        }

        let activity_id = activity.id;
        let activity_name = activity.name.unwrap_or("".to_string());
        let project_id = activity.project.unwrap_or(0);

        if activity.parent_title.is_some() {
            println!("{:<width$} {project_id:<width$} {}: {activity_name}", activity_id, activity.parent_title.unwrap(), width=6);
        } else {
            println!("{:<width$} {project_id:<width$} {activity_name}", activity_id, width=6);
        }
    }

    Ok(())
}