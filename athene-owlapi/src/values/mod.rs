//!
//! This module provides certain *value type* required by the OWL 2 specification, namely
//! [`Natural`] and [`UnlimitedNatural`], and cardinality constraints defined by relationships
//! in the specification.
//!
//! The type `UnlimitedNatural` (which implies `Natural`) is described in the UML diagrams
//! as the type of the common field `arity` on data ranges. However, as it does not appear
//! in the grammar there is no representation specified and therefore no syntax for
//! *unbounded* other than the UML notation adopted by the types herein.
//!
//! Both the grammar and UML model make clear the cardinality of relationships through
//! different notation, `1`, `0..1`, `1..*`, `2..*`, etc. These are captured in
//! `CardinalityConstraint` instances which can then be used to quickly test collection
//! types and which will return a `CardinalityConstraintViolation` error if the collection
//! is out of bounds.
//!

use crate::{
    error::ApiError,
    fmt::{DisplayPretty, Indenter},
};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    str::FromStr,
};
use num_traits::{One, Zero};
use rdftk_iri::{Iri, IriPrefixMap, IriRef, LocalName, Name, Namespace, PrefixedName};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{format, string::ToString, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// While no upper bound is defined on the natural number in the OWL 2 specification the
/// choice to limit the number to 128 bits here seems like enough for about most any
/// reasonable usage.
///
pub type Natural = u128;

///
/// An `UnlimitedNatural` is either unlimited, i.e. can be any value without any constraint
/// whatsoever, or limited to a specific value.
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumTryAs)]
pub enum UnlimitedNatural {
    ///
    /// The value is *unlimited*, or in some circumstances *unknown*.
    ///
    #[default]
    Unlimited,
    ///
    /// The value is limited to the specified `Natural`.
    ///
    Limited(Natural),
}

///
/// Represents a constraint on some element's cardinality.
///
/// Usually this is a collection type and while the collection itself has no bounds
/// checking this represents the bounds allowed by the specification for the collection
/// in the context of it's usage.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumIs, EnumTryAs)]
pub enum CardinalityConstraint {
    /// Unlimited, there is no upper or lower bounds.
    Unlimited(RangeFull),
    /// The lower, or minimum, bound has been set but no upper bound has been set.
    MinLimited(RangeFrom<Natural>),
    /// The upper, or maximum, bound has been set but no lower bound has been set.
    MaxLimited(RangeTo<Natural>),
    /// The lower, or minimum, bound **and**, the upper, or maximum, bound has been set.
    MinMaxLimited(Range<Natural>),
    /// The collection must have **exactly** this number of elements.
    Exactly(Natural),
}

///
/// A simple enum used by [`CardinalityConstraintViolation`] to denote the cause of
/// a constraint violation.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIs)]
pub enum CardinalityBound {
    /// Lower, or Minimum, bound exceeded.
    Min,
    /// Upper, or Maximum, bound exceeded.
    Max,
    /// Exact bound not met.
    Exact,
}

///
/// An error type used to signal a violation of a [`CardinalityConstraint`].
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardinalityConstraintViolation {
    bound: CardinalityBound,
    expecting: Natural,
    given: UnlimitedNatural,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Iri
// ------------------------------------------------------------------------------------------------

// By hand, these are not local types.
impl DisplayPretty for Iri {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        _: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult {
        write!(
            f,
            "{}",
            if let Some(name) = prefix_map.compress(self) {
                name.to_string()
            } else {
                self.to_string()
            }
        )
    }
}

// By hand, these are not local types.
impl DisplayPretty for IriRef {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult {
        match self {
            IriRef::Iri(iri) => iri.fmt_pretty(f, indenter, prefix_map),
            IriRef::PrefixedName(prefixed_name) => {
                prefixed_name.fmt_pretty(f, indenter, prefix_map)
            }
        }
    }
}

impl_display_pretty!(Name only self);
impl_display_pretty!(Namespace only self);
impl_display_pretty!(LocalName only self);
impl_display_pretty!(PrefixedName only self);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ UnboundedNatural
// ------------------------------------------------------------------------------------------------

const UNLIMITED_STR: &str = "*";
const UNLIMITED_MIN_STR: &str = "*..";
const UNLIMITED_MAX_STR: &str = "..*";
const LIMITED_STR: &str = "..";

impl Display for UnlimitedNatural {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Unlimited => UNLIMITED_STR.to_string(),
                Self::Limited(v) => v.to_string(),
            }
        )
    }
}

