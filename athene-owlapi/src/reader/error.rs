//! Error types for the OWL 2 functional-style syntax reader.
use crate::reader::ast::Span;
use thiserror::Error;

/// A parse error from Stage 1 (lexer / syntactic parser) or Stage 2 (semantic converter).
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected character {ch:?} at {span}")]
    UnexpectedChar { ch: char, span: Span },

    #[error("unexpected end of input: expected {expected}")]
    UnexpectedEof { expected: &'static str },

    #[error("unexpected token {got:?} at {span}: expected {expected}")]
    UnexpectedToken {
        got: String,
        expected: &'static str,
        span: Span,
    },

    #[error("unclosed string literal starting at {span}")]
    UnclosedString { span: Span },

    #[error("unknown prefix {prefix:?} at {span}")]
    UnknownPrefix { prefix: String, span: Span },

    #[error("unsupported or unknown function {name:?} at {span}")]
    UnknownFunction { name: String, span: Span },

    #[error("{function} at {span}: expected {expected} argument(s), got {got}")]
    WrongArgCount {
        function: &'static str,
        expected: &'static str,
        got: usize,
        span: Span,
    },

    #[error("invalid IRI {text:?} at {span}: {error}")]
    InvalidIri {
        text: String,
        error: String,
        span: Span,
    },

    #[error("invalid argument at {span}: {message}")]
    InvalidArgument { message: String, span: Span },
}
