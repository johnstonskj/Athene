//!
//! This module provides the set of Entity-related types deascriobed in section 5.
//!
//! Entities are the fundamental building blocks of OWL 2 ontologies, and they define the
//! vocabulary — the named terms — of an ontology. In logic, the set of entities is usually said to
//! constitute the signature of an ontology. Apart from entities, OWL 2 ontologies typically also
//! contain literals, such as strings or integers.
//!
//! The structure of entities and literals in OWL 2 is shown in Figure 2. Classes, datatypes, object
//! properties, data properties, annotation properties, and named individuals are entities, and they
//! are all uniquely identified by an IRI. Classes represent sets of individuals; datatypes are sets
//! of literals such as strings or integers; object and data properties can be used to represent
//! relationships in the domain; annotation properties can be used to associate nonlogical information
//! with ontologies, axioms, and entities; and named individuals can be used to represent actual objects
//! from the domain. Apart from named individuals, OWL 2 also provides for anonymous individuals — that
//! is, individuals that are analogous to blank nodes in RDF [RDF Concepts] and that are accessible only
//! from within the ontology they are used in. Finally, OWL 2 provides for literals, which consist of
//! a string called a lexical form and a datatype specifying how to interpret this string.
//!
//! ![Figure 2. Entities, Literals, and Anonymous Individuals in OWL 2](https://www.w3.org/TR/owl2-syntax/C_entities.gif)
//!

use crate::{
    axioms::Declaration,
    error::ApiError,
    fmt::{DisplayPretty, Indenter},
    ranges::HasArity,
    syntax::{
        ANONYMOUS_NAMESPACE, DELIM_ARGS_GROUP_START, DELIM_FN_ARGS_END, FN_ANNOTATION_PROPERTY,
        FN_ANONYMOUS_INDIVIDUAL, FN_CLASS, FN_DATA_PROPERTY, FN_DATATYPE, FN_NAMED_INDIVIDUAL,
        FN_OBJECT_PROPERTY, NAMESPACE_NAME_SEPARATOR,
    },
    things::{owl, rdf, rdfs, skos},
    values::UnlimitedNatural,
};
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use rdftk_iri::{Iri, IriPrefixMap, Name};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{format, string::ToString};

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
/// ## Specification (Section §5.8 - *Entity Declarations and Typing*)
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
/// ```rust
/// use athene_owlapi::{
///     entities::{Class, EntityTrait, IriAsEntity, NamedIndividual},
///     things::rdfs,
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let person = Iri::from_str("https://example.com/ns/Person").unwrap();
/// let peter = Iri::from_str("https://example.com/ns/Peter").unwrap();
///
/// // Using EntityTrait::new
/// assert_eq!(
///     "Declaration( Class( <https://example.com/ns/Person> ) )".to_string(),
///     Class::new(person.clone()).as_declaration().to_string()
/// );
///
/// // Using IriAsEntity::as_class
/// assert_eq!(
///     "Declaration( Class( <https://example.com/ns/Person> ) )".to_string(),
///     person.as_class().as_declaration().to_string()
/// );
///
/// // Using EntityTrait::new
/// assert_eq!(
///     "Declaration( NamedIndividual( <https://example.com/ns/Peter> ) )".to_string(),
///     NamedIndividual::new(peter.clone()).as_declaration().to_string()
/// );
///
/// // Using IriAsEntity::as_class
/// assert_eq!(
///     "Declaration( NamedIndividual( <https://example.com/ns/Peter> ) )".to_string(),
///     peter.as_named_individual().as_declaration().to_string()
/// );
// ```
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
/// Classes can be understood as sets of individuals.
///
/// The classes with the IRIs *owl:Thing* and *owl:Nothing* are available in OWL 2 as built-in
/// classes with a predefined semantics:
///
/// * The class with IRI *owl:Thing* represents the set of all individuals. (In the DL literature
///   this is often called the top concept.)
/// * The class with IRI *owl:Nothing* represents the empty set. (In the DL literature this is often
///   called the bottom concept.)
///
/// IRIs from the reserved vocabulary other than *owl:Thing* and *owl:Nothing* must not be used to
/// identify classes in an OWL 2 DL ontology.
///
/// ## Specification (Section §5.1 -- Classes
///
/// ```owl
/// Class := IRI
/// ```
///
/// ## Examples
///
/// ```rust
/// use athene_owlapi::{
///     entities::{Class, EntityTrait, IriAsEntity},
///     things::rdfs,
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let iri = Iri::from_str("https://example.com/ns/Foo").unwrap();
///
/// // Using EntityTrait::new
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     Class::new(iri.clone()).to_string()
/// );
///
/// // Using From<Iri>
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     Class::from(iri.clone()).to_string()
/// );
///
/// // Using IriAsEntity::as_class
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     iri.as_class().to_string()
/// );
///
/// // Using a predefined IRI
/// assert_eq!(
///     "rdfs:Resource".to_string(),
///     rdfs::resource().as_class().to_string()
/// );
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    entity_iri: Iri,
}

