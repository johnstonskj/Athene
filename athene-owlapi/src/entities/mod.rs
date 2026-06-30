//!
//! This module provides the set of Entity-related types referenced in Section 5.8 in support
//! of the `Declaration` Axiom.
//!
use crate::{
    error::ApiError,
    fmt::{DisplayPretty, Indenter},
    ranges::HasArity,
    values::UnboundedNatural,
};
use core::fmt::{Display, Formatter, Result as FmtResult};
use rdftk_iri::{Iri, IriPrefixMap, Name};
use std::str::FromStr;
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Each IRI $I$ used in an OWL 2 ontology $O$ can be, and sometimes even needs to be, declared in $O$;
/// roughly speaking, this means that the axiom closure of $O$ must contain an appropriate declaration
/// for $I$. A declaration for $I$ in $O$ serves two purposes:
///
/// * A declaration says that $I$ exists — that is, it says that $I$ is part of the vocabulary of $O$.
/// * A declaration associates with $I$ an entity type — that is, it says whether $I$ is used in $O$
///   as a class, datatype, object property, data property, annotation property, an individual, or
///   a combination thereof.
///
/// In OWL 2, declarations are a type of axiom; thus, to declare an entity in an ontology, one can
/// simply include the appropriate axiom in the ontology. These axioms are nonlogical in the sense
/// that they do not affect the consequences of an OWL 2 ontology.
///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// Entity :=
///     'Class' '(' Class ')' |
///     'Datatype' '(' Datatype ')' |
///     'ObjectProperty' '(' ObjectProperty ')' |
///     'DataProperty' '(' DataProperty ')' |
///     'AnnotationProperty' '(' AnnotationProperty ')' |
///     'NamedIndividual' '(' NamedIndividual ')'
/// ```
///
/// ## Examples
///
/// The following axioms state that the IRI `a:Person` is used as a class and that the IRI
/// `a:Peter` is used as an individual.
///
/// ```owl
/// Declaration( Class( a:Person ) )
/// Declaration( NamedIndividual( a:Peter ) )
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Entity {
    AnnotationProperty(AnnotationProperty),
    Class(Class),
    DataProperty(DataProperty),
    Datatype(Datatype),
    ObjectProperty(ObjectProperty),
    NamedIndividual(NamedIndividual),
}

///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'AnnotationProperty' '(' AnnotationProperty ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationProperty {
    entity_iri: Iri,
}

///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'Class' '(' Class ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    entity_iri: Iri,
}

///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'DataProperty' '(' DataProperty ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct DataProperty {
    entity_iri: Iri,
}

///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'Datatype' '(' Datatype ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Datatype {
    entity_iri: Iri,
    arity: UnboundedNatural,
}

///
/// ## Specification (§5.8 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'ObjectProperty' '(' ObjectProperty ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    entity_iri: Iri,
}

///
/// If an individual is not expected to be used outside a particular ontology, one can use an
/// anonymous individual, which is identified by a local node ID rather than a global IRI.
/// Anonymous individuals are analogous to blank nodes in RDF.
///
/// ## Specification (§5.8 - *Anonymous individuals*)
///
/// ```owl
/// AnonymousIndividual := nodeID
///
/// nodeID := SPARQL::BLANK_NODE_LABEL
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NamedIndividual {
    entity_iri: Iri,
}

pub trait EntityTrait: DisplayPretty + From<Iri> + Into<Iri> {
    fn new(entity_iri: Iri) -> Self;
    fn entity_iri(&self) -> &Iri;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Non-Entity
// ------------------------------------------------------------------------------------------------

///
/// ## Specification (§5.6.2 - *Entity Declarations and Typing*)
///
/// ```owl
/// 'NamedIndividual' '(' NamedIndividual ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousIndividual {
    node_id: Name,
}

#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Individual {
    NamedIndividual(NamedIndividual),
    AnonymousIndividual(AnonymousIndividual),
}

// ------------------------------------------------------------------------------------------------
// Implementations Macro
// ------------------------------------------------------------------------------------------------

macro_rules! impl_entity {
    ($type_name:ident) => {
        impl ::core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.fmt_pretty(
                    f,
                    &crate::fmt::Indenter::default(),
                    &::rdftk_iri::map::IriPrefixMap::default(),
                )
            }
        }
        impl crate::fmt::DisplayPretty for $type_name {
            fn fmt_pretty(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
                indenter: &crate::fmt::Indenter,
                prefix_map: &::rdftk_iri::map::IriPrefixMap,
            ) -> ::core::fmt::Result {
                //write!(f, "{}", indenter.separator_string(f.alternate()))?;
                self.entity_iri.fmt_pretty(f, &indenter, prefix_map)
            }
        }

        impl From<Iri> for $type_name {
            fn from(entity_iri: Iri) -> Self {
                Self { entity_iri }
            }
        }

        impl From<&Iri> for $type_name {
            fn from(entity_iri: &Iri) -> Self {
                Self {
                    entity_iri: entity_iri.clone(),
                }
            }
        }

        impl From<$type_name> for Iri {
            fn from(entity: $type_name) -> Iri {
                entity.entity_iri
            }
        }

        impl $crate::entities::EntityTrait for $type_name {
            fn new(entity_iri: Iri) -> Self {
                Self { entity_iri }
            }

            fn entity_iri(&self) -> &Iri {
                &self.entity_iri
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Entity
// ------------------------------------------------------------------------------------------------

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_pretty(f, &Indenter::default(), &IriPrefixMap::default())
    }
}

