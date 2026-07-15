//!
//! This module provides types specific to annotations themselves, while the `axioms`
//! module contains the methods by which annotations are attached to elements.
//!
//! ## Specification (§1 -- Introduction)
//!
//! Entities, axioms, and ontologies can be *annotated* in OWL 2. For example, a class can
//! be given a human-readable label that provides a more descriptive name for the class.
//! Annotations have no effect on the logical aspects of an ontology — that is, for the
//! purposes of the OWL 2 semantics, annotations are treated as not being present. Instead,
//! the use of annotations is left to the applications that use OWL 2. For example, a
//! graphical user interface might choose to visualize a class using one of its labels.
//!

use crate::{entities::AnonymousIndividual, fmt::DisplayPretty, literals::Literal};
use rdftk_iri::{Iri, Name};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Ontologies, axioms, and annotations themselves can be annotated using annotations ...
/// such annotations consist of an annotation property and an annotation value, where the
/// latter can be anonymous individuals, IRIs, and literals.
///
/// ## Specification (Section §10.1 -- Annotations of Ontologies, Axioms, and other Annotations)
///
/// ```bnf
/// Annotation :=
///     'Annotation' '('
///         annotationAnnotations AnnotationProperty AnnotationValue
///     ')'
///
/// annotationAnnotations  := { Annotation }
///
/// AnnotationValue := AnonymousIndividual | IRI | Literal
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Annotation {
    property: Iri,
    value: AnnotationValue,
    annotations: Vec<Annotation>, //0..*
}

///
/// This represents the internal production `AnnotationValue`.
///
/// ## Specification (Section §10.1 -- Annotations of Ontologies, Axioms, and other Annotations)
///
/// ```bnf
/// AnnotationValue :=
///     AnonymousIndividual | IRI | Literal
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum AnnotationValue {
    Iri(Iri),
    Literal(Literal),
    AnonymousIndividual(AnonymousIndividual),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn has_annotations(&self) -> bool;
    fn annotation_count(&self) -> usize;
    fn annotations(&self) -> Box<dyn Iterator<Item = &Annotation> + '_>;
    fn annotations_mut(&mut self) -> Box<dyn Iterator<Item = &mut Annotation> + '_>;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotation
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Annotation(property, value, @list annotations));
impl_has_annotations!(Annotation);

impl Annotation {
    pub fn new<AV>(property: Iri, value: AV) -> Self
    where
        AV: Into<AnnotationValue>,
    {
        Self::new_with_annotations(Vec::default(), property, value)
    }

    pub fn new_with_annotations<IA, AV>(annotations: IA, property: Iri, value: AV) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        AV: Into<AnnotationValue>,
    {
        Self {
            property,
            value: value.into(),
            annotations: annotations.into_iter().collect(),
        }
    }

    pub fn property(&self) -> &Iri {
        &self.property
    }

    pub fn value(&self) -> &AnnotationValue {
        &self.value
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ AnnotationValue
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(AnnotationValue enum Iri, Literal, AnonymousIndividual);

impl_from_for_variant!(AnnotationValue, Iri);
impl_from_for_variant!(AnnotationValue, Literal);
impl_from_for_variant!(AnnotationValue, Literal(from String));
impl_from_for_variant!(AnnotationValue, AnonymousIndividual);
impl_from_for_variant!(AnnotationValue, AnonymousIndividual(from Name));
