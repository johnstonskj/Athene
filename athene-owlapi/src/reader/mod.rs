//!
//! Provides a parser for the OWL 2 functional-style syntax.
//!
//! Two public entry points:
//!
//! * [`parse_str`] — parse a `&str` directly.
//! * [`parse_reader`] — buffer an [`std::io::Read`] then parse.
//!

use crate::{
    error::ApiError,
    reader::reporter::{InteractiveReporter, NoOpReporter, Reporter},
};

#[cfg(feature = "std")]
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Parse an OWL 2 functional-style syntax document from a string slice.
pub fn parse_str(input: &str, interactive: bool) -> Result<crate::OntologyDocument, ApiError> {
    let reporter: Box<dyn Reporter> = if interactive {
        Box::new(InteractiveReporter::default().with_source(String::new(), input.to_string()))
    } else {
        Box::new(NoOpReporter)
    };
    let doc = parser::Parser::new(input, reporter.as_ref())?.parse_document(reporter.as_ref())?;
    Ok(semantic::Converter::default().convert(doc, reporter.as_ref())?)
}

/// Parse an OWL 2 functional-style syntax document from any [`Read`] source.
#[cfg(feature = "std")]
pub fn parse_reader<R: Read>(
    mut reader: R,
    interactive: bool,
) -> Result<crate::OntologyDocument, ApiError> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e: io::Error| ReaderError::InvalidArgument {
            message: e.to_string(),
            span: ast::Span::default(),
        })?;
    parse_str(&buf, interactive)
}

/// Parse an OWL 2 functional-style syntax document from a [`Path`] source.
#[cfg(feature = "std")]
pub fn parse_file<P: AsRef<Path>>(
    file_path: P,
    interactive: bool,
) -> Result<crate::OntologyDocument, ApiError> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);
    parse_reader(reader, interactive)
}

// ------------------------------------------------------------------------------------------------
// Internal Modules
// ------------------------------------------------------------------------------------------------

mod lexer;
mod parser;
mod reporter;
mod semantic;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod ast;
pub mod error;
pub use error::ReaderError;
