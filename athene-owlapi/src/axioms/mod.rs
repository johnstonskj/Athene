//!
//! This module provides all the types corresponding to the axioms in OWL 2, found in sections
//! 9 and 10.2.
//!

use crate::{
    annotations::{Annotation, AnnotationValue},
    entities::{AnnotationProperty, AnonymousIndividual, Class, Datatype, Entity, Individual},
    error::ApiError,
    expressions::{ClassExpression, DataPropertyExpression, ObjectPropertyExpression},
    fmt::DisplayPretty,
    literals::Literal,
    ranges::DataRange,
    values::{CardinalityConstraintViolation, UnlimitedNatural},
};
use rdftk_iri::Iri;
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The main component of an OWL 2 ontology is a set of *axioms* — statements that say what is
/// true in the domain.
///
/// OWL 2 provides an extensive set of axioms, all of which extend the
/// **[Axiom]** class in the structural specification. As shown in Figure 12, axioms in OWL 2
/// can be declarations, axioms about classes, axioms about object or data properties, datatype
/// definitions, keys, assertions (sometimes also called facts), and axioms about annotations.
///
/// ![Figure 12. The Axioms of OWL 2](https://www.w3.org/TR/owl2-syntax/Axioms.gif)
///
/// As shown in Figure 1, OWL 2 axioms can contain axiom annotations, the structure of which is
/// defined in Section 10. Axiom annotations have no effect on the semantics of axioms — that is,
/// they do not affect the logical consequences of OWL 2 ontologies. In contrast, axiom
/// annotations do affect structural equivalence: axioms will not be structurally equivalent if
/// their axiom annotations are not structurally equivalent.
///
/// ## Specification (Section §9 -- Axioms)
///
/// ```bnf
/// Axiom :=
///     Declaration | ClassAxiom |
///     ObjectPropertyAxiom | DataPropertyAxiom |
///     DatatypeDefinition | HasKey |
///     Assertion | AnnotationAxiom
///
/// axiomAnnotations := { Annotation }
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Axiom {
    Declaration(Declaration),
    ClassAxiom(ClassAxiom),
    ObjectPropertyAxiom(ObjectPropertyAxiom),
    DataPropertyAxiom(DataPropertyAxiom),
    DatatypeDefinition(DatatypeDefinition),
    HasKey(HasKey),
    Assertion(Assertion),
    AnnotationAxiom(AnnotationAxiom),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Entity Declarations
// ------------------------------------------------------------------------------------------------

///
/// Each IRI $I$ used in an OWL 2 ontology $O$ can be, and sometimes even needs to be, declared
/// in $O$; roughly speaking, this means that the axiom closure of $O$ must contain an appropriate
/// declaration for $I$. A declaration for $I$ in $O$ serves two purposes:
///
/// * A declaration says that $I$ exists — that is, it says that $I$ is part of the vocabulary
///   of $O$.
/// * A declaration associates with $I$ an entity type — that is, it says whether $I$ is used in
///   $O$ as a class, datatype, object property, data property, annotation property, an individual,
///   or a combination thereof.
///
/// In OWL 2, declarations are a type of axiom; thus, to declare an entity in an ontology, one
/// can simply include the appropriate axiom in the ontology. These axioms are nonlogical in the
/// sense that they do not affect the consequences of an OWL 2 ontology.
///
/// ![Figure 3. Entity Declarations in OWL 2](https://www.w3.org/TR/owl2-syntax/A_declaration.gif)
///
/// ## Specification (Section §5.8)
///
/// ```bnf
/// Declaration :=
///     'Declaration' '('
///         axiomAnnotations
///         Entity
///     ')'
///
/// Entity :=
///     'Class' '(' Class ')' |
///     'Datatype' '(' Datatype ')' |
///     'ObjectProperty' '(' ObjectProperty ')' |
///     'DataProperty' '(' DataProperty ')' |
///     'AnnotationProperty' '(' AnnotationProperty ')' |
///     'NamedIndividual' '(' NamedIndividual ')'
/// ```
///
/// ## Example
///
/// ```owl
/// Declaration( Class( a:Person ) )
/// Declaration( NamedIndividual( a:Peter ) )
/// ```
///
/// ```rust
/// use athene_owlapi::{
///     Ontology, OntologyDocument,
///     builders::Builder,
///     entities::{Class, NamedIndividual},
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let ontology_iri = Iri::from_str("http://www.example.com/an-ontology/").unwrap();
///
/// let ontology = Ontology::builder()
///     .with_ontology_iri(ontology_iri.clone())
///     .with_declaration(Class::from(ontology_iri.with_new_path("Person")))
///     .with_declaration(NamedIndividual::from(ontology_iri.with_new_path("Peter")))
///     .build()
///     .expect("could not build Ontology");
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    axiom_annotations: Vec<Annotation>,
    entity: Entity,
}

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
// Public Types ❯ Datatype Definition
// ------------------------------------------------------------------------------------------------

