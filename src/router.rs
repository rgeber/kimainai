use std::sync::Arc;
use reqwest::Response;
use crate::error::AppError;
use crate::kimai::customer::Customer;
use crate::state::AppState;

pub async fn cli_router(app_state: Arc<AppState>) -> Result<(), AppError> {

    match &app_state.args.command {
        Some(command) => {
            match command {
                ListCustomers => Some(crate::kimai::customer::list::list_customers(app_state.clone()).await?),

            }
        },
        None => None
    };
    // let t = x.unwrap().text().await.unwrap();
    // dbg!(t);
    //

    // dbg!(&x.unwrap().json().await.unwrap());
    //
    // let p: Vec<Customer> = match x {
    //     Some(res ) => serde_json::from_str(res.text().await?.as_str())?,
    //     None=> Vec::new()
    // };
    //
    // dbg!(p);

    Ok(())
}
