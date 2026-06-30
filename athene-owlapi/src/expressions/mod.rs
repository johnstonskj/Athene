use crate::{
    entities::{Class, DataProperty, Individual, ObjectProperty},
    fmt::DisplayPretty,
    literals::Literal,
    ranges::DataRange,
    values::UnboundedNatural,
};
use strum::{EnumIs, EnumTryAs};

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Property Expressions
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum ObjectPropertyExpression {
    ///
    /// See specification §6.1 [*Object Property Expressions](https://www.w3.org/TR/owl2-syntax/#Object_Property_Expressions)
    ///
    ObjectProperty(ObjectProperty),
    ///
    /// See specification §6.1.1 [Inverse Object Properties](https://www.w3.org/TR/owl2-syntax/#Inverse_Object_Properties)
    ///
    InverseObjectProperty(InverseObjectProperty),
}

#[derive(Clone, Debug, PartialEq)]
pub struct InverseObjectProperty {
    object_property: ObjectProperty,
}

#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum DataPropertyExpression {
    ///
    /// See specification §6.2 [*Data Property Expressions*](https://www.w3.org/TR/owl2-syntax/#Data_Property_Expressions)
    DataProperty(DataProperty),
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Expressions
// ------------------------------------------------------------------------------------------------

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

// Propositional Connectives and Enumeration of Individuals

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectComplementOf {
    class_expression: Box<ClassExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectIntersectionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectUnionOf {
    class_expressions: Vec<ClassExpression>, // 2..*
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectOneOf {
    individuals: Vec<Individual>,
}

// Object Property Restrictions

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectAllValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasSelf {
    object_property_expression: ObjectPropertyExpression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectHasValue {
    object_property_expression: ObjectPropertyExpression,
    individual: Individual,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectSomeValuesFrom {
    object_property_expression: ObjectPropertyExpression,
    class_expression: Box<ClassExpression>,
}

// Object Property Cardinality Restrictions

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMaxCardinality {
    cardinality: UnboundedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectMinCardinality {
    cardinality: UnboundedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ObjectExactCardinality {
    cardinality: UnboundedNatural,
    class_expression: Option<Box<ClassExpression>>,
}

pub trait ObjectCardinalityConstraint {
    fn cardinality(&self) -> Option<usize>;
    fn class_expression(&self) -> Option<&ClassExpression>;
}

// Data Property Restrictions

#[derive(Clone, Debug, PartialEq)]
pub struct DataSomeValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataAllValuesFrom {
    data_range: DataRange,
    data_property_expressions: Vec<DataPropertyExpression>, // 1..* {ordered, nonunique}
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataHasValue {
    data_property_expression: DataPropertyExpression,
    literal: Literal,
}

// Data Property Cardinality Restrictions

#[derive(Clone, Debug, PartialEq)]
pub struct DataMaxCardinality {
    cardinality: UnboundedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataMinCardinality {
    cardinality: UnboundedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataExactCardinality {
    cardinality: UnboundedNatural,
    data_range: Option<DataRange>,
    data_property_expression: DataPropertyExpression,
}

pub trait DataCardinalityConstraint {
    fn cardinality(&self) -> UnboundedNatural;
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
    pub fn cardinality(&self) -> UnboundedNatural {
        self.cardinality
    }

    pub fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectMaxCardinality( @optional class_expression, @display cardinality ));

impl ObjectMaxCardinality {
    pub fn cardinality(&self) -> UnboundedNatural {
        self.cardinality
    }

    pub fn class_expression(&self) -> Option<&Box<ClassExpression>> {
        self.class_expression.as_ref()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ObjectExactCardinality( @optional class_expression, @display cardinality ));

impl ObjectExactCardinality {
    pub fn cardinality(&self) -> UnboundedNatural {
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
    pub fn cardinality(&self) -> UnboundedNatural {
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
    pub fn cardinality(&self) -> UnboundedNatural {
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
    pub fn cardinality(&self) -> UnboundedNatural {
        self.cardinality
    }

    pub fn data_range(&self) -> Option<&DataRange> {
        self.data_range.as_ref()
    }

    pub fn data_property_expression(&self) -> &DataPropertyExpression {
        &self.data_property_expression
    }
}
