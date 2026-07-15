use crate::{
    annotations::{Annotation, HasAnnotations},
    axioms::Axiom,
    entities::Class,
    expressions::ClassExpression,
    fmt::DisplayPretty,
    values::{CardinalityConstraintViolation, UnlimitedNatural},
};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Axioms
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 provides axioms that allow relationships to be established between class expressions,
/// as shown in Figure 13.
///
/// The **[SubClassOf]** axiom allows one to state that each instance of one class expression is
/// also an instance of another class expression, and thus to construct a hierarchy of classes.
/// The **[EquivalentClass]** axiom allows one to state that several class expressions are
/// equivalent to each other. The **[DisjointClasses]** axiom allows one to state that several
/// class expressions are pairwise disjoint — that is, that they have no instances in common.
/// Finally, the DisjointUnion class expression allows one to define a class as a disjoint union
/// of several class expressions and thus to express covering constraints.
///
/// ## Specification (Section §9.1 -- Class Expression Axioms)
///
/// ```bnf
/// ClassAxiom :=
///     SubClassOf | EquivalentClasses |
///     DisjointClasses | DisjointUnion
/// ```
///
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum ClassAxiom {
    SubClassOf(SubClassOf),
    EquivalentClass(EquivalentClass),
    DisjointClasses(DisjointClasses),
    DisjointUnion(DisjointUnion),
}

///
/// A subclass axiom $SubClassOf( CE_1 \ CE_2 )$ states that the class expression $CE_1$ is a
/// subclass of the class expression $CE_2$. Roughly speaking, this states that $CE_1$ is more
/// specific than $CE_2$. Subclass axioms are a fundamental type of axioms in OWL 2 and can be
/// used to construct a class hierarchy. Other kinds of class expression axiom can be seen as
/// syntactic shortcuts for one or more subclass axioms.
///
/// ## Specification (Section §9.1.1)
///
/// ```bnf
/// SubClassOf :=
///     'SubClassOf' '('
///         axiomAnnotations
///         subClassExpression superClassExpression
///     ')'
///
/// subClassExpression := ClassExpression
///
/// superClassExpression := ClassExpression
/// ```
///
/// ## Example
///
/// ```owl
/// SubClassOf( a:Baby a:Child )
/// SubClassOf( a:Child a:Person )
/// ClassAssertion( a:Baby a:Stewie )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SubClassOf {
    axiom_annotations: Vec<Annotation>,
    sub_class_expression: ClassExpression,
    super_class_expression: ClassExpression,
}

///
/// An equivalent classes axiom $EquivalentClasses( CE_1 \cdots CE_n )$ states that all of the
/// class expressions $CE_i, 1 \leq i \leq n$, are semantically equivalent to each other. This
/// axiom allows one to use each $CE_i$ as a synonym for each $CE_j$ — that is, in any
/// expression in the ontology containing such an axiom, $CE_i$ can be replaced with $CE_j$
/// without affecting the meaning of the ontology.
///
/// ## Specification (Section §9.1.2)
///
/// ```bnf
/// EquivalentClasses :=
///     'EquivalentClasses' '('
///         axiomAnnotations
///         ClassExpression ClassExpression { ClassExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct EquivalentClass {
    axiom_annotations: Vec<Annotation>,
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// A disjoint classes axiom $DisjointClasses( CE_1 \cdots CE_n )$ states that all of the
/// class expressions $CE_i, 1 \leq i \leq n$, are pairwise disjoint; that is, no individual
/// can be at the same time an instance of both $CE_i$ and $CE_j$ for $i \neq j$.
///
/// ## Specification (Section §9.1.3)
///
/// ```bnf
/// DisjointClasses :=
///     'DisjointClasses' '('
///         axiomAnnotations
///         ClassExpression ClassExpression { ClassExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DisjointClasses {
    axiom_annotations: Vec<Annotation>,
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// A disjoint union axiom $DisjointUnion( C \ CE_1 ... CE_n )$ states that a class $C$ is a
/// disjoint union of the class expressions $CE_i, 1 \leq i \leq n$, all of which are
/// pairwise disjoint. Such axioms are sometimes referred to as covering axioms, as they
/// state that the extensions of all $CE_i$ exactly cover the extension of $C$. Thus, each
/// instance of $C$ is an instance of exactly one $CE_i$, and each instance of $CE_i$ is
/// an instance of $C$.
///
/// ## Specification (Section §9.1.4)
///
/// ```bnf
/// DisjointUnion :=
///     'DisjointUnion' '('
///         axiomAnnotations
///         Class disjointClassExpressions
///     ')'
///
/// disjointClassExpressions :=
///     ClassExpression ClassExpression { ClassExpression }
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DisjointUnion {
    axiom_annotations: Vec<Annotation>,
    class: Class,
    disjoint_class_expressions: Vec<ClassExpression>, // 2..*
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ClassAxiom
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ClassAxiom enum SubClassOf,
    EquivalentClass,
    DisjointClasses,
    DisjointUnion);

