use crate::{
    annotations::{Annotation, HasAnnotations},
    axioms::Axiom,
    expressions::{ClassExpression, ObjectPropertyExpression},
    fmt::DisplayPretty,
};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Object Property Axioms
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 provides axioms that can be used to characterize and establish relationships between
/// object property expressions. For clarity, the structure of these axioms is shown in two
/// separate figures, Figure 14 and Figure 15. The **[SubObjectPropertyOf]** axiom allows one to
/// state that the extension of one object property expression is included in the extension of
/// another object property expression. The **[EquivalentObjectProperties]** axiom allows one to
/// state that the extensions of several object property expressions are the same. The
/// **[DisjointObjectProperties]** axiom allows one to state that the extensions of several object
/// property expressions are pairwise disjoint — that is, that they do not share pairs of connected
/// individuals. The **[InverseObjectProperties]** axiom can be used to state that two object
/// property expressions are the inverse of each other. The **[ObjectPropertyDomain]** and
/// **[ObjectPropertyRange]** axioms can be used to restrict the first and the second individual,
/// respectively, connected by an object property expression to be instances of the specified class
/// expression.
///
/// ![Figure 14. Object Property Axioms in OWL 2, Part I](https://www.w3.org/TR/owl2-syntax/A_objectproperty1.gif)
///
/// The **[FunctionalObjectProperty]** axiom allows one to state that an object property expression
/// is functional — that is, that each individual can have at most one outgoing connection of the
/// specified object property expression. The **[InverseFunctionalObjectProperty]** axiom allows
/// one to state that an object property expression is inverse-functional — that is, that each
/// individual can have at most one incoming connection of the specified object property expression.
/// Finally, the **[ReflexiveObjectProperty]**, **[IrreflexiveObjectProperty]**,
/// **[SymmetricObjectProperty]**, **[AsymmetricObjectProperty]**, and **[TransitiveObjectProperty]**
/// axioms allow one to state that an object property expression is reflexive, irreflexive, symmetric,
/// asymmetric, or transitive, respectively.
///
/// ![Figure 15. Axioms Defining Characteristics of Object Properties in OWL 2, Part II](https://www.w3.org/TR/owl2-syntax/A_objectproperty2.gif)
///
/// ## Specification (Section §9.2 -- Object Property Axioms)
///
/// ```bnf
/// ObjectPropertyAxiom :=
///     SubObjectPropertyOf | EquivalentObjectProperties |
///     DisjointObjectProperties | InverseObjectProperties |
///     ObjectPropertyDomain | ObjectPropertyRange |
///     FunctionalObjectProperty | InverseFunctionalObjectProperty |
///     ReflexiveObjectProperty | IrreflexiveObjectProperty |
///     SymmetricObjectProperty | AsymmetricObjectProperty |
///     TransitiveObjectProperty
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum ObjectPropertyAxiom {
    EquivalentObjectProperties(EquivalentObjectProperties),
    DisjointObjectProperties(DisjointObjectProperties),
    SubObjectPropertyOf(SubObjectPropertyOf),
    ObjectPropertyDomain(ObjectPropertyDomain),
    ObjectPropertyRange(ObjectPropertyRange),
    InverseObjectProperties(InverseObjectProperties),
    FunctionalObjectProperty(FunctionalObjectProperty),
    InverseFunctionalObjectProperty(InverseFunctionalObjectProperty),
    ReflexiveObjectProperty(ReflexiveObjectProperty),
    IrreflexiveObjectProperty(IrreflexiveObjectProperty),
    SymmetricObjectProperty(SymmetricObjectProperty),
    AsymmetricObjectProperty(AsymmetricObjectProperty),
    TransitiveObjectProperty(TransitiveObjectProperty),
}

