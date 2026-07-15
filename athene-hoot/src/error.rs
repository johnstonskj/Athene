//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use athene_owlapi::error::ApiError;
use colored::Colorize;
use serde_json::Error as JsonError;
use std::{
    fmt::{Display, Error as FmtError},
    io::Error as IoError,
    path::Path,
};
use strum::ParseError;
use strum::{Display as EnumDisplay, EnumIs, EnumIter, EnumString};
use thiserror::Error;
use tracing_subscriber::filter::FromEnvError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub(crate) enum CliError {
    #[error(
        "Could not retrieve value from environment variable for command-line argument; error: {0}"
    )]
    EnvError(#[from] FromEnvError),

    #[error("An error occured during formatting; error: {0}")]
    FmtError(#[from] FmtError),

    #[error("An error occured during file I/O; error: {0}")]
    IoError(#[from] IoError),

    #[error("An error occured during JSON parsing; error: {0}")]
    JsonError(#[from] JsonError),

    #[error("An error occured parsing an enum value; error: {0}")]
    ParseError(#[from] ParseError),

    #[error("An error occured parsing an enum value; error: {0}")]
    OwlApi(#[from] ApiError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CliMessage {
    severity: Severity,
    message: String,
    context: Option<String>,
    notes: Vec<Note>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Note {
    content: String,
    kind: NoteKind,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub(crate) enum Severity {
    Information,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString, EnumIter)]
pub(crate) enum NoteKind {
    Help,
    Note,
    #[strum(serialize = "See Also")]
    SeeAlso,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn missing_executable<S: AsRef<str>>(name: S) -> CliMessage {
    CliMessage::error("no executable found for external command")
        .with_context(format!("Command `{}`", name.as_ref()))
        .with_note(Note::help(format!(
            "check that the executable `rfham-{}` exists and is in $PATH",
            name.as_ref()
        )))
}

pub(crate) fn sub_process_error<P: AsRef<Path>, E: std::error::Error>(
    path: P,
    error: E,
) -> CliMessage {
    CliMessage::error("an error occured trying to run a sub-process")
        .with_context(format!("Executable path `{:?}`", path.as_ref()))
        .with_note(Note::note(format!("reported error: {error}")))
}

pub(crate) fn unhandled_error<E: std::error::Error>(error: E) -> CliMessage {
    CliMessage::error("an unhandled error has been detected, sorry about that")
        .with_context(format!("Error {error}"))
        .with_notes([
            Note::note("if this re-occurs, please file a bug"),
            Note::see_also("https://github.com/johnstonskj/rust-rfham/issues"),
        ])
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for CliMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (icon, severity, message) = match self.severity {
            Severity::Information => ("💡", self.severity.to_string(), self.message.to_string()),
            Severity::Warning => (
                "⚠️",
                self.severity.to_string().bold().yellow().to_string(),
                self.message.to_string(),
            ),
            Severity::Error => (
                "🛑",
                self.severity.to_string().bold().red().to_string(),
                self.message.to_string().bold().to_string(),
            ),
        };
        writeln!(f, "{} {}: {}", icon, severity, message)?;
        if let Some(context) = &self.context {
            writeln!(
                f,
                "{} {}",
                if self.notes.is_empty() {
                    "   └── 🔎"
                } else {
                    "   ├── 🔎"
                },
                context.blue()
            )?;
        }
        for (note, is_last) in self
            .notes
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i == self.notes.len() - 1))
        {
            writeln!(
                f,
                "{} {}",
                if is_last {
                    "   └──"
                } else {
                    "   ├──"
                },
                format!(
                    "{}  {} {}",
                    match note.kind {
                        NoteKind::Help => "ℹ️",
                        NoteKind::Note => "🗒️",
                        NoteKind::SeeAlso => "🔗",
                    },
                    note.kind,
                    note.content
                )
                .dimmed()
            )?;
        }
        Ok(())
    }
}

impl CliMessage {
    pub(crate) fn new<S: Into<String>>(severity: Severity, message: S) -> Self {
        Self {
            severity,
            message: message.into(),
            context: Default::default(),
            notes: Default::default(),
        }
    }

    pub(crate) fn error<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Error, message)
    }

    #[allow(dead_code)]
    pub(crate) fn warning<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Warning, message)
    }

    #[allow(dead_code)]
    pub(crate) fn infomation<S: Into<String>>(message: S) -> Self {
        Self::new(Severity::Information, message)
    }

    pub(crate) fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context = Some(context.into());
        self
    }

    pub(crate) fn with_notes<I: IntoIterator<Item = Note>>(mut self, notes: I) -> Self {
        self.notes = notes.into_iter().collect();
        self
    }

    pub(crate) fn with_note(mut self, note: Note) -> Self {
        self.notes = vec![note];
        self
    }

    pub(crate) fn print(&self) {
        match self.severity {
            Severity::Information => println!("{self}"),
            Severity::Warning => print!("{self}"),
            Severity::Error => eprint!("{self}"),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Note {
    pub(crate) fn new<S: Into<String>>(kind: NoteKind, content: S) -> Self {
        Self {
            kind,
            content: content.into(),
        }
    }

    pub(crate) fn help<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Help, content)
    }

    pub(crate) fn note<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::Note, content)
    }

    pub(crate) fn see_also<S: Into<String>>(content: S) -> Self {
        Self::new(NoteKind::SeeAlso, content)
    }
}
