//!
//! This module provides The `Literal` type and a number of conversions and constructors.
//!

use crate::{
    entities::Datatype,
    fmt::{DisplayPretty, Indenter},
    syntax::DELIM_LITERAL_DATATYPE,
    things::{rdf, xsd},
};
use core::fmt::{Display, Formatter, Result as FmtResult};
use rdftk_iri::{Iri, IriPrefixMap};

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Literals represent data values such as particular strings or integers. They are analogous
/// to typed RDF literals and can also be understood as individuals denoting data values.
///
/// Each literal consists of a lexical form, which is a string, and a datatype; the datatypes
/// supported in OWL 2 are described in more detail in Section 4. A literal consisting of a
/// lexical form `"abc"`` and a datatype identified by the IRI `datatypeIRI`` is written as
/// `"abc"^^datatypeIRI`. Furthermore, literals whose datatype is `rdf:PlainLiteral` can be
/// abbreviated in functional-style syntax ontology documents as plain RDF literals. These
/// abbreviations are purely syntactic shortcuts and are thus not reflected in the structural
/// specification of OWL 2. The observable behavior of OWL 2 implementation must be as if
/// these shortcuts were expanded during parsing.
///
/// * Literals of the form `"abc@"^^rdf:PlainLiteral` should be abbreviated in functional-style
///   syntax ontology documents to `"abc"` whenever possible.
/// * Literals of the form `"abc@langTag"^^rdf:PlainLiteral` where `"langTag"` is not empty
///   should be abbreviated in functional-style syntax documents to `"abc"@langTag` whenever
///   possible.
///
/// The lexical form of each literal occurring in an OWL 2 DL ontology must belong to the lexical
/// space of the literal's datatype.
///
/// ## Specification (Section §5.7 -- Literals)
///
/// ```owl
/// Literal := typedLiteral | stringLiteralNoLanguage | stringLiteralWithLanguage
///
/// typedLiteral := lexicalForm '^^' Datatype
///
/// lexicalForm := quotedString
///
/// stringLiteralNoLanguage := quotedString
///
/// stringLiteralWithLanguage := quotedString languageTag
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    lexical_form: String,
    datatype: Datatype,
    plain: bool, // saves IRI comparisons later.
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations Macro
// ------------------------------------------------------------------------------------------------

macro_rules! impl_into_literal {
    ($from_type:ident, $xsd_type:ident) => {
        impl From<$from_type> for $crate::literals::Literal {
            fn from(value: $from_type) -> Self {
                Self::from(&value)
            }
        }

        impl From<&$from_type> for $crate::literals::Literal {
            fn from(value: &$from_type) -> Self {
                Self::new(
                    value.to_string(),
                    ::rdftk_iri::vocab::VOCABULARY_XML_SCHEMA
                        .iri_as_iri()
                        .with_new_fragment(stringify!($xsd_type)),
                )
            }
        }
    };
    (copy $from_type:ident, $xsd_type:ident) => {
        impl From<$from_type> for $crate::literals::Literal {
            fn from(value: $from_type) -> Self {
                Self::new(
                    value.to_string(),
                    ::rdftk_iri::vocab::VOCABULARY_XML_SCHEMA
                        .iri_as_iri()
                        .with_new_fragment("boolean"),
                )
            }
        }

        impl From<&$from_type> for $crate::literals::Literal {
            fn from(value: &$from_type) -> Self {
                Self::from(*value)
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Literal
// ------------------------------------------------------------------------------------------------

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_pretty(
            f,
            &crate::fmt::Indenter::default(),
            &::rdftk_iri::map::IriPrefixMap::default(),
        )
    }
}

impl DisplayPretty for Literal {
    fn fmt_pretty(&self, f: &mut Formatter<'_>, _: &Indenter, _: &IriPrefixMap) -> FmtResult {
        write!(f, "{:?}", self.lexical_form)?;
        if !self.plain {
            write!(
                f,
                "{DELIM_LITERAL_DATATYPE}{DELIM_LITERAL_DATATYPE}{}",
                self.datatype
            )?;
        }
        Ok(())
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self::plain(value)
    }
}

impl From<&String> for Literal {
    fn from(value: &String) -> Self {
        Self::plain(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::plain(value)
    }
}

impl_into_literal!(copy bool, boolean);

impl_into_literal!(copy i8, byte);
impl_into_literal!(copy u8, unsignedByte);
impl_into_literal!(copy i16, short);
impl_into_literal!(copy u16, unsignedShort);
impl_into_literal!(copy i32, int);
impl_into_literal!(copy u32, unsignedInt);
impl_into_literal!(copy i64, long);
impl_into_literal!(copy u64, unsignedLong);

impl_into_literal!(copy f32, float);
impl_into_literal!(copy f64, double);

impl_into_literal!(Iri, anyURI);

impl Literal {
    pub fn new<S: Into<String>, T: Into<Datatype>>(lexical_form: S, datatype: T) -> Self {
        let datatype = datatype.into();
        let plain = datatype.entity_iri() == &rdf::plain_literal_iri();
        Self {
            lexical_form: lexical_form.into(),
            datatype,
            plain,
        }
    }

    ///
    /// See [rdf:PlainLiteral: A Datatype for RDF Plain Literals (Second Edition)](https://www.w3.org/TR/rdf-plain-literal/).
    ///
    pub fn plain<S: Into<String>>(lexical_form: S) -> Self {
        Self {
            lexical_form: lexical_form.into(),
            datatype: Datatype::new(rdf::plain_literal_iri()),
            plain: true,
        }
    }

    pub fn hex_binary(bytes: &[u8]) -> Self {
        Self {
            lexical_form: bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<String>(),
            datatype: Datatype::new(xsd::hex_binary()),
            plain: false,
        }
    }
}
