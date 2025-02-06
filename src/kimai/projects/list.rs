use std::sync::Arc;
use reqwest::Client;
use crate::error::AppError;
use crate::kimai::api::{get_api_url, get_request_header};
use crate::kimai::projects::Project;
use crate::state::AppState;

pub async fn fetch_projects_from_api(app_state: Arc<AppState>) -> Result<Vec<Project>, AppError> {
    let client = Client::new();

    let req_url = get_api_url(app_state.clone(), "projects")?;
    let headers = get_request_header(app_state.clone())?;

    let response = client
        .get(req_url)
        .headers(headers)
        .send()
        .await?;


    let mut projects: Vec<Project> = response.json().await?;
    projects.sort_by_key(|c| c.id);

    Ok(projects)
}

pub async fn print_project_list(app_state: Arc<AppState>) -> Result<(), AppError> {

    let projects = fetch_projects_from_api(app_state.clone()).await?;

    for project in projects.iter() {

        let project = project.to_owned();

        let project_id = project.id;
        let project_name = project.name.unwrap_or("".to_string());

        if project.parent_title.is_some() {
            println!("{:<width$} {}: {project_name}", project_id, project.parent_title.unwrap(), width=6);
        } else {
            println!("{:<width$} {project_name}", project_id, width=6);
        }
    }

    Ok(())
}