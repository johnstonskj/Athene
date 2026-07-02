//!
//! This package provides an OWL API that provides as an implementation as close as possible to
//! the OWL 2 Web Ontology Language [Structural Specification](https://www.w3.org/TR/owl2-syntax/).
//!
//! As the structural specification relies heavily on an object-oriented model with inheritence
//! of key concepts there has had to be a mapping from the modeling in the specification to the
//! API herein. That mapping can be describe as follows.
//!
//! 1. Parent types, which are usually abstract, in the OWL 2 specification are present in the
//!    API as enumerated types.
//!    1. The `strum` crate provides `is_{variant}` and `trye_as_{variant}` methods on the enum
//!       to access sub-types.
//!    2. As all sub-types are distinct each super-type provides implementations of `From`
//!       between parent and each child type.
//! 2. Attributes present on parent types are pushed down (replicated) on all sub-types; however,
//!    accessors are present on the enumeration for these attributes.
//! 3. Constructors, usually `new(...)`, are provided for simple cases such as `Declaration`
//!    with more complex cases using a builder pattern.
//!
//! As the primary goal in the design of the package interface is consistency with the OWL 2
//! structural specification, rather than writing documentation from scratch we will rely on the
//! text of the OWL specification. Each section of Rust documentation will have a sub-section
//! titled **Specification (Section X.Y)** denoting the source location. Examples in the Rust
//! documentation will reference examples in the same section of the source, with the OWL
//! functional syntax shown with Rust equivalent when relevant.
//!
//! ## OWL 2 (Section §1 -- Introduction)
//!
//! An OWL 2 ontology is a formal description of a domain of interest. OWL 2 ontologies consist
//! of the following three different syntactic categories:
//!
//! * *Entities*, such as classes, properties, and individuals, are identified by IRIs. They form
//!   the primitive *terms* of an ontology and constitute the basic elements of an ontology. For
//!   example, a class *a:Person* can be used to represent the set of all people. Similarly, the
//!   object property *a:parentOf* can be used to represent the parent-child relationship. Finally,
//!   the individual *a:Peter* can be used to represent a particular person called "Peter".
//! * *Expressions* represent complex notions in the domain being described. For example, a *class
//!   expression* describes a set of individuals in terms of the restrictions on the individuals'
//!   characteristics.
//! * *Axioms* are statements that are asserted to be true in the domain being described. For
//!   example, using a *subclass axiom*, one can state that the class *a:Student* is a subclass of
//!   the class *a:Person*.
//!
//! ## Specification (Section -- §Abstract)
//!
//! The OWL 2 Web Ontology Language, informally OWL 2, is an ontology language for the Semantic
//! Web with formally defined meaning. OWL 2 ontologies provide classes, properties, individuals,
//! and data values and are stored as Semantic Web documents. OWL 2 ontologies can be used along
//! with information written in RDF, and OWL 2 ontologies themselves are primarily exchanged as
//! RDF documents. The OWL 2 Document Overview describes the overall state of OWL 2, and should be
//! read before other OWL 2 documents.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use crate::{
    annotations::Annotation,
    axioms::Axiom,
    builders::{OntologyBuilder, OntologyDocumentBuilder},
    fmt::{DisplayPretty, Indenter},
    syntax::{DELIM_FN_ARGS_END, DELIM_FN_ARGS_START, FN_PREFIX},
};
use core::{
    cell::{Ref, RefCell},
    fmt::{Display, Formatter, Result as FmtResult},
};
use rdftk_iri::{Iri, IriPrefixMap};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[macro_use]
mod macros;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A functional-style syntax ontology document is a sequence of Unicode characters accessible via
/// some IRI by means of the standard protocols such that its text matches the `ontologyDocument`
/// production of the grammar defined in this specification document, and it can be converted into
/// an ontology by means of the canonical parsing process described in Section 3.6 and other parts
/// of [this specification document](https://www.w3.org/TR/owl2-syntax/). A functional-style syntax
/// ontology document should use the UTF-8 encoding.
///
/// Note that the Rust type `OntologyDocument` implements the internal production
/// `ontologyDocument`.
///
/// ## Specification (Section §3.7 -- Functional-Style Syntax)
///
/// ```bnf
/// ontologyDocument := { prefixDeclaration } Ontology
///
/// prefixDeclaration := 'Prefix' '(' prefixName '=' fullIRI ')'
/// ```
///
/// ## Examples
///
/// ```owl
/// Prefix(:=<http://www.example.com/ontology1#>)
/// Ontology( <http://www.example.com/ontology1>
///     Import( <http://www.example.com/ontology2> )
///     Annotation( rdfs:label "An example" )
///
///     SubClassOf( :Child owl:Thing )
/// )
/// ```
///
/// ```rust
/// use athene_owlapi::{
///     Ontology, OntologyDocument,
///     axioms::SubClassOf,
///     builders::{AnnotationBuilder, Builder},
///     entities::{Class, EntityTrait},
///     things::owl,
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let document = OntologyDocument::builder()
///    .with_default_namespace(Iri::from_str("http://www.example.com/ontology1#").unwrap())
///    .with_ontology(Ontology::builder()
///        .with_ontology_iri(Iri::from_str("http://www.example.com/ontology1").unwrap())
///        .with_direct_import(Iri::from_str("http://www.example.com/ontology2").unwrap())
///        .with_rdfs_label("An example")
///        .with_class_axiom(SubClassOf::new(
///            Class::new(Iri::from_str("http://www.example.com/ontology1#Child").unwrap()),
///            Class::new(owl::thing_iri()),
///        ))
///        .build()
///        .expect("could not build Ontology"))
///    .build()
///    .expect("could not build OntologyDocument");
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct OntologyDocument {
    prefix_map: RefCell<IriPrefixMap>,
    ontology: Ontology,
}

