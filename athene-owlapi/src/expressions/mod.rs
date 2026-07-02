//!
//! This module provides the types to model OWL 2 expressions; object property, data property,
//! and class expressions.
//!

use crate::{
    entities::{Class, DataProperty, Individual, ObjectProperty},
    fmt::DisplayPretty,
    literals::Literal,
    ranges::DataRange,
    values::UnlimitedNatural,
};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, vec::Vec};

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Property Expressions
// ------------------------------------------------------------------------------------------------

///
/// Object properties can by used in OWL 2 to form object property expressions, which represent
/// relationships between pairs of individuals. They are represented in the structural
/// specification of OWL 2 by **[ObjectPropertyExpression]**, and their structure is shown in
/// Figure 4.
///
/// ![Figure 4. Object Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_objectproperty.gif)
///
/// As one can see from the figure, OWL 2 supports only two kinds of object property
/// expressions. Object properties are the simplest form of object property expressions, and
/// inverse object properties allow for bidirectional navigation in class expressions and axioms.
///
/// ## Specification (Section §6.1 -- Object Property Expressions)
///
/// ```bnf
/// ObjectPropertyExpression :=
///     ObjectProperty | InverseObjectProperty
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum ObjectPropertyExpression {
    ObjectProperty(ObjectProperty),
    InverseObjectProperty(InverseObjectProperty),
}

///
/// An inverse object property expression $ObjectInverseOf( P )$ connects an individual $I_1$ with
/// $I_2$ if and only if the object property $P$ connects $I_2$ with $I_1$.
///
/// ## Specification (Section §6.1.1 -- Inverse Object Properties)
///
/// ```bnf
/// InverseObjectProperty :=
///     'ObjectInverseOf' '(' ObjectProperty ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct InverseObjectProperty {
    object_property: ObjectProperty,
}