///
/// Object subproperty axioms are analogous to subclass axioms, and they come in two forms.
///
/// The basic form is $SubObjectPropertyOf( OPE_1 \ OPE_2 )$. This axiom states that the object
/// property expression $OPE_1$ is a subproperty of the object property expression $OPE_2$ —
/// that is, if an individual $x$ is connected by $OPE_1$ to an individual $y$, then $x$ is
/// also connected by $OPE_2$ to $y$.
///
/// The more complex form is $SubObjectPropertyOf( ObjectPropertyChain( OPE_1 ... OPE_n ) OPE )$.
/// This axiom states that, if an individual $x$ is connected by a sequence of object property
/// expressions $OPE_1, ..., OPE_n$ with an individual $y$, then $x$ is also connected with
/// $y$ by the object property expression $OPE$. Such axioms are also known as complex role
/// inclusions.
///
/// ## Specification (Section §9.2.1)
///
/// ```bnf
/// SubObjectPropertyOf :=
///     'SubObjectPropertyOf' '('
///         axiomAnnotations
///         subObjectPropertyExpression superObjectPropertyExpression
///     ')'
///
/// subObjectPropertyExpression :=
///     ObjectPropertyExpression | propertyExpressionChain
///
/// propertyExpressionChain :=
///     'ObjectPropertyChain' '('
///         ObjectPropertyExpression ObjectPropertyExpression { ObjectPropertyExpression }
///     ')'
///
/// superObjectPropertyExpression := ObjectPropertyExpression
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SubObjectPropertyOf {
    axiom_annotations: Vec<Annotation>,
    sub_object_property_expressions: SubObjectPropertyExpression,
    super_object_property_expression: ObjectPropertyExpression,
}

///
/// Realization of the interior rule `subObjectPropertyExpression`.
///
/// ```bnf
/// subObjectPropertyExpression :=
///     ObjectPropertyExpression | propertyExpressionChain
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum SubObjectPropertyExpression {
    ObjectPropertyExpression(ObjectPropertyExpression),
    PropertyExpressionChain(PropertyExpressionChain),
}

///
/// Realization of the interior rule `propertyExpressionChain`.
///
/// ```bnf
/// propertyExpressionChain :=
///     'ObjectPropertyChain' '('
///         ObjectPropertyExpression ObjectPropertyExpression { ObjectPropertyExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyExpressionChain {
    object_property_expressions: Vec<ObjectPropertyExpression>, // 2..* {ordered,nonunique}
}

///
/// An equivalent object properties axiom $EquivalentObjectProperties( OPE_1 ... OPE_n )$
/// states that all of the object property expressions $OPE_i, 1 \leq i \leq n$, are
/// semantically equivalent to each other. This axiom allows one to use each $OPE_i$ as a
/// synonym for each $OPE_j$ — that is, in any expression in the ontology containing such
/// an axiom, $OPE_i$ can be replaced with $OPE_j$ without affecting the meaning of the
/// ontology.
///
/// ## Specification (Section §9.2.2)
///
/// ```bnf
/// EquivalentObjectProperties :=
///     'EquivalentObjectProperties' '('
///         axiomAnnotations
///         ObjectPropertyExpression ObjectPropertyExpression { ObjectPropertyExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct EquivalentObjectProperties {
    axiom_annotations: Vec<Annotation>,
    object_property_expressions: Vec<ObjectPropertyExpression>,
}

///
/// A disjoint object properties axiom $DisjointObjectProperties( OPE_1 ... OPE_n )$ states
/// that all of the object property expressions $OPE_i, 1 \leq i \leq n$, are pairwise
/// disjoint; that is, no individual $x$ can be connected to an individual $y$ by both
/// $OPE_i$ and $OPE_j$ for $i \neq j$.
///
/// ## Specification (Section §9.2.3)
///
/// ```bnf
/// DisjointObjectProperties :=
///     'DisjointObjectProperties' '('
///         axiomAnnotations
///         ObjectPropertyExpression ObjectPropertyExpression { ObjectPropertyExpression }
///     ')'
/// ```
///

#[derive(Clone, Debug, PartialEq)]
pub struct DisjointObjectProperties {
    axiom_annotations: Vec<Annotation>,
    object_property_expressions: Vec<ObjectPropertyExpression>,
}

///
/// An inverse object properties axiom $InverseObjectProperties( OPE_1 \ OPE_2 )$ states
/// that the object property expression $OPE_1$ is an inverse of the object property
/// expression $OPE_2$. Thus, if an individual $x$ is connected by $OPE_1$ to an individual
/// $y$, then $y$ is also connected by $OPE_2$ to $x$, and vice versa.
///
/// ## Specification (Section §9.2.4)
///
/// ```bnf
/// InverseObjectProperties :=
///     'InverseObjectProperties' '('
///         axiomAnnotations
///         ObjectPropertyExpression ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct InverseObjectProperties {
    axiom_annotations: Vec<Annotation>,
    object_property_expression_1: ObjectPropertyExpression,
    object_property_expression_2: ObjectPropertyExpression,
}

