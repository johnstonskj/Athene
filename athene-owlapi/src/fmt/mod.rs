//!
//! Traits and tools for formatting (almost) any API structure in Functional-Style Syntax.
//!
//! Most of the structs and enums in the API which implement the standard library's
//! `Display` trait will also implement the `DisplayPretty` trait defined here. the
//! purpose is to give the library a default `Display` implementation that writes in the
//! functional-style syntax, in fact supporting an entirely single-line space separated
//! version and a nested *term-per-line* version.
//!
//! ## Usage
//!
//! ```rust,ignore
//! impl Display for MyType {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//!         self.fmt_pretty(f, Indenter::default(), &:IriPrefixMap::default())
//!     }
//! }
//! ```
//!
//! `impl_display_pretty`
//!
//! Value Types
//!
//! ```rust,ignore
//! impl DisplayPretty for Literal {
//!     fn fmt_pretty(
//!         &self,
//!         f: &mut Formatter<'_>,
//!         _: &Indenter,
//!         _: &IriPrefixMap) -> FmtResult
//!     {
//!         write!(f, "{}", self.0)
//!     }
//! }
//! ```
//!
//!
use core::fmt::{Display, Formatter, Result as FmtResult};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This trait's `fmt_pretty` method takes two additional arguments beyond those in the
/// `Display::fmt` method.
///
/// * `indenter` -- used to get the current indentation level, the current indentation
///   prefix string, and to indent/outdent as necessary.
/// * `prefix_map` -- an immutable reference to the prefix map holding the ontology IRI
///   mappings to allow for IRI compression.
///
pub trait DisplayPretty: Display {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult;
}

mod indenter;
pub use indenter::Indenter;
use rdftk_iri::IriPrefixMap;
