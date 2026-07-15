
use crate::{
    axioms::Axiom,
    annotations::{Annotation, AnnotationValue, HasAnnotations},
    entities::{AnnotationProperty, AnonymousIndividual, },
    fmt::DisplayPretty,
};
use rdftk_iri::{Iri, Name};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;


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

impl_from_for_variant!(Axiom, AnnotationAxiom ( from SubAnnotationOf ) );
impl_from_for_variant!(Axiom, AnnotationAxiom ( from AnnotationPropertyDomain ) );
impl_from_for_variant!(Axiom, AnnotationAxiom ( from AnnotationPropertyRange ) );
impl_from_for_variant!(Axiom, AnnotationAxiom ( from AnnotationAssertion ) );

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    SubAnnotationOf(
        @list axiom_annotations, sub_annotation_property, super_annotation_property
    )
);
impl_has_annotations!(SubAnnotationOf, axiom_annotations);

impl SubAnnotationOf {
    pub fn new<AP1, AP2>(sub_annotation_property: AP1, super_annotation_property: AP2) -> Self 
    where
        AP1: Into<AnnotationProperty>,
        AP2: Into<AnnotationProperty>,
    {
        Self::new_with_annotations(
            Vec::default(),
            sub_annotation_property,
            super_annotation_property,
        )
    }

    pub fn new_with_annotations<IA, AP1, AP2>(
        axiom_annotations: IA,
        sub_annotation_property: AP1,
        super_annotation_property: AP2,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        AP1: Into<AnnotationProperty>,
        AP2: Into<AnnotationProperty>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            sub_annotation_property: sub_annotation_property.into(),
            super_annotation_property: super_annotation_property.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
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
    pub fn new<AP>(annotation_property: AP, domain: Iri) -> Self 
    where
        AP: Into<AnnotationProperty>,
    {
        Self::new_with_annotations(
            Vec::default(),
            annotation_property,
            domain,
        )
    }

    pub fn new_with_annotations<IA, AP>(axiom_annotations: IA, annotation_property: AP, domain: Iri) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        AP: Into<AnnotationProperty>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            annotation_property: annotation_property.into(),
            domain,
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
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
    pub fn new<AP>(annotation_property: AP, range: Iri) -> Self 
    where
        AP: Into<AnnotationProperty>,
    {
        Self::new_with_annotations(
            Vec::default(),
            annotation_property,
            range,
        )
    }

    pub fn new_with_annotations<IA, AP>(axiom_annotations: IA, annotation_property: AP, range: Iri) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        AP: Into<AnnotationProperty>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            annotation_property: annotation_property.into(),
            range,
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
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
    pub fn new<AP, AS, AV>(annotation_property: AP, annotation_subject: AS, annotation_value: AV) -> Self 
    where
        AP: Into<AnnotationProperty>,
        AS: Into<AnnotationSubject>,
        AV: Into<AnnotationValue>,
    {
        Self::new_with_annotations(
            Vec::default(),
            annotation_property,
            annotation_subject,
            annotation_value,
        )
    }

    pub fn new_with_annotations<IA, AP, AS, AV>(
        axiom_annotations: IA,
        annotation_property: AP,
        annotation_subject: AS,
        annotation_value: AV,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        AP: Into<AnnotationProperty>,
        AS: Into<AnnotationSubject>,
        AV: Into<AnnotationValue>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            annotation_property: annotation_property.into(),
            annotation_subject: annotation_subject.into(),
            annotation_value: annotation_value.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
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

impl_from_for_variant!(AnnotationSubject, AnonymousIndividual ( from Name ) );