///
/// For symmetry with object property expressions, the structural specification of OWL 2 also
/// introduces data property expressions, which represent relationships between an individual
/// and a literal. The structure of data property expressions is shown in Figure 5. The only
/// allowed data property expression is a data property; thus, **[DataPropertyExpression]** in
/// the structural specification of OWL 2 can be seen as a place-holder for possible future
/// extensions.
///
/// ![Figure 5. Data Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_dataproperty.gif)
///
/// ## Specification (Section §6.2 -- Data Property Expressions)
///
/// ```bnf
/// DataPropertyExpression := DataProperty
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum DataPropertyExpression {
    DataProperty(DataProperty),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions
// ------------------------------------------------------------------------------------------------

///
/// In OWL 2, classes and property expressions are used to construct class expressions, sometimes
/// also called descriptions, and, in the description logic literature, complex concepts. Class
/// expressions represent sets of individuals by formally specifying conditions on the
/// individuals' properties; individuals satisfying these conditions are said to be instances of
/// the respective class expressions. In the structural specification of OWL 2, class expressions
/// are represented by **[ClassExpression]**.
///
/// OWL 2 provides a rich set of primitives that can be used to construct class expressions. In
/// particular, it provides the well known Boolean connectives *and*, *or*, and *not*; a
/// restricted form of universal and existential quantification; number restrictions; enumeration
/// of individuals; and a special *self*-restriction.
///
/// As shown in Figure 2, classes are the simplest form of class expressions. The other, complex,
/// class expressions, are described in the following sections.
///
/// ## 8.1 Propositional Connectives and Enumeration of Individuals
///
/// OWL 2 provides for enumeration of individuals and all standard Boolean connectives, as shown
/// in Figure 7. The **[ObjectIntersectionOf]**, **[ObjectUnionOf]**, and **[ObjectComplementOf]**
/// class expressions provide for the standard set-theoretic operations on class expressions; in
/// logical languages these are usually called conjunction, disjunction, and negation, respectively.
/// The **ObjectOneOf** class expression contains exactly the specified individuals.
///
/// ![Figure 7. Propositional Connectives and Enumeration of Individuals in OWL 2](https://www.w3.org/TR/owl2-syntax/C_propositional.gif)
///
/// ## Object Property Restrictions
///
/// Class expressions in OWL 2 can be formed by placing restrictions on object property expressions,
/// as shown in Figure 8. The **[ObjectSomeValuesFrom]** class expression allows for existential
/// quantification over an object property expression, and it contains those individuals that are
/// connected through an object property expression to at least one instance of a given class
/// expression. The **[ObjectAllValuesFrom]** class expression allows for universal quantification
/// over an object property expression, and it contains those individuals that are connected
/// through an object property expression only to instances of a given class expression. The
/// **[ObjectHasValue]** class expression contains those individuals that are connected by an object
/// property expression to a particular individual. Finally, the **[ObjectHasSelf]** class expression
/// contains those individuals that are connected by an object property expression to themselves.
///
/// ![Figure 8. Restricting Object Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_objectmodal.gif)
///
/// ## Object Property Cardinality Restrictions
///
/// Class expressions in OWL 2 can be formed by placing restrictions on the cardinality of object
/// property expressions, as shown in Figure 9. All cardinality restrictions can be qualified or
/// unqualified: in the former case, the cardinality restriction only applies to individuals that
/// are connected by the object property expression and are instances of the qualifying class
/// expression; in the latter case the restriction applies to all individuals that are connected by
/// the object property expression (this is equivalent to the qualified case with the qualifying
/// class expression equal to *owl:Thing*). The class expressions **[ObjectMinCardinality]**,
/// **[ObjectMaxCardinality]**, and **[ObjectExactCardinality]** contain those individuals that are
/// connected by an object property expression to at least, at most, and exactly a given number of
/// instances of a specified class expression, respectively.
///
/// ![Figure 9. Restricting the Cardinality of Object Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_objectcardinality.gif)
///
/// ## Data Property Restrictions
///
/// Class expressions in OWL 2 can be formed by placing restrictions on data property expressions, as
/// shown in Figure 10. These are similar to the restrictions on object property expressions, the main
/// difference being that the expressions for existential and universal quantification allow for n-ary
/// data ranges. All data ranges explicitly supported by this specification are unary; however, the
/// provision of n-ary data ranges in existential and universal quantification allows OWL 2 tools to
/// support extensions such as value comparisons and, consequently, class expressions such as
/// "individuals whose width is greater than their height". Thus, the **[DataSomeValuesFrom]** class
/// expression allows for a restricted existential quantification over a list of data property
/// expressions, and it contains those individuals that are connected through the data property
/// expressions to at least one literal in the given data range. The **[DataAllValuesFrom]** class
/// expression allows for a restricted universal quantification over a list of data property
/// expressions, and it contains those individuals that are connected through the data property
/// expressions only to literals in the given data range. Finally, the DataHasValue class expression
/// contains those individuals that are connected by a data property expression to a particular literal.
///
/// ![Figure 10. Restricting Data Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_datamodal.gif)
///
/// ## Data Property Cardinality Restrictions
///
/// Class expressions in OWL 2 can be formed by placing restrictions on the cardinality of data property
/// expressions, as shown in Figure 11. These are similar to the restrictions on the cardinality of object
/// property expressions. All cardinality restrictions can be qualified or unqualified: in the former case,
/// the cardinality restriction only applies to literals that are connected by the data property expression
/// and are in the qualifying data range; in the latter case it applies to all literals that are connected
/// by the data property expression (this is equivalent to the qualified case with the qualifying data
/// range equal to *rdfs:Literal*). The class expressions **[DataMinCardinality]**,
/// **[DataMaxCardinality]**, and **[DataExactCardinality]** contain those individuals that are connected
/// by a data property expression to at least, at most, and exactly a given number of literals in the
/// specified data range, respectively.
///
/// ![Figure 11. Restricting the Cardinality of Data Property Expressions in OWL 2](https://www.w3.org/TR/owl2-syntax/C_datacardinality.gif)
///
/// ## Specification (Section §8 -- Class Expressions)
///
/// ```bnf
/// ClassExpression :=
///     Class | ObjectIntersectionOf | ObjectUnionOf |
///     ObjectComplementOf | ObjectOneOf | ObjectSomeValuesFrom |
///     ObjectAllValuesFrom | ObjectHasValue | ObjectHasSelf |
///     ObjectMinCardinality | ObjectMaxCardinality | ObjectExactCardinality |
///     DataSomeValuesFrom | DataAllValuesFrom | DataHasValue |
///     DataMinCardinality | DataMaxCardinality | DataExactCardinality
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum ClassExpression {
    ObjectIntersectionOf(ObjectIntersectionOf),
    ObjectUnionOf(ObjectUnionOf),
    ObjectComplementOf(ObjectComplementOf),
    ObjectOneOf(ObjectOneOf),
    Class(Class),
    ObjectSomeValuesFrom(ObjectSomeValuesFrom),
    ObjectAllValuesFrom(ObjectAllValuesFrom),
    ObjectHasValue(ObjectHasValue),
    ObjectHasSelf(ObjectHasSelf),
    ObjectMinCardinality(ObjectMinCardinality),
    ObjectMaxCardinality(ObjectMaxCardinality),
    ObjectExactCardinality(ObjectExactCardinality),
    DataSomeValuesFrom(DataSomeValuesFrom),
    DataAllValuesFrom(DataAllValuesFrom),
    DataHasValue(DataHasValue),
    DataMinCardinality(DataMinCardinality),
    DataMaxCardinality(DataMaxCardinality),
    DataExactCardinality(DataExactCardinality),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Propositional Connectives and Enumeration of Individuals
// ------------------------------------------------------------------------------------------------

///
/// An intersection class expression $ObjectIntersectionOf( CE_1 \cdots CE_n )$ contains all
/// individuals that are instances of all class expressions $CE_i$ for $1 \leq i \leq n$.
///
/// ## Specification (Section §8.1.1 -- Intersection of Class Expressions)
///
/// ```bnf
/// ObjectIntersectionOf :=
///     'ObjectIntersectionOf' '('
///         ClassExpression ClassExpression { ClassExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectIntersectionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// A union class expression $ObjectUnionOf( CE_1 \cdots CE_n )$ contains all individuals that
/// are instances of at least one class expression $CE_i$ for $1 \leq i \leq n$.
///
/// ## Specification (Section §8.1.2 -- Union of Class Expressions)
///
/// ```bnf
/// ObjectUnionOf :=
///     'ObjectUnionOf' '('
///         ClassExpression ClassExpression { ClassExpression }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectUnionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// A complement class expression $ObjectComplementOf( CE )$ contains all individuals that are
/// not instances of the class expression $CE$.
///
/// ## Specification (Section §8.1.3 -- Complement of Class Expressions)
///
/// ```bnf
/// ObjectComplementOf :=
///     'ObjectComplementOf' '(' ClassExpression ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectComplementOf {
    class_expression: Box<ClassExpression>,
}

///
/// An enumeration of individuals $ObjectOneOf( a_1 \cdots a_n )$ contains exactly the
/// individuals $a_i$ with $1 \leq i \leq n$.
///
/// ## Specification (Section §8.1.4 -- Enumeration of Individuals)
///
/// ```bnf
/// ObjectOneOf :=
///     'ObjectOneOf' '('
///         Individual { Individual }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectOneOf {
    individuals: Vec<Individual>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Object Property Restrictions
// ------------------------------------------------------------------------------------------------

///
/// An existential class expression $ObjectSomeValuesFrom( OPE \ CE )$ consists of an object
/// property expression $OPE$ and a class expression $CE$, and it contains all those
/// individuals that are connected by $OPE$ to an individual that is an instance of $CE$.
///
/// Provided that $OPE$ is simple according to the definition in Section 11, such a class
/// expression can be seen as a syntactic shortcut for the class expression
/// $ObjectMinCardinality( 1 \ OPE \ CE )$.
///
/// ## Specification (Section §8.2.1 -- Existential Quantification)
///
/// ```bnf
/// ObjectSomeValuesFrom :=
///     'ObjectSomeValuesFrom' '('
///         ObjectPropertyExpression ClassExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSomeValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

///
/// A universal class expression $ObjectAllValuesFrom( OPE \ CE )$ consists of an object
/// property expression $OPE$ and a class expression $CE$, and it contains all those
/// individuals that are connected by $OPE$ only to individuals that are instances of $CE$.
///
/// Provided that $OPE$ is simple according to the definition in Section 11, such a class
/// expression can be seen as a syntactic shortcut for the class expression
/// $ObjectMaxCardinality( 0 \ OPE \ ObjectComplementOf( CE ) )$.
///
/// ## Specification (Section §8.2.2 -- Universal Quantification)
///
/// ```bnf
/// ObjectAllValuesFrom :=
///     'ObjectAllValuesFrom' '('
///         ObjectPropertyExpression ClassExpression
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAllValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

///
/// A has-value class expression $ObjectHasValue( OPE \ a )$ consists of an object property
/// expression $OPE$ and an individual $a$, and it contains all those individuals that are
/// connected by $OPE$ to $a$.
///
/// Each such class expression can be seen as a syntactic shortcut for the class expression
/// $ObjectSomeValuesFrom( OPE \ ObjectOneOf( a ) )$.
///
/// ## Specification (Section §8.2.3 -- Individual Value Restriction)
///
/// ```bnf
/// ObjectHasValue :=
///     'ObjectHasValue' '('
///         ObjectPropertyExpression Individual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasValue {
    object_property_expression: ObjectPropertyExpression,
    individual: Individual,
}