///
/// An OWL 2 *ontology& is an instance $O$ of the **Ontology** UML class from the structural
/// specification of OWL 2 shown in Figure 1 that satisfies certain conditions given below.
///
/// The main component of an OWL 2 ontology is its set of axioms, the structure of which is
/// described in more detail in Section 9. Because the association between an ontology and its
/// axioms is a set, an ontology cannot contain two axioms that are structurally equivalent.
/// Apart from axioms, ontologies can also contain ontology annotations (as described in more
/// detail in Section 3.5), and they can also import other ontologies (as described in Section
/// 3.4).
///
/// ![Figure 1. The Structure of OWL 2 Ontologies](https://www.w3.org/TR/owl2-syntax/Ontology.gif)
///
/// ## Specification (Section §3.7 -- Functional-Style Syntax)
///
/// ```bnf
/// Ontology :=
///     'Ontology' '(' [ ontologyIRI [ versionIRI ] ]
///        directlyImportsDocuments
///        ontologyAnnotations
///        axioms
///     ')'
///
/// ontologyIRI := IRI
///
/// versionIRI := IRI
///
/// directlyImportsDocuments := { 'Import' '(' IRI ')' }
///
/// axioms := { Axiom }
/// ```
///
/// ## Examples
///
/// From section 3.4 *Imports*.
///
/// ```owl
/// Ontology( <http://www.example.com/importing-ontology>
///     Import( <http://www.example.com/my/2.0> )
///
///     # ...
/// )
/// ```
///
/// ```rust
/// use athene_owlapi::{Ontology, OntologyDocument, builders::Builder};
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let document = OntologyDocument::builder()
///     .with_default_namespace(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
///     .with_ontology(
///         Ontology::builder()
///             .with_ontology_iri(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
///             .with_direct_import(Iri::from_str("http://www.example.com/my/2.0").unwrap())
///             .build()
///             .expect("could not build Ontology"))
///     .build()
///     .expect("could not build OntologyDocument");
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Ontology {
    ontology_iri: Option<Iri>,
    version_iri: Option<Iri>,
    direct_imports: Vec<Import>,
    annotations: Vec<Annotation>,
    axioms: Vec<Axiom>,
}

