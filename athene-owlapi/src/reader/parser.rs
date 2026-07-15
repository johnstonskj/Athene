//!
//!  Stage 1 parser: token stream → [`SyntaxDocument`].
//!

use crate::reader::{
    ast::{
        Atom, FunctionNode, IriRef, LiteralSyntax, PrefixedIriRef, Span, SyntaxDocument,
        SyntaxNode, SyntaxNodeKind,
    },
    error::ReaderError,
    lexer::{Lexer, Token, TokenKind},
    reporter::Reporter,
};
use core::mem;

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(crate) struct Parser {
    lexer: Lexer,
    /// One-token lookahead.
    current: Token,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Parser {
    pub(super) fn new(input: &str, reporter: &dyn Reporter) -> Result<Self, ReaderError> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token(reporter)?;
        Ok(Self { lexer, current })
    }

    fn advance(&mut self, reporter: &dyn Reporter) -> Result<Token, ReaderError> {
        let next = self.lexer.next_token(reporter)?;
        Ok(mem::replace(&mut self.current, next))
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.current.kind
    }

    fn current_span(&self) -> Span {
        self.current.span
    }

    fn expect_rparen(
        &mut self,
        open_span: Span,
        reporter: &dyn Reporter,
    ) -> Result<(), ReaderError> {
        match &self.current.kind {
            TokenKind::RParen => {
                self.advance(reporter)?;
                Ok(())
            }
            TokenKind::Eof => Err(ReaderError::UnexpectedEof { expected: ")" }),
            _ => Err(ReaderError::UnexpectedToken {
                got: format!("{:?}", self.current.kind),
                expected: vec![")".to_string()],
                span: open_span,
            }),
        }
    }

    // ── Top-level ─────────────────────────────────────────────────────────────

    pub(super) fn parse_document(
        &mut self,
        reporter: &dyn Reporter,
    ) -> Result<SyntaxDocument, ReaderError> {
        let mut nodes = Vec::new();
        loop {
            match self.peek_kind() {
                TokenKind::Eof => break,
                _ => nodes.push(self.parse_node(reporter)?),
            }
        }
        Ok(SyntaxDocument { nodes })
    }

    fn parse_node(&mut self, reporter: &dyn Reporter) -> Result<SyntaxNode, ReaderError> {
        let span = self.current_span();
        match self.peek_kind().clone() {
            TokenKind::Comment(text) => {
                self.advance(reporter)?;
                Ok(SyntaxNode {
                    span,
                    kind: SyntaxNodeKind::Comment(text),
                })
            }
            TokenKind::Name(_) => self.parse_name_or_function(reporter),
            TokenKind::FullIri(_)
            | TokenKind::PrefixedName(_)
            | TokenKind::Namespace(_)
            | TokenKind::NodeId(_) => self.parse_iri_or_node_id(reporter),
            TokenKind::QuotedString(_) => self.parse_literal(reporter),
            // Anonymous group — used by HasKey's ( OPE* ) and ( DPE* ) sub-lists.
            TokenKind::LParen => {
                self.advance(reporter)?; // consume '('
                let mut args = Vec::new();
                loop {
                    match self.peek_kind() {
                        TokenKind::RParen | TokenKind::Eof => break,
                        _ => args.push(self.parse_node(reporter)?),
                    }
                }
                self.expect_rparen(span, reporter)?;
                let end_span = self.current_span();
                Ok(SyntaxNode {
                    span: span.extend_to(end_span),
                    kind: SyntaxNodeKind::Function(FunctionNode {
                        name: String::new(),
                        args,
                    }),
                })
            }
            TokenKind::Integer(n) => {
                self.advance(reporter)?;
                Ok(SyntaxNode {
                    span,
                    kind: SyntaxNodeKind::Atom(Atom::Integer(n)),
                })
            }
            TokenKind::Equals => {
                self.advance(reporter)?;
                Ok(SyntaxNode {
                    span,
                    kind: SyntaxNodeKind::Atom(Atom::Equals),
                })
            }
            _ => {
                let got = format!("{:?}", self.current.kind);
                Err(ReaderError::UnexpectedToken {
                    got,
                    expected: vec!["node".to_string()],
                    span,
                })
            }
        }
    }

    /// A `Name` token is either a function call (`Name(…)`) or a bare IRI atom.
    fn parse_name_or_function(
        &mut self,
        reporter: &dyn Reporter,
    ) -> Result<SyntaxNode, ReaderError> {
        let span = self.current_span();
        let name = match self.advance(reporter)?.kind {
            TokenKind::Name(n) => n,
            _ => unreachable!(),
        };

        if matches!(self.peek_kind(), TokenKind::LParen) {
            self.advance(reporter)?; // consume '('
            let mut args = Vec::new();
            loop {
                match self.peek_kind() {
                    TokenKind::RParen | TokenKind::Eof => break,
                    _ => args.push(self.parse_node(reporter)?),
                }
            }
            self.expect_rparen(span, reporter)?;
            let end_span = self.current_span();
            Ok(SyntaxNode {
                span: span.extend_to(end_span),
                kind: SyntaxNodeKind::Function(FunctionNode { name, args }),
            })
        } else {
            Ok(SyntaxNode {
                span,
                kind: SyntaxNodeKind::Atom(Atom::Iri(IriRef::Prefixed(PrefixedIriRef {
                    prefix: None,
                    local: name,
                }))),
            })
        }
    }

    fn parse_iri_or_node_id(&mut self, reporter: &dyn Reporter) -> Result<SyntaxNode, ReaderError> {
        let span = self.current_span();
        let token = self.advance(reporter)?;
        let kind = match token.kind {
            TokenKind::FullIri(s) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Full(s))),
            TokenKind::PrefixedName(p) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Prefixed(p))),
            TokenKind::Namespace(ns) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Namespace(ns))),
            TokenKind::NodeId(id) => SyntaxNodeKind::Atom(Atom::NodeId(id)),
            _ => unreachable!(),
        };
        Ok(SyntaxNode { span, kind })
    }

    /// Parse a quoted string literal, optionally followed by `^^<type>` or `@lang`.
    fn parse_literal(&mut self, reporter: &dyn Reporter) -> Result<SyntaxNode, ReaderError> {
        let start_span = self.current_span();
        let lexical_form = match self.advance(reporter)?.kind {
            TokenKind::QuotedString(s) => s,
            _ => unreachable!(),
        };

        let (datatype, lang_tag) = match self.peek_kind() {
            TokenKind::DataTypeSep => {
                self.advance(reporter)?; // consume '^^'
                let dt = self.parse_iri_ref(reporter)?;
                (Some(dt), None)
            }
            TokenKind::LangTag(_) => {
                let tag = match self.advance(reporter)?.kind {
                    TokenKind::LangTag(t) => t,
                    _ => unreachable!(),
                };
                (None, Some(tag))
            }
            _ => (None, None),
        };

        let end_span = self.current_span();
        let span = start_span.extend_to(end_span);
        Ok(SyntaxNode {
            span,
            kind: SyntaxNodeKind::Atom(Atom::Literal(LiteralSyntax {
                lexical_form,
                datatype,
                lang_tag,
                span,
            })),
        })
    }

    fn parse_iri_ref(&mut self, reporter: &dyn Reporter) -> Result<IriRef, ReaderError> {
        let token = self.advance(reporter)?;
        match token.kind {
            TokenKind::FullIri(s) => Ok(IriRef::Full(s)),
            TokenKind::PrefixedName(p) => Ok(IriRef::Prefixed(p)),
            TokenKind::Namespace(ns) => Ok(IriRef::Namespace(ns)),
            TokenKind::Name(n) => Ok(IriRef::Prefixed(PrefixedIriRef {
                prefix: None,
                local: n,
            })),
            _ => Err(reporter.unexpected_token(&token, &["IRI"])),
        }
    }
}
