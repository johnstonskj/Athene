use crate::{
    entities::Datatype,
    error::ApiError,
    fmt::DisplayPretty,
    literals::Literal,
    values::{CardinalityConstraintViolation, UnboundedNatural},
};
use core::str::FromStr;
use rdftk_iri::Iri;
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum DataRange {
    DataComplementOf(DataComplementOf),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataOneOf(DataOneOf),
    Datatype(Datatype),
    DatatypeRestriction(DatatypeRestriction),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataComplementOf {
    arity: UnboundedNatural,
    data_range: Box<DataRange>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataIntersectionOf {
    arity: UnboundedNatural,
    data_ranges: Vec<DataRange>, // 2..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataUnionOf {
    arity: UnboundedNatural,
    data_ranges: Vec<DataRange>, // 2..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataOneOf {
    arity: UnboundedNatural,
    literals: Vec<Literal>, // 1..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct DatatypeRestriction {
    arity: UnboundedNatural,
    datatype: Datatype,
    restrictions: Vec<FacetRestriction>, // 1..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct FacetRestriction {
    constraining_facet: Iri,
    restriction_value: Literal,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasArity {
    fn arity(&self) -> UnboundedNatural;
}

pub trait HasDataRange {
    fn data_range(&self) -> &DataRange;
    fn data_range_mut(&mut self) -> &mut DataRange;
}

pub trait HasDataRanges {
    fn has_data_ranges(&self) -> bool;
    fn data_ranges(&self) -> impl Iterator<Item = &DataRange>;
    fn data_ranges_mut(&mut self) -> impl Iterator<Item = &mut DataRange>;
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataRange
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataRange enum DataComplementOf,
    DataIntersectionOf,
    DataUnionOf,
    DataOneOf,
    Datatype,
    DatatypeRestriction
);

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataComplementOf
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataComplementOf(data_range));

impl HasArity for DataComplementOf {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl HasDataRange for DataComplementOf {
    fn data_range(&self) -> &DataRange {
        &self.data_range
    }
    fn data_range_mut(&mut self) -> &mut DataRange {
        &mut self.data_range
    }
}

impl DataComplementOf {
    pub fn new(data_range: DataRange) -> Self {
        Self {
            arity: Default::default(),
            data_range: Box::new(data_range),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataIntersectionOf
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataIntersectionOf( @list data_ranges ));

impl HasArity for DataIntersectionOf {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl HasDataRanges for DataIntersectionOf {
    fn has_data_ranges(&self) -> bool {
        !self.data_ranges.is_empty()
    }
    fn data_ranges(&self) -> impl Iterator<Item = &DataRange> {
        self.data_ranges.iter()
    }
    fn data_ranges_mut(&mut self) -> impl Iterator<Item = &mut DataRange> {
        self.data_ranges.iter_mut()
    }
}

impl DataIntersectionOf {
    pub fn new<I: IntoIterator<Item = DataRange>>(data_ranges: I) -> Result<Self, ApiError> {
        let data_ranges: Vec<DataRange> = data_ranges.into_iter().collect();

        if data_ranges.len() >= 2 {
            Ok(Self {
                arity: Default::default(),
                data_ranges,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnboundedNatural::Bounded(data_ranges.len() as u128),
            )
            .into())
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataUnionOf
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataUnionOf( @list data_ranges ));

impl HasArity for DataUnionOf {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl HasDataRanges for DataUnionOf {
    fn has_data_ranges(&self) -> bool {
        !self.data_ranges.is_empty()
    }
    fn data_ranges(&self) -> impl Iterator<Item = &DataRange> {
        self.data_ranges.iter()
    }
    fn data_ranges_mut(&mut self) -> impl Iterator<Item = &mut DataRange> {
        self.data_ranges.iter_mut()
    }
}

impl DataUnionOf {
    pub fn new<I: IntoIterator<Item = DataRange>>(data_ranges: I) -> Result<Self, ApiError> {
        let data_ranges: Vec<DataRange> = data_ranges.into_iter().collect();

        if data_ranges.len() >= 2 {
            Ok(Self {
                arity: Default::default(),
                data_ranges,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnboundedNatural::Bounded(data_ranges.len() as u128),
            )
            .into())
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataOneOf
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataOneOf( @list literals ));

impl HasArity for DataOneOf {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl DataOneOf {
    pub fn new<I: IntoIterator<Item = Literal>>(literals: I) -> Result<Self, ApiError> {
        let literals: Vec<Literal> = literals.into_iter().collect();

        if literals.len() >= 2 {
            Ok(Self {
                arity: Default::default(),
                literals,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnboundedNatural::Bounded(literals.len() as u128),
            )
            .into())
        }
    }

    pub fn has_literals(&self) -> bool {
        !self.literals.is_empty()
    }

    pub fn literals(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }

    pub fn literals_mut(&mut self) -> impl Iterator<Item = &mut Literal> {
        self.literals.iter_mut()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DatatypeRestriction
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DatatypeRestriction( datatype, @list restrictions ));

impl HasArity for DatatypeRestriction {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl DatatypeRestriction {
    pub fn new<I: IntoIterator<Item = FacetRestriction>>(
        datatype: Datatype,
        restrictions: I,
    ) -> Result<Self, ApiError> {
        let restrictions: Vec<FacetRestriction> = restrictions.into_iter().collect();

        if restrictions.len() >= 1 {
            Ok(Self {
                arity: Default::default(),
                datatype,
                restrictions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnboundedNatural::Bounded(restrictions.len() as u128),
            )
            .into())
        }
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    pub fn has_restrictions(&self) -> bool {
        !self.restrictions.is_empty()
    }

    pub fn restrictions(&self) -> impl Iterator<Item = &FacetRestriction> {
        self.restrictions.iter()
    }

    pub fn restrictions_mut(&mut self) -> impl Iterator<Item = &mut FacetRestriction> {
        self.restrictions.iter_mut()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ FacetRestriction
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(FacetRestriction(constraining_facet, restriction_value));

impl FacetRestriction {
    pub fn new(constraining_facet: Iri, restriction_value: Literal) -> Self {
        Self {
            constraining_facet,
            restriction_value,
        }
    }

    pub fn new_min_inclusive(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#minInclusive")
                .unwrap(),
            restriction_value,
        }
    }

    // usize
    pub fn new_min_length(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#minLength")
                .unwrap(),
            restriction_value,
        }
    }

    // usize
    pub fn new_max_length(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#maxLength")
                .unwrap(),
            restriction_value,
        }
    }

    // usize
    pub fn new_total_digits(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#totalDigits")
                .unwrap(),
            restriction_value,
        }
    }

    // usize
    pub fn new_fraction_digits(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#fractionDigits")
                .unwrap(),
            restriction_value,
        }
    }

    // {required|prohibited|optional}
    pub fn new_explicit_timezone(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#explicitTimezone")
                .unwrap(),
            restriction_value,
        }
    }

    // array of strings?
    pub fn new_pattern(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#pattern").unwrap(),
            restriction_value,
        }
    }

    // usize
    pub fn new_length(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#length").unwrap(),
            restriction_value,
        }
    }

    pub fn new_max_inclusive(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#maxInclusive")
                .unwrap(),
            restriction_value,
        }
    }

    pub fn new_min_exclusive(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#minExclusive")
                .unwrap(),
            restriction_value,
        }
    }

    pub fn new_max_exclusive(restriction_value: Literal) -> Self {
        Self {
            constraining_facet: Iri::from_str("http://www.w3.org/2001/XMLSchema#maxExclusive")
                .unwrap(),
            restriction_value,
        }
    }

    pub fn constraining_facet(&self) -> &Iri {
        &self.constraining_facet
    }

    pub fn restriction_value(&self) -> &Literal {
        &self.restriction_value
    }
}
