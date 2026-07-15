use crate::{
    OnceCommand,
    error::{CliError, missing_executable, sub_process_error},
};
use colorchoice::ColorChoice;
use search_path::SearchPath;
use std::path::PathBuf;
use std::process::{Command, ExitCode};
use tracing::{error, info};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct RunExternalSubCommand {
    command_name: String,
    arguments: Vec<String>,
    verbosity: u8,
    quietness: u8,
    color_choice: ColorChoice,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------

impl RunExternalSubCommand {
    pub(crate) fn new(command_name: String, arguments: &[String]) -> Self {
        Self {
            command_name,
            arguments: arguments.to_vec(),
            verbosity: 0,
            quietness: 0,
            color_choice: ColorChoice::Auto,
        }
    }

    pub(crate) fn with_verbosity_level(mut self, verbosity: u8) -> Self {
        self.verbosity = verbosity;
        self
    }
    pub(crate) fn with_quietness_level(mut self, quietness: u8) -> Self {
        self.quietness = quietness;
        self
    }
    pub(crate) fn with_color_choice(mut self, color_choice: ColorChoice) -> Self {
        self.color_choice = color_choice;
        self
    }
}

impl OnceCommand for RunExternalSubCommand {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let command_name = format!("rfham-{}", self.command_name);
        let search_path = SearchPath::path().unwrap();
        if let Some(exec_path) = search_path.find_file(&PathBuf::from(command_name)) {
            let mut arguments = Vec::new();
            if self.verbosity > 0 {
                arguments.push(format!("-{}", "v".repeat(self.verbosity as usize)))
            }
            if self.quietness > 0 {
                arguments.push(format!("-{}", "q".repeat(self.quietness as usize)))
            }
            if self.color_choice != ColorChoice::Auto {
                arguments.push(format!(
                    "--color {}",
                    if self.color_choice != ColorChoice::Always {
                        "always"
                    } else {
                        "never"
                    }
                ));
            }
            arguments.extend(self.arguments);
            info!(
                "executing command `{:?}`, with arguments {:?}",
                exec_path, arguments
            );

            match Command::new(exec_path.clone())
                .args(arguments.as_slice())
                .status()
            {
                Ok(status) => {
                    info!("sub-process {:?} returned status {status}", &exec_path);
                    Ok(if status.success() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    })
                }
                Err(e) => {
                    error!("sub-process {exec_path:?} returned error {e}");
                    sub_process_error(exec_path, e).print();
                    Ok(ExitCode::FAILURE)
                }
            }
        } else {
            missing_executable(self.command_name).print();
            Ok(ExitCode::FAILURE)
        }
    }
}
