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
use rdftk_iri::{
    Iri,
    vocab::{VOCABULARY_OWL, VOCABULARY_RDF_SCHEMA, VOCABULARY_SKOS},
};
use strum::{EnumIs, EnumTryAs};

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
/// Annotation := 'Annotation' '(' annotationAnnotations AnnotationProperty AnnotationValue ')'
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
    annotations: Vec<Annotation>,
}

///
/// This represents the internal production `AnnotationValue`.
///
/// ## Specification (Section §10.1 -- Annotations of Ontologies, Axioms, and other Annotations)
///
/// ```bnf
/// AnnotationValue := AnonymousIndividual | IRI | Literal
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum AnnotationValue {
    Iri(Iri),
    Literal(Literal),
    AnonymousIndividual(AnonymousIndividual),
}

// ------------------------------------------------------------------------------------------------
// Public Constants
// ------------------------------------------------------------------------------------------------

macro_rules! make_static_annotation_iri {
    ($name:ident => $vocab_name:ident : $type_name:ident) => {
        #[doc = "Constant IRI for common annotation property."]
        pub static $name: ::std::sync::LazyLock<Iri> = ::std::sync::LazyLock::new(|| {
            $vocab_name
                .iri_as_iri()
                .with_new_fragment(stringify!($type_name))
        });
    };
}

make_static_annotation_iri!(ANN_RDFS_COMMENT => VOCABULARY_RDF_SCHEMA:comment);
make_static_annotation_iri!(ANN_RDFS_IS_DEFINED_BY => VOCABULARY_RDF_SCHEMA:isDefinedBy);
make_static_annotation_iri!(ANN_RDFS_LABEL => VOCABULARY_RDF_SCHEMA:label);
make_static_annotation_iri!(ANN_RDFS_SEE_ALSO => VOCABULARY_RDF_SCHEMA:seeAlso);

make_static_annotation_iri!(ANN_OWL_DEPRECATED => VOCABULARY_OWL:deprecated);
make_static_annotation_iri!(ANN_OWL_BACKWARD_COMPATIBLE_WITH => VOCABULARY_OWL:deprecated);
make_static_annotation_iri!(ANN_OWL_INCOMPATIBLE_WITH => VOCABULARY_OWL:incompatibleWith);
make_static_annotation_iri!(ANN_OWL_PRIOR_VERSION => VOCABULARY_OWL:priorVersion);
make_static_annotation_iri!(ANN_OWL_VERSION_INFO => VOCABULARY_OWL:versionInfo);

make_static_annotation_iri!(ANN_SKOS_ALT_LABEL => VOCABULARY_SKOS:altLabel);
make_static_annotation_iri!(ANN_SKOS_PREF_LABEL => VOCABULARY_SKOS:prefLabel);
make_static_annotation_iri!(ANN_SKOS_HIDDEN_LABEL => VOCABULARY_SKOS:hiddenLabel);
make_static_annotation_iri!(ANN_SKOS_NOTE => VOCABULARY_SKOS:note);
make_static_annotation_iri!(ANN_SKOS_CHANGE_NOTE => VOCABULARY_SKOS:changeNote);
make_static_annotation_iri!(ANN_SKOS_EDITORIAL_NOTE => VOCABULARY_SKOS:editorialNote);
make_static_annotation_iri!(ANN_SKOS_HISTORY_NOTE => VOCABULARY_SKOS:historyNote);
make_static_annotation_iri!(ANN_SKOS_SCOPE_NOTE => VOCABULARY_SKOS:scopeNote);
make_static_annotation_iri!(ANN_SKOS_DEFINITION => VOCABULARY_SKOS:definition);
make_static_annotation_iri!(ANN_SKOS_EXAMPLE => VOCABULARY_SKOS:example);

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Traits
// ------------------------------------------------------------------------------------------------

pub trait HasAnnotations {
    fn has_annotations(&self) -> bool;
    fn annotations(&self) -> Box<dyn Iterator<Item = &Annotation> + '_>;
    fn annotations_mut(&mut self) -> Box<dyn Iterator<Item = &mut Annotation> + '_>;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotation
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Annotation(property, value, @list annotations));
impl_has_annotations!(Annotation);

impl Annotation {
    pub fn new(property: Iri, value: AnnotationValue) -> Self {
        Self {
            property,
            value,
            annotations: Default::default(),
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
