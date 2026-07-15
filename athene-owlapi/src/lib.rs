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
//!    1. The [`strum`](https://crates.io/crates/strum) crate provides `is_{variant}` and
//!       `try_as_{variant}` methods on the enum to access sub-type variants.
//!    2. As all sub-types are distinct, each super-type provides implementations of `From`
//!       between parent and each child type. This also has to be implemented transitively.
//!    3. API methods taking parent types do so using generics `T: Into<Parent>`.
//! 2. Attributes present on parent types are pushed down (replicated) on all sub-types; however,
//!    accessors are present on the enumeration for these attributes.
//!    1. Where it makes sense, this is then extracted to a trait implemented by all child
//!       types *and* the parent enumeration.
//! 3. Constructors, usually `new(...)`, are provided for simple cases such as `Declaration`
//!    with more complex cases using a builder pattern.
//!    1. For the simplest cases where a type, say `Declaration`, only contains a single other
//!       value, in this case an `Entity`, a `From` implementation will be provided.
//!    2. Addition constructors of the form `new_with_annotations(...)` are provided for those
//!       types that may be annotated.
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
    axioms::{
        AnnotationAxiom, Assertion, Axiom, ClassAxiom, DataPropertyAxiom, DatatypeDefinition,
        Declaration, HasKey, ObjectPropertyAxiom,
    },
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
///     axioms::classes::SubClassOf,
///     builders::{AnnotationBuilder, Builder, HasBuilder},
///     entities::{Class, EntityTrait},
///     things::owl,
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let document = OntologyDocument::builder()
///    .default_prefix(Iri::from_str("http://www.example.com/ontology1#").unwrap())
///    .ontology(Ontology::builder()
///        .ontology_iri(Iri::from_str("http://www.example.com/ontology1").unwrap())
///        .import(Iri::from_str("http://www.example.com/ontology2").unwrap())
///        .rdfs_label("An example")
///        .axiom(SubClassOf::new(
///            Class::new(Iri::from_str("http://www.example.com/ontology1#Child").unwrap()),
///            Class::new(owl::thing()),
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
    reserved_len: usize,
    ontology: Ontology,
}

///
/// An OWL 2 *ontology* is an instance $O$ of the **Ontology** UML class from the structural
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
/// use athene_owlapi::{Ontology, OntologyDocument, builders::{Builder, HasBuilder}};
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let document = OntologyDocument::builder()
///     .default_prefix(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
///     .ontology(
///         Ontology::builder()
///             .ontology_iri(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
///             .import(Iri::from_str("http://www.example.com/my/2.0").unwrap())
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

impl OntologyDocument {
    pub fn has_prefix_mappings(&self) -> bool {
        !self.prefix_map.borrow().is_empty()
    }