///
/// A datatype definition $DatatypeDefinition( DT \ DR )$ defines a new datatype $DT$ as being
/// semantically equivalent to the data range $DR$; the latter must be a unary data range.
/// This axiom allows one to use the defined datatype $DT$ as a synonym for $DR$ — that is,
/// in any expression in the ontology containing such an axiom, $DT$ can be replaced with $DR$
/// without affecting the meaning of the ontology.
///
/// ## Specification (Section §9.4)
///
/// ```bnf
/// DatatypeDefinition :=
///     'DatatypeDefinition' '('
///         axiomAnnotations
///         Datatype DataRange
///     ')'
/// ```
///
/// ## Example
///
/// ```owl
/// Declaration( Datatype( a:SSN ) )
/// DatatypeDefinition(
///     a:SSN
///     DatatypeRestriction( xsd:string xsd:pattern "[0-9]{3}-[0-9]{2}-[0-9]{4}" )
/// )
/// DataPropertyRange( a:hasSSN a:SSN )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DatatypeDefinition {
    axiom_annotations: Vec<Annotation>,
    datatype: Datatype,
    data_range: DataRange,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Has Key
// ------------------------------------------------------------------------------------------------

///
/// A key axiom $HasKey( CE ( OPE_1 ... OPE_m ) ( DPE_1 ... DPE_n ) )$ states that each (named)
/// instance of the class expression $CE$ is uniquely identified by the object property expressions
/// $OPE_i$ and/or the data property experssions $DPE_j$ — that is, no two distinct (named)
/// instances of $CE$ can coincide on the values of all object property expressions $OPE_i$ and all
/// data property expressions $DPE_j$. In each such axiom in an OWL ontology, $m$ or $n$ (or both)
/// must be larger than zero. A key axiom of the form $HasKey( owl:Thing ( OPE ) () )$ is similar
/// to the axiom $InverseFunctionalObjectProperty( OPE )$, the main differences being that the
/// former axiom is applicable only to individuals that are explicitly named in an ontology, while
/// the latter axiom is also applicable to anonymous individuals and individuals whose existence is
/// implied by existential quantification.
///
/// ## Specification (Section §9.5)
///
/// ```bnf
/// HasKey :=
///     'HasKey' '(' axiomAnnotations
///         ClassExpression
///         '(' { ObjectPropertyExpression } ')'
///         '(' { DataPropertyExpression } ')'
///     ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct HasKey {
    axiom_annotations: Vec<Annotation>,
    class_expression: ClassExpression,
    object_property_expressions: Vec<ObjectPropertyExpression>,
    data_property_expressions: Vec<DataPropertyExpression>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Assertion
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 supports a rich set of axioms for stating assertions — axioms about individuals that are
/// often also called facts.
///
/// ## Specification (Section §9.6)
///
/// ```bnf
/// Assertion :=
///     SameIndividual | DifferentIndividuals | ClassAssertion |
///     ObjectPropertyAssertion | NegativeObjectPropertyAssertion |
///     DataPropertyAssertion | NegativeDataPropertyAssertion
///
/// sourceIndividual := Individual
///
/// targetIndividual := Individual
///
/// targetValue := Literal
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Assertion {
    SameIndividual(SameIndividual),
    DifferentIndividuals(DifferentIndividuals),
    ClassAssertion(ClassAssertion),
    ObjectPropertyAssertion(ObjectPropertyAssertion),
    NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
    DataPropertyAssertion(DataPropertyAssertion),
    NegativeDataPropertyAssertion(NegativeDataPropertyAssertion),
}

///
/// An individual equality axiom $SameIndividual( a_1 \cdots a_n )$ states that all of the
/// individuals $a_i, 1 \leq i \leq n$, are equal to each other. This axiom allows one
/// to use each $a_i$ as a synonym for each $a_j$ — that is, in any expression in the
/// ontology containing such an axiom, $a_i$ can be replaced with $a_j$ without affecting
/// the meaning of the ontology.
///
/// ## Specification (Section §9.6.1)
///
/// ```bnf
/// SameIndividual :=
///     'SameIndividual' '('
///         axiomAnnotations Individual Individual { Individual }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SameIndividual {
    axiom_annotations: Vec<Annotation>,
    individuals: Vec<Individual>, // 2..*
}