///
/// *Datatypes* are entities that refer to sets of data values.
///
/// Thus, datatypes are analogous to classes, the main difference being that the former contain data
/// values such as strings and numbers, rather than individuals. Datatypes are a kind of data range,
/// which allows them to be used in restrictions. As explained in Section 7, each data range is
/// associated with an arity; for datatypes, the arity is always one. The built-in datatype *rdfs:Literal*
/// denotes any set of data values that contains the union of the value spaces of all datatypes.
///
/// An IRI used to identify a datatype in an OWL 2 DL ontology must
///
/// * be *rdfs:Literal*, or
/// * identify a datatype in the OWL 2 datatype map (see Section 4), or
/// * not be in the reserved vocabulary of OWL 2 (see Section 2.4).
///
/// The conditions from the previous paragraph and the restrictions on datatypes in Section 11.2 require
/// each datatype in an OWL 2 DL ontology to be *rdfs:Literal*, one of the datatypes from Section 4, or
/// a datatype defined by means of a datatype definition (see Section 9.4).
///
///
/// ## Specification (Section §5.2 -- Datatypes)
///
/// ```owl
/// Datatype := IRI
/// ```
///
/// ## Examples
///
/// ```rust
/// use athene_owlapi::{
///     entities::{Datatype, EntityTrait, IriAsEntity},
///     things::rdf,
/// };
/// use rdftk_iri::Iri;
/// use std::str::FromStr;
///
/// let iri = Iri::from_str("https://example.com/ns/Foo").unwrap();
///
/// // Using EntityTrait::new
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     Datatype::new(iri.clone()).to_string()
/// );
///
/// // Using From<Iri>
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     Datatype::from(iri.clone()).to_string()
/// );
///
/// // Using IriAsEntity::as_datatype
/// assert_eq!(
///     "<https://example.com/ns/Foo>".to_string(),
///     iri.as_datatype().to_string()
/// );
///
/// // Using a predefined IRI
/// assert_eq!(
///     "rdf:PlainLiteral".to_string(),
///     rdf::plain_literal().as_datatype().to_string()
/// );
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Datatype {
    entity_iri: Iri,
    arity: UnlimitedNatural,
}

///
/// *Object properties* connect pairs of individuals.
///
/// The object properties with the IRIs *owl:topObjectProperty* and *owl:bottomObjectProperty* are
/// available in OWL 2 as built-in object properties with a predefined semantics:
///
/// * The object property with IRI *owl:topObjectProperty* connects all possible pairs of individuals.
/// * The object property with IRI *owl:bottomObjectProperty* does not connect any pair of individuals.
///
/// IRIs from the reserved vocabulary other than *owl:topObjectProperty* and *owl:bottomObjectProperty*
/// must not be used to identify object properties in an OWL 2 DL ontology.
///
/// ## Specification (Section §5.3 -- Object Properties)
///
/// ```owl
/// ObjectProperty := IRI
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    entity_iri: Iri,
}

