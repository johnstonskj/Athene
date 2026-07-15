
use crate::{
    axioms::Axiom,
    annotations::{Annotation, HasAnnotations},
    expressions::{ClassExpression, DataPropertyExpression},
    fmt::DisplayPretty,
    ranges::DataRange,
};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;


// ------------------------------------------------------------------------------------------------
// Public Types ❯ Data Property Axioms
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 also provides for data property axioms.
///
/// ## Specification (Section §9.3)
///
/// ```bnf
/// DataPropertyAxiom :=
///     SubDataPropertyOf | EquivalentDataProperties |
///     DisjointDataProperties | DataPropertyDomain |
///     DataPropertyRange | FunctionalDataProperty
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum DataPropertyAxiom {
    SubDataPropertyOf(SubDataPropertyOf),
    DisjointDataProperties(DisjointDataProperties),
    EquivalentDataProperties(EquivalentDataProperties),
    FunctionalDataProperty(FunctionalDataProperty),
    DataPropertyDomain(DataPropertyDomain),
    DataPropertyRange(DataPropertyRange),
}

///
/// A data subproperty axiom $SubDataPropertyOf( DPE_1 \ DPE_2 )$ states that the data property
/// expression $DPE_1$ is a subproperty of the data property expression $DPE_2$ — that is, if an
/// individual $x$ is connected by $DPE_1$ to a literal $y$, then $x$ is connected by $DPE_2$ to
/// $y$ as well.
///
/// ## Specification (Section §9.3.1)
///
/// ```bnf
/// SubDataPropertyOf :=
///     'SubDataPropertyOf' '('
///         axiomAnnotations
///         subDataPropertyExpression superDataPropertyExpression
///     ')'
///
/// subDataPropertyExpression := DataPropertyExpression
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SubDataPropertyOf {
    axiom_annotations: Vec<Annotation>,
    sub_data_property_expression: DataPropertyExpression, // 1
    super_data_property_expression: DataPropertyExpression, // 1
}

///
/// An equivalent data properties axiom $EquivalentDataProperties( DPE_1 ... DPE_n )$ states that
/// all the data property expressions $DPE_i, 1 \leq i \leq n$, are semantically equivalent to each
/// other. This axiom allows one to use each $DPE_i$ as a synonym for each $DPE_j$ — that is, in any
/// expression in the ontology containing such an axiom, $DPE_i$ can be replaced with $DPE_j$ without
/// affecting the meaning of the ontology.
///
/// ## Specification (Section §9.3.2)
///
/// ```bnf
/// EquivalentDataProperties :=
///     'EquivalentDataProperties' '('
///         axiomAnnotations
///         DataPropertyExpression DataPropertyExpression { DataPropertyExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct EquivalentDataProperties {
    axiom_annotations: Vec<Annotation>,
    data_property_expressions: Vec<DataPropertyExpression>, // 2..*
}

///
/// A disjoint data properties axiom $DisjointDataProperties( DPE_1 ... DPE_n )$ states that all
/// of the data property expressions $DPE_i, 1 \leq i \leq n$, are pairwise disjoint; that is, no
/// individual $x$ can be connected to a literal $y$ by both $DPE_i$ and $DPE_j$ for $i \neq j$.
///
/// ## Specification (Section §9.3.3)
///
/// ```bnf
/// DisjointDataProperties :=
///     'DisjointDataProperties' '('
///         axiomAnnotations
///         DataPropertyExpression DataPropertyExpression { DataPropertyExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DisjointDataProperties {
    axiom_annotations: Vec<Annotation>,
    data_property_expressions: Vec<DataPropertyExpression>, // 2..*
}

///
/// A data property functionality axiom $FunctionalDataProperty( DPE )$ states that the data property
/// expression $DPE$ is functional — that is, for each individual $x$, there can be at most one
/// distinct literal $y$ such that $x$ is connected by $DPE$ with $y$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// FunctionalDataProperty :=
///     'FunctionalDataProperty' '('
///         axiomAnnotations
///         DataPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionalDataProperty {
    axiom_annotations: Vec<Annotation>,
    data_property_expression: DataPropertyExpression, // 1
}

///
/// A data property domain axiom $DataPropertyDomain( DPE \ CE )$ states that the domain of the data
/// property expression $DPE$ is the class expression $CE$ — that is, if an individual $x$ is
/// connected by $DPE$ with some literal, then $x$ is an instance of $CE$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// DataPropertyDomain :=
///     'DataPropertyDomain' '('
///         axiomAnnotations
///         DataPropertyExpression ClassExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataPropertyDomain {
    axiom_annotations: Vec<Annotation>,
    data_property_expression: DataPropertyExpression, // 1
    domain: ClassExpression,
}

///
/// A data property range axiom $DataPropertyRange( DPE \ DR )$ states that the range of the data
/// property expression $DPE$ is the data range $DR$ — that is, if some individual is connected
/// by $DPE$ with a literal $x$, then $x$ is in $DR$. The arity of $DR$ must be one.
///
/// ## Specification (Section §)
///
/// ```bnf
/// DataPropertyRange :=
///     'DataPropertyRange' '('
///         axiomAnnotations
///         DataPropertyExpression DataRange
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataPropertyRange {
    axiom_annotations: Vec<Annotation>,
    data_property_expression: DataPropertyExpression, // 1
    range: DataRange,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DataPropertyAxiom
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyAxiom enum SubDataPropertyOf,
    DisjointDataProperties,
    EquivalentDataProperties,
    FunctionalDataProperty,
    DataPropertyDomain,
    DataPropertyRange);