///
/// An individual inequality axiom $DifferentIndividuals( a_1 \cdots an )$ states that all of
/// the individuals $a_i, 1 \leq i \leq n$, are different from each other; that is, no
/// individuals $a_i$ and $a_j$ with $i \neq j$ can be derived to be equal. This axiom can be
/// used to axiomatize the unique name assumption — the assumption that all different individual
/// names denote different individuals.
///
/// ## Specification (Section §9.6.2)
///
/// ```bnf
/// DifferentIndividuals :=
///     'DifferentIndividuals' '('
///         axiomAnnotations
///         Individual Individual { Individual }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DifferentIndividuals {
    axiom_annotations: Vec<Annotation>,
    individuals: Vec<Individual>, // 2..*
}

///
/// A class assertion $ClassAssertion( CE \ a )$ states that the individual $a$ is an instance of
/// the class expression $CE$.
///
/// ## Specification (Section §9.6.3)
///
/// ```bnf
/// ClassAssertion :=
///     'ClassAssertion' '('
///         axiomAnnotations ClassExpression Individual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ClassAssertion {
    axiom_annotations: Vec<Annotation>,
    individual: Individual,
    class_expression: ClassExpression,
}

///
/// A positive object property assertion $ObjectPropertyAssertion( OPE \ a_1 \ a_2 )$ states that the
/// individual $a_1$ is connected by the object property expression $OPE$ to the individual $a_2$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// ObjectPropertyAssertion :=
///     'ObjectPropertyAssertion' '('
///         axiomAnnotations
///         ObjectPropertyExpression sourceIndividual targetIndividual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_individual: Individual,
    object_property_expression: ObjectPropertyExpression,
}

///
/// A negative object property assertion $NegativeObjectPropertyAssertion( OPE \ a_1 \ a_2 )$ states
/// that the individual $a_1$ is not connected by the object property expression $OPE$ to the
/// individual $a_2$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// NegativeObjectPropertyAssertion :=
///     'NegativeObjectPropertyAssertion' '('
///         axiomAnnotations
///         ObjectPropertyExpression sourceIndividual targetIndividual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NegativeObjectPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_individual: Individual,
    object_property_expression: ObjectPropertyExpression,
}

///
/// A positive data property assertion $DataPropertyAssertion( DPE \ a \ lt )$ states that the
/// individual $a$ is connected by the data property expression $DPE$ to the literal $lt$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// DataPropertyAssertion :=
///     'DataPropertyAssertion' '('
///         axiomAnnotations
///         DataPropertyExpression sourceIndividual targetValue
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_value: Literal,
    data_property_expression: DataPropertyExpression,
}

///
/// A negative data property assertion $NegativeDataPropertyAssertion( DPE \ a \ lt )$ states
/// that the individual $a$ is not connected by the data property expression $DPE$ to the
/// literal $lt$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// NegativeDataPropertyAssertion :=
///     'NegativeDataPropertyAssertion' '('
///         axiomAnnotations
///         DataPropertyExpression sourceIndividual targetValue
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NegativeDataPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_value: Literal,
    data_property_expression: DataPropertyExpression,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Annotation Axioms (10.2)
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 applications often need ways to associate additional information with ontologies,
/// entities, and axioms. To this end, OWL 2 provides for annotations on ontologies, axioms,
/// and entities.
///
/// Various OWL 2 syntaxes, such as the functional-style syntax, provide a mechanism for
/// embedding comments into ontology documents. The structure of such comments is, however,
/// dependent on the syntax, so these are simply discarded during parsing. In contrast,
/// annotations are "first-class citizens" in the structural specification of OWL 2, and
/// their structure is independent of the underlying syntax.
///
/// ## Specification (Section 10.2)
///
/// ```bnf
/// AnnotationAxiom :=
///     AnnotationAssertion | SubAnnotationPropertyOf |
///     AnnotationPropertyDomain | AnnotationPropertyRange
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum AnnotationAxiom {
    SubAnnotationOf(SubAnnotationOf),
    AnnotationPropertyDomain(AnnotationPropertyDomain),
    AnnotationPropertyRange(AnnotationPropertyRange),
    AnnotationAssertion(AnnotationAssertion),
}

