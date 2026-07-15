//!
//! This module provides all the types corresponding to the axioms in OWL 2, found in sections
//! 9 and 10.2.
//!

use crate::fmt::DisplayPretty;
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
#[allow(clippy::large_enum_variant)]
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
// Subtype Modules
// ------------------------------------------------------------------------------------------------

pub mod assertions;
pub use assertions::Assertion;
pub mod annotations;
pub use annotations::AnnotationAxiom;
pub mod classes;
pub use classes::ClassAxiom;
pub mod data_properties;
pub use data_properties::DataPropertyAxiom;
pub mod datatypes;
pub use datatypes::DatatypeDefinition;
pub mod declarations;
pub use declarations::Declaration;
pub mod has_keys;
pub use has_keys::HasKey;
pub mod object_properties;
pub use object_properties::ObjectPropertyAxiom;
