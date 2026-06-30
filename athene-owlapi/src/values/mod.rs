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

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
pub type Natural = u128;

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIs, EnumTryAs)]
pub enum UnlimitedNatural {
    #[default]
    Unlimited,
    Limited(Natural),
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CardinalityConstraint {
    #[default]
    Unlimited,
    MinLimited(Natural),
    MaxLimited(Natural),
    MinMaxLimited(Natural, Natural),
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

impl Add<Natural> for UnlimitedNatural {
    type Output = Option<Self>;

    fn add(self, rhs: Natural) -> Self::Output {
        match self {
            Self::Unlimited => None,
            Self::Limited(lhs) => Some(Self::Limited(lhs + rhs)),
        }
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
            match self {
                Self::Unlimited => UNLIMITED_STR.to_string(),
                Self::MinLimited(v) => format!("{v}{UNLIMITED_MAX_STR}"),
                Self::MaxLimited(v) => format!("{UNLIMITED_MIN_STR}{v}"),
                Self::MinMaxLimited(v, k) => format!("{v}{LIMITED_STR}{k}"),
                Self::Exactly(v) => format!("{v}"),
            }
        )
    }
}

impl FromStr for CardinalityConstraint {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            Ok(Self::Unlimited)
        } else if let Some(bound) = s.strip_suffix(UNLIMITED_MAX_STR) {
            Ok(Self::MinLimited(u128::from_str(bound).map_err(|e| {
                ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
            })?))
        } else if let Some(bound) = s.strip_prefix(UNLIMITED_MIN_STR) {
            Ok(Self::MaxLimited(u128::from_str(bound).map_err(|e| {
                ApiError::ValueParser("CardinalityConstraint", e.to_string(), s.to_string())
            })?))
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
        Self::Unlimited
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::Exactly(0)
    }

    #[inline(always)]
    pub fn zero_or_one() -> Self {
        Self::MinMaxLimited(0, 1)
    }

    #[inline(always)]
    pub fn zero_or_more() -> Self {
        Self::MinLimited(0)
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::Exactly(1)
    }

    #[inline(always)]
    pub fn one_or_more() -> Self {
        Self::MinLimited(1)
    }

    #[inline(always)]
    pub fn two() -> Self {
        Self::Exactly(2)
    }

    #[inline(always)]
    pub fn two_or_more() -> Self {
        Self::MinLimited(2)
    }

    pub fn is_bounded(&self) -> bool {
        !matches!(self, Self::Unlimited)
    }

    pub fn validate(&self, value: UnlimitedNatural) -> Result<(), ApiError> {
        match self {
            Self::Unlimited => Ok(()),
            Self::MinLimited(expecting) => {
                if let UnlimitedNatural::Limited(given) = value
                    && given >= *expecting
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::min_fail(*expecting, value).into())
                }
            }
            Self::MaxLimited(expecting) => {
                if let UnlimitedNatural::Limited(given) = value
                    && given <= *expecting
                {
                    Ok(())
                } else {
                    Err(CardinalityConstraintViolation::max_fail(*expecting, value).into())
                }
            }
            Self::MinMaxLimited(expecting_min, expecting_max) => match value {
                UnlimitedNatural::Unlimited => todo!(),
                UnlimitedNatural::Limited(given) => {
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

impl std::error::Error for CardinalityConstraintViolation {}

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
