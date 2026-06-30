//! This module provides builder types and traits for more complex constructs to make the
//! process of ontology development more ergonomic.
//!
use crate::{
    Import, Ontology, OntologyDocument,
    annotations::{self, Annotation},
    axioms::{
        AnnotationAxiom, Assertion, Axiom, ClassAxiom, DataPropertyAxiom, DatatypeDefinition,
        Declaration, HasKey, ObjectPropertyAxiom,
    },
    error::BuilderError,
};
use core::cell::RefCell;
use rdftk_iri::{Iri, IriPrefixMap, Namespace};

// ------------------------------------------------------------------------------------------------
// Implementation Macros
// ------------------------------------------------------------------------------------------------

macro_rules! with_this_annotation {
    ($vis:vis $fn_name:ident => $ann_iri:expr) => {
        #[doc = "Add this annotation to the accumulated set within the builder."]
        $vis fn $fn_name<T: Into<$crate::literals::Literal>>(self, value: T) -> Self
        where
            Self: Sized,
        {
            self.with_annotation($crate::annotations::Annotation::new(
                $ann_iri.clone(),
                $crate::annotations::AnnotationValue::Literal(value.into()),
            ))
        }
    };
}
macro_rules! impl_annotation_builder {
    ($type_name:ident, $member_name:ident) => {
        impl $crate::builders::AnnotationBuilder for $type_name {
            fn with_annotation(mut self, $member_name: $crate::annotations::Annotation) -> Self {
                self.$member_name.push($member_name);
                self
            }
            fn with_annotations<I>(mut self, $member_name: I) -> Self
            where
                I: IntoIterator<Item = $crate::annotations::Annotation>,
            {
                self.$member_name.extend($member_name.into_iter());
                self
            }
        }
    };
    ($type_name:ident) => {
        impl_annotation_builder!($type_name, annotations);
    };
}

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This trait is implemented by all builder objects, call `build` to create an instance
/// of the type `Output`
///
pub trait Builder {
    ///
    /// The type of the object created by this builder.
    ///
    /// Usually this is the type that created the builder instance in the first place,
    /// particularly if using the [`HasBuilder`] trait.
    type Output;

    ///
    /// Construct a new instance of `Self::Output`.
    ///
    /// This performs any validation of the collected values before construction rather
    /// than during assignments to allow for transient invalid states. Note also that
    /// self is an immutable reference, allowing the same builder to build multiple
    /// objects, or to build one, modify and build another.
    ///
    fn build(&self) -> Result<Self::Output, BuilderError>;
}

///
/// This trait is implemented by ontology types that provide builder objects.
///
pub trait HasBuilder {
    ///
    /// The type of the object created by the `Builder`.
    ///
    type Output;
    ///
    /// The type of the builder object returned by this ontology type.
    ///
    type Builder: Builder<Output = Self::Output>;

    ///
    /// Fetch the builder for this ontology type.
    ///
    fn builder() -> Self::Builder;
}

///
/// This trait is implemented by any other builder object whose ontology object also
/// has annotations.
///
/// This trait provides a common interface for adding annotations even though the API
/// may have different names for annotations in different ontology types. It also
/// provides a set of *short-cut* methods for adding annotations from popular
/// vocabularies.
///
pub trait AnnotationBuilder {
    ///
    /// Add this annotation to the accumulated set within the builder.
    ///
    fn with_annotation(self, annotation: Annotation) -> Self;

    ///
    /// Add all these annotations to the accumulated set within the builder.
    ///
    fn with_annotations<I: IntoIterator<Item = Annotation>>(self, annotations: I) -> Self;

    with_this_annotation!(with_rdfs_comment => annotations::ANN_RDFS_COMMENT);
    with_this_annotation!(with_rdfs_label => annotations::ANN_RDFS_LABEL);
    with_this_annotation!(with_rdfs_see_also => annotations::ANN_RDFS_SEE_ALSO);
    with_this_annotation!(with_rdfs_is_defined_by => annotations::ANN_RDFS_IS_DEFINED_BY);

    with_this_annotation!(with_owl_deprecated => annotations::ANN_OWL_DEPRECATED);

    with_this_annotation!(with_skos_pref_label => annotations::ANN_SKOS_PREF_LABEL);
    with_this_annotation!(with_skos_alt_label => annotations::ANN_SKOS_ALT_LABEL);
    with_this_annotation!(with_skos_hidden_label => annotations::ANN_SKOS_HIDDEN_LABEL);
    with_this_annotation!(with_skos_note => annotations::ANN_SKOS_NOTE);
    with_this_annotation!(with_skos_change_note => annotations::ANN_SKOS_CHANGE_NOTE);
    with_this_annotation!(with_skos_definition => annotations::ANN_SKOS_DEFINITION);
    with_this_annotation!(with_skos_editorial_note => annotations::ANN_SKOS_EDITORIAL_NOTE);
    with_this_annotation!(with_skos_example => annotations::ANN_SKOS_EXAMPLE);
    with_this_annotation!(with_skos_history_note => annotations::ANN_SKOS_HISTORY_NOTE);
    with_this_annotation!(with_skos_scope_note => annotations::ANN_SKOS_SCOPE_NOTE);
}

// ------------------------------------------------------------------------------------------------
// Public Builders
// ------------------------------------------------------------------------------------------------

