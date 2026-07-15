//!
//! This module provides a set of constants for syntax elements.
//!

// ------------------------------------------------------------------------------------------------
// Reserved Delimited Characters (Section 2.2 -- BNF Notation)
// ------------------------------------------------------------------------------------------------

pub const DELIM_FN_ARGS_START: char = '(';
pub const DELIM_FN_ARGS_END: char = ')';

pub const DELIM_ARGS_GROUP_START: char = '(';
pub const DELIM_ARGS_GROUP_END: char = ')';

pub const DELIM_IRI_START: char = '<';
pub const DELIM_IRI_END: char = '>';

pub const DELIM_PREFIX_ASSIGN: char = '=';

pub const DELIM_LITERAL_LANGUAGE: char = '@';
pub const DELIM_LITERAL_DATATYPE: char = '^';

pub const DELIM_COMMENT_START: char = '#';

pub const DELIM_QUOTED_STRING: char = '"';

// ------------------------------------------------------------------------------------------------
// Other Reserved
// ------------------------------------------------------------------------------------------------

pub const NAMESPACE_NAME_SEPARATOR: char = ':';
pub const ANONYMOUS_NAMESPACE: char = '_';

// ------------------------------------------------------------------------------------------------
// Function Names
// ------------------------------------------------------------------------------------------------

pub const FN_ANNOTATION_ASSERTION: &str = "AnnotationAssertion";
pub const FN_ANNOTATION_PROPERTY_DOMAIN: &str = "AnnotationPropertyDomain";
pub const FN_ANNOTATION_PROPERTY_RANGE: &str = "AnnotationPropertyRange";
pub const FN_ANNOTATION_PROPERTY: &str = "AnnotationProperty";
pub const FN_ANNOTATION: &str = "Annotation";
pub const FN_ANONYMOUS_INDIVIDUAL: &str = "AnonymousIndividual";
pub const FN_ASYMMETRIC_OBJECT_PROPERTY: &str = "AsymmetricObjectProperty";
pub const FN_CLASS_ASSERTION: &str = "ClassAssertion";
pub const FN_CLASS: &str = "Class";
pub const FN_DATA_ALL_VALUES_FROM: &str = "DataAllValuesFrom";
pub const FN_DATA_EXACT_CARDINALITY: &str = "DataExactCardinality";
pub const FN_DATA_HAS_VALUE: &str = "DataHasValue";
pub const FN_DATA_MAX_CARDINALITY: &str = "DataMaxCardinality";
pub const FN_DATA_MIN_CARDINALITY: &str = "DataMinCardinality";
pub const FN_DATA_PROPERTY_ASSERTION: &str = "DataPropertyAssertion";
pub const FN_DATA_PROPERTY_DOMAIN: &str = "DataPropertyDomain";
pub const FN_DATA_PROPERTY_RANGE: &str = "DataPropertyRange";
pub const FN_DATA_PROPERTY: &str = "DataProperty";
pub const FN_DATA_SOME_VALUES_FROM: &str = "DataSomeValuesFrom";
pub const FN_DATATYPE: &str = "Datatype";
pub const FN_DATATYPE_DEFINITION: &str = "DatatypeDefinition";
pub const FN_DECLARATION: &str = "Declaration";
pub const FN_DIFFERENT_INDIVIDUALS: &str = "DifferentIndividuals";
pub const FN_DISJOINT_CLASSES: &str = "DisjointClasses";
pub const FN_DISJOINT_DATA_PROPERTIES: &str = "DisjointDataProperties";
pub const FN_DISJOINT_OBJECT_PROPERTIES: &str = "DisjointObjectProperties";
pub const FN_DISJOINT_UNION: &str = "DisjointUnion";
pub const FN_EQUIVALENT_CLASS: &str = "EquivalentClass";
pub const FN_EQUIVALENT_DATA_PROPERTIES: &str = "EquivalentDataProperties";
pub const FN_EQUIVALENT_OBJECT_PROPERTIES: &str = "EquivalentObjectProperties";
pub const FN_FUNCTIONAL_DATA_PROPERTY: &str = "FunctionalDataProperty";
pub const FN_FUNCTIONAL_OBJECT_PROPERTY: &str = "FunctionalObjectProperty";
pub const FN_HAS_KEY: &str = "HasKey";
pub const FN_IMPORT: &str = "Import";
pub const FN_INVERSE_FUNCTIONAL_OBJECT_PROPERTY: &str = "InverseFunctionalObjectProperty";
pub const FN_INVERSE_OBJECT_PROPERTIES: &str = "InverseObjectProperties";
pub const FN_INVERSE_OBJECT_PROPERTY: &str = "InverseObjectProperty";
pub const FN_IRREFLEXIVE_OBJECT_PROPERTY: &str = "IrreflexiveObjectProperty";
pub const FN_NAMED_INDIVIDUAL: &str = "NamedIndividual";
pub const FN_NEGATIVE_DATA_PROPERTY_ASSERTION: &str = "NegativeDataPropertyAssertion";
pub const FN_NEGATIVE_OBJECT_PROPERTY_ASSERTION: &str = "NegativeObjectPropertyAssertion";
pub const FN_OBJECT_ALL_VALUES_FROM: &str = "ObjectAllValuesFrom";
pub const FN_OBJECT_COMPLEMENT_OF: &str = "ObjectComplementOf";
pub const FN_OBJECT_EXACT_CARDINALITY: &str = "ObjectExactCardinality";
pub const FN_OBJECT_HAS_SELF: &str = "ObjectHasSelf";
pub const FN_OBJECT_HAS_VALUE: &str = "ObjectHasValue";
pub const FN_OBJECT_INTERSECTION_OF: &str = "ObjectIntersectionOf";
pub const FN_OBJECT_MAX_CARDINALITY: &str = "ObjectMaxCardinality";
pub const FN_OBJECT_MIN_CARDINALITY: &str = "ObjectMinCardinality";
pub const FN_OBJECT_ONE_OF: &str = "ObjectOneOf";
pub const FN_OBJECT_PROPERTY_ASSERTION: &str = "ObjectPropertyAssertion";
pub const FN_OBJECT_PROPERTY_DOMAIN: &str = "ObjectPropertyDomain";
pub const FN_OBJECT_PROPERTY_RANGE: &str = "ObjectPropertyRange";
pub const FN_OBJECT_PROPERTY: &str = "ObjectProperty";
pub const FN_OBJECT_SOME_VALUES_FROM: &str = "ObjectSomeValuesFrom";
pub const FN_OBJECT_UNION_OF: &str = "ObjectUnionOf";
pub const FN_ONTOLOGY: &str = "Ontology";
pub const FN_PREFIX: &str = "Prefix";
pub const FN_REFLEXIVE_OBJECT_PROPERTY: &str = "ReflexiveObjectProperty";
pub const FN_SAME_INDIVIDUAL: &str = "SameIndividual";
pub const FN_SUB_ANNOTATION_OF: &str = "SubAnnotationOf";
pub const FN_SUB_CLASS_OF: &str = "SubClassOf";
pub const FN_SUB_DATA_PROPERTY_OF: &str = "SubDataPropertyOf";
pub const FN_SUB_OBJECT_PROPERTY_OF: &str = "SubObjectPropertyOf";
pub const FN_SYMMETRIC_OBJECT_PROPERTY: &str = "SymmetricObjectProperty";
pub const FN_TRANSITIVE_OBJECT_PROPERTY: &str = "TransitiveObjectProperty";