///
/// An annotation assertion $AnnotationAssertion( AP \ as \ av )$ states that the annotation
/// subject $as$ — an IRI or an anonymous individual — is annotated with the annotation
/// property $AP$ and the annotation value $av$.
///
/// ## Specification (Section §10.2.1)
///
/// ```bnf
/// AnnotationAssertion :=
///     'AnnotationAssertion' '('
///         axiomAnnotations
///         AnnotationProperty AnnotationSubject AnnotationValue
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationAssertion {
    axiom_annotations: Vec<Annotation>,
    annotation_property: AnnotationProperty,
    annotation_subject: AnnotationSubject,
    annotation_value: AnnotationValue,
}

///
/// Represents the internal production `AnnotationSubject`.
///
/// ## Specification (Section §10.2.1)
///
/// ```bnf
/// AnnotationSubject := IRI | AnonymousIndividual
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub enum AnnotationSubject {
    Iri(Iri),
    AnonymousIndividual(AnonymousIndividual),
}

///
/// An annotation subproperty axiom $SubAnnotationPropertyOf( AP_1 \ AP_2 )$ states that the
/// annotation property $AP_1$ is a subproperty of the annotation property $AP_2$.
///
/// ## Specification (Section §10.2.2)
///
/// ```bnf
/// SubAnnotationPropertyOf :=
///     'SubAnnotationPropertyOf' '('
///         axiomAnnotations
///         subAnnotationProperty superAnnotationProperty
///     ')'
///
/// subAnnotationProperty := AnnotationProperty
///
/// superAnnotationProperty := AnnotationProperty
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SubAnnotationOf {
    axiom_annotations: Vec<Annotation>,
    sub_annotation_property: AnnotationProperty,
    super_annotation_property: AnnotationProperty,
}

///
/// An annotation property domain axiom $AnnotationPropertyDomain( AP \ U )$ states that the domain
/// of the annotation property $AP$ is the IRI $U$.
///
/// ## Specification (Section §10.2.3)
///
/// ```bnf
/// AnnotationPropertyDomain :=
///     'AnnotationPropertyDomain' '('
///         axiomAnnotations
///         AnnotationProperty IRI
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationPropertyDomain {
    axiom_annotations: Vec<Annotation>,
    annotation_property: AnnotationProperty,
    domain: Iri,
}

///
/// An annotation property range axiom $AnnotationPropertyRange( AP \ U ) $states that the range
/// of the annotation property $AP$ is the IRI $U$.
///
/// ## Specification (Section §10.2.4)
///
/// ```bnf
/// AnnotationPropertyRange :=
///     'AnnotationPropertyRange' '('
///         axiomAnnotations
///         AnnotationProperty IRI
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationPropertyRange {
    axiom_annotations: Vec<Annotation>,
    annotation_property: AnnotationProperty,
    range: Iri,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Axiom
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    Axiom enum Declaration,
    ClassAxiom,
    ObjectPropertyAxiom,
    DataPropertyAxiom,
    DatatypeDefinition,
    HasKey,
    Assertion,
    AnnotationAxiom);

impl_has_annotations!(
    Axiom enum Declaration,
    ClassAxiom,
    ObjectPropertyAxiom,
    DataPropertyAxiom,
    DatatypeDefinition,
    HasKey,
    Assertion,
    AnnotationAxiom);

impl_from_for_variant!(Axiom, Declaration);
impl_from_for_variant!(Axiom, ClassAxiom);
impl_from_for_variant!(Axiom, ObjectPropertyAxiom);
impl_from_for_variant!(Axiom, DataPropertyAxiom);
impl_from_for_variant!(Axiom, DatatypeDefinition);
impl_from_for_variant!(Axiom, HasKey);
impl_from_for_variant!(Axiom, Assertion);
impl_from_for_variant!(Axiom, AnnotationAxiom);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Declaration
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Declaration(@list axiom_annotations, entity));
impl_has_annotations!(Declaration, axiom_annotations);

impl<E: Into<Entity>> From<E> for Declaration {
    fn from(entity: E) -> Self {
        Self::new(entity.into())
    }
}