    pub fn prefix_mappings(&self) -> Ref<'_, IriPrefixMap> {
        self.prefix_map.borrow()
    }

    pub fn set_prefix_mappings(&mut self, prefix_mappings: IriPrefixMap) {
        self.prefix_map = RefCell::new(prefix_mappings);
    }

    pub fn clear_prefix_mappings(&mut self) {
        self.prefix_map = RefCell::new(IriPrefixMap::default());
    }

    pub fn prefix_mapping_count(&self) -> usize {
        self.prefix_map.borrow().len()
    }

    pub fn user_prefix_mapping_count(&self) -> usize {
        self.prefix_map.borrow().len() - self.reserved_len
    }

    pub fn is_user_prefix_mapping_empty(&self) -> bool {
        self.user_prefix_mapping_count() == 0
    }

    pub fn ontology(&self) -> &Ontology {
        &self.ontology
    }

    pub fn set_ontology(&mut self, ontology: Ontology) {
        self.ontology = ontology;
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
    pub fn has_ontology_iri(&self) -> bool {
        self.ontology_iri.is_some()
    }

    pub fn ontology_iri(&self) -> Option<&Iri> {
        self.ontology_iri.as_ref()
    }

    pub fn set_ontology_iri(&mut self, ontology_iri: Iri) {
        self.ontology_iri = Some(ontology_iri);
    }

    pub fn unset_ontology_iri(&mut self) {
        self.ontology_iri = None;
    }

    pub fn has_version_iri(&self) -> bool {
        self.version_iri.is_some()
    }

    pub fn version_iri(&self) -> Option<&Iri> {
        self.version_iri.as_ref()
    }

    pub fn set_version_iri(&mut self, version_iri: Iri) {
        self.version_iri = Some(version_iri);
    }

    pub fn unset_version_iri(&mut self) {
        self.version_iri = None;
    }

    pub fn has_direct_imports(&self) -> bool {
        !self.direct_imports.is_empty()
    }

    pub fn direct_import_count(&self) -> usize {
        self.direct_imports.len()
    }

    pub fn direct_imports(&self) -> impl Iterator<Item = &Import> {
        self.direct_imports.iter()
    }

    pub fn add_direct_import<I>(&mut self, direct_import: I)
    where
        I: Into<Import>,
    {
        self.direct_imports.push(direct_import.into());
    }

    pub fn set_direct_imports<I>(&mut self, direct_imports: I)
    where
        I: IntoIterator<Item = Import>,
    {
        self.direct_imports = direct_imports.into_iter().collect();
    }

    pub fn extend_direct_imports<I>(&mut self, direct_imports: I)
    where
        I: IntoIterator<Item = Import>,
    {
        self.direct_imports.extend(direct_imports);
    }

    pub fn has_axioms(&self) -> bool {
        !self.axioms.is_empty()
    }

    pub fn axioms(&self) -> impl Iterator<Item = &Axiom> {
        self.axioms.iter()
    }

    ///
    /// A query returning only `Declaration` axioms in this Ontology.
    ///
    #[inline(always)]
    pub fn declarations(&self) -> impl Iterator<Item = &Declaration> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::Declaration(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `ClassAxiom`s in this Ontology.
    ///
    #[inline(always)]
    pub fn classes(&self) -> impl Iterator<Item = &ClassAxiom> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::ClassAxiom(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `ObjectPropertyAxiom`s in this Ontology.
    ///
    #[inline(always)]
    pub fn object_properties(&self) -> impl Iterator<Item = &ObjectPropertyAxiom> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::ObjectPropertyAxiom(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `DataPropertyAxiom`s in this Ontology.
    ///
    #[inline(always)]
    pub fn data_properties(&self) -> impl Iterator<Item = &DataPropertyAxiom> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::DataPropertyAxiom(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `DatatypeDefinition`s in this Ontology.
    ///
    #[inline(always)]
    pub fn datatypes(&self) -> impl Iterator<Item = &DatatypeDefinition> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::DatatypeDefinition(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `HasKey` axioms in this Ontology.
    ///
    #[inline(always)]
    pub fn has_keys(&self) -> impl Iterator<Item = &HasKey> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::HasKey(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `Assertion` axioms in this Ontology.
    ///
    #[inline(always)]
    pub fn assertions(&self) -> impl Iterator<Item = &Assertion> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::Assertion(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
    }

    ///
    /// A query returning only `AnnotationAxiom`s in this Ontology.
    ///
    #[inline(always)]
    pub fn annotation_axioms(&self) -> impl Iterator<Item = &AnnotationAxiom> {
        self.axioms().filter_map(|axiom| {
            if let Axiom::AnnotationAxiom(axiom) = axiom {
                Some(axiom)
            } else {
                None
            }
        })
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

pub mod annotations;
pub mod axioms;
pub mod builders;
pub mod entities;
pub mod expressions;
pub mod fmt;
pub mod literals;
pub mod ranges;
pub mod reader;
pub mod syntax;
pub mod things;
pub mod values;

#[cfg(feature = "help")]
pub mod help;