impl_has_annotations!(
    ClassAxiom enum SubClassOf,
    EquivalentClass,
    DisjointClasses,
    DisjointUnion);

impl_from_for_variant!(ClassAxiom, SubClassOf);
impl_from_for_variant!(ClassAxiom, EquivalentClass);
impl_from_for_variant!(ClassAxiom, DisjointClasses);
impl_from_for_variant!(ClassAxiom, DisjointUnion);

impl_from_for_variant!(Axiom, ClassAxiom ( from SubClassOf ) );
impl_from_for_variant!(Axiom, ClassAxiom ( from EquivalentClass ) );
impl_from_for_variant!(Axiom, ClassAxiom ( from DisjointClasses ) );
impl_from_for_variant!(Axiom, ClassAxiom ( from DisjointUnion ) );

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(SubClassOf(@list axiom_annotations, sub_class_expression, super_class_expression));
impl_has_annotations!(SubClassOf, axiom_annotations);

impl SubClassOf {
    pub fn new<CE1, CE2>(sub_class: CE1, super_class: CE2) -> Self
    where
        CE1: Into<ClassExpression>,
        CE2: Into<ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), sub_class, super_class)
    }

    pub fn new_with_annotations<CE1, CE2, IA>(
        annotations: IA,
        sub_class: CE1,
        super_class: CE2,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        CE1: Into<ClassExpression>,
        CE2: Into<ClassExpression>,
    {
        Self {
            axiom_annotations: annotations.into_iter().collect(),
            sub_class_expression: sub_class.into(),
            super_class_expression: super_class.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn sub_class_expression(&self) -> &ClassExpression {
        &self.sub_class_expression
    }

    pub fn super_class_expression(&self) -> &ClassExpression {
        &self.super_class_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(EquivalentClass( @list axiom_annotations, @list class_expressions ));
impl_has_annotations!(EquivalentClass, axiom_annotations);

impl EquivalentClass {
    pub fn new<ICE>(class_expressions: ICE) -> Result<Self, CardinalityConstraintViolation>
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), class_expressions)
    }

    pub fn new_with_annotations<IA, ICE>(
        axiom_annotations: IA,
        class_expressions: ICE,
    ) -> Result<Self, CardinalityConstraintViolation>
    where
        IA: IntoIterator<Item = Annotation>,
        ICE: IntoIterator<Item = ClassExpression>,
    {
        let class_expressions: Vec<ClassExpression> = class_expressions.into_iter().collect();

        if class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: axiom_annotations.into_iter().collect(),
                class_expressions: class_expressions.into_iter().collect(),
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(class_expressions.len() as u128),
            ))
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DisjointClasses(@list axiom_annotations, @list class_expressions));
impl_has_annotations!(DisjointClasses, axiom_annotations);

impl DisjointClasses {
    pub fn new<ICE>(class_expressions: ICE) -> Result<Self, CardinalityConstraintViolation>
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), class_expressions)
    }

    pub fn new_with_annotations<IA, ICE>(
        axiom_annotations: IA,
        class_expressions: ICE,
    ) -> Result<Self, CardinalityConstraintViolation>
    where
        IA: IntoIterator<Item = Annotation>,
        ICE: IntoIterator<Item = ClassExpression>,
    {
        let class_expressions: Vec<ClassExpression> = class_expressions.into_iter().collect();

        if class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: axiom_annotations.into_iter().collect(),
                class_expressions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(class_expressions.len() as u128),
            ))
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DisjointUnion(@list axiom_annotations, class, @list disjoint_class_expressions));
impl_has_annotations!(DisjointUnion, axiom_annotations);

impl DisjointUnion {
    pub fn new<C, ICE>(
        class: C,
        disjoint_classes: ICE,
    ) -> Result<Self, CardinalityConstraintViolation>
    where
        C: Into<Class>,
        ICE: IntoIterator<Item = ClassExpression>,
    {
        Self::new_with_annotations(Vec::default(), class, disjoint_classes)
    }

    pub fn new_with_annotations<IA, C, ICE>(
        axiom_annotations: IA,
        class: C,
        disjoint_classes: ICE,
    ) -> Result<Self, CardinalityConstraintViolation>
    where
        IA: IntoIterator<Item = Annotation>,
        C: Into<Class>,
        ICE: IntoIterator<Item = ClassExpression>,
    {
        let disjoint_class_expressions: Vec<ClassExpression> =
            disjoint_classes.into_iter().collect();

        if disjoint_class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: axiom_annotations.into_iter().collect(),
                class: class.into(),
                disjoint_class_expressions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(disjoint_class_expressions.len() as u128),
            ))
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn disjoint_class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.disjoint_class_expressions.iter()
    }
}