impl Declaration {
    pub fn new(entity: Entity) -> Self {
        Self {
            axiom_annotations: Default::default(),
            entity,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, entity: Entity) -> Self {
        Self {
            axiom_annotations: ann,
            entity,
        }
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }
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

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(SubClassOf(@list axiom_annotations, sub_class_expression, super_class_expression));
impl_has_annotations!(SubClassOf, axiom_annotations);

impl SubClassOf {
    pub fn new(sub_class: ClassExpression, super_class: ClassExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            sub_class_expression: sub_class,
            super_class_expression: super_class,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        sub_class: ClassExpression,
        super_class: ClassExpression,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            sub_class_expression: sub_class,
            super_class_expression: super_class,
        }
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
    pub fn new<I: IntoIterator<Item = ClassExpression>>(classes: I) -> Result<Self, ApiError> {
        let class_expressions: Vec<ClassExpression> = classes.into_iter().collect();

        if class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: Default::default(),
                class_expressions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(class_expressions.len() as u128),
            )
            .into())
        }
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DisjointClasses(@list axiom_annotations, @list class_expressions));
impl_has_annotations!(DisjointClasses, axiom_annotations);

impl DisjointClasses {
    pub fn new<I: IntoIterator<Item = ClassExpression>>(classes: I) -> Result<Self, ApiError> {
        let class_expressions: Vec<ClassExpression> = classes.into_iter().collect();

        if class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: Default::default(),
                class_expressions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(class_expressions.len() as u128),
            )
            .into())
        }
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DisjointUnion(@list axiom_annotations, class, @list disjoint_class_expressions));
impl_has_annotations!(DisjointUnion, axiom_annotations);

impl DisjointUnion {
    pub fn new<I: IntoIterator<Item = ClassExpression>>(
        class: Class,
        disjoint_classes: I,
    ) -> Result<Self, ApiError> {
        let disjoint_class_expressions: Vec<ClassExpression> =
            disjoint_classes.into_iter().collect();

        if disjoint_class_expressions.len() >= 2 {
            Ok(Self {
                axiom_annotations: Default::default(),
                class,
                disjoint_class_expressions,
            })
        } else {
            Err(CardinalityConstraintViolation::min_fail(
                2,
                UnlimitedNatural::Limited(disjoint_class_expressions.len() as u128),
            )
            .into())
        }
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn disjoint_class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.disjoint_class_expressions.iter()
    }
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

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    EquivalentObjectProperties( @list axiom_annotations, @list object_property_expressions )
);
impl_has_annotations!(EquivalentObjectProperties, axiom_annotations);

impl EquivalentObjectProperties {
    pub fn new<I: IntoIterator<Item = ObjectPropertyExpression>>(opes: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expressions: opes.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = ObjectPropertyExpression>>(
        annotations: Vec<Annotation>,
        opes: I,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            object_property_expressions: opes.into_iter().collect(),
        }
    }

    pub fn object_property_expressions(&self) -> impl Iterator<Item = &ObjectPropertyExpression> {
        self.object_property_expressions.iter()
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
    pub fn new<I: IntoIterator<Item = ObjectPropertyExpression>>(opes: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expressions: opes.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = ObjectPropertyExpression>>(
        annotations: Vec<Annotation>,
        opes: I,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            object_property_expressions: opes.into_iter().collect(),
        }
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
    pub fn new(sub: SubObjectPropertyExpression, sup: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            sub_object_property_expressions: sub,
            super_object_property_expression: sup,
        }
    }

    pub fn new_with_annotations(
        annotations: Vec<Annotation>,
        sub: SubObjectPropertyExpression,
        sup: ObjectPropertyExpression,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            sub_object_property_expressions: sub,
            super_object_property_expression: sup,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubObjectPropertyExpression enum ObjectPropertyExpression, PropertyExpressionChain
);

impl_from_for_variant!(SubObjectPropertyExpression, ObjectPropertyExpression);
impl_from_for_variant!(SubObjectPropertyExpression, PropertyExpressionChain);

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    PropertyExpressionChain, ObjectPropertyChain( @list object_property_expressions )
);

impl PropertyExpressionChain {
    pub fn new<I: IntoIterator<Item = ObjectPropertyExpression>>(opes: I) -> Self {
        Self {
            object_property_expressions: opes.into_iter().collect(),
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
    pub fn new(ope: ObjectPropertyExpression, domain: ClassExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
            domain,
        }
    }

    pub fn new_with_annotations(
        annotations: Vec<Annotation>,
        ope: ObjectPropertyExpression,
        domain: ClassExpression,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            object_property_expression: ope,
            domain,
        }
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
    pub fn new(ope: ObjectPropertyExpression, range: ClassExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
            range,
        }
    }