impl_has_annotations!(
    DataPropertyAxiom enum SubDataPropertyOf,
    DisjointDataProperties,
    EquivalentDataProperties,
    FunctionalDataProperty,
    DataPropertyDomain,
    DataPropertyRange);

impl_from_for_variant!(DataPropertyAxiom, SubDataPropertyOf);
impl_from_for_variant!(DataPropertyAxiom, DisjointDataProperties);
impl_from_for_variant!(DataPropertyAxiom, EquivalentDataProperties);
impl_from_for_variant!(DataPropertyAxiom, FunctionalDataProperty);
impl_from_for_variant!(DataPropertyAxiom, DataPropertyDomain);
impl_from_for_variant!(DataPropertyAxiom, DataPropertyRange);

impl_from_for_variant!(Axiom, DataPropertyAxiom ( from SubDataPropertyOf ) );
impl_from_for_variant!(Axiom, DataPropertyAxiom ( from DisjointDataProperties ) );
impl_from_for_variant!(Axiom, DataPropertyAxiom ( from EquivalentDataProperties ) );
impl_from_for_variant!(Axiom, DataPropertyAxiom ( from FunctionalDataProperty ) );
impl_from_for_variant!(Axiom, DataPropertyAxiom ( from DataPropertyDomain ) );
impl_from_for_variant!(Axiom, DataPropertyAxiom ( from DataPropertyRange ) );


// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubDataPropertyOf(
        @list axiom_annotations, sub_data_property_expression, super_data_property_expression
    )
);
impl_has_annotations!(SubDataPropertyOf, axiom_annotations);

impl SubDataPropertyOf {
    pub fn new<DPE1, DPE2>(sub_data_property_expression: DPE1, super_data_property_expression: DPE2) -> Self 
    where
        DPE1: Into<DataPropertyExpression>,
        DPE2: Into<DataPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            sub_data_property_expression,
            super_data_property_expression,
        )
    }

    pub fn new_with_annotations<IA, DPE1, DPE2>(
        axiom_annotations: IA,
        sub_data_property_expression: DPE1,
        super_data_property_expression: DPE2,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE1: Into<DataPropertyExpression>,
        DPE2: Into<DataPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            sub_data_property_expression: sub_data_property_expression.into(),
            super_data_property_expression: super_data_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DisjointDataProperties( @list axiom_annotations, @list data_property_expressions )
);
impl_has_annotations!(DisjointDataProperties, axiom_annotations);

impl DisjointDataProperties {
    pub fn new<IDPE>(data_property_expressions: IDPE) -> Self 
    where
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expressions,
        )
    }

    pub fn new_with_annotations<IA, IDPE>(
        axiom_annotations: IA,
        data_property_expressions: IDPE,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expressions: data_property_expressions.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    EquivalentDataProperties( @list axiom_annotations, @list data_property_expressions )
);
impl_has_annotations!(EquivalentDataProperties, axiom_annotations);

impl EquivalentDataProperties {
    pub fn new<IDPE>(data_property_expressions: IDPE) -> Self 
    where
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expressions,
        )
    }

    pub fn new_with_annotations<IA, IDPE>(
        axiom_annotations: IA,
        data_property_expressions: IDPE,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expressions: data_property_expressions.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    FunctionalDataProperty( @list axiom_annotations, data_property_expression )
);
impl_has_annotations!(FunctionalDataProperty, axiom_annotations);

impl FunctionalDataProperty {
    pub fn new<DPE>(data_property_expression: DPE) -> Self 
    where
        DPE: Into<DataPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expression,
        )
    }

    pub fn new_with_annotations<IA, DPE>(
        axiom_annotations: IA,
        data_property_expression: DPE,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE: Into<DataPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expression: data_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyDomain( @list axiom_annotations, data_property_expression, domain )
);
impl_has_annotations!(DataPropertyDomain, axiom_annotations);

impl DataPropertyDomain {
    pub fn new<DPE, CE>(data_property_expression: DPE, domain: CE) -> Self 
    where
        DPE: Into<DataPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expression,
            domain,
        )
    }

    pub fn new_with_annotations<IA, DPE, CE>(
        axiom_annotations: IA,
        data_property_expression: DPE,
        domain: CE,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE: Into<DataPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expression: data_property_expression.into(),
            domain: domain.into()
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyRange( @list axiom_annotations, data_property_expression, range )
);
impl_has_annotations!(DataPropertyRange, axiom_annotations);

impl DataPropertyRange {
    pub fn new<DPE, DR>(data_property_expression: DPE, range: DR) -> Self 
    where
        DPE: Into<DataPropertyExpression>,
        DR: Into<DataRange>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expression,
            range,
        )
    }

    pub fn new_with_annotations<IA, DPE, DR>(
        axiom_annotations: IA,
        data_property_expression: DPE,
        range: DR,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE: Into<DataPropertyExpression>,
        DR: Into<DataRange>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expression: data_property_expression.into(),
            range: range.into()
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}
