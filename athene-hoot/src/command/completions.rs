use crate::{COMMAND_NAME, OnceCommand, cli::Cli, error::CliError};
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::{io, process::ExitCode};
use tracing::{info, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct GenerateCompletions {
    shell: Option<Shell>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GenerateCompletions {
    pub(crate) fn new(shell: Option<Shell>) -> Self {
        Self { shell }
    }
}

impl OnceCommand for GenerateCompletions {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        trace!("run(shell: {:?})", self.shell);
        let shell = self.shell.unwrap_or_else(|| {
            // Try to detect from $SHELL
            std::env::var("SHELL")
                .ok()
                .and_then(|s| {
                    let basename = std::path::Path::new(&s)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    basename.parse::<Shell>().ok()
                })
                .unwrap_or(Shell::Bash)
        });
        info!("creating completions for shell `{shell:?}`");

        let mut cmd = Cli::command();
        generate(shell, &mut cmd, COMMAND_NAME, &mut io::stdout());
        Ok(ExitCode::SUCCESS)
    }
}