///
/// A self-restriction $ObjectHasSelf( OPE )$ consists of an object property expression
/// $OPE$, and it contains all those individuals that are connected by $OPE$ to themselves.
///
/// ## Specification (Section §8.2.4 -- Self-Restriction)
///
/// ```bnf
/// ObjectHasSelf :=
///     'ObjectHasSelf' '(' ObjectPropertyExpression ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasSelf {
    object_property_expression: ObjectPropertyExpression,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Object Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

///
/// Implemented by all expressions in Section §8.3
///
pub trait ObjectPropertyCardinalityRestriction {
    fn cardinality(&self) -> UnlimitedNatural;
    fn object_property_expression(&self) -> &ObjectPropertyExpression;
    fn class_expression(&self) -> Option<&Box<ClassExpression>>;
}

///
/// A minimum cardinality expression $ObjectMinCardinality( n \ OPE \ CE )$ consists of a
/// nonnegative integer $n$, an object property expression $OPE$, and a class expression
/// $CE$, and it contains all those individuals that are connected by $OPE$ to at least $n$
/// different individuals that are instances of $CE$.
///
/// If $CE$ is missing, it is taken to be *owl:Thing*.
///
/// ## Specification (Section §8.3.1 -- Minimum Cardinality)
///
/// ```bnf
/// ObjectMinCardinality :=
///     'ObjectMinCardinality' '('
///         nonNegativeInteger ObjectPropertyExpression
///         [ ClassExpression ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMinCardinality {
    cardinality: UnlimitedNatural,
    object_property_expression: ObjectPropertyExpression,
    class_expression: Option<Box<ClassExpression>>,
}

