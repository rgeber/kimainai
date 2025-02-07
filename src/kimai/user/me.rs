use std::sync::Arc;
use reqwest::Client;
use crate::error::AppError;
use crate::kimai::api::{get_api_url, get_request_header};
use crate::kimai::user::User;
use crate::state::AppState;

pub async fn fetch_user_me_from_api(app_state: Arc<AppState>) -> Result<User, AppError> {
    let client = Client::new();

    let req_url = get_api_url(app_state.clone(), "users/me")?;
    let headers = get_request_header(app_state.clone())?;

    let response = client
        .get(req_url)
        .headers(headers)
        .send()
        .await?;

    let user: User = response.json().await?;

    Ok(user)
}

pub async fn get_my_user_id(app_state: Arc<AppState>) -> Result<u32, AppError> {
    let user = fetch_user_me_from_api(app_state.clone()).await?;
    Ok(user.id)
}

pub async fn print_user_me(app_state: Arc<AppState>) -> Result<(), AppError> {

    let user = fetch_user_me_from_api(app_state.clone()).await?;

    //dbg!(&user);

    let user_id = user.id;
    let username = user.username.unwrap_or("".to_string());
    let alias = user.alias.unwrap_or("".to_string());

    println!("{user_id:<width$} {username} / {alias}", width=6);

    Ok(())
}