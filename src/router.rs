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
        CliCommands::ListCustomers { list: _ } => {
            crate::kimai::customer::list::print_customer_list(app_state.clone()).await?
        },
        CliCommands::ListProjects {} => {
            crate::kimai::projects::list::print_project_list(app_state.clone()).await?;
        }
    }

    Ok(())
}