///
/// A maximum cardinality expression $ObjectMaxCardinality( n \ OPE \ CE )$ consists of a
/// nonnegative integer $n$, an object property expression $OPE$, and a class expression
/// $CE$, and it contains all those individuals that are connected by $OPE$ to at most $n$
/// different individuals that are instances of $CE$.
///
/// If $CE$ is missing, it is taken to be *owl:Thing*.
///
/// ## Specification (Section §8.3.2 -- Maximum Cardinality)
///
/// ```bnf
/// ObjectMaxCardinality :=
///     'ObjectMaxCardinality' '('
///         nonNegativeInteger ObjectPropertyExpression
///         [ ClassExpression ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMaxCardinality {
    cardinality: UnlimitedNatural,
    object_property_expression: ObjectPropertyExpression,
    class_expression: Option<Box<ClassExpression>>,
}

///
/// An exact cardinality expression $ObjectExactCardinality( n \ OPE \ CE )$ consists of a
/// nonnegative integer $n$, an object property expression $OPE$, and a class expression $CE$,
/// and it contains all those individuals that are connected by $OPE$ to exactly $n$ different
/// individuals that are instances of $CE$.
///
/// If $CE$ is missing, it is taken to be *owl:Thing*.
///
/// Such an expression is actually equivalent to the expression
///
/// ```owl
/// ObjectIntersectionOf(
///     ObjectMinCardinality( n OPE CE )
///     ObjectMaxCardinality( n OPE CE )
/// )
/// ```
///
/// ## Specification (Section §8.3.3 -- Exact Cardinality)
///
/// ```bnf
/// ObjectExactCardinality :=
///     'ObjectExactCardinality' '('
///         nonNegativeInteger ObjectPropertyExpression
///         [ ClassExpression ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExactCardinality {
    cardinality: UnlimitedNatural,
    object_property_expression: ObjectPropertyExpression,
    class_expression: Option<Box<ClassExpression>>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Data Property Restrictions