///
/// This *pseudo-function* simply wraps an IRI value, but by providing a type of it's own it
/// has it's own implementation behavior and serialization rules.
///
/// ## Specification (Section §3.7 -- Functional-Style Syntax)
///
/// ```bnf
/// 'Import' '(' IRI ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    iri: Iri,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// This is a list of all reserved IRIs defined in the standard, the prefix names for
/// these are fixed and **may not** be redefined.
///
/// ## Specification (Section §2.4 -- IRIs)
///
/// See Table 2, *Declarations of the Standard Prefix Names*.
///
pub fn reserved_prefix_map() -> IriPrefixMap {
    IriPrefixMap::default()
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ OntologyDocument
// ------------------------------------------------------------------------------------------------

impl OntologyDocument {
    ///
    /// Returns a new builder object to construct an ontology document.
    ///
    pub fn builder() -> OntologyDocumentBuilder {
        OntologyDocumentBuilder::default()
    }

    pub fn has_prefix_mappings(&self) -> bool {
        !self.prefix_map.borrow().is_empty()
    }

    pub fn prefix_mappings(&self) -> Ref<'_, IriPrefixMap> {
        self.prefix_map.borrow()
    }

    pub fn ontology(&self) -> &Ontology {
        &self.ontology
    }
}

impl Display for OntologyDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_pretty(f, &Indenter::default(), &self.prefix_map.borrow())
    }
}

impl DisplayPretty for OntologyDocument {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        _: &IriPrefixMap,
    ) -> FmtResult {
        let reserved_prefix_map_copy = reserved_prefix_map();
        let reserved_prefix_iris: Vec<&Iri> = reserved_prefix_map_copy
            .mappings()
            .map(|(_, ns)| ns)
            .collect();
        let outer_separator = indenter.separator_string(f.alternate());
        for (i, (prefix, iri)) in self.prefix_mappings().mappings().enumerate() {
            if !reserved_prefix_iris.contains(&iri) {
                if i > 0 {
                    write!(f, "{outer_separator}",)?;
                }
                write!(f, "{FN_PREFIX}{DELIM_FN_ARGS_START}")?;
                let inner_separator = if f.alternate() {
                    indenter.indent();
                    indenter.separator_string(f.alternate())
                } else {
                    outer_separator.clone()
                };
                // Use Iri.fmt, not fmt_pretty, we do not want IRI compression.
                write!(f, "{inner_separator}{prefix} = {iri}",)?;
                write!(f, "{outer_separator}{DELIM_FN_ARGS_END}")?;
                if f.alternate() {
                    indenter.outdent();
                }
            }
        }

        write!(f, "{outer_separator}",)?;
        self.ontology()
            .fmt_pretty(f, indenter, &self.prefix_mappings())?;

        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Ontology
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
Ontology(
    @optional ontology_iri,
    @optional version_iri,
    @list direct_imports,
    @list annotations,
    @list axioms
));

impl_has_annotations!(Ontology);

impl Ontology {
    pub fn builder() -> OntologyBuilder {
        OntologyBuilder::default()
    }

    pub fn has_ontology_iri(&self) -> bool {
        self.ontology_iri.is_some()
    }

    pub fn ontology_iri(&self) -> Option<&Iri> {
        self.ontology_iri.as_ref()
    }

    pub fn has_version_iri(&self) -> bool {
        self.version_iri.is_some()
    }

    pub fn version_iri(&self) -> Option<&Iri> {
        self.version_iri.as_ref()
    }

    pub fn has_direct_imports(&self) -> bool {
        !self.direct_imports.is_empty()
    }

    pub fn direct_imports(&self) -> impl Iterator<Item = &Import> {
        self.direct_imports.iter()
    }

    pub fn has_axioms(&self) -> bool {
        !self.axioms.is_empty()
    }

    pub fn axioms(&self) -> impl Iterator<Item = &Axiom> {
        self.axioms.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Import
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Import(iri));

impl From<Iri> for Import {
    fn from(iri: Iri) -> Self {
        Import { iri }
    }
}

impl From<&Iri> for Import {
    fn from(iri: &Iri) -> Self {
        Import { iri: iri.clone() }
    }
}

impl AsRef<Iri> for Import {
    fn as_ref(&self) -> &Iri {
        &self.iri
    }
}

impl Import {
    pub fn iri(&self) -> &Iri {
        &self.iri
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;
pub mod fmt;

pub mod annotations;
pub mod axioms;
pub mod entities;
pub mod expressions;
pub mod literals;
pub mod ranges;

pub mod values;

pub mod builders;

pub mod reader;
pub mod syntax;

pub mod things;
