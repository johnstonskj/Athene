//! Stage 1 lexer for the OWL 2 functional-style syntax.
use crate::reader::ast::{Position, PrefixedIriRef, Span};
use crate::reader::error::ParseError;

// ── Token ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TokenKind {
    /// `<http://example.org/>`
    FullIri(String),
    /// `owl:Thing` or `:local`
    PrefixedName(PrefixedIriRef),
    /// `owl:` or `:` — prefix name without a local part
    Namespace(Option<String>),
    /// `_:name`
    NodeId(String),
    /// Bare name / keyword / function name
    Name(String),
    /// `"…"` with escape sequences resolved
    QuotedString(String),
    /// Language tag text (without the leading `@`)
    LangTag(String),
    /// `^^`
    DataTypeSep,
    /// Non-negative integer
    Integer(u32),
    /// `=` (used in `Prefix(…)` declarations)
    Equals,
    LParen,
    RParen,
    /// Line comment text (trimmed, without the leading `#`)
    Comment(String),
    Eof,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

// ── Lexer ─────────────────────────────────────────────────────────────────────

pub(crate) struct Lexer {
    chars: Vec<char>,
    pos: usize,
    /// Byte offset of `chars[pos]` from the start of the input.
    byte_offset: u32,
    line: u32,
    column: u32,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            byte_offset: 0,
            line: 1,
            column: 1,
        }
    }

    // ── Position helpers ──────────────────────────────────────────────────────

    fn current_position(&self) -> Position {
        Position { line: self.line, column: self.column, offset: self.byte_offset }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek2(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    /// Advance past the current character and return it.
    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        self.byte_offset += ch.len_utf8() as u32;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_ascii_whitespace()) {
            self.advance();
        }
    }

    fn make_token(&self, kind: TokenKind, start: Position) -> Token {
        Token { kind, span: Span::new(start, self.current_position()) }
    }

    // ── Public interface ──────────────────────────────────────────────────────

    pub fn next_token(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();

        let start = self.current_position();

        let ch = match self.peek() {
            None => return Ok(self.make_token(TokenKind::Eof, start)),
            Some(c) => c,
        };

        match ch {
            '(' => {
                self.advance();
                Ok(self.make_token(TokenKind::LParen, start))
            }
            ')' => {
                self.advance();
                Ok(self.make_token(TokenKind::RParen, start))
            }
            '=' => {
                self.advance();
                Ok(self.make_token(TokenKind::Equals, start))
            }
            '#' => self.lex_comment(start),
            '<' => self.lex_full_iri(start),
            '"' => self.lex_quoted_string(start),
            '@' => self.lex_lang_tag(start),
            '^' => self.lex_datatype_sep(start),
            '_' if self.peek2() == Some(':') => self.lex_node_id(start),
            ':' => {
                // Default-prefix bare namespace ":"
                self.advance(); // consume ':'
                if self.peek().map_or(false, is_pn_local_start) {
                    let local = self.read_pn_local();
                    Ok(self.make_token(
                        TokenKind::PrefixedName(PrefixedIriRef { prefix: None, local }),
                        start,
                    ))
                } else {
                    Ok(self.make_token(TokenKind::Namespace(None), start))
                }
            }
            c if is_name_start(c) => self.lex_name_or_prefixed(start),
            c if c.is_ascii_digit() => self.lex_integer(start),
            c => {
                self.advance();
                Err(ParseError::UnexpectedChar { ch: c, span: Span::at(start) })
            }
        }
    }

    // ── Lexer sub-routines ────────────────────────────────────────────────────

    fn lex_comment(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // consume '#'
        let mut text = String::new();
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            text.push(c);
            self.advance();
        }
        Ok(self.make_token(TokenKind::Comment(text.trim().to_owned()), start))
    }

    fn lex_full_iri(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // consume '<'
        let mut iri = String::new();
        loop {
            match self.peek() {
                None => return Err(ParseError::UnclosedString { span: Span::at(start) }),
                Some('>') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.advance() {
                        Some(c) => iri.push(c),
                        None => return Err(ParseError::UnclosedString { span: Span::at(start) }),
                    }
                }
                Some(c) => {
                    iri.push(c);
                    self.advance();
                }
            }
        }
        Ok(self.make_token(TokenKind::FullIri(iri), start))
    }

    fn lex_quoted_string(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // consume '"'
        let mut s = String::new();
        loop {
            match self.peek() {
                None => return Err(ParseError::UnclosedString { span: Span::at(start) }),
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    let escaped = match self.advance() {
                        None => {
                            return Err(ParseError::UnclosedString { span: Span::at(start) })
                        }
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some('"') => '"',
                        Some('\\') => '\\',
                        Some('u') => self.read_unicode_escape(start, 4)?,
                        Some('U') => self.read_unicode_escape(start, 8)?,
                        Some(c) => c,
                    };
                    s.push(escaped);
                }
                Some(c) => {
                    s.push(c);
                    self.advance();
                }
            }
        }
        Ok(self.make_token(TokenKind::QuotedString(s), start))
    }

    fn read_unicode_escape(&mut self, start: Position, digits: usize) -> Result<char, ParseError> {
        let mut code = 0u32;
        for _ in 0..digits {
            match self.advance() {
                Some(c) if c.is_ascii_hexdigit() => {
                    code = code * 16 + c.to_digit(16).unwrap();
                }
                _ => {
                    return Err(ParseError::InvalidArgument {
                        message: "invalid unicode escape".to_owned(),
                        span: Span::at(start),
                    });
                }
            }
        }
        char::from_u32(code).ok_or_else(|| ParseError::InvalidArgument {
            message: format!("invalid unicode code point U+{code:X}"),
            span: Span::at(start),
        })
    }

    fn lex_lang_tag(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // consume '@'
        let mut tag = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '-' {
                tag.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if tag.is_empty() {
            return Err(ParseError::InvalidArgument {
                message: "empty language tag".to_owned(),
                span: Span::at(start),
            });
        }
        Ok(self.make_token(TokenKind::LangTag(tag), start))
    }

    fn lex_datatype_sep(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // consume first '^'
        match self.peek() {
            Some('^') => {
                self.advance();
                Ok(self.make_token(TokenKind::DataTypeSep, start))
            }
            Some(c) => Err(ParseError::UnexpectedChar { ch: c, span: self.current_span() }),
            None => Err(ParseError::UnexpectedEof { expected: "^" }),
        }
    }

    fn lex_node_id(&mut self, start: Position) -> Result<Token, ParseError> {
        self.advance(); // '_'
        self.advance(); // ':'
        let name = self.read_pn_local();
        Ok(self.make_token(TokenKind::NodeId(name), start))
    }

    /// Lex a bare name, which may be:
    /// - `prefix:local` → `PrefixedName`
    /// - `prefix:` (no local) → `Namespace`
    /// - `name` (no colon) → `Name`
    /// - a non-negative integer → `Integer`
    fn lex_name_or_prefixed(&mut self, start: Position) -> Result<Token, ParseError> {
        let name = self.read_name();

        match self.peek() {
            Some(':') => {
                self.advance(); // consume ':'
                // Decide: namespace or prefixed name
                if self.peek().map_or(false, is_pn_local_start) {
                    let local = self.read_pn_local();
                    Ok(self.make_token(
                        TokenKind::PrefixedName(PrefixedIriRef {
                            prefix: Some(name),
                            local,
                        }),
                        start,
                    ))
                } else {
                    Ok(self.make_token(TokenKind::Namespace(Some(name)), start))
                }
            }
            _ => Ok(self.make_token(TokenKind::Name(name), start)),
        }
    }

    fn lex_integer(&mut self, start: Position) -> Result<Token, ParseError> {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let n: u32 = s.parse().map_err(|_| ParseError::InvalidArgument {
            message: format!("integer {s:?} out of range"),
            span: Span::at(start),
        })?;
        Ok(self.make_token(TokenKind::Integer(n), start))
    }

    // ── String readers ────────────────────────────────────────────────────────

    fn read_name(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if is_name_char(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    /// Read the local part of a prefixed name (after `prefix:`).
    fn read_pn_local(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if is_pn_local_char(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }

    fn current_span(&self) -> Span {
        let p = self.current_position();
        Span::at(p)
    }
}

// ── Character classification ──────────────────────────────────────────────────

fn is_name_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_pn_local_start(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '%' || c == '\\'
}

fn is_pn_local_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '.' | ':' | '%' | '\\')
}
