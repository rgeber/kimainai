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
        CliCommands::GetMe {} => crate::kimai::user::me::print_user_me(app_state.clone()).await?,
        CliCommands::ListCustomers { list: _ } => crate::kimai::customer::list::print_customer_list(app_state.clone()).await?,
        CliCommands::ListProjects {} => crate::kimai::projects::list::print_project_list(app_state.clone()).await?,
        CliCommands::ListActivities { project_id } => crate::kimai::activities::list::print_activities_list(app_state.clone(), project_id).await?,
        CliCommands::FileWorkday(args) => crate::kimai::workday::file::file_work_day(app_state.clone(), args).await?,
        CliCommands::BulkFileWorkdays(args) => crate::kimai::workday::file::bulk_file_work_days(app_state.clone(), args).await?,
    }

    Ok(())
}