// ------------------------------------------------------------------------------------------------

///
/// An existential class expression $DataSomeValuesFrom( DPE_1 \cdots DPE_n \ DR )$ consists of $n$
/// data property expressions $DPE_i, 1 \leq i \leq n$, and a data range $DR$ whose arity must be
/// $n$.
///
/// Such a class expression contains all those individuals that are connected by $DPE_i$ to literals
/// $lt_i, 1 \leq i \leq n$, such that the tuple $( lt_1 , \cdots, lt_n )$ is in $DR$. A class
/// expression of the form $DataSomeValuesFrom( DPE \ DR )$ can be seen as a syntactic shortcut
/// for the class expression $DataMinCardinality( 1 \ DPE \ DR )$.
///
/// ## Specification (Section §8.4.1 -- Existential Quantification)
///
/// ```bnf
/// DataSomeValuesFrom :=
///     'DataSomeValuesFrom' '('
///         DataPropertyExpression { DataPropertyExpression }
///         DataRange
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataSomeValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

///
/// A universal class expression $DataAllValuesFrom( DPE_1 \cdots DPE_n \ DR )$ consists of $n$
/// data property expressions $DPE_i, 1 \leq i \leq n$, and a data range $DR$ whose arity must be
/// $n$.
///
/// Such a class expression contains all those individuals that are connected by $DPE_i$ only
/// to literals $lt_i, 1 \leq i \leq n$, such that each tuple $( lt_1 , \cdots, lt_n )$ is in $DR$.
/// A class expression of the form $DataAllValuesFrom( DPE \ DR )$ can be seen as a syntactic
/// shortcut for the class expression $DataMaxCardinality( 0 \ DPE \ DataComplementOf( DR ) )$.
///
/// ## Specification (Section §8.4.2 -- Universal Quantification)
///
/// ```bnf
/// DataAllValuesFrom :=
///     'DataAllValuesFrom' '('
///         DataPropertyExpression { DataPropertyExpression }
///         DataRange
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataAllValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

