//!
//! Syntactic AST for the OWL 2 functional-style syntax reader (Stage 1).
//!

use core::fmt;
use std::ops::Range;
use strum::{AsRefStr, EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Span / Position
// ------------------------------------------------------------------------------------------------

/// A contiguous range of source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// A byte position in the source text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based character column within the line.
    pub column: u32,
    /// Character offset from the start of the input.
    pub char_offset: u32,
    /// Byte offset from the start of the input.
    pub byte_offset: u32,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ IriRef
// ------------------------------------------------------------------------------------------------

/// An abbreviated IRI with an optional prefix and a local name: `owl:Thing` or `:local`.
#[derive(Clone, Debug, PartialEq)]
pub struct PrefixedIriRef {
    /// The prefix name, or `None` for the default namespace.
    pub prefix: Option<String>,
    pub local: String,
}

/// An IRI reference as it appears in source text, before prefix expansion.
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum IriRef {
    /// `<http://example.org/>`
    Full(String),
    /// `owl:Thing` or `:local`
    Prefixed(PrefixedIriRef),
    /// `owl:` or `:` — prefix name without a local part, used in `Prefix(…)` declarations.
    Namespace(Option<String>),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ LiteralSyntax
// ------------------------------------------------------------------------------------------------

/// A literal value as it appears in source text.
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralSyntax {
    /// String content with escape sequences resolved.
    pub lexical_form: String,
    /// Datatype IRI after `^^`.  `None` means `rdf:PlainLiteral`.
    pub datatype: Option<IriRef>,
    /// Language tag after `@`, if any.
    pub lang_tag: Option<String>,
    pub span: Span,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Atom
// ------------------------------------------------------------------------------------------------

/// A terminal (leaf) value in the syntactic tree.
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Atom {
    Iri(IriRef),
    /// Blank-node identifier: `_:name`.
    NodeId(String),
    Literal(LiteralSyntax),
    /// Non-negative integer (cardinality constraints).
    Integer(u32),
    /// The `=` separator inside `Prefix(…)` declarations.
    Equals,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ SyntaxNode
// ------------------------------------------------------------------------------------------------

/// A function call node: `Name( arg… )`.
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub args: Vec<SyntaxNode>,
}

/// The kind of a node in the Stage 1 syntactic tree.
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs, AsRefStr)]
pub enum SyntaxNodeKind {
    /// A line comment — text after `#`, trimmed.
    Comment(String),
    Atom(Atom),
    Function(FunctionNode),
}

/// A node in the Stage 1 syntactic tree.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxNode {
    pub span: Span,
    pub kind: SyntaxNodeKind,
}

/// The top-level document produced by Stage 1.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SyntaxDocument {
    pub nodes: Vec<SyntaxNode>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Span / Position
// ------------------------------------------------------------------------------------------------

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(
                f,
                "{}:{}-{}",
                self.start.line, self.start.column, self.end.column
            )
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn at(pos: Position) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    pub fn extend_to(self, other: Span) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ FunctionNode
// ------------------------------------------------------------------------------------------------

impl FunctionNode {
    /// Non-comment args.
    pub fn semantic_args(&self) -> impl Iterator<Item = &SyntaxNode> {
        self.args.iter().filter(|n| !n.kind.is_comment())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ SyntaxNode
// ------------------------------------------------------------------------------------------------

impl SyntaxNode {
    pub fn function_name(&self) -> Option<&str> {
        match &self.kind {
            SyntaxNodeKind::Function(f) => Some(f.name.as_str()),
            _ => None,
        }
    }

    pub fn function_args(&self) -> Option<&[SyntaxNode]> {
        match &self.kind {
            SyntaxNodeKind::Function(f) => Some(f.args.as_slice()),
            _ => None,
        }
    }

    pub fn semantic_args(&self) -> Option<impl Iterator<Item = &SyntaxNode>> {
        match &self.kind {
            SyntaxNodeKind::Function(f) => Some(f.semantic_args()),
            _ => None,
        }
    }

    pub fn try_as_iri_ref(&self) -> Option<&IriRef> {
        match &self.kind {
            SyntaxNodeKind::Atom(Atom::Iri(r)) => Some(r),
            _ => None,
        }
    }

    pub fn try_as_integer(&self) -> Option<u32> {
        match &self.kind {
            SyntaxNodeKind::Atom(v) => v.try_as_integer_ref().copied(),
            _ => None,
        }
    }

    pub fn as_literal_syntax(&self) -> Option<&LiteralSyntax> {
        match &self.kind {
            SyntaxNodeKind::Atom(v) => v.try_as_literal_ref(),
            _ => None,
        }
    }

    pub fn as_node_id(&self) -> Option<&String> {
        match &self.kind {
            SyntaxNodeKind::Atom(v) => v.try_as_node_id_ref(),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Span
// ------------------------------------------------------------------------------------------------

impl Span {
    pub fn byte_range(&self) -> Range<usize> {
        (self.start.byte_offset as usize)..(self.end.byte_offset as usize)
    }

    pub fn char_range(&self) -> Range<usize> {
        (self.start.char_offset as usize)..(self.end.char_offset as usize)
    }
}
