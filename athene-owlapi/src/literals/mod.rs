//!
//! This module provides The `Literal` type and a number of conversions and constructors.
//!

use crate::{
    entities::Datatype,
    fmt::{DisplayPretty, Indenter},
    syntax::DELIM_LITERAL_DATATYPE,
    things::{owl, rdf, xsd},
};
use core::fmt::{Display, Formatter, Result as FmtResult};
use rdftk_iri::{Iri, IriPrefixMap};
use strum::{Display, EnumIs};

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

#[cfg(feature = "lang-tags")]
use language_tags::LanguageTag;

#[cfg(feature = "date-time")]
use chrono::{DateTime, NaiveDateTime};

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

#[derive(Clone, Copy, Display, Debug, PartialEq, Eq, EnumIs)]
pub enum DateTimeParseError {
    InvalidNumberOfParts,
    InvalidPartLength(DateTimePart),
    InvalidNumberOfFragments(DateTimePart),
    InvalidFragmentLength(DateTimeFragment),
    InvalidFragmentFormat(DateTimeFragment),
    MissingTimezoneOffset,
}

#[derive(Clone, Copy, Display, Debug, PartialEq, Eq, EnumIs)]
pub enum DateTimePart {
    Date,
    Time,
    TimeZone,
}

#[derive(Clone, Copy, Display, Debug, PartialEq, Eq, EnumIs)]
pub enum DateTimeFragment {
    Year,
    Month,
    Day,
    Hours,
    Minutes,
    Seconds,
    SecondDecimals,
    EndOfDay,
    Timezone,
    TimezoneHours,
    TimezoneMinutes,
}

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
                        .with_new_fragment(stringify!($xsd_type)),
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
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult {
        #[cfg(feature = "lang-tags")]
        fn is_language_tag(s: &str) -> bool {
            LanguageTag::parse(s).is_ok()
        }
        #[cfg(not(feature = "lang-tags"))]
        fn is_language_tag(s: &str) -> bool {
            s.split('-')
                .all(|s| s.chars().all(|c| c.is_ascii_alphanumeric()))
        }
        if self.plain {
            if let Some((_, lang_tag)) = self.lexical_form.rsplit_once('@')
                && is_language_tag(lang_tag)
            {
                write!(
                    f,
                    "{:?}@{}",
                    &self.lexical_form[..self.lexical_form.len() - lang_tag.len() - 1],
                    lang_tag
                )?;
            } else {
                write!(f, "{:?}", self.lexical_form)?;
            }
        } else {
            write!(
                f,
                "{:?}{DELIM_LITERAL_DATATYPE}{DELIM_LITERAL_DATATYPE}",
                self.lexical_form
            )?;
            self.datatype
                .entity_iri()
                .fmt_pretty(f, indenter, prefix_map)?;
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
impl_into_literal!(copy i128, integer);

impl_into_literal!(copy f32, float);
impl_into_literal!(copy f64, double);

impl_into_literal!(Iri, anyURI);

#[cfg(feature = "lang-tags")]
impl From<LanguageTag> for Literal {
    fn from(tag: LanguageTag) -> Self {
        Self::language(tag)
    }
}

#[cfg(feature = "date-time")]
impl From<NaiveDateTime> for Literal {
    fn from(dt: NaiveDateTime) -> Self {
        Self::date_time(dt.dt.format("%+"))
    }
}

#[cfg(feature = "date-time")]
impl From<DateTime<_>> for Literal {
    fn from(dt: NaiveDateTime<_>) -> Self {
        Self::date_time(dt.dt.format("%+"))
    }
}

impl Literal {
    pub fn new<S: Into<String>, T: Into<Datatype>>(lexical_form: S, datatype: T) -> Self {
        let datatype = datatype.into();
        let plain = datatype.entity_iri() == &rdf::plain_literal();
        Self {
            lexical_form: lexical_form.into(),
            datatype,
            plain,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"hello\"".to_string(),
    ///     Literal::plain("hello").to_string()
    /// );
    /// ```
    ///
    /// ## Specification
    ///
    /// See [rdf:PlainLiteral: A Datatype for RDF Plain Literals (Second Edition)](https://www.w3.org/TR/rdf-plain-literal/).
    ///
    pub fn plain<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(rdf::plain_literal()),
            plain: true,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    /// use language_tags::LanguageTag;
    ///
    /// assert_eq!(
    ///     "\"hello\"@en-GB".to_string(),
    ///     Literal::plain_in_language(
    ///         "hello",
    ///         LanguageTag::parse("en-GB").unwrap()
    ///     ).to_string()
    /// );
    /// ```
    ///
    /// ## Specification
    ///
    /// See [rdf:PlainLiteral: A Datatype for RDF Plain Literals (Second Edition)](https://www.w3.org/TR/rdf-plain-literal/).
    ///
    #[cfg(feature = "lang-tags")]
    pub fn plain_in_language<S>(value: S, language_tag: LanguageTag) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: format!("{}@{}", value.into(), language_tag),
            datatype: Datatype::new(rdf::plain_literal()),
            plain: true,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"true\"^^xsd:boolean".to_string(),
    ///     Literal::boolean(true).to_string()
    /// );
    /// assert_eq!(
    ///     "\"true\"^^xsd:boolean".to_string(),
    ///     Literal::from(true).to_string()
    /// );
    /// ```
    ///
    pub fn boolean(value: bool) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::boolean()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:byte".to_string(),
    ///     Literal::byte(21).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:byte".to_string(),
    ///     Literal::from(21_i8).to_string()
    /// );
    /// ```
    ///
    pub fn byte(value: i8) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::byte()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedByte".to_string(),
    ///     Literal::unsigned_byte(21).to_string(),
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedByte".to_string(),
    ///     Literal::from(21_u8).to_string()
    /// );
    /// ```
    ///
    pub fn unsigned_byte(value: u8) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::unsigned_byte()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:short".to_string(),
    ///     Literal::short(21_i16).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:short".to_string(),
    ///     Literal::from(21_i16).to_string()
    /// );
    /// ```
    ///
    pub fn short(value: i16) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::short()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedShort".to_string(),
    ///     Literal::unsigned_short(21_u16).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedShort".to_string(),
    ///     Literal::from(21_u16).to_string()
    /// );
    /// ```
    ///
    pub fn unsigned_short(value: u16) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::unsigned_short()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:int".to_string(),
    ///     Literal::int(21_i32).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:int".to_string(),
    ///     Literal::from(21_i32).to_string()
    /// );
    /// ```
    ///
    pub fn int(value: i32) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::int()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedInt".to_string(),
    ///     Literal::unsigned_int(21_u32).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedInt".to_string(),
    ///     Literal::from(21_u32).to_string()
    /// );
    /// ```
    ///
    pub fn unsigned_int(value: u32) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::unsigned_int()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:long".to_string(),
    ///     Literal::long(21_i64).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:long".to_string(),
    ///     Literal::from(21_i64).to_string()
    /// );
    /// ```
    ///
    pub fn long(value: i64) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::long()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedLong".to_string(),
    ///     Literal::unsigned_long(21_u64).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:unsignedLong".to_string(),
    ///     Literal::from(21_u64).to_string()
    /// );
    /// ```
    ///
    pub fn unsigned_long(value: u64) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::unsigned_long()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:integer".to_string(),
    ///     Literal::integer(21_i128).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:integer".to_string(),
    ///     Literal::from(21_i128).to_string()
    /// );
    /// ```
    ///
    pub fn integer(value: i128) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::integer()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:float".to_string(),
    ///     Literal::float(21.0_f32).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:float".to_string(),
    ///     Literal::from(21.0_f32).to_string()
    /// );
    /// ```
    ///
    pub fn float(value: f32) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::float()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21\"^^xsd:double".to_string(),
    ///     Literal::double(21.0_f64).to_string()
    /// );
    /// assert_eq!(
    ///     "\"21\"^^xsd:double".to_string(),
    ///     Literal::from(21.0_f64).to_string()
    /// );
    /// ```
    ///
    pub fn double(value: f64) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::double()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"21/200\"^^owl:rational".to_string(),
    ///     Literal::rational(21, 200).to_string()
    /// );
    /// ```
    ///
    pub fn rational(numerator: i64, denominator: u64) -> Self {
        assert!(denominator != 0);
        Self {
            lexical_form: format!("{numerator}/{denominator}"),
            datatype: Datatype::new(owl::rational()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"hello\"^^xsd:string".to_string(),
    ///     Literal::string("hello").to_string()
    /// );
    /// ```
    ///
    pub fn string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(xsd::string()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    /// use language_tags::LanguageTag;
    ///
    /// assert_eq!(
    ///     "\"hello@en\"^^xsd:string".to_string(),
    ///     Literal::string_in_language("hello", LanguageTag::parse("en").unwrap()).to_string()
    /// );
    /// ```
    ///
    #[cfg(feature = "lang-tags")]
    #[inline(always)]
    pub fn string_in_language<S>(value: S, language_tag: LanguageTag) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: format!("{}@{}", value.into(), language_tag),
            datatype: Datatype::new(xsd::string()),
            plain: false,
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

    pub fn token<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(xsd::token()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    /// use language_tags::LanguageTag;
    ///
    /// assert_eq!(
    ///     "\"en-US\"^^xsd:language".to_string(),
    ///     Literal::language(LanguageTag::parse("en-US").unwrap()).to_string()
    /// );
    /// ```
    ///
    pub fn language(value: LanguageTag) -> Self {
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::language()),
            plain: false,
        }
    }

    pub fn name<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(xsd::name()),
            plain: false,
        }
    }

    pub fn nc_name<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(xsd::nc_name()),
            plain: false,
        }
    }

    pub fn nm_token<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lexical_form: value.into(),
            datatype: Datatype::new(xsd::nm_token()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    /// use rdftk_iri::Iri;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     "\"https://example.com/ns/Foo\"^^xsd:anyURI".to_string(),
    ///     Literal::any_uri(Iri::from_str("https://example.com/ns/Foo").unwrap()).to_string()
    /// );
    /// ```
    ///
    pub fn any_uri(value: Iri) -> Self {
        Self {
            lexical_form: format!("{value:#}"),
            datatype: Datatype::new(xsd::any_uri()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    /// use rdftk_iri::Iri;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(
    ///     "\"https://example.com/ns/Foo\"^^xsd:anyURI".to_string(),
    ///     Literal::iri(Iri::from_str("https://example.com/ns/Foo").unwrap()).to_string()
    /// );
    /// ```
    ///
    pub fn iri(value: Iri) -> Self {
        Self {
            lexical_form: format!("{value:#}"),
            datatype: Datatype::new(xsd::any_uri()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"2023-10-01T12:00:00\"^^xsd:dateTime".to_string(),
    ///     Literal::date_time("2023-10-01T12:00:00").to_string()
    /// );
    /// ```
    ///
    /// See also `From<DateTime>` and `From<NaiveDateTime` if the feature flag `date-time`
    /// is set.
    ///
    pub fn date_time<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        let value = value.into();
        validate_date_time(&value, false).expect("Invalid xsd:dateTime value");
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::date_time()),
            plain: false,
        }
    }

    ///
    /// ## Example
    ///
    /// ```rust
    /// use athene_owlapi::literals::Literal;
    ///
    /// assert_eq!(
    ///     "\"2023-10-01T12:00:00-08:00\"^^xsd:dateTimeStamp".to_string(),
    ///     Literal::date_time_stamp("2023-10-01T12:00:00-08:00").to_string()
    /// );
    ///
    /// assert_eq!(
    ///     "\"2023-10-01T12:00:00Z\"^^xsd:dateTimeStamp".to_string(),
    ///     Literal::date_time_stamp("2023-10-01T12:00:00Z").to_string()
    /// );
    /// ```
    ///
    pub fn date_time_stamp<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        let value = value.into();
        validate_date_time(&value, true).expect("Invalid xsd:dateTimeStamp value");
        Self {
            lexical_form: value.to_string(),
            datatype: Datatype::new(xsd::date_time_stamp()),
            plain: false,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions ❯ xsd:dateTime/xsd:dateTimeStamp Validator
// ------------------------------------------------------------------------------------------------

const ZERO_TO_ONE: [char; 2] = ['0', '1'];
const ZERO_TO_TWO: [char; 3] = ['0', '1', '2'];
const ZERO_TO_THREE: [char; 4] = ['0', '1', '2', '3'];
const ZERO_TO_FIVE: [char; 6] = ['0', '1', '2', '3', '4', '5'];
const ONE_TO_NINE: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn str_take_one(s: &str) -> (char, &str) {
    let c = s.chars().next().unwrap();
    (c, &s[c.len_utf8()..])
}

//
// [16]   dateTimeLexicalRep ::=
//            yearFrag '-' monthFrag '-' dayFrag
//            'T' ((hourFrag ':' minuteFrag ':' secondFrag) | endOfDayFrag)
//            timezoneFrag?
//
// [44]   dateTimeStampLexicalRep ::=
//            yearFrag '-' monthFrag '-' dayFrag
//            'T' ((hourFrag ':' minuteFrag ':' secondFrag) | endOfDayFrag)
//            timezoneFrag
//
// [56]   yearFrag ::= '-'? (([1-9] digit digit digit+)) | ('0' digit digit digit))
// [57]   monthFrag ::= ('0' [1-9]) | ('1' [0-2])
// [58]   dayFrag ::= ('0' [1-9]) | ([12] digit) | ('3' [01])
// [59]   hourFrag ::= ([01] digit) | ('2' [0-3])
// [60]   minuteFrag ::= [0-5] digit
// [61]   secondFrag ::= ([0-5] digit) ('.' digit+)?
// [62]   endOfDayFrag ::= '24:00:00' ('.' '0'+)?
// [63]   timezoneFrag ::= 'Z' | ('+' | '-') (('0' digit | '1' [0-3]) ':' minuteFrag | '14:00')
//
fn validate_date_time(s: &str, timezone_offset_required: bool) -> Result<(), DateTimeParseError> {
    let date_time = s.split('T').collect::<Vec<_>>();
    if date_time.len() == 2 {
        let time_tzone = date_time[1]
            .split(|c| ['Z', '+', '-'].contains(&c))
            .collect::<Vec<_>>();
        match (time_tzone.len(), timezone_offset_required) {
            (1, false) => {
                validate_date_part(date_time[0])?;
                validate_time_part(time_tzone[0])
            }
            (2, _) => {
                validate_date_part(date_time[0])?;
                validate_time_part(time_tzone[0])?;
                validate_timezone_fragment(time_tzone[1])
            }
            _ => Err(DateTimeParseError::MissingTimezoneOffset),
        }
    } else {
        Err(DateTimeParseError::InvalidNumberOfParts)
    }
}

//
// yearFrag '-' monthFrag '-' dayFrag
//
fn validate_date_part(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() >= 10 {
        let parts = s.split('-').collect::<Vec<_>>();
        if parts.len() == 3 {
            validate_year_fragment(parts[0])?;
            validate_month_fragment(parts[1])?;
            validate_day_fragment(parts[2])
        } else {
            Err(DateTimeParseError::InvalidNumberOfFragments(
                DateTimePart::Date,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidPartLength(DateTimePart::Date))
    }
}

//
// [56]   yearFrag ::= '-'? (([1-9] digit digit digit+)) | ('0' digit digit digit))
//
fn validate_year_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() >= 4 {
        let s = if let Some(rest) = s.strip_prefix('-') {
            rest
        } else {
            s
        };
        let (first, rest) = str_take_one(s);

        if (ONE_TO_NINE.contains(&first) && rest.chars().all(|c| c.is_ascii_digit()))
            || (first == '0' && rest.chars().all(|c| ZERO_TO_THREE.contains(&c)) && s.len() == 4)
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Year,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Year,
        ))
    }
}

//
// [57]   monthFrag ::= ('0' [1-9]) | ('1' [0-2])
//
fn validate_month_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if (first == '0' && rest.chars().all(|c| ONE_TO_NINE.contains(&c)))
            || (first == '1' && rest.chars().all(|c| ZERO_TO_TWO.contains(&c)))
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Month,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Month,
        ))
    }
}

//
// [58]   dayFrag ::= ('0' [1-9]) | ([12] digit) | ('3' [01])
//
fn validate_day_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if (first == '0' && rest.chars().all(|c| ONE_TO_NINE.contains(&c)))
            || (first == '1' && rest.chars().all(|c| ZERO_TO_TWO.contains(&c)))
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Day,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Day,
        ))
    }
}

