/// This module contains a top-level struct `Cli` which uses `clap_derive` to implement the
/// Command-Line structure. A `Commands` enum contains the set of commands and their specific
/// argument structs.
///
/// All types implement `OnceCommand` to allow the simple cascade of execution.
pub(crate) mod cli;

/// Actual implementation types, if necessary, are here. These are typically structs with their
/// context set by the cli (using explicit `new` constructors) and then their implementation of
/// `OnceCommand::execute` is called.
pub(crate) mod command;

/// Implements `Error` using `thiserror`.
pub(crate) mod error;

/// Initializes any/all of tracing, logging, and metrics. A top-level `init()` function performs
/// all necessary initialization.
pub(crate) mod telemetry;

use self::cli::Cli;
use crate::error::unhandled_error;
use clap::Parser;
use std::process::ExitCode;

pub trait OnceCommand {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: std::error::Error;

    /// Executes the command, consuming self.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}

pub trait OnceCommandWith {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: std::error::Error;
    type Value: ?Sized;

    fn execute_with(self, value: Self::Value) -> Result<Self::Output, Self::Error>;
}

const COMMAND_NAME: &str = env!("CARGO_BIN_NAME");

// CLI functions that propogate errors _should_ use `ExitCode` to denote success/failure
// even if no explicit errors occurred.
fn main() -> ExitCode {
    let command_line = Cli::parse();

    match telemetry::init(command_line.max_level_filter()) {
        Ok(exit_code) => exit_code,
        Err(e) => {
            unhandled_error(e).print();
            return ExitCode::FAILURE;
        }
    };

    match command_line.execute() {
        Ok(exit_code) => exit_code,
        Err(e) => {
            unhandled_error(e).print();
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}

impl<T, O, E, V> OnceCommand for T
where
    E: std::error::Error,
    V: Default,
    T: OnceCommandWith<Output = O, Error = E, Value = V>,
{
    type Output = O;
    type Error = E;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        Self::execute_with(self, V::default())
    }
}