impl FromStr for UnlimitedNatural {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == UNLIMITED_STR {
            Ok(Self::Unlimited)
        } else {
            Ok(Self::Limited(u128::from_str(s).map_err(|e| {
                ApiError::ValueParser("UnlimitedNatural", e.to_string(), s.to_string())
            })?))
        }
    }
}

impl From<u128> for UnlimitedNatural {
    fn from(value: u128) -> Self {
        Self::Limited(value)
    }
}

impl UnlimitedNatural {
    #[inline(always)]
    pub fn unbounded() -> Self {
        Self::Unlimited
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::Limited(0)
    }

    #[inline(always)]
    pub fn is_zero(&self) -> Option<bool> {
        self.try_as_limited().map(|v| v.is_zero())
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::Limited(1)
    }

    #[inline(always)]
    pub fn is_one(&self) -> Option<bool> {
        self.try_as_limited().map(|v| v.is_one())
    }

    pub fn value(&self) -> Option<u128> {
        match self {
            Self::Unlimited => None,
            Self::Limited(v) => Some(*v),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CardinalityConstraint
// ------------------------------------------------------------------------------------------------

impl Display for CardinalityConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            if f.alternate() {
                match self {
                    Self::Unlimited(_) => "zero or more".to_string(),
                    Self::MinLimited(v) => format!("at least {}", v.start),
                    Self::MaxLimited(v) => format!("at most {}", v.end),
                    Self::MinMaxLimited(v) => format!("between {} and {}", v.start, v.end),
                    Self::Exactly(v) => format!("exactly {v}"),
                }
            } else {
                match self {
                    Self::Unlimited(_) => UNLIMITED_STR.to_string(),
                    Self::MinLimited(v) => format!("{}{UNLIMITED_MAX_STR}", v.start),
                    Self::MaxLimited(v) => format!("{UNLIMITED_MIN_STR}{}", v.end),
                    Self::MinMaxLimited(v) => format!("{}{LIMITED_STR}{}", v.start, v.end),
                    Self::Exactly(v) => format!("{v}"),
                }
            }
        )
    }
}

impl From<RangeFull> for CardinalityConstraint {
    fn from(range: RangeFull) -> Self {
        Self::Unlimited(range)
    }
}

impl From<RangeFrom<Natural>> for CardinalityConstraint {
    fn from(range: RangeFrom<Natural>) -> Self {
        Self::MinLimited(range)
    }
}

impl From<RangeTo<Natural>> for CardinalityConstraint {
    fn from(range: RangeTo<Natural>) -> Self {
        Self::MaxLimited(range)
    }
}

impl From<Range<Natural>> for CardinalityConstraint {
    fn from(range: Range<Natural>) -> Self {
        Self::MinMaxLimited(range)
    }
}

impl From<Natural> for CardinalityConstraint {
    fn from(range: Natural) -> Self {
        Self::Exactly(range)
    }
}

impl FromStr for CardinalityConstraint {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Self::Unlimited(RangeFull))
        } else if let Some(bound) = s.strip_suffix(UNLIMITED_MAX_STR) {
            Ok(Self::MinLimited(
                u128::from_str(bound).map_err(|e| {
                    ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                })?..,
            ))
        } else if let Some(bound) = s.strip_prefix(UNLIMITED_MIN_STR) {
            Ok(Self::MaxLimited(
                ..u128::from_str(bound).map_err(|e| {
                    ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                })?,
            ))
        } else {
            let parts = s.split(LIMITED_STR).collect::<Vec<_>>();
            if parts.len() == 1 {
                Ok(Self::Exactly(u128::from_str(s).map_err(|e| {
                    ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                })?))
            } else if parts.len() == 2 {
                Ok(Self::MinMaxLimited(
                    u128::from_str(s).map_err(|e| {
                        ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                    })?..u128::from_str(s).map_err(|e| {
                        ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                    })?,
                ))
            } else {
                Err(ApiError::ValueParser(
                    "CardinalityConstraint",
                    "badly formatted min/max constraint".to_string(),
                    s.to_string(),
                ))
            }
        }
    }
}

impl CardinalityConstraint {
    /// Construct a new, unlimited constraint; this allows **any** number of elements.
    #[inline(always)]
    pub fn unlimited() -> Self {
        Self::Unlimited(RangeFull)
    }

