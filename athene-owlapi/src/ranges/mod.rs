//!
//! This module provides the types to support OWL 2 Data Ranges.
//!
//! ![**Figure 6**. Data Ranges in OWL 2](https://www.w3.org/TR/owl2-syntax/C_datarange.gif)
//!

use crate::{
    entities::Datatype,
    error::ApiError,
    fmt::DisplayPretty,
    literals::Literal,
    values::{CardinalityConstraintViolation, UnlimitedNatural},
};
use core::str::FromStr;
use rdftk_iri::Iri;
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Datatypes, such as *xsd:string* or *xsd:integer*, and literals such as *"1"^^xsd:integer*,
/// can be used to express data ranges — sets of tuples of literals, where tuples consisting of
/// only one literal are identified with the literal itself. Each data range is associated with
/// a positive arity, which determines the size of the tuples in the data range. All datatypes
/// have arity one. This specification currently does not define data ranges of arity more than
/// one; however, by allowing for n-ary data ranges, the syntax of OWL 2 provides a "hook"
/// allowing implementations to introduce extensions such as comparisons and arithmetic.
///
/// Data ranges can be used in restrictions on data properties, as discussed in Sections 8.4 and
/// 8.5. The structure of data ranges in OWL 2 is shown in Figure 6. The simplest data ranges
/// are datatypes. The **DataIntersectionOf**, **DataUnionOf**, and **DataComplementOf** data
/// ranges provide for the standard set-theoretic operations on data ranges; in logical languages
/// these are usually called conjunction, disjunction, and negation, respectively. The
/// **DataOneOf** data range consists of exactly the specified set of literals. Finally, the
/// **DatatypeRestriction** data range restricts the value space of a datatype by a constraining
/// facet.
///
/// ## Specification (Section §7 -- Data Ranges)
///
/// ```bnf
/// DataRange :=
///     Datatype | DataIntersectionOf |
///     DataUnionOf | DataComplementOf |
///     DataOneOf | DatatypeRestriction
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum DataRange {
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataComplementOf(DataComplementOf),
    DataOneOf(DataOneOf),
    Datatype(Datatype),
    DatatypeRestriction(DatatypeRestriction),
}

///
/// An intersection data range $DataIntersectionOf( DR_1 \cdots DR_n )$ contains all tuples of
/// literals that are contained in each data range $DR_i$ for $1 \leq i \leq n$. All data ranges
/// $DR_i$ must be of the same arity, and the resulting data range is of that arity as well.
///
/// ## Specification (Section §7.1 -- Intersection of Data Ranges)
///
/// ```bnf
/// DataIntersectionOf :=
///     'DataIntersectionOf' '('
///         DataRange DataRange { DataRange }
///     ')'
/// ```
///
/// ## Example
///
/// The following data range contains exactly the integer 0:
///
/// ```owl
/// DataIntersectionOf( xsd:nonNegativeInteger xsd:nonPositiveInteger )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataIntersectionOf {
    arity: UnlimitedNatural,
    data_ranges: Vec<DataRange>, // 2..*
}

///
/// A union data range $DataUnionOf( DR_1 \cdots DR_n )$ contains all tuples of literals that are
/// contained in the at least one data range $DR_i$ for $1 \leq i \leq n$. All data ranges $DR_i$
/// must be of the same arity, and the resulting data range is of that arity as well.
///
/// ## Specification (Section §7.2 -- Union of Data Ranges)
///
/// ```bnf
/// DataUnionOf :=
///     'DataUnionOf' '('
///         DataRange DataRange { DataRange }
///     ')'
/// ```
///
/// ## Example
///
/// The following data range contains all strings and all integers:
///
/// ```owl
/// DataUnionOf( xsd:string xsd:integer )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataUnionOf {
    arity: UnlimitedNatural,
    data_ranges: Vec<DataRange>, // 2..*
}

///
/// A complement data range $DataComplementOf( DR )$ contains all tuples of literals that are not
/// contained in the data range $DR$. The resulting data range has the arity equal to the arity
/// of $DR$.
///
/// ## Specification (Section §7.3 -- Complement of Data Ranges)
///
/// ```bnf
/// DataComplementOf :=
///     'DataComplementOf' '(' DataRange ')'
/// ```
///
/// ## Example
///
/// The following complement data range contains literals that are not positive integers. In
/// particular, this data range contains the integer zero and all negative integers; however,
/// it also contains all strings (since strings are not positive integers).
///
/// ```owl
/// DataComplementOf( xsd:positiveInteger )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataComplementOf {
    arity: UnlimitedNatural,
    data_range: Box<DataRange>,
}

