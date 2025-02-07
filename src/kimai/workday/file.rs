use std::sync::Arc;
use crate::error::AppError;
use crate::state::AppState;

pub async  fn file_work_day(app_state: Arc<AppState>) -> Result<(), AppError> {
    Ok(())
}