//
// ((hourFrag ':' minuteFrag ':' secondFrag) | endOfDayFrag)
//
fn validate_time_part(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() >= 8 {
        if s.starts_with("24") {
            validate_end_of_day_fragment(s)
        } else {
            let fragments = s.split(':').collect::<Vec<_>>();
            if fragments.len() == 3 {
                validate_hour_fragment(fragments[0])?;
                validate_minute_fragment(fragments[1], DateTimeFragment::Minutes)?;
                validate_second_fragment(fragments[2])
            } else {
                Err(DateTimeParseError::InvalidNumberOfFragments(
                    DateTimePart::Time,
                ))
            }
        }
    } else {
        Err(DateTimeParseError::InvalidPartLength(DateTimePart::Time))
    }
}

//
// [59]   hourFrag ::= ([01] digit) | ('2' [0-3])
//
fn validate_hour_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if (ZERO_TO_ONE.contains(&first) && rest.chars().all(|c| c.is_ascii_digit()))
            || (first == '2' && rest.chars().all(|c| ZERO_TO_THREE.contains(&c)))
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Hours,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Hours,
        ))
    }
}

//
// [60]   minuteFrag ::= [0-5] digit
//
fn validate_minute_fragment(s: &str, fragment: DateTimeFragment) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if ZERO_TO_FIVE.contains(&first) && rest.chars().all(|c| c.is_ascii_digit()) {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(fragment))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(fragment))
    }
}