    /// Construct a new, zero-limited constraint; this allows **zero** elements.
    #[inline(always)]
    pub fn zero() -> Self {
        Self::Exactly(0)
    }

    /// Construct a new, zero-or-one constraint; this allows optional elements.
    #[inline(always)]
    pub fn zero_or_one() -> Self {
        Self::MinMaxLimited(0..1)
    }

    /// Construct a new, zero-or-more constraint; this allows optionally many elements.
    #[inline(always)]
    pub fn zero_or_more() -> Self {
        Self::MinLimited(0..)
    }

    /// Construct a new, one-limited constraint; this requires an element.
    #[inline(always)]
    pub fn one() -> Self {
        Self::Exactly(1)
    }

    /// Construct a new, one-or-more constraint; this requires one or many elements.
    #[inline(always)]
    pub fn one_or_more() -> Self {
        Self::MinLimited(1..)
    }

    /// Construct a new, two-limited constraint; this requires exactly two elements.
    #[inline(always)]
    pub fn two() -> Self {
        Self::Exactly(2)
    }

    /// Construct a new, two-or-more constraint; this requires two or many elements.
    #[inline(always)]
    pub fn two_or_more() -> Self {
        Self::MinLimited(2..)
    }

    pub fn assert_valid_length(&self, length: usize) -> Result<(), ApiError> {
        self.assert_valid(UnlimitedNatural::Limited(length as Natural))
    }

    pub fn min_limit(&self) -> Option<Natural> {
        match self {
            Self::Unlimited(_) => None,
            Self::MinLimited(range) => Some(range.start),
            Self::MaxLimited(_) => None,
            Self::MinMaxLimited(range) => Some(range.start),
            Self::Exactly(min) => Some(*min),
        }
    }

    pub fn max_limit(&self) -> Option<Natural> {
        match self {
            Self::Unlimited(_) => None,
            Self::MinLimited(_) => None,
            Self::MaxLimited(range) => Some(range.end),
            Self::MinMaxLimited(range) => Some(range.end),
            Self::Exactly(min) => Some(*min),
        }
    }

    pub fn assert_valid(&self, value: UnlimitedNatural) -> Result<(), ApiError> {
        match self {
            Self::Unlimited(_) => Ok(()),
            Self::MinLimited(expecting) => {
                if let UnlimitedNatural::Limited(given) = value
                    && expecting.contains(&given)
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::min_fail(expecting.start, value).into())
                }
            }
            Self::MaxLimited(expecting) => {
                if let UnlimitedNatural::Limited(given) = value
                    && expecting.contains(&given)
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::max_fail(expecting.end, value).into())
                }
            }
            Self::MinMaxLimited(expecting) => match value {
                UnlimitedNatural::Unlimited => todo!(),
                UnlimitedNatural::Limited(given) => {
                    if given < expecting.start {
                        Err(CardinalityConstraintViolation::min_fail(expecting.start, value).into())
                    } else if given > expecting.end {
                        Err(CardinalityConstraintViolation::max_fail(expecting.end, value).into())
                    } else {
                        Ok(())
                    }
                }
            },
            Self::Exactly(expecting) => {
                if let UnlimitedNatural::Limited(given) = value
                    && given == *expecting
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::exact_fail(*expecting, value).into())
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CardinalityBound
// ------------------------------------------------------------------------------------------------

impl Display for CardinalityBound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Min => "at least",
                Self::Max => "at most",
                Self::Exact => "exactly",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ CardinalityConstraintViolation
// ------------------------------------------------------------------------------------------------

impl Display for CardinalityConstraintViolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Constrant Violation: cardinality of `{}` does not meet the expected constraint of *`{}`* `{}`.",
            self.given, self.bound, self.expecting
        )
    }
}

impl core::error::Error for CardinalityConstraintViolation {}

impl CardinalityConstraintViolation {
    #[inline(always)]
    pub fn fail(bound: CardinalityBound, expecting: Natural, given: UnlimitedNatural) -> Self {
        Self {
            bound,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn min_fail(expecting: Natural, given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn max_fail(expecting: Natural, given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_fail(expecting: Natural, given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn min_zero_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn max_zero_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_zero_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn min_one_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting: 1,
            given,
        }
    }

    #[inline(always)]
    pub fn max_one_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting: 1,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_one_fail(given: UnlimitedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting: 1,
            given,
        }
    }
}