pub const ALL_FUNCTION_NAMES: &[&str] = &[
    FN_ANNOTATION_ASSERTION,
    FN_ANNOTATION_PROPERTY_DOMAIN,
    FN_ANNOTATION_PROPERTY_RANGE,
    FN_ANNOTATION_PROPERTY,
    FN_ANNOTATION,
    FN_ANONYMOUS_INDIVIDUAL,
    FN_ASYMMETRIC_OBJECT_PROPERTY,
    FN_CLASS_ASSERTION,
    FN_CLASS,
    FN_DATA_ALL_VALUES_FROM,
    FN_DATA_EXACT_CARDINALITY,
    FN_DATA_HAS_VALUE,
    FN_DATA_MAX_CARDINALITY,
    FN_DATA_MIN_CARDINALITY,
    FN_DATA_PROPERTY_ASSERTION,
    FN_DATA_PROPERTY_DOMAIN,
    FN_DATA_PROPERTY_RANGE,
    FN_DATA_PROPERTY,
    FN_DATA_SOME_VALUES_FROM,
    FN_DATATYPE,
    FN_DATATYPE_DEFINITION,
    FN_DIFFERENT_INDIVIDUALS,
    FN_DISJOINT_CLASSES,
    FN_DISJOINT_DATA_PROPERTIES,
    FN_DISJOINT_OBJECT_PROPERTIES,
    FN_DISJOINT_UNION,
    FN_EQUIVALENT_CLASS,
    FN_EQUIVALENT_DATA_PROPERTIES,
    FN_EQUIVALENT_OBJECT_PROPERTIES,
    FN_FUNCTIONAL_DATA_PROPERTY,
    FN_FUNCTIONAL_OBJECT_PROPERTY,
    FN_HAS_KEY,
    FN_IMPORT,
    FN_INVERSE_FUNCTIONAL_OBJECT_PROPERTY,
    FN_INVERSE_OBJECT_PROPERTIES,
    FN_INVERSE_OBJECT_PROPERTY,
    FN_IRREFLEXIVE_OBJECT_PROPERTY,
    FN_NAMED_INDIVIDUAL,
    FN_NEGATIVE_DATA_PROPERTY_ASSERTION,
    FN_NEGATIVE_OBJECT_PROPERTY_ASSERTION,
    FN_OBJECT_ALL_VALUES_FROM,
    FN_OBJECT_COMPLEMENT_OF,
    FN_OBJECT_EXACT_CARDINALITY,
    FN_OBJECT_HAS_SELF,
    FN_OBJECT_HAS_VALUE,
    FN_OBJECT_INTERSECTION_OF,
    FN_OBJECT_MAX_CARDINALITY,
    FN_OBJECT_MIN_CARDINALITY,
    FN_OBJECT_ONE_OF,
    FN_OBJECT_PROPERTY_ASSERTION,
    FN_OBJECT_PROPERTY_DOMAIN,
    FN_OBJECT_PROPERTY_RANGE,
    FN_OBJECT_PROPERTY,
    FN_OBJECT_SOME_VALUES_FROM,
    FN_OBJECT_UNION_OF,
    FN_ONTOLOGY,
    FN_PREFIX,
    FN_REFLEXIVE_OBJECT_PROPERTY,
    FN_SAME_INDIVIDUAL,
    FN_SUB_ANNOTATION_OF,
    FN_SUB_CLASS_OF,
    FN_SUB_DATA_PROPERTY_OF,
    FN_SUB_OBJECT_PROPERTY_OF,
    FN_SYMMETRIC_OBJECT_PROPERTY,
    FN_TRANSITIVE_OBJECT_PROPERTY,
];