//
// [61]   secondFrag ::= ([0-5] digit) ('.' digit+)?
//
fn validate_second_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if ZERO_TO_FIVE.contains(&first) && rest.chars().all(|c| c.is_ascii_digit()) {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Seconds,
            ))
        }
    } else if s.len() > 3 {
        let (first, rest) = str_take_one(s);
        let (next, rest) = str_take_one(rest);
        let (dot, rest) = str_take_one(rest);

        if ZERO_TO_FIVE.contains(&first)
            && next.is_ascii_digit()
            && dot == '.'
            && rest.chars().all(|c| c.is_ascii_digit())
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::Seconds,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Seconds,
        ))
    }
}

//
// [62]   endOfDayFrag ::= '24:00:00' ('.' '0'+)?
//
fn validate_end_of_day_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() > 8 {
        if let Some(rest) = s.strip_prefix("24:00:00") {
            match rest.len() {
                0 => Ok(()),
                1 => Err(DateTimeParseError::InvalidFragmentLength(
                    DateTimeFragment::Seconds,
                )),
                _ => {
                    let (first, rest) = str_take_one(s);
                    if first == '.' && rest.chars().all(|c| c == '0') {
                        Ok(())
                    } else {
                        Err(DateTimeParseError::InvalidFragmentFormat(
                            DateTimeFragment::EndOfDay,
                        ))
                    }
                }
            }
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::EndOfDay,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::EndOfDay,
        ))
    }
}