///
/// A has-value class expression $DataHasValue( DPE \ lt )$ consists of a data property expression
/// $DPE$ and a literal $lt$, and it contains all those individuals that are connected by $DPE$
/// to $lt$.
///
/// Each such class expression can be seen as a syntactic shortcut for the class expression
/// $DataSomeValuesFrom( DPE \ DataOneOf( lt ) )$.
///
/// ## Specification (Section §8.4.3 -- Literal Value Restriction)
///
/// ```bnf
/// DataHasValue := '
///     DataHasValue' '('
///         DataPropertyExpression Literal
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataHasValue {
    data_property_expression: DataPropertyExpression,
    literal: Literal,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Data Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

///
/// Implemented by all expressions in Section §8.5
///
pub trait DataPropertyCardinalityRestriction {
    fn cardinality(&self) -> UnlimitedNatural;
    fn data_property_expression(&self) -> &DataPropertyExpression;
    fn data_range(&self) -> Option<&DataRange>;
}

///
/// A minimum cardinality expression $DataMinCardinality( n \ DPE \ DR )$ consists of a nonnegative
/// integer $n$, a data property expression $DPE$, and a unary data range $DR$, and it contains all
/// those individuals that are connected by $DPE$ to at least n different literals in $DR$.
///
/// If $DR$ is not present, it is taken to be *rdfs:Literal*.
///
/// Note that some datatypes from the OWL 2 datatype map distinguish between equal and identical
/// data values, and that the semantics of cardinality restrictions in OWL 2 is defined with respect
/// to the latter. For an example demonstrating the effects such such a definition, please refer to
/// Section 9.3.6.
///
/// ## Specification (Section §8.5.1 -- Minimum Cardinality)
///
/// ```bnf
/// DataMinCardinality :=
///     'DataMinCardinality' '('
///         nonNegativeInteger DataPropertyExpression
///         [ DataRange ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataMinCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

///
/// A maximum cardinality expression $DataMaxCardinality( n \ DPE \ DR )$ consists of a nonnegative
/// integer $n$, a data property expression $DPE$, and a unary data range $DR$, and it contains all
/// those individuals that are connected by $DPE$ to at most $n$ different literals in $DR$.
///
/// If $DR$ is not present, it is taken to be *rdfs:Literal*.
///
/// Note that some datatypes from the OWL 2 datatype map distinguish between equal and identical data
/// values, and that the semantics of cardinality restrictions in OWL 2 is defined with respect to the
/// latter. For an example demonstrating the effects such such a definition, please refer to
/// Section 9.3.6.
///
/// ## Specification (Section §8.5.2 -- Maximum Cardinality)
///
/// ```bnf
/// DataMaxCardinality :=
///     'DataMaxCardinality' '('
///         nonNegativeInteger DataPropertyExpression
///         [ DataRange ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataMaxCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

///
/// An exact cardinality expression $DataExactCardinality( n \ DPE \ DR )$ consists of a nonnegative
/// integer $n$, a data property expression $DPE$, and a unary data range $DR$, and it contains all
/// those individuals that are connected by $DPE$ to exactly $n$ different literals in $DR$.
///
/// If $DR$  is not present, it is taken to be *rdfs:Literal*.
///
/// Note that some datatypes from the OWL 2 datatype map distinguish between equal and identical data
/// values, and that the semantics of cardinality restrictions in OWL 2 is defined with respect to the
/// latter. For an example demonstrating the effects such such a definition, please refer to
/// Section 9.3.6.
///
/// ## Specification (Section §8.5.3 -- Exact Cardinality)
///
/// ```bnf
/// DataExactCardinality :=
///     'DataExactCardinality' '('
///         nonNegativeInteger DataPropertyExpression
///         [ DataRange ]
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataExactCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Property Expressions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectPropertyExpression enum ObjectProperty, InverseObjectProperty);

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(InverseObjectProperty(object_property));

impl InverseObjectProperty {
    pub fn new(object_property: ObjectProperty) -> Self {
        Self { object_property }
    }

