---
description: 'Rust file templates for library and binary projects'
applyTo: '**/*.rs'
---

# Rust File Templates

## Library Template

File Name: `src/lib.rs`

```rust
//!
//! One-line description.
//!
//! More detailed description.
//!
//! # Examples
//!
//! ```rust
//! ```
//!
//! # Features
//!
//! - **feature-name**; Feature description
//!

// use statements

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
```

## Cli Main Template

File Name: `src/main.rs` or `src/bin/{name}.rs`

```rust
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

use self::{cli::Cli, error::Error};
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

const COMMAND_NAME: &str = env!("CARGO_BIN_NAME");

// CLI functions that propogate errors _should_ use `ExitCode` to denote success/failure
// even if no explicit errors occurred.
fn main() -> Result<ExitCode, Error> {
    telemetry::init()?;
    Cli::parse().execute()
}
```

## Error Module Template

File Name: `src/error.rs`

```rust
//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use thiserror::Error;
use std::io::Error as IoError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum PackageNameError {
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    #[error("OsString to String conversion error; bytes: {:?}", bytes)]
    OsString { bytes: Vec<u8> },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type PackageNameResult<T> = std::result::Result<T, PackageNameError>;
```

## Module Template

File Name: `src/{module_name}.rs` or `src/{module_name}/mod.rs`

```rust
//!
//! Provides ..., a one-line description
//!
//! More detailed description
//!
//! # Examples
//!
//! ```rust
//! ```
//!

// use statements here

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Sub-Modules
// ------------------------------------------------------------------------------------------------
```

## Unit Test Template

```rust
// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_something() {
        todo!()
    }
}
```