//
//        ,--- stripped ---,
// [63]   'Z' | ('+' | '-')  (('0' digit | '1' [0-3]) ':' minuteFrag | '14:00')
//
fn validate_timezone_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.is_empty()
    /* implies "Z" */
    {
        Ok(())
    } else if s.len() < 5 {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::Timezone,
        ))
    } else {
        let s = if s.starts_with('+') || s.starts_with('-') {
            &s[1..]
        } else {
            s
        };
        if s == "14:00" {
            Ok(())
        } else {
            let sub_fragments = s.split(':').collect::<Vec<_>>();
            if sub_fragments.len() == 2 {
                validate_timezone_hour_fragment(sub_fragments[0])?;
                validate_minute_fragment(sub_fragments[1], DateTimeFragment::TimezoneMinutes)
            } else {
                Err(DateTimeParseError::InvalidFragmentFormat(
                    DateTimeFragment::Timezone,
                ))
            }
        }
    }
}

// '0' digit | '1' [0-3]
fn validate_timezone_hour_fragment(s: &str) -> Result<(), DateTimeParseError> {
    if s.len() == 2 {
        let (first, rest) = str_take_one(s);

        if (first == '0' && rest.chars().all(|c| c.is_ascii_digit()))
            || (first == '1' && rest.chars().all(|c| ZERO_TO_THREE.contains(&c)))
        {
            Ok(())
        } else {
            Err(DateTimeParseError::InvalidFragmentFormat(
                DateTimeFragment::TimezoneHours,
            ))
        }
    } else {
        Err(DateTimeParseError::InvalidFragmentLength(
            DateTimeFragment::TimezoneHours,
        ))
    }
}
