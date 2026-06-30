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
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectComplementOf {
    class_expression: Box<ClassExpression>,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectIntersectionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectUnionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
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
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAllValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasSelf {
    object_property_expression: ObjectPropertyExpression,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasValue {
    object_property_expression: ObjectPropertyExpression,
    individual: Individual,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSomeValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Object Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMaxCardinality {
    cardinality: UnlimitedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMinCardinality {
    cardinality: UnlimitedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExactCardinality {
    cardinality: UnlimitedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
pub trait ObjectCardinalityConstraint {
    fn cardinality(&self) -> Option<usize>;
    fn class_expression(&self) -> Option<&ClassExpression>;
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions ❯ Data Property Restrictions
// ------------------------------------------------------------------------------------------------

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataSomeValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataAllValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
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
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataMaxCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataMinCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataExactCardinality {
    cardinality: UnlimitedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

///
/// TBD
///
/// ## Specification (Section § -- )
///
/// ```bnf
/// ```
///
pub trait DataCardinalityConstraint {
    fn cardinality(&self) -> UnlimitedNatural;
    fn data_range(&self) -> Option<&DataRange>;
    fn data_property_expression(&self) -> &DataPropertyExpression;
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Property Expressions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectPropertyExpression enum ObjectProperty, InverseObjectProperty);

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(InverseObjectProperty(object_property));

impl InverseObjectProperty {
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
    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectUnionOf( @list class_expressions ));

impl ObjectUnionOf {
    pub fn class_expressions(&self) -> impl Iterator<Item = &ClassExpression> {
        self.class_expressions.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectComplementOf(class_expression));

impl ObjectComplementOf {
    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectOneOf( @list individuals ));

impl ObjectOneOf {
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
    pub fn object_property_expression(&self) -> &ObjectPropertyExpression {
        &self.object_property_expression
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Object Property Cardinality Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectMinCardinality( @optional class_expression, @display cardinality ));

impl ObjectMinCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectMaxCardinality( @optional class_expression, @display cardinality ));

impl ObjectMaxCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectExactCardinality( @optional class_expression, @display cardinality ));

impl ObjectExactCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Class Expressions  ❯ Data Property Restrictions
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DataSomeValuesFrom( @list data_property_expressions, data_range ));

impl DataSomeValuesFrom {
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

impl DataMinCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }

    pub fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataMaxCardinality( data_property_expression, @optional data_range, @display cardinality )
);

impl DataMaxCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }

    pub fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataExactCardinality( data_property_expression, @optional data_range, @display cardinality )
);

impl DataExactCardinality {
    pub fn cardinality(&self) -> UnlimitedNatural {
        self.cardinality
    }

    pub fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }

    pub fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }
}
