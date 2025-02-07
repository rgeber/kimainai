use std::sync::Arc;
use crate::args::CliCommands;
use crate::error::AppError;
use crate::state::AppState;

pub async fn cli_router(app_state: Arc<AppState>) -> Result<(), AppError> {

    let command: Option<CliCommands> = match &app_state.args.command {
        Some(command) => Some(command.to_owned()),
        None => None
    };

    if command.is_none() {
        println!("No Command given. Have a nice day.");
        return Ok(());
    }

    let command = command.unwrap();

    match command {
        CliCommands::ListCustomers { list: _ } => crate::kimai::customer::list::print_customer_list(app_state.clone()).await?,
        CliCommands::ListProjects {} => crate::kimai::projects::list::print_project_list(app_state.clone()).await?,
        CliCommands::ListActivities { project_id } => crate::kimai::activities::list::print_activities_list(app_state.clone(), project_id).await?,
        CliCommands::FileWorkDay { user_id, customer_id, activitiy_id, description, fixed_rate, hourly_rate, exported, billable, duration_hours, duration_minutes, start_time_year, start_time_month, start_time_day, break_length, break_start_hour, break_start_minute } => {
            crate::kimai::workday::file::file_work_day(app_state.clone()).await?
        }
    }

    Ok(())
}