///
/// An object property domain axiom $ObjectPropertyDomain( OPE \ CE )$ states that the domain
/// of the object property expression $OPE$ is the class expression $CE$ — that is, if an
/// individual $x$ is connected by $OPE$ with some other individual, then $x$ is an instance
/// of $CE$.
///
/// ## Specification (Section §9.2.5)
///
/// ```bnf
/// ObjectPropertyDomain :=
///     'ObjectPropertyDomain' '('
///         axiomAnnotations
///         ObjectPropertyExpression ClassExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectPropertyDomain {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression,
    domain: ClassExpression,
}

///
/// An object property range axiom $ObjectPropertyRange( OPE \ CE )$ states that the range of
/// the object property expression $OPE$ is the class expression $CE$ — that is, if some
/// individual is connected by $OPE$ with an individual $x$, then x is an instance of $CE$.
///
/// ## Specification (Section §9.2.6)
///
/// ```bnf
/// ObjectPropertyRange :=
///     'ObjectPropertyRange' '('
///         axiomAnnotations
///         ObjectPropertyExpression ClassExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectPropertyRange {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression,
    range: ClassExpression,
}

///
/// An object property functionality axiom $FunctionalObjectProperty( OPE )$ states that the
/// object property expression $OPE $is functional — that is, for each individual $x$, there can
/// be at most one distinct individual $y$ such that $x$ is connected by $OPE$ to $y$.
///
/// ## Specification (Section §9.2.7)
///
/// ```bnf
/// FunctionalObjectProperty :=
///     'FunctionalObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionalObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression,
}

///
/// An object property inverse functionality axiom $InverseFunctionalObjectProperty( OPE )$ states
/// that the object property expression $OPE$ is inverse-functional — that is, for each individual
/// $x$, there can be at most one individual $y$ such that $y$ is connected by $OPE$ with $x$.
///
/// ## Specification (Section §9.2.8)
///
/// ```bnf
/// InverseFunctionalObjectProperty :=
///     'InverseFunctionalObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct InverseFunctionalObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression, // 1
}

///
/// An object property reflexivity axiom $ReflexiveObjectProperty( OPE )$ states that the object
/// property expression $OPE$ is reflexive — that is, each individual is connected by $OPE$ to itself.
///
/// ## Specification (Section §9.2.9)
///
/// ```bnf
/// ObjectProperty :=
///     'ReflexiveObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ReflexiveObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression,
}

///
/// An object property irreflexivity axiom $IrreflexiveObjectProperty( OPE ) $states that the
/// object property expression $OPE$ is irreflexive — that is, no individual is connected by
/// $OPE$ to itself.
///
/// ## Specification (Section §9.2.10)
///
/// ```bnf
/// IrreflexiveObjectProperty := '
///     IrreflexiveObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct IrreflexiveObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression, // 1
}

///
/// An object property symmetry axiom $SymmetricObjectProperty( OPE )$ states that the object
/// property expression $OPE$ is symmetric — that is, if an individual $x$ is connected by
/// $OPE$ to an individual $y$, then $y$ is also connected by $OPE$ to $x$.
///
/// ## Specification (Section §9.2.11)
///
/// ```bnf
/// SymmetricObjectProperty :=
///     'SymmetricObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SymmetricObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression, // 1
}

///
/// An object property asymmetry axiom $AsymmetricObjectProperty( OPE )$ states that the object
/// property expression $OPE$ is asymmetric — that is, if an individual $x$ is connected by $OPE$
/// to an individual $y$, then y cannot be connected by $OPE$ to $x$.
///
/// ## Specification (Section §9.2.12)
///
/// ```bnf
/// AsymmetricObjectProperty :=
///     'AsymmetricObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AsymmetricObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression, // 1
}