    pub fn object_property(&self) -> &ObjectProperty {
        &self.object_property
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Expressions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataPropertyExpression enum DataProperty);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ClassExpression enum ObjectIntersectionOf,
    ObjectUnionOf,
    ObjectComplementOf,
    ObjectOneOf,
    Class,
    ObjectSomeValuesFrom,
    ObjectAllValuesFrom,
    ObjectHasValue,
    ObjectHasSelf,
    ObjectMinCardinality,
    ObjectMaxCardinality,
    ObjectExactCardinality,
    DataSomeValuesFrom,
    DataAllValuesFrom,
    DataHasValue,
    DataMinCardinality,
    DataMaxCardinality,
    DataExactCardinality
);

impl_from_for_variant!(ClassExpression, ObjectIntersectionOf);
impl_from_for_variant!(ClassExpression, ObjectUnionOf);
impl_from_for_variant!(ClassExpression, ObjectComplementOf);
impl_from_for_variant!(ClassExpression, ObjectOneOf);
impl_from_for_variant!(ClassExpression, Class);
impl_from_for_variant!(ClassExpression, ObjectSomeValuesFrom);
impl_from_for_variant!(ClassExpression, ObjectAllValuesFrom);
impl_from_for_variant!(ClassExpression, ObjectHasValue);
impl_from_for_variant!(ClassExpression, ObjectHasSelf);
impl_from_for_variant!(ClassExpression, ObjectMinCardinality);
impl_from_for_variant!(ClassExpression, ObjectMaxCardinality);
impl_from_for_variant!(ClassExpression, ObjectExactCardinality);
impl_from_for_variant!(ClassExpression, DataSomeValuesFrom);
impl_from_for_variant!(ClassExpression, DataAllValuesFrom);
impl_from_for_variant!(ClassExpression, DataHasValue);
impl_from_for_variant!(ClassExpression, DataMinCardinality);
impl_from_for_variant!(ClassExpression, DataMaxCardinality);
impl_from_for_variant!(ClassExpression, DataExactCardinality);

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Propositional Connectives and Enumeration of Individuals
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectIntersectionOf( @list class_expressions ));

