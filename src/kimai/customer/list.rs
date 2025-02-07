use std::sync::Arc;
use reqwest::Client;
use crate::error::AppError;
use crate::kimai::api::{get_api_url, get_request_header};
use crate::kimai::customer::Customer;
use crate::state::AppState;

pub async fn fetch_customers_from_api(app_state: Arc<AppState>) -> Result<Vec<Customer>, AppError> {
    let client = Client::new();

    let req_url = get_api_url(app_state.clone(), "customers")?;
    let headers = get_request_header(app_state.clone())?;

    let response = client
        .get(req_url)
        .headers(headers)
        .send()
        .await?;

    let mut customers: Vec<Customer> = response.json().await?;
    customers.sort_by_key(|c| c.id);

    Ok(customers)
}

pub async fn print_customer_list(app_state: Arc<AppState>) -> Result<(), AppError> {

    let customers = fetch_customers_from_api(app_state.clone()).await?;

    for customer in customers.iter() {
        let customer_id = customer.id;
        let customer_name = customer.to_owned().name.unwrap_or("".to_string());
        println!("{:<width$} {customer_name}", customer_id, width=6);
    }

    Ok(())
}