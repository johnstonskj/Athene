use crate::{
    reader::{
        ReaderError,
        ast::{Span, SyntaxNode},
        lexer::Token,
    },
    syntax::FN_PREFIX,
    values::CardinalityConstraint,
};
use ariadne::{Cache, Color, Fmt, Label, Report, ReportKind, Source};
use core::fmt::{Debug, Display};
use rdftk_iri::error::Error as IriError;
use std::collections::BTreeMap;
use strum::EnumProperty;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NoOpReporter;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct InteractiveReporter {
    sources: SimpleSourceCache,
}

pub(crate) trait Reporter {
    // --------------------------------------------------------------------------------------------
    // Lexer Errors
    // --------------------------------------------------------------------------------------------

    // unexpected_char, unexpected_eof, unclosed_string

    // --------------------------------------------------------------------------------------------
    // Parser Errors
    // --------------------------------------------------------------------------------------------

    fn unexpected_token(&self, token: &Token, expected: &[&str]) -> ReaderError;
    fn missing_ast_node(&self, span: &Span, expected: &[&str]) -> ReaderError;
    fn unexpected_ast_node(&self, node: &SyntaxNode, expected: &[&str]) -> ReaderError;

    // --------------------------------------------------------------------------------------------
    // Conversion Errors
    // --------------------------------------------------------------------------------------------

    fn invalid_iri(&self, input: &str, error: IriError, span: Span) -> ReaderError;
    fn empty_language_tag(&self, span: Span) -> ReaderError;
    #[allow(dead_code)]
    fn invalid_language_tag(&self, input: &str, error: String, span: Span) -> ReaderError;
    fn invalid_namespace(&self, input: &str, error: String, span: Span) -> ReaderError;
    #[allow(dead_code)]
    fn invalid_name(&self, input: &str, error: String, span: Span) -> ReaderError;

    // --------------------------------------------------------------------------------------------
    // Semantics Errors
    // --------------------------------------------------------------------------------------------

    fn unknown_prefix(&self, input: &str, span: Span) -> ReaderError;
    fn using_reserved_prefix(&self, prefix: &str, span: Span) -> ReaderError;
    fn using_reserved_iri(&self, iri: &str, span: Span) -> ReaderError;

    fn unknown_function(&self, node: &SyntaxNode) -> ReaderError;
    fn unexpected_function(&self, node: &SyntaxNode, expected: &[&str]) -> ReaderError;
    fn argument_arity(
        &self,
        function: &str,
        got: usize,
        expected: CardinalityConstraint,
        span: Span,
    ) -> ReaderError;