///
/// *Data properties* connect individuals with literals. In some knowledge representation systems,
/// functional data properties are called attributes.
///
/// The data properties with the IRIs *owl:topDataProperty* and *owl:bottomDataProperty* are available
/// in OWL 2 as built-in data properties with a predefined semantics:
///
/// * The data property with IRI *owl:topDataProperty* connects all possible individuals with all literals.
/// * The data property with IRI *owl:bottomDataProperty* does not connect any individual with a literal.
///
/// IRIs from the reserved vocabulary other than owl:topDataProperty and owl:bottomDataProperty must not
/// be used to identify data properties in an OWL 2 DL ontology.
///
/// ## Specification (Section §5.4 -- Data Properties)
///
/// ```owl
/// DataProperty := IRI
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct DataProperty {
    entity_iri: Iri,
}

///
/// *Annotation properties* can be used to provide an annotation for an ontology, axiom, or an IRI. The
/// structure of annotations is further described in Section 10.
///
/// The annotation properties with the IRIs listed below are available in OWL 2 as built-in annotation
/// properties with a predefined semantics:
///
/// * The *rdfs:label* annotation property can be used to provide an IRI with a human-readable label.
/// * The *rdfs:comment* annotation property can be used to provide an IRI with a human-readable comment.
/// * The *rdfs:seeAlso* annotation property can be used to provide an IRI with another IRI such that the
///   latter provides additional information about the former.
/// * The *rdfs:isDefinedBy* annotation property can be used to provide an IRI with another IRI such that
///   the latter provides information about the definition of the former; the way in which this information
///   is provided is not described by this specification.
/// * An annotation with the *owl:deprecated* annotation property and the value equal to `"true"^^xsd:boolean`
///   can be used to specify that an IRI is deprecated.
/// * The *owl:versionInfo* annotation property can be used to provide an IRI with a string that describes
///   the IRI's version.
/// * The *owl:priorVersion* annotation property is described in more detail in Section 3.5.
/// * The *owl:backwardCompatibleWith* annotation property is described in more detail in Section 3.5.
/// * The *owl:incompatibleWith* annotation property is described in more detail in Section 3.5.
/// * IRIs from the reserved vocabulary other than the ones listed above must not be used to identify
///   annotation properties in an OWL 2 DL ontology.
///
/// ## Specification (Section §5.5 -- Annotation Properties)
///
/// ```owl
/// AnnotationProperty := IRI
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationProperty {
    entity_iri: Iri,
}

///
/// *Individuals* in the OWL 2 syntax represent actual objects from the domain.
///
/// There are two types of individuals in the syntax of OWL 2. *Named individuals* are given an explicit
/// name that can be used in any ontology to refer to the same object. *Anonymous individuals* do not
/// have a global name and are thus local to the ontology they are contained in.
///
/// ## Specification (Section §5.6 -- Individuals)
///
/// ```owl
/// Individual := NamedIndividual | AnonymousIndividual
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Individual {
    NamedIndividual(NamedIndividual),
    AnonymousIndividual(AnonymousIndividual),
}

///
/// *Named individuals* are identified using an IRI. Since they are given an IRI, named individuals
/// are entities.
///
/// IRIs from the reserved vocabulary *must not* be used to identify named individuals in an OWL 2
/// DL ontology.
///
/// ## Specification (Section §5.6.1 -- Named Individuals)
///
/// ```owl
/// NamedIndividual := IRI
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NamedIndividual {
    entity_iri: Iri,
}

///
/// If an individual is not expected to be used outside a particular ontology, one can use an
/// *anonymous individual*, which is identified by a local node ID rather than a global IRI.
///
/// Anonymous individuals are analogous to blank nodes in RDF.
///
/// ## Specification (Section §5.6.2 -- Anonymous Individuals)
///
/// ```owl
/// AnonymousIndividual := nodeID
///
/// nodeID := SPARQL::BLANK_NODE_LABEL
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousIndividual {
    node_id: Name,
}

///
/// This trait is used to capture the common feature of all entities, that they have an
/// `entitiy_iri` field.
///
pub trait EntityTrait: DisplayPretty + From<Iri> + Into<Iri> {
    fn new(entity_iri: Iri) -> Self;
    fn entity_iri(&self) -> &Iri;
    fn as_declaration(&self) -> Declaration;
}

