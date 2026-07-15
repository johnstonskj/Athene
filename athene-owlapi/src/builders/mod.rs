//! This module provides builder types and traits for more complex constructs to make the
//! process of ontology development more ergonomic.
//!

use crate::{
    Import, Ontology, OntologyDocument,
    annotations::Annotation,
    axioms::{
        AnnotationAxiom, Assertion, Axiom, ClassAxiom, DataPropertyAxiom, DatatypeDefinition,
        Declaration, HasKey, ObjectPropertyAxiom,
    },
    error::ApiError,
    reserved_prefix_map,
    things::{owl, rdfs, skos},
};
use core::cell::RefCell;
use rdftk_iri::{Iri, IriPrefixMap, Namespace};

#[cfg(not(feature = "std"))]
use alloc::{string::ToString, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Traits
// ------------------------------------------------------------------------------------------------

///
/// This trait is implemented by all builder objects, call `build` to create an instance
/// of the type `Output`
///
pub trait Builder: Default + TryInto<Self::Output, Error = ApiError> {
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
    fn build(&self) -> Result<Self::Output, ApiError>;
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
    fn annotation(self, annotation: Annotation) -> Self;

    ///
    /// Add all these annotations to the accumulated set within the builder.
    ///
    fn annotations<I: IntoIterator<Item = Annotation>>(self, annotations: I) -> Self;

    with_this_annotation!(rdfs_comment => rdfs::comment());
    with_this_annotation!(rdfs_label => rdfs::label());
    with_this_annotation!(rdfs_see_also => rdfs::see_also());
    with_this_annotation!(rdfs_is_defined_by => rdfs::is_defined_by());

    with_this_annotation!(owl_deprecated => owl::deprecated());

    with_this_annotation!(skos_pref_label => skos::pref_label());
    with_this_annotation!(skos_alt_label => skos::alt_label());
    with_this_annotation!(skos_hidden_label => skos::hidden_label());
    with_this_annotation!(skos_note => skos::note());
    with_this_annotation!(skos_change_note => skos::change_note());
    with_this_annotation!(skos_definition => skos::definition());
    with_this_annotation!(skos_editorial_note => skos::editorial_note());
    with_this_annotation!(skos_example => skos::example());
    with_this_annotation!(skos_history_note => skos::history_note());
    with_this_annotation!(skos_scope_note => skos::scope_note());
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

impl_has_builder!(OntologyDocument, OntologyDocumentBuilder);

impl TryFrom<OntologyDocumentBuilder> for OntologyDocument {
    type Error = ApiError;

    fn try_from(builder: OntologyDocumentBuilder) -> Result<Self, Self::Error> {
        if let Some(ontology) = builder.ontology {
            Ok(OntologyDocument {
                prefix_map: RefCell::new(builder.mappings),
                reserved_len: reserved_prefix_map().len(),
                ontology,
            })
        } else {
            Err(ApiError::MissingField {
                name: "ontology".to_string(),
            })
        }
    }
}

impl_builder_try_from!(OntologyDocument, OntologyDocumentBuilder);

impl OntologyDocumentBuilder {
    ///
    /// Set the default prefix for this document.
    ///
    pub fn default_prefix(mut self, namespace_iri: Iri) -> Self {
        self.mappings.set_default_namespace(namespace_iri);
        self
    }

    ///
    /// Set a `name: iri` prefix mapping for this document.
    ///
    pub fn prefix(mut self, prefix: Namespace, namespace_iri: Iri) -> Self {
        self.mappings.insert(prefix, namespace_iri);
        self
    }

    ///
    /// Set the ontology embedded in this document.
    ///
    pub fn ontology(mut self, ontology: Ontology) -> Self {
        self.ontology = Some(ontology);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ OntologyBuilder
// ------------------------------------------------------------------------------------------------

impl_has_builder!(Ontology, OntologyBuilder);

impl TryFrom<OntologyBuilder> for Ontology {
    type Error = ApiError;

    fn try_from(builder: OntologyBuilder) -> Result<Self, Self::Error> {
        if builder.version_iri.is_some() && builder.ontology_iri.is_none() {
            Err(ApiError::UnexpectedDependentField {
                antecedent: "ontology_iri".to_string(),
                dependent: "version_iri".to_string(),
            })
        } else {
            Ok(Ontology {
                ontology_iri: builder.ontology_iri,
                version_iri: builder.version_iri,
                direct_imports: builder.direct_imports,
                annotations: builder.annotations,
                axioms: builder.axioms,
            })
        }
    }
}

impl_builder_try_from!(Ontology, OntologyBuilder);
impl_annotation_builder!(OntologyBuilder);

impl OntologyBuilder {
    pub fn ontology_iri(mut self, ontology_iri: Iri) -> Self {
        self.ontology_iri = Some(ontology_iri);
        self
    }

    pub fn version_iri(mut self, version_iri: Iri) -> Self {
        self.version_iri = Some(version_iri);
        self
    }

    pub fn imports<I>(mut self, direct_imports: I) -> Self
    where
        I: IntoIterator<Item = Import>,
    {
        self.direct_imports = direct_imports.into_iter().collect();
        self
    }

    pub fn import<T>(mut self, direct_import: T) -> Self
    where
        T: Into<Import>,
    {
        self.direct_imports.push(direct_import.into());
        self
    }

    with_this_annotation!(pub with_owl_backward_compatible_with => owl::backward_compatible_with());
    with_this_annotation!(pub with_owl_incompatible_with => owl::incompatible_with());
    with_this_annotation!(pub with_owl_prior_version => owl::prior_version());
    with_this_annotation!(pub with_owl_version_info => owl::version_info());

    pub fn axioms<I>(mut self, axioms: I) -> Self
    where
        I: IntoIterator<Item = Axiom>,
    {
        self.axioms = axioms.into_iter().collect();
        self
    }

    pub fn axiom<T>(mut self, axiom: T) -> Self
    where
        T: Into<Axiom>,
    {
        self.axioms.push(axiom.into());
        self
    }

    pub fn declaration<T>(self, declaration: T) -> Self
    where
        T: Into<Declaration>,
    {
        self.axiom(Axiom::from(declaration.into()))
    }

    pub fn class<T>(self, class_axiom: T) -> Self
    where
        T: Into<ClassAxiom>,
    {
        self.axiom(Axiom::ClassAxiom(class_axiom.into()))
    }

    pub fn object_property<T>(self, object_property_axiom: T) -> Self
    where
        T: Into<ObjectPropertyAxiom>,
    {
        self.axiom(Axiom::ObjectPropertyAxiom(object_property_axiom.into()))
    }

    pub fn data_property<T>(self, data_property_axiom: T) -> Self
    where
        T: Into<DataPropertyAxiom>,
    {
        self.axiom(Axiom::DataPropertyAxiom(data_property_axiom.into()))
    }

    pub fn datatype_definition(self, datatype_definition: DatatypeDefinition) -> Self {
        self.axiom(datatype_definition)
    }

    pub fn has_key(self, has_key: HasKey) -> Self {
        self.axiom(has_key)
    }

    pub fn assertion<T>(self, assertion: T) -> Self
    where
        T: Into<Assertion>,
    {
        self.axiom(Axiom::Assertion(assertion.into()))
    }

    pub fn annotation_axiom<T>(self, annotation_axiom: T) -> Self
    where
        T: Into<AnnotationAxiom>,
    {
        self.axiom(Axiom::AnnotationAxiom(annotation_axiom.into()))
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod axioms;
pub use axioms::*;

mod expressions;

mod ranges;
