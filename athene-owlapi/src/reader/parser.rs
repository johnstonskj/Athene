//! Stage 1 parser: token stream → [`SyntaxDocument`].
use crate::reader::ast::{
    Atom, FunctionNode, IriRef, LiteralSyntax, PrefixedIriRef, Span, SyntaxDocument, SyntaxNode,
    SyntaxNodeKind,
};
use crate::reader::error::ParseError;
use crate::reader::lexer::{Lexer, Token, TokenKind};

// ── Parser ────────────────────────────────────────────────────────────────────

pub(crate) struct Parser {
    lexer: Lexer,
    /// One-token lookahead.
    current: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.current, next))
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.current.kind
    }

    fn current_span(&self) -> Span {
        self.current.span
    }

    fn expect_rparen(&mut self, open_span: Span) -> Result<(), ParseError> {
        match &self.current.kind {
            TokenKind::RParen => {
                self.advance()?;
                Ok(())
            }
            TokenKind::Eof => Err(ParseError::UnexpectedEof { expected: ")" }),
            _ => Err(ParseError::UnexpectedToken {
                got: format!("{:?}", self.current.kind),
                expected: ")",
                span: open_span,
            }),
        }
    }

    // ── Top-level ─────────────────────────────────────────────────────────────

    pub fn parse_document(&mut self) -> Result<SyntaxDocument, ParseError> {
        let mut nodes = Vec::new();
        loop {
            match self.peek_kind() {
                TokenKind::Eof => break,
                _ => nodes.push(self.parse_node()?),
            }
        }
        Ok(SyntaxDocument { nodes })
    }

    fn parse_node(&mut self) -> Result<SyntaxNode, ParseError> {
        let span = self.current_span();
        match self.peek_kind().clone() {
            TokenKind::Comment(text) => {
                self.advance()?;
                Ok(SyntaxNode { span, kind: SyntaxNodeKind::Comment(text) })
            }
            TokenKind::Name(_) => self.parse_name_or_function(),
            TokenKind::FullIri(_)
            | TokenKind::PrefixedName(_)
            | TokenKind::Namespace(_)
            | TokenKind::NodeId(_) => self.parse_iri_or_node_id(),
            TokenKind::QuotedString(_) => self.parse_literal(),
            TokenKind::Integer(n) => {
                let n = n;
                self.advance()?;
                Ok(SyntaxNode { span, kind: SyntaxNodeKind::Atom(Atom::Integer(n)) })
            }
            TokenKind::Equals => {
                self.advance()?;
                Ok(SyntaxNode { span, kind: SyntaxNodeKind::Atom(Atom::Equals) })
            }
            _ => {
                let got = format!("{:?}", self.current.kind);
                Err(ParseError::UnexpectedToken { got, expected: "node", span })
            }
        }
    }

    /// A `Name` token is either a function call (`Name(…)`) or a bare IRI atom.
    fn parse_name_or_function(&mut self) -> Result<SyntaxNode, ParseError> {
        let span = self.current_span();
        let name = match self.advance()?.kind {
            TokenKind::Name(n) => n,
            _ => unreachable!(),
        };

        if matches!(self.peek_kind(), TokenKind::LParen) {
            self.advance()?; // consume '('
            let mut args = Vec::new();
            loop {
                match self.peek_kind() {
                    TokenKind::RParen | TokenKind::Eof => break,
                    _ => args.push(self.parse_node()?),
                }
            }
            self.expect_rparen(span)?;
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

    fn parse_iri_or_node_id(&mut self) -> Result<SyntaxNode, ParseError> {
        let span = self.current_span();
        let tok = self.advance()?;
        let kind = match tok.kind {
            TokenKind::FullIri(s) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Full(s))),
            TokenKind::PrefixedName(p) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Prefixed(p))),
            TokenKind::Namespace(ns) => SyntaxNodeKind::Atom(Atom::Iri(IriRef::Namespace(ns))),
            TokenKind::NodeId(id) => SyntaxNodeKind::Atom(Atom::NodeId(id)),
            _ => unreachable!(),
        };
        Ok(SyntaxNode { span, kind })
    }

    /// Parse a quoted string literal, optionally followed by `^^<type>` or `@lang`.
    fn parse_literal(&mut self) -> Result<SyntaxNode, ParseError> {
        let start_span = self.current_span();
        let lexical_form = match self.advance()?.kind {
            TokenKind::QuotedString(s) => s,
            _ => unreachable!(),
        };

        let (datatype, lang_tag) = match self.peek_kind() {
            TokenKind::DataTypeSep => {
                self.advance()?; // consume '^^'
                let dt = self.parse_iri_ref()?;
                (Some(dt), None)
            }
            TokenKind::LangTag(_) => {
                let tag = match self.advance()?.kind {
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

    fn parse_iri_ref(&mut self) -> Result<IriRef, ParseError> {
        let span = self.current_span();
        match self.advance()?.kind {
            TokenKind::FullIri(s) => Ok(IriRef::Full(s)),
            TokenKind::PrefixedName(p) => Ok(IriRef::Prefixed(p)),
            TokenKind::Namespace(ns) => Ok(IriRef::Namespace(ns)),
            TokenKind::Name(n) => Ok(IriRef::Prefixed(PrefixedIriRef { prefix: None, local: n })),
            _ => Err(ParseError::UnexpectedToken {
                got: format!("{:?}", self.current.kind),
                expected: "IRI",
                span,
            }),
        }
    }
}