///
/// An object property transitivity axiom $TransitiveObjectProperty( OPE )$ states that the object
/// property expression $OPE$ is transitive — that is, if an individual $x$ is connected by $OPE$
/// to an individual $y$ that is connected by $OPE$ to an individual $z$, then $x$ is also
/// connected by $OPE$ to $z$.
///
/// ## Specification (Section §9.2.13)
///
/// ```bnf
/// TransitiveObjectProperty :=
///     'TransitiveObjectProperty' '('
///         axiomAnnotations
///         ObjectPropertyExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct TransitiveObjectProperty {
    axiom_annotations: Vec<Annotation>,
    object_property_expression: ObjectPropertyExpression, // 1
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ObjectPropertyAxiom
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ObjectPropertyAxiom enum EquivalentObjectProperties,
    DisjointObjectProperties,
    SubObjectPropertyOf,
    ObjectPropertyDomain,
    ObjectPropertyRange,
    InverseObjectProperties,
    FunctionalObjectProperty,
    InverseFunctionalObjectProperty,
    ReflexiveObjectProperty,
    IrreflexiveObjectProperty,
    SymmetricObjectProperty,
    AsymmetricObjectProperty,
    TransitiveObjectProperty);

impl_has_annotations!(
    ObjectPropertyAxiom enum EquivalentObjectProperties,
    DisjointObjectProperties,
    SubObjectPropertyOf,
    ObjectPropertyDomain,
    ObjectPropertyRange,
    InverseObjectProperties,
    FunctionalObjectProperty,
    InverseFunctionalObjectProperty,
    ReflexiveObjectProperty,
    IrreflexiveObjectProperty,
    SymmetricObjectProperty,
    AsymmetricObjectProperty,
    TransitiveObjectProperty);

impl_from_for_variant!(ObjectPropertyAxiom, EquivalentObjectProperties);
impl_from_for_variant!(ObjectPropertyAxiom, DisjointObjectProperties);
impl_from_for_variant!(ObjectPropertyAxiom, SubObjectPropertyOf);
impl_from_for_variant!(ObjectPropertyAxiom, ObjectPropertyDomain);
impl_from_for_variant!(ObjectPropertyAxiom, ObjectPropertyRange);
impl_from_for_variant!(ObjectPropertyAxiom, InverseObjectProperties);
impl_from_for_variant!(ObjectPropertyAxiom, FunctionalObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, InverseFunctionalObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, ReflexiveObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, IrreflexiveObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, SymmetricObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, AsymmetricObjectProperty);
impl_from_for_variant!(ObjectPropertyAxiom, TransitiveObjectProperty);

impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from EquivalentObjectProperties ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from DisjointObjectProperties ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from SubObjectPropertyOf ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from ObjectPropertyDomain ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from ObjectPropertyRange ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from InverseObjectProperties ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from FunctionalObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from InverseFunctionalObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from ReflexiveObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from IrreflexiveObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from SymmetricObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from AsymmetricObjectProperty ) );
impl_from_for_variant!(Axiom, ObjectPropertyAxiom ( from TransitiveObjectProperty ) );

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    EquivalentObjectProperties( @list axiom_annotations, @list object_property_expressions )
);
impl_has_annotations!(EquivalentObjectProperties, axiom_annotations);

impl EquivalentObjectProperties {
    pub fn new<IOPE>(object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expressions)
    }

    pub fn new_with_annotations<IA, IOPE>(
        axiom_annotations: IA,
        object_property_expressions: IOPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expressions: object_property_expressions.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DisjointObjectProperties, ObjectPropertyChain(
        @list axiom_annotations, @list object_property_expressions
    )
);
impl_has_annotations!(DisjointObjectProperties, axiom_annotations);

impl DisjointObjectProperties {
    pub fn new<IOPE>(object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expressions)
    }

    pub fn new_with_annotations<IA, IOPE>(
        annotations: IA,
        object_property_expressions: IOPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: annotations.into_iter().collect(),
            object_property_expressions: object_property_expressions.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expressions(&self) -> impl Iterator<Item = &ObjectPropertyExpression> {
        self.object_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubObjectPropertyOf(
        @list axiom_annotations, sub_object_property_expressions, super_object_property_expression
    )
);
impl_has_annotations!(SubObjectPropertyOf, axiom_annotations);

impl SubObjectPropertyOf {
    pub fn new<SOPE, OPE>(
        sub_object_property_expressions: SOPE,
        super_object_property_expression: OPE,
    ) -> Self
    where
        SOPE: Into<SubObjectPropertyExpression>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            sub_object_property_expressions,
            super_object_property_expression,
        )
    }

    pub fn new_with_annotations<IA, SOPE, OPE>(
        axiom_annotations: IA,
        sub_object_property_expressions: SOPE,
        super_object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        SOPE: Into<SubObjectPropertyExpression>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            sub_object_property_expressions: sub_object_property_expressions.into(),
            super_object_property_expression: super_object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubObjectPropertyExpression enum ObjectPropertyExpression, PropertyExpressionChain
);

