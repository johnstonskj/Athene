use crate::{
    annotations::{Annotation, HasAnnotations},
    entities::Entity,
    fmt::DisplayPretty,
};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

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
/// The following Rust is quite long, but wraps these two axioms in a complete ontology
/// so they can be seen in context. This also allows the display functions to use the
/// prefix mapping to compress IRIs into the qualified name form.
///
/// ```rust
/// use athene_owlapi::{
///     Ontology, OntologyDocument,
///     builders::{Builder, HasBuilder},
///     entities::{Class, NamedIndividual},
/// };
/// use rdftk_iri::{Iri, Name, Namespace};
/// use std::str::FromStr;
///
/// let ontology_iri = Iri::from_str("http://www.example.com/an-ontology/").unwrap();
/// let person_iri = ontology_iri.make_name(Name::new_unchecked("Person")).unwrap();
/// let peter_iri = ontology_iri.make_name(Name::new_unchecked("Peter")).unwrap();
///
/// let ontology = OntologyDocument::builder()
///     .prefix(Namespace::new_unchecked("a"), ontology_iri.clone())
///     .ontology(
///         Ontology::builder()
///             .ontology_iri(ontology_iri.clone())
///             .declaration(Class::from(person_iri))
///             .declaration(NamedIndividual::from(peter_iri))
///             .build()
///             .expect("could not build Ontology"))
///     .build()
///     .expect("could not build Ontology");
///
/// assert_eq!(
///     "Prefix(
///     a: = <http://www.example.com/an-ontology/>
/// )
/// Ontology(
///     <http://www.example.com/an-ontology/>
///     Declaration(
///         Class(
///             a:Person
///         )
///     )
///     Declaration(
///         NamedIndividual(
///             a:Peter
///         )
///     )
/// )",
///     format!("{ontology:#}")
/// );
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Declaration {
    axiom_annotations: Vec<Annotation>,
    entity: Entity,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Declaration
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Declaration(@list axiom_annotations, entity));
impl_has_annotations!(Declaration, axiom_annotations);

impl<E> From<E> for Declaration
where
    E: Into<Entity>,
{
    fn from(entity: E) -> Self {
        Self::new(entity)
    }
}

impl Declaration {
    pub fn new<E>(entity: E) -> Self
    where
        E: Into<Entity>,
    {
        Self::new_with_annotations(Vec::default(), entity)
    }

    pub fn new_with_annotations<I, E>(axiom_annotations: I, entity: E) -> Self
    where
        I: IntoIterator<Item = Annotation>,
        E: Into<Entity>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            entity: entity.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn set_entity<E>(&mut self, entity: E)
    where
        E: Into<Entity>,
    {
        self.entity = entity.into();
    }
}
