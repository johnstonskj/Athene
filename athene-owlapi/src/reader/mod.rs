//!
//! Provides a parser for the OWL 2 functional-style syntax.
//!
//! Two public entry points:
//!
//! * [`parse_str`] — parse a `&str` directly.
//! * [`parse_reader`] — buffer an [`std::io::Read`] then parse.
//!

#[cfg(feature = "std")]
use std::io::{self, Read};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Parse an OWL 2 functional-style syntax document from a string slice.
pub fn parse_str(input: &str) -> Result<crate::OntologyDocument, ParseError> {
    let doc = parser::Parser::new(input)?.parse_document()?;
    semantic::Converter::default().convert(doc)
}

/// Parse an OWL 2 functional-style syntax document from any [`Read`] source.
#[cfg(feature = "std")]
pub fn parse_reader<R: Read>(mut reader: R) -> Result<crate::OntologyDocument, ParseError> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e: io::Error| ParseError::InvalidArgument {
            message: e.to_string(),
            span: ast::Span::default(),
        })?;
    parse_str(&buf)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod ast;
pub mod error;
pub use error::ParseError;

mod lexer;
mod parser;
mod semantic;
