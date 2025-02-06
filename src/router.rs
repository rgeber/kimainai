use std::sync::Arc;
use crate::error::AppError;
use crate::state::AppState;

pub fn cli_router(app_state: Arc<AppState>) -> Result<(), AppError> {

    match &app_state.args.command {
        Some(command) => {
            dbg!(command);
        },
        None => {}
    }

    Ok(())
}