pub trait IriAsEntity {
    fn as_annotation_property(&self) -> AnnotationProperty;
    fn as_class(&self) -> Class;
    fn as_data_property(&self) -> DataProperty;
    fn as_datatype(&self) -> Datatype;
    fn as_object_property(&self) -> ObjectProperty;
    fn as_named_individual(&self) -> NamedIndividual;
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
                Self::new(entity_iri)
            }
        }

        impl From<&Iri> for $type_name {
            fn from(entity_iri: &Iri) -> Self {
                Self::new(entity_iri.clone())
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

            fn as_declaration(&self) -> Declaration {
                Declaration::new(self.clone())
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
                Self::AnnotationProperty(_) =>
                    format!("{FN_ANNOTATION_PROPERTY}{DELIM_ARGS_GROUP_START}"),
                Self::Class(_) => format!("{FN_CLASS}{DELIM_ARGS_GROUP_START}"),
                Self::DataProperty(_) => format!("{FN_DATA_PROPERTY}{DELIM_ARGS_GROUP_START}"),
                Self::Datatype(_) => format!("{FN_DATATYPE}{DELIM_ARGS_GROUP_START}"),
                Self::ObjectProperty(_) => format!("{FN_OBJECT_PROPERTY}{DELIM_ARGS_GROUP_START}"),
                Self::NamedIndividual(_) =>
                    format!("{FN_NAMED_INDIVIDUAL}{DELIM_ARGS_GROUP_START}"),
            }
        )?;
        if f.alternate() {
            indenter.indent();
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
            indenter.outdent();
        }
        write!(
            f,
            "{}{DELIM_FN_ARGS_END}",
            indenter.separator_string(f.alternate())
        )
    }
}

impl_from_for_variant!(Entity, Class);
impl_from_for_variant!(Entity, AnnotationProperty);
impl_from_for_variant!(Entity, DataProperty);
impl_from_for_variant!(Entity, Datatype);
impl_from_for_variant!(Entity, ObjectProperty);
impl_from_for_variant!(Entity, NamedIndividual);

impl Entity {
    pub fn entity_iri(&self) -> &Iri {
        match self {
            Entity::AnnotationProperty(annotation_property) => annotation_property.entity_iri(),
            Entity::Class(class) => class.entity_iri(),
            Entity::DataProperty(data_property) => data_property.entity_iri(),
            Entity::Datatype(datatype) => datatype.entity_iri(),
            Entity::ObjectProperty(object_property) => object_property.entity_iri(),
            Entity::NamedIndividual(named_individual) => named_individual.entity_iri(),
        }
    }