impl_from_for_variant!(SubObjectPropertyExpression, ObjectPropertyExpression);
impl_from_for_variant!(SubObjectPropertyExpression, PropertyExpressionChain);

// TODO: more?

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    PropertyExpressionChain, ObjectPropertyChain( @list object_property_expressions )
);

impl PropertyExpressionChain {
    pub fn new<I>(object_property_expressions: I) -> Self
    where
        I: IntoIterator<Item = ObjectPropertyExpression>,
    {
        Self {
            object_property_expressions: object_property_expressions.into_iter().collect(),
        }
    }

    pub fn object_property_expressions(&self) -> impl Iterator<Item = &ObjectPropertyExpression> {
        self.object_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ObjectPropertyDomain( @list axiom_annotations, object_property_expression, domain
));
impl_has_annotations!(ObjectPropertyDomain, axiom_annotations);

impl ObjectPropertyDomain {
    pub fn new<OPE, CE>(object_property_expression: OPE, domain: CE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression, domain)
    }

    pub fn new_with_annotations<IA, OPE, CE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
        domain: CE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
            domain: domain.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    pub fn domain(&self) -> &ClassExpression {
        &self.domain
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ObjectPropertyRange( @list axiom_annotations, object_property_expression, range )
);
impl_has_annotations!(ObjectPropertyRange, axiom_annotations);

impl ObjectPropertyRange {
    pub fn new<OPE, CE>(object_property_expression: OPE, range: CE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression, range)
    }

    pub fn new_with_annotations<IA, OPE, CE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
        range: CE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
        CE: Into<ClassExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
            range: range.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    pub fn range(&self) -> &ClassExpression {
        &self.range
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
InverseObjectProperties(
    @list axiom_annotations, object_property_expression_1, object_property_expression_2 )
);
impl_has_annotations!(InverseObjectProperties, axiom_annotations);

impl InverseObjectProperties {
    pub fn new<OPE1, OPE2>(
        object_property_expression_1: OPE1,
        object_property_expression_2: OPE2,
    ) -> Self
    where
        OPE1: Into<ObjectPropertyExpression>,
        OPE2: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            object_property_expression_1,
            object_property_expression_2,
        )
    }

    pub fn new_with_annotations<IA, OPE1, OPE2>(
        axiom_annotations: IA,
        object_property_expression_1: OPE1,
        object_property_expression_2: OPE2,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE1: Into<ObjectPropertyExpression>,
        OPE2: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression_1: object_property_expression_1.into(),
            object_property_expression_2: object_property_expression_2.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression_1(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression_1
    }

    pub fn object_property_expression_2(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression_2
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    FunctionalObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(FunctionalObjectProperty, axiom_annotations);

impl FunctionalObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    InverseFunctionalObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(InverseFunctionalObjectProperty, axiom_annotations);

impl InverseFunctionalObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ReflexiveObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(ReflexiveObjectProperty, axiom_annotations);

impl ReflexiveObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    IrreflexiveObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(IrreflexiveObjectProperty, axiom_annotations);

impl IrreflexiveObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SymmetricObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(SymmetricObjectProperty, axiom_annotations);

impl SymmetricObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AsymmetricObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(AsymmetricObjectProperty, axiom_annotations);

impl AsymmetricObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    TransitiveObjectProperty( @list axiom_annotations, object_property_expression )
);
impl_has_annotations!(TransitiveObjectProperty, axiom_annotations);

impl TransitiveObjectProperty {
    pub fn new<OPE>(object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        Self::new_with_annotations(Vec::default(), object_property_expression)
    }

    pub fn new_with_annotations<IA, OPE>(
        axiom_annotations: IA,
        object_property_expression: OPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}