///
/// An enumeration of literals $DataOneOf( lt_1 \cdots lt_n )$ contains exactly the explicitly
/// specified literals $lt_i$ with $1 \leq i \leq n$. The resulting data range has arity one.
///
/// ## Specification (Section §7.4 -- Enumeration of Literals)
///
/// ```bnf
/// DataOneOf :=
///     'DataOneOf' '('
///         Literal { Literal }
///     ')'
/// ```
///
/// ## Example
///
/// The following data range contains exactly two literals: the string `"Peter"` and the integer one.
///
/// ```owl
/// DataOneOf( "Peter" "1"^^xsd:integer )
/// ```
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataOneOf {
    arity: UnlimitedNatural,
    literals: Vec<Literal>, // 1..*
}

///
/// A datatype restriction $DatatypeRestriction( DT F_1 lt_1 \cdots F_n lt_n )$ consists of a unary
/// datatype $DT$ and $n$ pairs $( F_i , lt_i )$. The resulting data range is unary and is obtained
/// by restricting the value space of $DT$ according to the semantics of all $( F_i , v_i )$
/// (multiple pairs are interpreted conjunctively), where $v_i$ are the data values of the literals
/// $lt_i$.
///
/// In an OWL 2 DL ontology, each pair $( F_i , v_i )$ must be contained in the facet space of $DT$
/// (see Section 4).
///
/// ## Specification (Section §7.5 -- Datatype Restrictions)
///
/// ```bnf
/// DatatypeRestriction := 'DatatypeRestriction' '(' Datatype constrainingFacet restrictionValue { constrainingFacet restrictionValue } ')'
///
/// constrainingFacet := IRI
///
/// restrictionValue := Literal
/// ```
///
/// ## Example
///
/// The following data range contains exactly the integers 5, 6, 7, 8, and 9:
///
/// ```owl
/// DatatypeRestriction(
///     xsd:integer
///     xsd:minInclusive "5"^^xsd:integer
///     xsd:maxExclusive "10"^^xsd:integer
/// )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DatatypeRestriction {
    arity: UnlimitedNatural,
    datatype: Datatype,
    restrictions: Vec<FacetRestriction>, // 1..*
}

///
/// This corresponds to the repeating pair `(constrainingFacet, restrictionValue)` in the rule
/// [`DatatypeRestriction`]. Note that this is not a production in the source grammar.
///
/// ## Specification (Section §7.5 -- Datatype Restrictions)
///
/// ```bnf
/// __FacetRestriction := constrainingFacet restrictionValue
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct FacetRestriction {
    constraining_facet: Iri,
    restriction_value: Literal,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Traits
// ------------------------------------------------------------------------------------------------

///
/// Trait for all `DataRange` types that have an `arity` field, see Section §7 -- Data Ranges.
///
pub trait HasArity {
    fn arity(&self) -> UnlimitedNatural;
}

///
/// Trait for all `DataRange` types that have a `dataRange` field (with a cardinality
/// of `1`), see Section §7 -- Data Ranges.
///
pub trait HasDataRange {
    fn data_range(&self) -> &DataRange;
    fn data_range_mut(&mut self) -> &mut DataRange;
}

///
/// Trait for all `DataRange` types that have a `dataRanges` field (with a cardinality
/// of `1..*`), see Section §7 -- Data Ranges.
///
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

impl_from_for_variant!(DataRange, DataComplementOf);
impl_from_for_variant!(DataRange, DataIntersectionOf);
impl_from_for_variant!(DataRange, DataUnionOf);
impl_from_for_variant!(DataRange, DataOneOf);
impl_from_for_variant!(DataRange, Datatype);
impl_from_for_variant!(DataRange, DatatypeRestriction);

impl HasArity for DataRange {
    fn arity(&self) -> UnlimitedNatural {
        match self {
            Self::DataComplementOf(v) => v.arity(),
            Self::DataIntersectionOf(v) => v.arity(),
            Self::DataUnionOf(v) => v.arity(),
            Self::DataOneOf(v) => v.arity(),
            Self::Datatype(v) => v.arity(),
            Self::DatatypeRestriction(v) => v.arity(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementation ❯ DataComplementOf
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataComplementOf(data_range));

impl HasArity for DataComplementOf {
    fn arity(&self) -> UnlimitedNatural {
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
    fn arity(&self) -> UnlimitedNatural {
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
                UnlimitedNatural::Limited(data_ranges.len() as u128),
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
    fn arity(&self) -> UnlimitedNatural {
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
                UnlimitedNatural::Limited(data_ranges.len() as u128),
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
    fn arity(&self) -> UnlimitedNatural {
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
                UnlimitedNatural::Limited(literals.len() as u128),
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
    fn arity(&self) -> UnlimitedNatural {
        self.arity
    }
}

impl DatatypeRestriction {
    pub fn new<I: IntoIterator<Item = FacetRestriction>>(
        datatype: Datatype,
        restrictions: I,
    ) -> Result<Self, ApiError> {
        let restrictions: Vec<FacetRestriction> = restrictions.into_iter().collect();

        if !restrictions.is_empty() {
            Ok(Self {
                arity: Default::default(),
                datatype,
                restrictions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(restrictions.len() as u128),
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
