use std::sync::Arc;
use clap::Parser;
use kimainai::error::AppError;
use kimainai::router::cli_router;
use kimainai::state::AppState;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    let args = kimainai::args::Args::parse();
    let app_state = Arc::new(AppState {
        args
    });

    let _ = cli_router(app_state.clone());

    dbg!(&app_state.args);

    Ok(())
}
