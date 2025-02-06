use std::sync::Arc;
use clap::Parser;
use kimainai::config::parse_config_file;
use kimainai::error::AppError;
use kimainai::router::cli_router;
use kimainai::state::AppState;

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    let args = kimainai::args::Args::parse();
    let app_config = parse_config_file(&args.config_file)?;
    let app_state = Arc::new(AppState {
        args,
        app_config
    });

    let _ = cli_router(app_state.clone()).await?;

    // dbg!(&app_state.args);
    // dbg!(&app_state.app_config);

    Ok(())
}
