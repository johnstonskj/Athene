use crate::{
    error::ApiError,
    fmt::{DisplayPretty, Indenter},
};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Add,
    str::FromStr,
};
use num_traits::{One, Zero};
use rdftk_iri::{Iri, IriPrefixMap, IriRef, LocalName, Name, Namespace, PrefixedName};
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type Natural = u128;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumTryAs)]
pub enum UnboundedNatural {
    #[default]
    Unbounded,
    Bounded(Natural),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CardinalityConstraint {
    #[default]
    Unbounded,
    MinBounded(Natural),
    MaxBounded(Natural),
    MinMaxBounded(Natural, Natural),
    Exactly(Natural),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardinalityBound {
    Min,
    Max,
    Exact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardinalityConstraintViolation {
    bound: CardinalityBound,
    expecting: Natural,
    given: UnboundedNatural,
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

const UNBOUNDED_STR: &str = "*";
const UNBOUNDED_MIN_STR: &str = "*..";
const UNBOUNDED_MAX_STR: &str = "..*";
const BOUNDED_STR: &str = "..";

impl Display for UnboundedNatural {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Unbounded => UNBOUNDED_STR.to_string(),
                Self::Bounded(v) => v.to_string(),
            }
        )
    }
}

impl FromStr for UnboundedNatural {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == UNBOUNDED_STR {
            Ok(Self::Unbounded)
        } else {
            Ok(Self::Bounded(u128::from_str(s).map_err(|e| {
                ApiError::ValueParser("UnboundedNatural", e.to_string(), s.to_string())
            })?))
        }
    }
}

impl From<u128> for UnboundedNatural {
    fn from(value: u128) -> Self {
        Self::Bounded(value)
    }
}

impl Add<Natural> for UnboundedNatural {
    type Output = Option<Self>;

    fn add(self, rhs: Natural) -> Self::Output {
        match self {
            Self::Unbounded => None,
            Self::Bounded(lhs) => Some(Self::Bounded(lhs + rhs)),
        }
    }
}

impl UnboundedNatural {
    #[inline(always)]
    pub fn unbounded() -> Self {
        Self::Unbounded
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::Bounded(0)
    }

    #[inline(always)]
    pub fn is_zero(&self) -> Option<bool> {
        self.try_as_bounded().map(|v| v.is_zero())
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::Bounded(1)
    }

    #[inline(always)]
    pub fn is_one(&self) -> Option<bool> {
        self.try_as_bounded().map(|v| v.is_one())
    }

    pub fn value(&self) -> Option<u128> {
        match self {
            Self::Unbounded => None,
            Self::Bounded(v) => Some(*v),
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
            match self {
                Self::Unbounded => UNBOUNDED_STR.to_string(),
                Self::MinBounded(v) => format!("{v}{UNBOUNDED_MAX_STR}"),
                Self::MaxBounded(v) => format!("{UNBOUNDED_MIN_STR}{v}"),
                Self::MinMaxBounded(v, k) => format!("{v}{BOUNDED_STR}{k}"),
                Self::Exactly(v) => format!("{v}"),
            }
        )
    }
}

impl FromStr for CardinalityConstraint {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Self::Unbounded)
        } else if let Some(bound) = s.strip_suffix(UNBOUNDED_MAX_STR) {
            Ok(Self::MinBounded(u128::from_str(bound).map_err(|e| {
                ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
            })?))
        } else if let Some(bound) = s.strip_prefix(UNBOUNDED_MIN_STR) {
            Ok(Self::MaxBounded(u128::from_str(bound).map_err(|e| {
                ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
            })?))
        } else {
            let parts = s.split(BOUNDED_STR).collect::<Vec<_>>();
            if parts.len() == 1 {
                Ok(Self::Exactly(u128::from_str(s).map_err(|e| {
                    ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                })?))
            } else if parts.len() == 2 {
                Ok(Self::MinMaxBounded(
                    u128::from_str(s).map_err(|e| {
                        ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
                    })?,
                    u128::from_str(s).map_err(|e| {
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
    #[inline(always)]
    pub fn unbounded() -> Self {
        Self::Unbounded
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::Exactly(0)
    }

    #[inline(always)]
    pub fn zero_or_one() -> Self {
        Self::MinMaxBounded(0, 1)
    }

    #[inline(always)]
    pub fn zero_or_more() -> Self {
        Self::MinBounded(0)
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::Exactly(1)
    }

    #[inline(always)]
    pub fn one_or_more() -> Self {
        Self::MinBounded(1)
    }

    #[inline(always)]
    pub fn two() -> Self {
        Self::Exactly(2)
    }

    #[inline(always)]
    pub fn two_or_more() -> Self {
        Self::MinBounded(2)
    }

    pub fn is_bounded(&self) -> bool {
        !matches!(self, Self::Unbounded)
    }

    pub fn validate(&self, value: UnboundedNatural) -> Result<(), ApiError> {
        match self {
            Self::Unbounded => Ok(()),
            Self::MinBounded(expecting) => {
                if let UnboundedNatural::Bounded(given) = value
                    && given >= *expecting
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::min_fail(*expecting, value).into())
                }
            }
            Self::MaxBounded(expecting) => {
                if let UnboundedNatural::Bounded(given) = value
                    && given <= *expecting
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::max_fail(*expecting, value).into())
                }
            }
            Self::MinMaxBounded(expecting_min, expecting_max) => match value {
                UnboundedNatural::Unbounded => todo!(),
                UnboundedNatural::Bounded(given) => {
                    if given < *expecting_min {
                        Err(CardinalityConstraintViolation::min_fail(*expecting_min, value).into())
                    } else if given > *expecting_max {
                        Err(CardinalityConstraintViolation::max_fail(*expecting_max, value).into())
                    } else {
                        Ok(())
                    }
                }
            },
            Self::Exactly(expecting) => {
                if let UnboundedNatural::Bounded(given) = value
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

impl std::error::Error for CardinalityConstraintViolation {}

impl CardinalityConstraintViolation {
    #[inline(always)]
    pub fn fail(bound: CardinalityBound, expecting: Natural, given: UnboundedNatural) -> Self {
        Self {
            bound,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn min_fail(expecting: Natural, given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn max_fail(expecting: Natural, given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_fail(expecting: Natural, given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting,
            given,
        }
    }

    #[inline(always)]
    pub fn min_zero_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn max_zero_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_zero_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting: 0,
            given,
        }
    }

    #[inline(always)]
    pub fn min_one_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Min,
            expecting: 1,
            given,
        }
    }

    #[inline(always)]
    pub fn max_one_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Max,
            expecting: 1,
            given,
        }
    }

    #[inline(always)]
    pub fn exact_one_fail(given: UnboundedNatural) -> Self {
        Self {
            bound: CardinalityBound::Exact,
            expecting: 1,
            given,
        }
    }
}