///
/// A builder for the core `OntologyDocument` type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OntologyDocumentBuilder {
    mappings: IriPrefixMap,
    ontology: Option<Ontology>,
}

///
/// A builder for the core `Ontology` type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OntologyBuilder {
    ontology_iri: Option<Iri>,
    version_iri: Option<Iri>,
    direct_imports: Vec<Import>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ OntologyDocumentBuilder
// ------------------------------------------------------------------------------------------------

impl OntologyDocumentBuilder {
    pub fn with_default_namespace(mut self, namespace_iri: Iri) -> Self {
        self.mappings.set_default_namespace(namespace_iri);
        self
    }

    pub fn with_namespace_prefix(mut self, prefix: Namespace, namespace_iri: Iri) -> Self {
        self.mappings.insert(prefix, namespace_iri);
        self
    }

    pub fn with_ontology(mut self, ontology: Ontology) -> Self {
        self.ontology = Some(ontology);
        self
    }
}

impl Builder for OntologyDocumentBuilder {
    type Output = OntologyDocument;

    fn build(&self) -> Result<Self::Output, BuilderError> {
        if self.ontology.is_none() {
            Err(BuilderError::MissingField {
                name: "ontology".to_string(),
            })
        } else {
            Ok(OntologyDocument {
                prefix_map: RefCell::new(self.mappings.clone()),
                ontology: self.ontology.as_ref().map(|v| v.clone()).unwrap(),
            })
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ OntologyBuilder
// ------------------------------------------------------------------------------------------------

impl_annotation_builder!(OntologyBuilder);

impl OntologyBuilder {
    pub fn with_ontology_iri(mut self, ontology_iri: Iri) -> Self {
        self.ontology_iri = Some(ontology_iri);
        self
    }

    pub fn with_version_iri(mut self, version_iri: Iri) -> Self {
        self.version_iri = Some(version_iri);
        self
    }

    pub fn with_direct_imports(mut self, direct_imports: impl IntoIterator<Item = Import>) -> Self {
        self.direct_imports = direct_imports.into_iter().collect();
        self
    }

    pub fn with_direct_import<I: Into<Import>>(mut self, direct_import: I) -> Self {
        self.direct_imports.push(direct_import.into());
        self
    }

    with_this_annotation!(pub with_owl_backward_compatible_with => annotations::ANN_OWL_BACKWARD_COMPATIBLE_WITH);
    with_this_annotation!(pub with_owl_incompatible_with => annotations::ANN_OWL_INCOMPATIBLE_WITH);
    with_this_annotation!(pub with_owl_prior_version => annotations::ANN_OWL_PRIOR_VERSION);
    with_this_annotation!(pub with_owl_version_info => annotations::ANN_OWL_VERSION_INFO);

    pub fn with_axioms(mut self, axioms: impl IntoIterator<Item = Axiom>) -> Self {
        self.axioms = axioms.into_iter().collect();
        self
    }

    pub fn with_axiom(mut self, axiom: Axiom) -> Self {
        self.axioms.push(axiom);
        self
    }

    pub fn with_declaration<D: Into<Declaration>>(self, declaration: D) -> Self {
        self.with_axiom(Axiom::from(declaration.into()))
    }

    pub fn with_class_axiom<A: Into<ClassAxiom>>(self, class_axiom: A) -> Self {
        self.with_axiom(Axiom::ClassAxiom(class_axiom.into()))
    }

    pub fn with_object_property_axiom<A: Into<ObjectPropertyAxiom>>(
        self,
        object_property_axiom: A,
    ) -> Self {
        self.with_axiom(Axiom::ObjectPropertyAxiom(object_property_axiom.into()))
    }

    pub fn with_data_property_axiom<A: Into<DataPropertyAxiom>>(
        self,
        data_property_axiom: A,
    ) -> Self {
        self.with_axiom(Axiom::DataPropertyAxiom(data_property_axiom.into()))
    }

    pub fn with_datatype_definition(self, datatype_definition: DatatypeDefinition) -> Self {
        self.with_axiom(datatype_definition.into())
    }

    pub fn with_has_key(self, has_key: HasKey) -> Self {
        self.with_axiom(has_key.into())
    }

    pub fn with_assertion<A: Into<Assertion>>(self, assertion: A) -> Self {
        self.with_axiom(Axiom::Assertion(assertion.into()))
    }

    pub fn with_annotation_axiom<A: Into<AnnotationAxiom>>(self, annotation_axiom: A) -> Self {
        self.with_axiom(Axiom::AnnotationAxiom(annotation_axiom.into()))
    }
}

impl Builder for OntologyBuilder {
    type Output = Ontology;

    fn build(&self) -> Result<Self::Output, BuilderError> {
        if self.version_iri.is_some() && self.ontology_iri.is_none() {
            Err(BuilderError::UnexpectedDependentField {
                antecedent: "ontology_iri".to_string(),
                dependent: "version_iri".to_string(),
            })
        } else {
            Ok(Ontology {
                ontology_iri: self.ontology_iri.clone(),
                version_iri: self.version_iri.clone(),
                direct_imports: self.direct_imports.clone(),
                annotations: self.annotations.clone(),
                axioms: self.axioms.clone(),
            })
        }
    }
}
