//!
//! Error types for the OWL 2 functional-style syntax reader.
//!

use crate::{reader::ast::Span, values::CardinalityConstraint};
use strum::EnumProperty;
use thiserror::Error;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A parse error from Stage 1 (lexer / syntactic parser) or Stage 2 (semantic converter).
#[derive(Debug, Error, EnumProperty)]
pub enum ReaderError {
    #[strum(props(class = "I/O", code = 1))]
    #[error("source not found for `{ontology}`")]
    SourceNotFound { ontology: String },

    // --------------------------------------------------------------------------------------------
    // Lexer Errors
    // --------------------------------------------------------------------------------------------
    #[strum(props(class = "Lexer", code = 1001))]
    #[error("unexpected character {ch:?} at {span}")]
    UnexpectedChar { ch: char, span: Span },

    #[strum(props(class = "Lexer", code = 1002))]
    #[error("unexpected end of input: expected {expected}")]
    UnexpectedEof { expected: &'static str },

    #[strum(props(class = "Lexer", code = 1003))]
    #[error("unclosed string literal starting at {span}")]
    UnclosedString { span: Span },

    // --------------------------------------------------------------------------------------------
    // Parser Errors
    // --------------------------------------------------------------------------------------------
    #[strum(props(class = "Parser", code = 2001))]
    #[error("unexpected token at {span}; token {got:?}, expecting one of {expected:?}")]
    UnexpectedToken {
        got: String,
        expected: Vec<String>,
        span: Span,
    },

    #[strum(props(class = "Parser", code = 2002))]
    #[error("unexpected node at {span}; node: {got:?}, expecting one of {expected:?}")]
    UnexpectedNode {
        got: String,
        expected: Vec<String>,
        span: Span,
    },

    #[strum(props(class = "Parser", code = 2003))]
    // ERROR: 2003
    #[error("missing node at {span}; expecting one of {expected:?}")]
    MissingNode { expected: Vec<String>, span: Span },

    // --------------------------------------------------------------------------------------------
    // Conversion Errors
    // --------------------------------------------------------------------------------------------
    #[strum(props(class = "Conversion", code = 3001))]
    #[error("invalid IRI representation `{input:?}` at {span}: {error}")]
    InvalidIri {
        input: String,
        error: String,
        span: Span,
    },

    #[strum(props(class = "Conversion", code = 3002))]
    #[error("empty LanguageTag at {span}")]
    EmptyLanguageTag { span: Span },

    #[strum(props(class = "Conversion", code = 3002))]
    #[error("invalid LanguageTag representation `{input:?}` at {span}: {error}")]
    InvalidLanguageTag {
        input: String,
        error: String,
        span: Span,
    },

    #[strum(props(class = "Conversion", code = 3003))]
    #[error("invalid Namespace representation `{input:?}` at {span}: {error}")]
    InvalidNamespace {
        input: String,
        error: String,
        span: Span,
    },

    #[strum(props(class = "Conversion", code = 3004))]
    #[error("invalid Name representation `{input:?}` at {span}: {error}")]
    InvalidName {
        input: String,
        error: String,
        span: Span,
    },

    // --------------------------------------------------------------------------------------------
    // Semantics Errors
    // --------------------------------------------------------------------------------------------
    #[strum(props(class = "Semantics", code = 4001))]
    #[error("unknown prefix {prefix:?} at {span}")]
    UnknownPrefix { prefix: String, span: Span },

    #[strum(props(class = "Semantics", code = 4002))]
    #[error("cannot use reserved prefix {prefix:?} in a Prefix declaration, at {span}")]
    ReservedPrefix { prefix: String, span: Span },

    #[strum(props(class = "Semantics", code = 4003))]
    #[error("cannot use reserved IRI {iri:?} in a Prefix declaration, at {span}")]
    ReservedIri { iri: String, span: Span },

    #[strum(props(class = "Semantics", code = 4004))]
    #[error("unsupported or unknown function {name:?} at {span}")]
    UnknownFunction { name: String, span: Span },

    #[strum(props(class = "Semantics", code = 4005))]
    #[error("unexpected function at {span}; name {got}, expecting one of {expected:?}")]
    UnexpectedFunction {
        got: String,
        expected: Vec<String>,
        span: Span,
    },

    #[strum(props(class = "Semantics", code = 4006))]
    #[error("function `{function:?}` at {span} expected {expected} arguments, but got {got}")]
    ArgumentArity {
        function: String,
        got: usize,
        expected: CardinalityConstraint,
        span: Span,
    },

    #[strum(props(class = "Semantics", code = 4007))]
    #[error("invalid argument at {span}: {message}")]
    InvalidArgument { message: String, span: Span },
    // --------------------------------------------------------------------------------------------
    // Lint Errors
    // --------------------------------------------------------------------------------------------
}