impl ObjectIntersectionOf {
    pub fn new<I: IntoIterator<Item = ClassExpression>>(expressions: I) -> Self {
        Self {
            class_expressions: expressions.into_iter().collect(),
        }
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectUnionOf( @list class_expressions ));

impl ObjectUnionOf {
    pub fn new<I: IntoIterator<Item = ClassExpression>>(expressions: I) -> Self {
        Self {
            class_expressions: expressions.into_iter().collect(),
        }
    }

    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectComplementOf(class_expression));

impl ObjectComplementOf {
    pub fn new(class_expression: ClassExpression) -> Self {
        Self {
            class_expression: Box::new(class_expression),
        }
    }

    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectOneOf( @list individuals ));

impl ObjectOneOf {
    pub fn new<I: IntoIterator<Item = Individual>>(individuals: I) -> Self {
        Self {
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn individuals(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Object Property Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectSomeValuesFrom(
    object_property_expression,
    class_expression
));

impl ObjectSomeValuesFrom {
    pub fn new(ope: ObjectPropertyExpression, ce: ClassExpression) -> Self {
        Self {
            object_property_expression: ope,
            class_expression: Box::new(ce),
        }
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectAllValuesFrom(
    object_property_expression,
    class_expression
));

impl ObjectAllValuesFrom {
    pub fn new(ope: ObjectPropertyExpression, ce: ClassExpression) -> Self {
        Self {
            object_property_expression: ope,
            class_expression: Box::new(ce),
        }
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectHasValue(object_property_expression, individual));

impl ObjectHasValue {
    pub fn new(ope: ObjectPropertyExpression, individual: Individual) -> Self {
        Self {
            object_property_expression: ope,
            individual,
        }
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    pub fn individual(&self) -> &Individual {
        &self.individual
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectHasSelf(object_property_expression));

impl ObjectHasSelf {
    pub fn new(ope: ObjectPropertyExpression) -> Self {
        Self {
            object_property_expression: ope,
        }
    }

    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Object Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectMinCardinality( @display cardinality, object_property_expression, @optional class_expression ));

impl ObjectPropertyCardinalityRestriction for ObjectMinCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

impl ObjectMinCardinality {
    pub fn new(
        cardinality: u32,
        object_property_expression: ObjectPropertyExpression,
        class_expression: Option<ClassExpression>,
    ) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(cardinality as u128),
            object_property_expression,
            class_expression: class_expression.map(Box::new),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectMaxCardinality( @display cardinality, object_property_expression, @optional class_expression ));

impl ObjectPropertyCardinalityRestriction for ObjectMaxCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

impl ObjectMaxCardinality {
    pub fn new(
        cardinality: u32,
        object_property_expression: ObjectPropertyExpression,
        class_expression: Option<ClassExpression>,
    ) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(cardinality as u128),
            object_property_expression,
            class_expression: class_expression.map(Box::new),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectExactCardinality( @display cardinality, object_property_expression, @optional class_expression ));

impl ObjectPropertyCardinalityRestriction for ObjectExactCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }

    fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

impl ObjectExactCardinality {
    pub fn new(
        cardinality: u32,
        object_property_expression: ObjectPropertyExpression,
        class_expression: Option<ClassExpression>,
    ) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(cardinality as u128),
            object_property_expression,
            class_expression: class_expression.map(Box::new),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Data Property Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataSomeValuesFrom( @list data_property_expressions, data_range ));

impl DataSomeValuesFrom {
    pub fn new<I: IntoIterator<Item = DataPropertyExpression>>(dpes: I, dr: DataRange) -> Self {
        Self {
            data_property_expressions: dpes.into_iter().collect(),
            data_range: dr,
        }
    }

    pub fn data_range(&self) -> &DataRange {
        &self.data_range
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataAllValuesFrom( @list data_property_expressions, data_range ));

impl DataAllValuesFrom {
    pub fn new<I: IntoIterator<Item = DataPropertyExpression>>(dpes: I, dr: DataRange) -> Self {
        Self {
            data_property_expressions: dpes.into_iter().collect(),
            data_range: dr,
        }
    }

    pub fn data_range(&self) -> &DataRange {
        &self.data_range
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataHasValue(data_property_expression, literal));

impl DataHasValue {
    pub fn new(dpe: DataPropertyExpression, literal: Literal) -> Self {
        Self {
            data_property_expression: dpe,
            literal,
        }
    }

    pub fn literal(&self) -> &Literal {
        &self.literal
    }

    pub fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Data Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataMinCardinality( data_property_expression, @optional data_range, @display cardinality )
);

impl DataPropertyCardinalityRestriction for DataMinCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }

    fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }
}

impl DataMinCardinality {
    pub fn new(n: u32, dpe: DataPropertyExpression, dr: Option<DataRange>) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(n as u128),
            data_property_expression: dpe,
            data_range: dr,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataMaxCardinality( data_property_expression, @optional data_range, @display cardinality )
);

impl DataPropertyCardinalityRestriction for DataMaxCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }

    fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }
}

impl DataMaxCardinality {
    pub fn new(n: u32, dpe: DataPropertyExpression, dr: Option<DataRange>) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(n as u128),
            data_property_expression: dpe,
            data_range: dr,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataExactCardinality( data_property_expression, @optional data_range, @display cardinality )
);

impl DataPropertyCardinalityRestriction for DataExactCardinality {
    fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }

    fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }
}

impl DataExactCardinality {
    pub fn new(n: u32, dpe: DataPropertyExpression, dr: Option<DataRange>) -> Self {
        Self {
            cardinality: UnlimitedNatural::Limited(n as u128),
            data_property_expression: dpe,
            data_range: dr,
        }
    }
}