    pub fn as_declaration(&self) -> Declaration {
        Declaration::new(self.clone())
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ AnnotationProperty
// ------------------------------------------------------------------------------------------------

impl_entity!(AnnotationProperty);

impl AnnotationProperty {
    #[inline(always)]
    pub fn rdfs_label() -> Self {
        rdfs::label().into()
    }

    #[inline(always)]
    pub fn rdfs_comment() -> Self {
        rdfs::comment().into()
    }

    #[inline(always)]
    pub fn rdfs_see_also() -> Self {
        rdfs::see_also().into()
    }

    #[inline(always)]
    pub fn rdfs_is_defined_by() -> Self {
        rdfs::is_defined_by().into()
    }

    #[inline(always)]
    pub fn owl_deprecated() -> Self {
        owl::deprecated().into()
    }

    #[inline(always)]
    pub fn owl_backward_compatible_with() -> Self {
        owl::backward_compatible_with().into()
    }

    #[inline(always)]
    pub fn owl_incompatible_with() -> Self {
        owl::incompatible_with().into()
    }

    #[inline(always)]
    pub fn owl_prior_version() -> Self {
        owl::prior_version().into()
    }

    #[inline(always)]
    pub fn owl_version_info() -> Self {
        owl::version_info().into()
    }

    #[inline(always)]
    pub fn skos_alt_label() -> Self {
        skos::alt_label().into()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class
// ------------------------------------------------------------------------------------------------

impl_entity!(Class);

impl Class {
    #[inline(always)]
    pub fn rdfs_resource() -> Self {
        rdfs::resource().into()
    }

    #[inline(always)]
    pub fn rdfs_literal() -> Self {
        rdfs::literal().into()
    }

    #[inline(always)]
    pub fn rdfs_datatype() -> Self {
        rdfs::datatype().into()
    }

    #[inline(always)]
    pub fn rdfs_class() -> Self {
        rdfs::class().into()
    }

    #[inline(always)]
    pub fn rdf_property() -> Self {
        rdf::property().into()
    }

    #[inline(always)]
    pub fn rdf_list() -> Self {
        rdf::list().into()
    }

    #[inline(always)]
    pub fn rdf_statement() -> Self {
        rdf::statement().into()
    }

    #[inline(always)]
    pub fn owl_thing() -> Self {
        owl::thing().into()
    }

    #[inline(always)]
    pub fn owl_nothing() -> Self {
        owl::nothing().into()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DataProperty
// ------------------------------------------------------------------------------------------------

impl_entity!(DataProperty);

// ------------------------------------------------------------------------------------------------
// Implementation ❯ Datatype
// ------------------------------------------------------------------------------------------------

impl Display for Datatype {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.fmt_pretty(f, &Indenter::default(), &IriPrefixMap::default())
    }
}
impl DisplayPretty for Datatype {
    fn fmt_pretty(
        &self,
        f: &mut Formatter<'_>,
        indenter: &Indenter,
        prefix_map: &IriPrefixMap,
    ) -> FmtResult {
        self.entity_iri.fmt_pretty(f, indenter, prefix_map)
    }
}

impl HasArity for Datatype {
    fn arity(&self) -> UnlimitedNatural {
        self.arity
    }
}

impl From<Iri> for Datatype {
    fn from(entity_iri: Iri) -> Datatype {
        Self::new(entity_iri)
    }
}

impl From<&Iri> for Datatype {
    fn from(entity_iri: &Iri) -> Datatype {
        Self::new(entity_iri.clone())
    }
}

impl From<Datatype> for Iri {
    fn from(value: Datatype) -> Self {
        value.entity_iri
    }
}

impl Datatype {
    pub fn new(entity_iri: Iri) -> Self {
        Self {
            entity_iri,
            arity: UnlimitedNatural::Limited(1), // See note on Figure 2, Section 5.0
        }
    }

    #[inline(always)]
    pub fn owl_real() -> Self {
        owl::real().into()
    }

    #[inline(always)]
    pub fn owl_rational() -> Self {
        owl::rational().into()
    }

    #[inline(always)]
    pub fn rdf_plain_literal() -> Self {
        rdf::plain_literal().into()
    }

    #[inline(always)]
    pub fn rdf_lang_string() -> Self {
        rdf::lang_string().into()
    }

    #[inline(always)]
    pub fn entity_iri(&self) -> &Iri {
        &self.entity_iri
    }

    #[inline(always)]
    pub fn as_declaration(&self) -> Declaration {
        Declaration::new(self)
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
        write!(
            f,
            "{ANONYMOUS_NAMESPACE}{NAMESPACE_NAME_SEPARATOR}{}",
            self.node_id
        )
    }
}

impl FromStr for AnonymousIndividual {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            node_id: Name::from_str(s).map_err(|e| {
                ApiError::ValueParser(FN_ANONYMOUS_INDIVIDUAL, e.to_string(), s.to_string())
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

// ------------------------------------------------------------------------------------------------
// Implementations ❯ IriAsEntity
// ------------------------------------------------------------------------------------------------

impl IriAsEntity for Iri {
    fn as_annotation_property(&self) -> AnnotationProperty {
        AnnotationProperty::new(self.clone())
    }

    fn as_class(&self) -> Class {
        Class::new(self.clone())
    }

    fn as_data_property(&self) -> DataProperty {
        DataProperty::new(self.clone())
    }

    fn as_datatype(&self) -> Datatype {
        Datatype::new(self.clone())
    }

    fn as_object_property(&self) -> ObjectProperty {
        ObjectProperty::new(self.clone())
    }

    fn as_named_individual(&self) -> NamedIndividual {
        NamedIndividual::new(self.clone())
    }
}