impl DisplayPretty for Entity {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::AnnotationProperty(_) => "AnnotationProperty(",
                Self::Class(_) => "Class(",
                Self::DataProperty(_) => "DataProperty(",
                Self::Datatype(_) => "Datatype(",
                Self::ObjectProperty(_) => "ObjectProperty(",
                Self::NamedIndividual(_) => "NamedIndividual(",
            }
        )?;
        if f.alternate() {
            let _ = indenter.indent();
        }
        let entity_iri = match self {
            Self::AnnotationProperty(v) => v.entity_iri(),
            Self::Class(v) => v.entity_iri(),
            Self::DataProperty(v) => v.entity_iri(),
            Self::Datatype(v) => v.entity_iri(),
            Self::ObjectProperty(v) => v.entity_iri(),
            Self::NamedIndividual(v) => v.entity_iri(),
        };
        write!(f, "{}", indenter.separator_string(f.alternate()))?;
        entity_iri.fmt_pretty(f, indenter, prefix_map)?;
        if f.alternate() {
            let _ = indenter.outdent();
        }
        write!(f, "{})", indenter.separator_string(f.alternate()))
    }
}

impl From<AnnotationProperty> for Entity {
    fn from(value: AnnotationProperty) -> Self {
        Self::AnnotationProperty(value)
    }
}

impl From<Class> for Entity {
    fn from(value: Class) -> Self {
        Self::Class(value)
    }
}

impl From<DataProperty> for Entity {
    fn from(value: DataProperty) -> Self {
        Self::DataProperty(value)
    }
}

impl From<Datatype> for Entity {
    fn from(value: Datatype) -> Self {
        Self::Datatype(value)
    }
}

impl From<ObjectProperty> for Entity {
    fn from(value: ObjectProperty) -> Self {
        Self::ObjectProperty(value)
    }
}

impl From<NamedIndividual> for Entity {
    fn from(value: NamedIndividual) -> Self {
        Self::NamedIndividual(value)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ AnnotationProperty
// ------------------------------------------------------------------------------------------------

impl_entity!(AnnotationProperty);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class
// ------------------------------------------------------------------------------------------------

impl_entity!(Class);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DataProperty
// ------------------------------------------------------------------------------------------------

impl_entity!(DataProperty);

// ------------------------------------------------------------------------------------------------
// Implementation ❯ Datatype
// ------------------------------------------------------------------------------------------------

// TODO: !! This is now invalid!
impl_display_pretty!(Datatype(entity_iri));

impl HasArity for Datatype {
    fn arity(&self) -> UnboundedNatural {
        self.arity
    }
}

impl From<Iri> for Datatype {
    fn from(entity_iri: Iri) -> Datatype {
        Self::new(entity_iri)
    }
}

impl Datatype {
    pub fn new(entity_iri: Iri) -> Self {
        Self {
            entity_iri,
            arity: UnboundedNatural::Bounded(1), // See note on Figure 2, Section 5.0
        }
    }

    pub fn entity_iri(&self) -> &Iri {
        &self.entity_iri
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ObjectProperty
// ------------------------------------------------------------------------------------------------

impl_entity!(ObjectProperty);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Individuals
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(Individual enum NamedIndividual, AnonymousIndividual);

// ------------------------------------------------------------------------------------------------

impl_entity!(NamedIndividual);

// ------------------------------------------------------------------------------------------------

impl Display for AnonymousIndividual {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_pretty(f, &Indenter::default(), &IriPrefixMap::default())
    }
}

impl DisplayPretty for AnonymousIndividual {
    fn fmt_pretty(&self, f: &mut Formatter<'_>, _: &Indenter, _: &IriPrefixMap) -> FmtResult {
        write!(f, "_:{}", self.node_id)
    }
}

impl FromStr for AnonymousIndividual {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            node_id: Name::from_str(s).map_err(|e| {
                ApiError::ValueParser("AnonymousIndividual", e.to_string(), s.to_string())
            })?,
        })
    }
}

impl From<Name> for AnonymousIndividual {
    fn from(node_id: Name) -> Self {
        Self { node_id }
    }
}

impl AnonymousIndividual {
    pub fn node_id(&self) -> &Name {
        &self.node_id
    }
}
