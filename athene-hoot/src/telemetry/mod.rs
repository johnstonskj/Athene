use crate::error::CliError;
use std::process::ExitCode;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn init(max_log_level: LevelFilter) -> Result<ExitCode, CliError> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(max_log_level.into())
                .from_env()?,
        )
        .init();

    Ok(ExitCode::SUCCESS)
}
