use std::sync::Arc;
use crate::args::FileWorkdayArgs;
use crate::error::AppError;
use crate::kimai::workday::util::{get_activity_id, get_customer_id, get_project_id, get_user_id};
use crate::state::AppState;



pub async  fn file_work_day(app_state: Arc<AppState>, args: FileWorkdayArgs) -> Result<(), AppError> {


    let user = get_user_id(app_state.clone(), &args).await?;
    let project = get_project_id(app_state.clone(), &args)?;
    let customer = get_customer_id(app_state.clone(), &args)?;
    let activity = get_activity_id(app_state.clone(), &args)?;
    let description = args.description.unwrap_or("".to_string());
    let hourly_rate = args.hourly_rate;
    let fixed_rate = args.fixed_rate;
    let tags:Vec<String> = Vec::new();
    let exported = args.exported;
    let billable = args.billable;




    Ok(())

}