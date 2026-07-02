//! Error and result types for `athene-owlapi`.
//!
//! [`ApiError`] is the primary error enum exposed in the crate's API. Howevever, the
//! `BuilderError` here, and the `CardinalityConstraintViolation` in the `values`
//! module are used locally for more ergonomic error handling and are then wrapped
//! into the `ApiError`.
//!

use crate::values::CardinalityConstraintViolation;
use strum::EnumIs;
use thiserror::Error;

#[cfg(not(feature = "std"))]
use alloc::string::String;

#[cfg(feature = "std")]
use std::io::Error as IoError;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error, EnumIs)]
pub enum ApiError {
    #[error("An error occured in one of the builder objects; error: {0}")]
    Builder(#[from] BuilderError),

    #[error("A collection cardinality constraint violation occured; error: {0}")]
    CardinalityConstraintViolation(#[from] CardinalityConstraintViolation),

    #[error("An error occured trying to parse a value into type `{0}`; error: {1}, input: '{2}'")]
    ValueParser(&'static str, String, String),

    #[cfg(feature = "std")]
    #[error("An error occured performing standard I/O; error: {0}")]
    Io(#[from] IoError),
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type ApiResult<T> = core::result::Result<T, ApiError>;

///
/// An `Error` type for builder-specific.
///
#[derive(Debug, Error, EnumIs)]
pub enum BuilderError {
    #[error("The required field named *{name}* is missing")]
    MissingField { name: String },

    #[error(
        "The field named *{dependent}* is missing, it is **required** due to the value of (or presence of) the field *{antecedent}*"
    )]
    MissingDependentField {
        antecedent: String,
        dependent: String,
    },

    #[error(
        "The field named *{dependent}* is present, it is **prohibited** due to the value of (or presence of) the field *{antecedent}*"
    )]
    UnexpectedDependentField {
        antecedent: String,
        dependent: String,
    },
}