    // invalid_argument_kind
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
struct SimpleSourceCache {
    sources: BTreeMap<String, Source<String>>,
}

// ------------------------------------------------------------------------------------------------
// Macros
// ------------------------------------------------------------------------------------------------

macro_rules! impl_reporter_method {
    (
        $fn_name:ident
        ( $( $arg_name:ident : $arg_type:ty ),+ ) =>
        $variant_name:ident
        { $( $field_name:ident $( : $field_value:expr )? ),+ }
    ) => {
        fn $fn_name(
            &self,
            $(
                $arg_name: $arg_type,
            )+
        ) -> ReaderError {
            ReaderError::$variant_name {
                $(
                    $field_name$(: $field_value )?,
                )+
            }
        }
    };
    (
        $fn_name:ident
        ( $( $arg_name:ident : $arg_type:ty ),+ ) =>
        ( $report_fn:expr ) =>
        $variant_name:ident
        { $( $field_name:ident $( : $field_value:expr )? ),+ }
    ) => {
        fn $fn_name(
            &self,
            $(
                $arg_name: $arg_type,
            )+
        ) -> ReaderError {
            let __error = ReaderError::$variant_name {
                $(
                    $field_name$(: $field_value )?,
                )+
            };
            let code = __error.get_int("code").expect("could not extract error code");
            let report = $report_fn(self, code, $( &$arg_name, )+);
            report.eprint(self.sources.clone()).unwrap();
            __error
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Reporter for NoOpReporter {
    fn unexpected_function(&self, node: &SyntaxNode, expected: &[&str]) -> ReaderError {
        if let Some(name) = node.function_name() {
            ReaderError::UnexpectedFunction {
                got: name.to_string(),
                span: node.span,
                expected: expected.iter().map(|s| s.to_string()).collect(),
            }
        } else {
            self.unexpected_ast_node(node, &["DataRange"])
        }
    }

    fn unknown_function(&self, node: &SyntaxNode) -> ReaderError {
        if let Some(name) = node.function_name() {
            ReaderError::UnknownFunction {
                name: name.to_string(),
                span: node.span,
            }
        } else {
            self.unexpected_ast_node(node, &["DataRange"])
        }
    }

    impl_reporter_method!(
        invalid_iri(input: &str, error: IriError, span: Span)
        => InvalidIri {input: input.to_owned(), error: error.to_string(), span }
    );

    impl_reporter_method!(
        unknown_prefix(prefix: &str, span: Span)
        => UnknownPrefix { prefix: prefix.to_string(), span }
    );

    impl_reporter_method!(empty_language_tag(span: Span) => EmptyLanguageTag { span });

    impl_reporter_method!(
        invalid_language_tag(input: &str, error: String, span: Span)
        => InvalidLanguageTag { input: input.to_string(), error, span }
    );

    impl_reporter_method!(
        invalid_namespace(input: &str, error: String, span: Span)
        => InvalidNamespace { input: input.to_string(), error, span }
    );

    impl_reporter_method!(
        invalid_name(input: &str, error: String, span: Span)
        => InvalidNamespace { input: input.to_string(), error, span }
    );

    impl_reporter_method!(
        argument_arity(function: &str, got: usize, expected: CardinalityConstraint, span: Span)
      => ArgumentArity { function: function.to_string(), got, expected, span }
    );

    impl_reporter_method!(
        using_reserved_prefix(prefix: &str, span: Span)
        => ReservedPrefix { prefix: prefix.to_string(), span }
    );

    impl_reporter_method!(
        using_reserved_iri(iri: &str, span: Span)
        => ReservedIri { iri: iri.to_string(), span }
    );

    impl_reporter_method!(
        unexpected_token(token: &Token, expected: &[&str])
        => UnexpectedToken {
            got: format!("{token:?}"),
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: token.span
        }
    );

    impl_reporter_method!(
        unexpected_ast_node(node: &SyntaxNode, expected: &[&str])
        => UnexpectedNode {
            got: format!("{node:?}"),
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: node.span
        }
    );

    impl_reporter_method!(
        missing_ast_node(span: &Span, expected: &[&str])
        => MissingNode {
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: *span
        }
    );
}

// ------------------------------------------------------------------------------------------------

const COLOR_MATCH_INITIAL: Color = Color::Red;
const COLOR_MATCH_REFERENCE: Color = Color::Cyan;
const COLOR_MATCH_MORE: Color = Color::Magenta;
const COLOR_MATCH_URL: Color = Color::Blue;

impl Reporter for InteractiveReporter {
    fn unexpected_function(&self, node: &SyntaxNode, expected: &[&str]) -> ReaderError {
        if let Some(name) = node.function_name() {
            ReaderError::UnexpectedFunction {
                got: name.to_string(),
                span: node.span,
                expected: expected.iter().map(|s| s.to_string()).collect(),
            }
        } else {
            self.unexpected_ast_node(node, &["DataRange"])
        }
    }

    fn unknown_function(&self, node: &SyntaxNode) -> ReaderError {
        if let Some(name) = node.function_name() {
            ReaderError::UnknownFunction {
                name: name.to_string(),
                span: node.span,
            }
        } else {
            self.unexpected_ast_node(node, &["DataRange"])
        }
    }

    impl_reporter_method!(
        invalid_iri(input: &str, error: IriError, span: Span)
        => (
            | _, code, _: &&str, error: &IriError, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Invalid value for an IRI")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message("This cannot be parsed as an IRI value")
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "Parser error was: {}.",
                        error.to_string().fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => InvalidIri { input: input.to_string(), error: error.to_string(), span }
    );

    impl_reporter_method!(
        empty_language_tag(span: Span)
        => (
            | _, code, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("LanguageTag may not be empty")
                    .finish()
            }
        )
        => EmptyLanguageTag { span }
    );

    impl_reporter_method!(
        invalid_language_tag(input: &str, error: String, span: Span)
        => (
            | _, code, _: &&str, error: &String, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Invalid value for a LanguageTag")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message("This cannot be parsed as an LanguageTag value")
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "Parser error was: {}.",
                        error.to_string().fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => InvalidLanguageTag { input: input.to_string(), error: error.clone(), span }
    );

    impl_reporter_method!(
        invalid_namespace(input: &str, error: String, span: Span)
        => (
            | _, code, _: &&str, error: &String, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Invalid value for a namespace prefix")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message("This cannot be parsed as an namespace prefix")
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "Parser error was: {}.",
                        error.to_string().fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => InvalidNamespace { input: input.to_string(), error: error.clone(), span }
    );

    impl_reporter_method!(
        invalid_name(input: &str, error: String, span: Span)
        => (
            | _, code, _: &&str, error: &String, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Invalid value for an entity name")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message("This cannot be parsed as an entity name")
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "Parser error was: {}.",
                        error.to_string().fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => InvalidNamespace { input: input.to_string(), error: error.clone(), span }
    );

    impl_reporter_method!(
        argument_arity(function: &str, got: usize, expected: CardinalityConstraint, span: Span)
        => (
            | _, code, function: &&str, got: &usize, expected: &CardinalityConstraint, span: &Span| {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Incorrect number of arguments provided for function")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message(format!("function {function} requires {expected:#} arguments, but {got} were provided"))
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "`try running `hoot describe {}` for more information.",
                        function.to_string().fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => ArgumentArity { function: function.to_string(), got, expected: expected.clone(), span }
    );

    impl_reporter_method!(
        unknown_prefix(prefix_str: &str, span: Span)
        => (
            | _, code, prefix_str: &&str, span: &Span | {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Use of undeclared prefix")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message(
                                format!(
                                    "The prefix `{}` has not been previously declared.",
                                    prefix_str.fg(COLOR_MATCH_REFERENCE)
                                )
                            )
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "Did you forget to add a {} function for this ontology?",
                        FN_PREFIX.fg(COLOR_MATCH_MORE)
                    ))
                    .finish()
            }
        )
        => UnknownPrefix { prefix: prefix_str.to_string(), span }
    );

    impl_reporter_method!(
        using_reserved_prefix(prefix: &str, span: Span)
        => (
            | _, code, prefix: &&str, span: &Span | {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Cannot use a reserved prefix")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message(
                                format!(
                                    "The prefix {} may not be used as it is reserved by the OWL specification.",
                                    prefix.fg(COLOR_MATCH_REFERENCE)
                                )
                            )
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "See section 2.4 {} of the OWL 2 Structural Specification and Functional-Style Syntax",
                        "<https://www.w3.org/TR/owl2-syntax/#IRIs>".fg(COLOR_MATCH_URL)
                    ))
                    .finish()
            }
        )
        => ReservedPrefix { prefix: prefix.to_string(), span }
    );

    impl_reporter_method!(
        using_reserved_iri(iri: &str, span: Span)
        => (
            | _, code, _: &&str, span: &Span | {
                Report::build(ReportKind::Error, ("", span.char_range()))
                    .with_code(code)
                    .with_message("Cannot use a reserved IRI")
                    .with_label(
                        Label::new(("", span.char_range()))
                            .with_message("This IRI may not be used as it is reserved by the OWL specification.")
                            .with_color(COLOR_MATCH_INITIAL),
                    )
                    .with_note(format!(
                        "See section 2.4 {} of the OWL 2 Structural Specification and Functional-Style Syntax",
                        "<https://www.w3.org/TR/owl2-syntax/#IRIs>".fg(COLOR_MATCH_URL)
                    ))
                    .finish()
            }
        )
        => ReservedIri { iri: iri.to_string(), span }
    );

    fn unexpected_token(&self, token: &Token, expected: &[&str]) -> ReaderError {
        Report::build(ReportKind::Error, ("", token.span.char_range()))
            .with_code(3)
            .with_message("Found an unexpected token")
            .with_label(
                Label::new(("", token.span.char_range()))
                    .with_message(format!(
                        "This is a token of type {}",
                        token.kind.as_ref().fg(COLOR_MATCH_INITIAL)
                    ))
                    .with_color(COLOR_MATCH_INITIAL),
            )
            .with_note(format!(
                "Expecting {}{}.",
                if expected.len() == 1 { "a " } else { "one of " },
                expected
                    .iter()
                    .map(|s| s.fg(COLOR_MATCH_MORE).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .finish()
            .eprint(self.sources.clone())
            .unwrap();
        ReaderError::UnexpectedToken {
            got: format!("{token:?}"),
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: token.span,
        }
    }

    fn unexpected_ast_node(&self, node: &SyntaxNode, expected: &[&str]) -> ReaderError {
        Report::build(ReportKind::Error, ("", node.span.char_range()))
            .with_code(3)
            .with_message("Unexpected AST Node")
            .with_label(
                Label::new(("", node.span.char_range()))
                    .with_message(format!(
                        "This is a node of type {}",
                        node.kind.as_ref().fg(COLOR_MATCH_INITIAL)
                    ))
                    .with_color(COLOR_MATCH_INITIAL),
            )
            .with_note(format!(
                "Expecting {}{}.",
                if expected.len() == 1 { "a " } else { "one of " },
                expected
                    .iter()
                    .map(|s| s.fg(COLOR_MATCH_MORE).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .finish()
            .eprint(self.sources.clone())
            .unwrap();
        ReaderError::UnexpectedNode {
            got: format!("{node:?}"),
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: node.span,
        }
    }

    fn missing_ast_node(&self, span: &Span, expected: &[&str]) -> ReaderError {
        Report::build(ReportKind::Error, ("", span.char_range()))
            .with_code(3)
            .with_message("Missing AST Node")
            .with_note(format!(
                "Was expecting {}{}.",
                if expected.len() == 1 { "a " } else { "one of " },
                expected
                    .iter()
                    .map(|s| s.fg(COLOR_MATCH_MORE).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .finish()
            .eprint(self.sources.clone())
            .unwrap();
        ReaderError::MissingNode {
            expected: expected.iter().map(|s| s.to_string()).collect(),
            span: *span,
        }
    }
}

impl InteractiveReporter {
    pub(crate) fn with_source(mut self, id: String, source: String) -> Self {
        self.sources = self.sources.with_source(id, source);
        self
    }
}

// ------------------------------------------------------------------------------------------------

impl Cache<&str> for SimpleSourceCache {
    type Storage = String;

    fn fetch(&mut self, id: &&str) -> Result<&Source<Self::Storage>, impl Debug> {
        self.sources.get(*id).ok_or(ReaderError::SourceNotFound {
            ontology: id.to_string(),
        })
    }

    #[allow(refining_impl_trait)]
    fn display<'a>(&self, id: &&'a str) -> Option<impl Display + 'a> {
        Some(*id)
    }
}

impl SimpleSourceCache {
    pub(crate) fn with_source(mut self, id: String, source: String) -> Self {
        self.sources.insert(id, Source::from(source));
        self
    }
}