    pub fn new_with_annotations(
        annotations: Vec<Annotation>,
        ope: ObjectPropertyExpression,
        range: ClassExpression,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            object_property_expression: ope,
            range,
        }
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
    pub fn new(ope1: ObjectPropertyExpression, ope2: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression_1: ope1,
            object_property_expression_2: ope2,
        }
    }

    pub fn new_with_annotations(
        annotations: Vec<Annotation>,
        ope1: ObjectPropertyExpression,
        ope2: ObjectPropertyExpression,
    ) -> Self {
        Self {
            axiom_annotations: annotations,
            object_property_expression_1: ope1,
            object_property_expression_2: ope2,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
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
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ope: ObjectPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
        }
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
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

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubDataPropertyOf(
        @list axiom_annotations, sub_data_property_expression, super_data_property_expression
    )
);
impl_has_annotations!(SubDataPropertyOf, axiom_annotations);

impl SubDataPropertyOf {
    pub fn new(sub: DataPropertyExpression, sup: DataPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            sub_data_property_expression: sub,
            super_data_property_expression: sup,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        sub: DataPropertyExpression,
        sup: DataPropertyExpression,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            sub_data_property_expression: sub,
            super_data_property_expression: sup,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DisjointDataProperties( @list axiom_annotations, @list data_property_expressions )
);
impl_has_annotations!(DisjointDataProperties, axiom_annotations);

impl DisjointDataProperties {
    pub fn new<I: IntoIterator<Item = DataPropertyExpression>>(dpes: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expressions: dpes.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = DataPropertyExpression>>(
        ann: Vec<Annotation>,
        dpes: I,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expressions: dpes.into_iter().collect(),
        }
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
    pub fn new<I: IntoIterator<Item = DataPropertyExpression>>(dpes: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expressions: dpes.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = DataPropertyExpression>>(
        ann: Vec<Annotation>,
        dpes: I,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expressions: dpes.into_iter().collect(),
        }
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
    pub fn new(dpe: DataPropertyExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expression: dpe,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, dpe: DataPropertyExpression) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expression: dpe,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyDomain( @list axiom_annotations, data_property_expression, domain )
);
impl_has_annotations!(DataPropertyDomain, axiom_annotations);

impl DataPropertyDomain {
    pub fn new(dpe: DataPropertyExpression, domain: ClassExpression) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expression: dpe,
            domain,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        dpe: DataPropertyExpression,
        domain: ClassExpression,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expression: dpe,
            domain,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyRange( @list axiom_annotations, data_property_expression, range )
);
impl_has_annotations!(DataPropertyRange, axiom_annotations);

impl DataPropertyRange {
    pub fn new(dpe: DataPropertyExpression, range: DataRange) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expression: dpe,
            range,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        dpe: DataPropertyExpression,
        range: DataRange,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expression: dpe,
            range,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DatatypeDefinition
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DatatypeDefinition( @list axiom_annotations, datatype, data_range )
);
impl_has_annotations!(DatatypeDefinition, axiom_annotations);

impl DatatypeDefinition {
    pub fn new(datatype: Datatype, data_range: DataRange) -> Self {
        Self {
            axiom_annotations: Default::default(),
            datatype,
            data_range,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        datatype: Datatype,
        data_range: DataRange,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            datatype,
            data_range,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ HasKey
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    HasKey(
        @list axiom_annotations,
        class_expression,
        @list object_property_expressions, /* ( ... ) */
        @list data_property_expressions    /* ( ... ) */
    )
);
impl_has_annotations!(HasKey, axiom_annotations);

impl HasKey {
    pub fn new<I, J>(ce: ClassExpression, opes: I, dpes: J) -> Self
    where
        I: IntoIterator<Item = ObjectPropertyExpression>,
        J: IntoIterator<Item = DataPropertyExpression>,
    {
        Self {
            axiom_annotations: Default::default(),
            class_expression: ce,
            object_property_expressions: opes.into_iter().collect(),
            data_property_expressions: dpes.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I, J>(
        ann: Vec<Annotation>,
        ce: ClassExpression,
        opes: I,
        dpes: J,
    ) -> Self
    where
        I: IntoIterator<Item = ObjectPropertyExpression>,
        J: IntoIterator<Item = DataPropertyExpression>,
    {
        Self {
            axiom_annotations: ann,
            class_expression: ce,
            object_property_expressions: opes.into_iter().collect(),
            data_property_expressions: dpes.into_iter().collect(),
        }
    }

    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }

    pub fn object_property_expressions(&self) -> impl Iterator<Item = &ObjectPropertyExpression> {
        self.object_property_expressions.iter()
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertion
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    Assertion enum SameIndividual,
    DifferentIndividuals,
    ClassAssertion,
    ObjectPropertyAssertion,
    NegativeObjectPropertyAssertion,
    DataPropertyAssertion,
    NegativeDataPropertyAssertion);

impl_has_annotations!(
    Assertion enum SameIndividual,
    DifferentIndividuals,
    ClassAssertion,
    ObjectPropertyAssertion,
    NegativeObjectPropertyAssertion,
    DataPropertyAssertion,
    NegativeDataPropertyAssertion);

impl_from_for_variant!(Assertion, SameIndividual);
impl_from_for_variant!(Assertion, DifferentIndividuals);
impl_from_for_variant!(Assertion, ClassAssertion);
impl_from_for_variant!(Assertion, ObjectPropertyAssertion);
impl_from_for_variant!(Assertion, NegativeObjectPropertyAssertion);
impl_from_for_variant!(Assertion, DataPropertyAssertion);
impl_from_for_variant!(Assertion, NegativeDataPropertyAssertion);

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(SameIndividual( @list axiom_annotations, @list individuals ));
impl_has_annotations!(SameIndividual, axiom_annotations);

impl SameIndividual {
    pub fn new<I: IntoIterator<Item = Individual>>(individuals: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = Individual>>(
        ann: Vec<Annotation>,
        individuals: I,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn individuals(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DifferentIndividuals( @list axiom_annotations, @list individuals ));
impl_has_annotations!(DifferentIndividuals, axiom_annotations);

impl DifferentIndividuals {
    pub fn new<I: IntoIterator<Item = Individual>>(individuals: I) -> Self {
        Self {
            axiom_annotations: Default::default(),
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn new_with_annotations<I: IntoIterator<Item = Individual>>(
        ann: Vec<Annotation>,
        individuals: I,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn individuals(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ClassAssertion( @list axiom_annotations, individual, class_expression ));
impl_has_annotations!(ClassAssertion, axiom_annotations);

impl ClassAssertion {
    pub fn new(ce: ClassExpression, individual: Individual) -> Self {
        Self {
            axiom_annotations: Default::default(),
            class_expression: ce,
            individual,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        ce: ClassExpression,
        individual: Individual,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            class_expression: ce,
            individual,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ObjectPropertyAssertion(
        @list axiom_annotations, source_individual, target_individual, object_property_expression
    )
);
impl_has_annotations!(ObjectPropertyAssertion, axiom_annotations);

impl ObjectPropertyAssertion {
    pub fn new(ope: ObjectPropertyExpression, source: Individual, target: Individual) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
            source_individual: source,
            target_individual: target,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        ope: ObjectPropertyExpression,
        source: Individual,
        target: Individual,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
            source_individual: source,
            target_individual: target,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    NegativeObjectPropertyAssertion(
        @list axiom_annotations, source_individual, target_individual, object_property_expression
    )
);
impl_has_annotations!(NegativeObjectPropertyAssertion, axiom_annotations);

impl NegativeObjectPropertyAssertion {
    pub fn new(ope: ObjectPropertyExpression, source: Individual, target: Individual) -> Self {
        Self {
            axiom_annotations: Default::default(),
            object_property_expression: ope,
            source_individual: source,
            target_individual: target,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        ope: ObjectPropertyExpression,
        source: Individual,
        target: Individual,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            object_property_expression: ope,
            source_individual: source,
            target_individual: target,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyAssertion(
        @list axiom_annotations, source_individual, target_value, data_property_expression
    )
);
impl_has_annotations!(DataPropertyAssertion, axiom_annotations);

impl DataPropertyAssertion {
    pub fn new(dpe: DataPropertyExpression, source: Individual, value: Literal) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expression: dpe,
            source_individual: source,
            target_value: value,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        dpe: DataPropertyExpression,
        source: Individual,
        value: Literal,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expression: dpe,
            source_individual: source,
            target_value: value,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    NegativeDataPropertyAssertion(
        @list axiom_annotations, source_individual, target_value, data_property_expression
    )
);
impl_has_annotations!(NegativeDataPropertyAssertion, axiom_annotations);

impl NegativeDataPropertyAssertion {
    pub fn new(dpe: DataPropertyExpression, source: Individual, value: Literal) -> Self {
        Self {
            axiom_annotations: Default::default(),
            data_property_expression: dpe,
            source_individual: source,
            target_value: value,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        dpe: DataPropertyExpression,
        source: Individual,
        value: Literal,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            data_property_expression: dpe,
            source_individual: source,
            target_value: value,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ AnnotationAxiom
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AnnotationAxiom enum SubAnnotationOf,
    AnnotationPropertyDomain,
    AnnotationPropertyRange,
    AnnotationAssertion
);

impl_has_annotations!(
    AnnotationAxiom enum SubAnnotationOf,
    AnnotationPropertyDomain,
    AnnotationPropertyRange,
    AnnotationAssertion
);

impl_from_for_variant!(AnnotationAxiom, SubAnnotationOf);
impl_from_for_variant!(AnnotationAxiom, AnnotationPropertyDomain);
impl_from_for_variant!(AnnotationAxiom, AnnotationPropertyRange);
impl_from_for_variant!(AnnotationAxiom, AnnotationAssertion);

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubAnnotationOf(
        @list axiom_annotations, sub_annotation_property, super_annotation_property
    )
);
impl_has_annotations!(SubAnnotationOf, axiom_annotations);

impl SubAnnotationOf {
    pub fn new(sub: AnnotationProperty, sup: AnnotationProperty) -> Self {
        Self {
            axiom_annotations: Default::default(),
            sub_annotation_property: sub,
            super_annotation_property: sup,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        sub: AnnotationProperty,
        sup: AnnotationProperty,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            sub_annotation_property: sub,
            super_annotation_property: sup,
        }
    }

    pub fn sub_annotation_property(&self) -> &AnnotationProperty {
        &self.sub_annotation_property
    }

    pub fn super_annotation_property(&self) -> &AnnotationProperty {
        &self.super_annotation_property
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AnnotationPropertyDomain(@list axiom_annotations, annotation_property, domain)
);
impl_has_annotations!(AnnotationPropertyDomain, axiom_annotations);

impl AnnotationPropertyDomain {
    pub fn new(ap: AnnotationProperty, domain: Iri) -> Self {
        Self {
            axiom_annotations: Default::default(),
            annotation_property: ap,
            domain,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ap: AnnotationProperty, domain: Iri) -> Self {
        Self {
            axiom_annotations: ann,
            annotation_property: ap,
            domain,
        }
    }

    pub fn annotation_property(&self) -> &AnnotationProperty {
        &self.annotation_property
    }

    pub fn domain(&self) -> &Iri {
        &self.domain
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AnnotationPropertyRange(@list axiom_annotations, annotation_property, range)
);
impl_has_annotations!(AnnotationPropertyRange, axiom_annotations);

impl AnnotationPropertyRange {
    pub fn new(ap: AnnotationProperty, range: Iri) -> Self {
        Self {
            axiom_annotations: Default::default(),
            annotation_property: ap,
            range,
        }
    }

    pub fn new_with_annotations(ann: Vec<Annotation>, ap: AnnotationProperty, range: Iri) -> Self {
        Self {
            axiom_annotations: ann,
            annotation_property: ap,
            range,
        }
    }

    pub fn annotation_property(&self) -> &AnnotationProperty {
        &self.annotation_property
    }

    pub fn range(&self) -> &Iri {
        &self.range
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AnnotationAssertion(
        @list axiom_annotations, annotation_property, annotation_subject, annotation_value
    )
);
impl_has_annotations!(AnnotationAssertion, axiom_annotations);

impl AnnotationAssertion {
    pub fn new(ap: AnnotationProperty, subject: AnnotationSubject, value: AnnotationValue) -> Self {
        Self {
            axiom_annotations: Default::default(),
            annotation_property: ap,
            annotation_subject: subject,
            annotation_value: value,
        }
    }

    pub fn new_with_annotations(
        ann: Vec<Annotation>,
        ap: AnnotationProperty,
        subject: AnnotationSubject,
        value: AnnotationValue,
    ) -> Self {
        Self {
            axiom_annotations: ann,
            annotation_property: ap,
            annotation_subject: subject,
            annotation_value: value,
        }
    }

    pub fn annotation_property(&self) -> &AnnotationProperty {
        &self.annotation_property
    }

    pub fn annotation_subject(&self) -> &AnnotationSubject {
        &self.annotation_subject
    }

    pub fn annotation_value(&self) -> &AnnotationValue {
        &self.annotation_value
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    AnnotationSubject enum Iri, AnonymousIndividual
);

impl_from_for_variant!(AnnotationSubject, Iri);
impl_from_for_variant!(AnnotationSubject, AnonymousIndividual);
