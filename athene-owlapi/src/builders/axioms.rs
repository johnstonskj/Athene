use crate::{
    annotations::Annotation,
    axioms::{
        Declaration,
        classes::{DisjointClasses, DisjointUnion, EquivalentClass, SubClassOf},
        object_properties::{
            DisjointObjectProperties, EquivalentObjectProperties, SubObjectPropertyExpression,
        },
    },
    entities::{
        AnnotationProperty, Class, DataProperty, Datatype, Entity, NamedIndividual, ObjectProperty,
    },
    error::ApiError,
    expressions::{ClassExpression, ObjectPropertyExpression},
};

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Entity Declarations
// ------------------------------------------------------------------------------------------------

///
/// A builder for the `Declaration` axiom type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeclarationBuilder {
    annotations: Vec<Annotation>,
    entity: Option<Entity>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Class Axioms
// ------------------------------------------------------------------------------------------------

///
/// A builder for the `SubClassOf` axiom type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubClassOfBuilder {
    annotations: Vec<Annotation>,
    sub_class_expression: Option<ClassExpression>,
    super_class_expression: Option<ClassExpression>,
}

///
/// A builder for the `EquivalentClass` axiom type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EquivalentClassBuilder {
    annotations: Vec<Annotation>,
    class_expressions: Vec<ClassExpression>,
}

///
/// A builder for the `DisjointClasses` axiom type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisjointClassesBuilder {
    annotations: Vec<Annotation>,
    class_expressions: Vec<ClassExpression>,
}

///
/// A builder for the `DisjointUnion` axiom type.
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisjointUnionBuilder {
    annotations: Vec<Annotation>,
    class: Option<Class>,
    disjoint_class_expressions: Vec<ClassExpression>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Object Property Axioms
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EquivalentObjectPropertiesBuilder {
    annotations: Vec<Annotation>,
    object_property_expressions: Vec<ObjectPropertyExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisjointObjectPropertiesBuilder {
    annotations: Vec<Annotation>,
    object_property_expressions: Vec<ObjectPropertyExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SubObjectPropertyOfBuilder {
    annotations: Vec<Annotation>,
    sub_object_property_expressions: Option<SubObjectPropertyExpression>,
    super_object_property_expression: Option<ObjectPropertyExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectPropertyDomainBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>,
    domain: Option<ClassExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectPropertyRangeBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>,
    range: Option<ClassExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InverseObjectPropertiesBuilder {
    annotations: Vec<Annotation>,
    object_property_expression_1: Option<ObjectPropertyExpression>,
    object_property_expression_2: Option<ObjectPropertyExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FunctionalObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InverseFunctionalObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>, // 1
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReflexiveObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct IrreflexiveObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>, // 1
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SymmetricObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>, // 1
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AsymmetricObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>, // 1
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TransitiveObjectPropertyBuilder {
    annotations: Vec<Annotation>,
    object_property_expression: Option<ObjectPropertyExpression>, // 1
}

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Data Property Axioms
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Datatype Definitions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Has Key
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Assertions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❯ Annotation Axioms
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Entity Declarations
// ------------------------------------------------------------------------------------------------

impl_has_builder!(Declaration, DeclarationBuilder);

impl TryFrom<DeclarationBuilder> for Declaration {
    type Error = ApiError;

    fn try_from(builder: DeclarationBuilder) -> Result<Self, Self::Error> {
        if let Some(entity) = builder.entity {
            Ok(Declaration::new_with_annotations(
                builder.annotations,
                entity,
            ))
        } else {
            Err(ApiError::MissingField {
                name: "entity".to_string(),
            })
        }
    }
}

impl_builder_try_from!(Declaration, DeclarationBuilder);
impl_annotation_builder!(DeclarationBuilder);

impl DeclarationBuilder {
    pub fn annotation_property<AP>(mut self, annotation_property: AP) -> Self
    where
        AP: Into<AnnotationProperty>,
    {
        self.entity = Some(Entity::AnnotationProperty(annotation_property.into()));
        self
    }

    pub fn class<C>(mut self, class: C) -> Self
    where
        C: Into<Class>,
    {
        self.entity = Some(Entity::Class(class.into()));
        self
    }

    pub fn data_property<DP>(mut self, data_property: DP) -> Self
    where
        DP: Into<DataProperty>,
    {
        self.entity = Some(Entity::DataProperty(data_property.into()));
        self
    }

    pub fn datatype<DT>(mut self, datatype: DT) -> Self
    where
        DT: Into<Datatype>,
    {
        self.entity = Some(Entity::Datatype(datatype.into()));
        self
    }

    pub fn object_property<OP>(mut self, object_property: OP) -> Self
    where
        OP: Into<ObjectProperty>,
    {
        self.entity = Some(Entity::ObjectProperty(object_property.into()));
        self
    }

    pub fn named_individual<NI>(mut self, named_individual: NI) -> Self
    where
        NI: Into<NamedIndividual>,
    {
        self.entity = Some(Entity::NamedIndividual(named_individual.into()));
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ClassAxioms ❯ SubClassOf
// ------------------------------------------------------------------------------------------------

impl_has_builder!(SubClassOf, SubClassOfBuilder);

impl TryFrom<SubClassOfBuilder> for SubClassOf {
    type Error = ApiError;

    #[allow(clippy::unnecessary_unwrap)]
    fn try_from(builder: SubClassOfBuilder) -> Result<Self, Self::Error> {
        if builder.sub_class_expression.is_none() {
            Err(ApiError::MissingField {
                name: "sub_class_expression".to_string(),
            })
        } else if builder.super_class_expression.is_none() {
            Err(ApiError::MissingField {
                name: "super_class_expression".to_string(),
            })
        } else {
            Ok(SubClassOf::new_with_annotations(
                builder.annotations,
                builder.sub_class_expression.unwrap(),
                builder.super_class_expression.unwrap(),
            ))
        }
    }
}

impl_builder_try_from!(SubClassOf, SubClassOfBuilder);
impl_annotation_builder!(SubClassOfBuilder);

impl SubClassOfBuilder {
    pub fn sub_class_expression<CE>(mut self, sub_class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.sub_class_expression = Some(sub_class_expression.into());
        self
    }

    pub fn super_class_expression<CE>(mut self, super_class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.super_class_expression = Some(super_class_expression.into());
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ClassAxioms ❯ EquivalentClass
// ------------------------------------------------------------------------------------------------

impl_has_builder!(EquivalentClass, EquivalentClassBuilder);

impl TryFrom<EquivalentClassBuilder> for EquivalentClass {
    type Error = ApiError;

    fn try_from(builder: EquivalentClassBuilder) -> Result<Self, Self::Error> {
        if builder.class_expressions.len() < 2 {
            Err(ApiError::MissingField {
                name: "class_expressions".to_string(),
            })
        } else {
            Ok(EquivalentClass::new_with_annotations(
                builder.annotations,
                builder.class_expressions,
            )?)
        }
    }
}

impl_builder_try_from!(EquivalentClass, EquivalentClassBuilder);
impl_annotation_builder!(EquivalentClassBuilder);

impl EquivalentClassBuilder {
    pub fn class_expression<CE>(mut self, class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.class_expressions.push(class_expression.into());
        self
    }

    pub fn class_expressions<ICE>(mut self, class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.class_expressions = class_expressions.into_iter().collect();
        self
    }

    #[inline(always)]
    pub fn equivalent_to<CE>(self, class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.class_expression(class_expression)
    }

    #[inline(always)]
    pub fn equivalent_to_all<ICE>(self, class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.class_expressions(class_expressions)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ClassAxioms ❯ DisjointClasses
// ------------------------------------------------------------------------------------------------

impl_has_builder!(DisjointClasses, DisjointClassesBuilder);

impl TryFrom<DisjointClassesBuilder> for DisjointClasses {
    type Error = ApiError;

    fn try_from(builder: DisjointClassesBuilder) -> Result<Self, Self::Error> {
        if builder.class_expressions.len() < 2 {
            Err(ApiError::MissingField {
                name: "class_expressions".to_string(),
            })
        } else {
            Ok(DisjointClasses::new_with_annotations(
                builder.annotations,
                builder.class_expressions,
            )
            .expect("Unexpected API error"))
        }
    }
}

impl_builder_try_from!(DisjointClasses, DisjointClassesBuilder);
impl_annotation_builder!(DisjointClassesBuilder);

impl DisjointClassesBuilder {
    pub fn class_expression<CE>(mut self, class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.class_expressions.push(class_expression.into());
        self
    }

    #[inline(always)]
    pub fn class_expressions<ICE>(mut self, class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.class_expressions = class_expressions.into_iter().collect();
        self
    }

    #[inline(always)]
    pub fn disjoint_with<CE>(self, class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.class_expression(class_expression)
    }

    pub fn disjoint_with_all<ICE>(self, class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.class_expressions(class_expressions)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ClassAxioms ❯ DisjointUnion
// ------------------------------------------------------------------------------------------------

impl_has_builder!(DisjointUnion, DisjointUnionBuilder);

impl TryFrom<DisjointUnionBuilder> for DisjointUnion {
    type Error = ApiError;

    fn try_from(builder: DisjointUnionBuilder) -> Result<Self, Self::Error> {
        if builder.disjoint_class_expressions.len() < 2 {
            Err(ApiError::MissingField {
                name: "disjoint_class_expressions".to_string(),
            })
        } else {
            Ok(DisjointUnion::new_with_annotations(
                builder.annotations,
                builder.class.unwrap(),
                builder.disjoint_class_expressions,
            )
            .expect("Unexpected API error"))
        }
    }
}

impl_builder_try_from!(DisjointUnion, DisjointUnionBuilder);
impl_annotation_builder!(DisjointUnionBuilder);

impl DisjointUnionBuilder {
    pub fn class<C>(mut self, class: C) -> Self
    where
        C: Into<Class>,
    {
        self.class = Some(class.into());
        self
    }

    pub fn disjoint_class_expression<CE>(mut self, disjoint_class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.disjoint_class_expressions
            .push(disjoint_class_expression.into());
        self
    }

    pub fn disjoint_class_expressions<ICE>(mut self, disjoint_class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.disjoint_class_expressions = disjoint_class_expressions.into_iter().collect();
        self
    }

    #[inline(always)]
    pub fn disjoint_with<CE>(self, disjoint_class_expression: CE) -> Self
    where
        CE: Into<ClassExpression>,
    {
        self.disjoint_class_expression(disjoint_class_expression)
    }

    #[inline(always)]
    pub fn disjoint_with_all<ICE>(self, disjoint_class_expressions: ICE) -> Self
    where
        ICE: IntoIterator<Item = ClassExpression>,
    {
        self.disjoint_class_expressions(disjoint_class_expressions)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ EquivalentObjectProperties
// ------------------------------------------------------------------------------------------------

impl_has_builder!(
    EquivalentObjectProperties,
    EquivalentObjectPropertiesBuilder
);

impl TryFrom<EquivalentObjectPropertiesBuilder> for EquivalentObjectProperties {
    type Error = ApiError;

    fn try_from(builder: EquivalentObjectPropertiesBuilder) -> Result<Self, Self::Error> {
        if builder.object_property_expressions.len() < 2 {
            Err(ApiError::MissingField {
                name: "object_property_expressions".to_string(),
            })
        } else {
            Ok(EquivalentObjectProperties::new_with_annotations(
                builder.annotations,
                builder.object_property_expressions,
            ))
        }
    }
}

impl_builder_try_from!(
    EquivalentObjectProperties,
    EquivalentObjectPropertiesBuilder
);
impl_annotation_builder!(EquivalentObjectPropertiesBuilder);

impl EquivalentObjectPropertiesBuilder {
    #[inline(always)]
    pub fn object_property_expression<OPE>(mut self, object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        self.object_property_expressions
            .push(object_property_expression.into());
        self
    }

    #[inline(always)]
    pub fn object_property_expressions<IOPE>(mut self, object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        self.object_property_expressions = object_property_expressions.into_iter().collect();
        self
    }

    #[inline(always)]
    pub fn equivalent_to<OPE>(self, object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        self.object_property_expression(object_property_expression)
    }

    #[inline(always)]
    pub fn equivalent_to_all<IOPE>(self, object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        self.object_property_expressions(object_property_expressions)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ DisjointObjectProperties
// ------------------------------------------------------------------------------------------------

impl_has_builder!(DisjointObjectProperties, DisjointObjectPropertiesBuilder);

impl TryFrom<DisjointObjectPropertiesBuilder> for DisjointObjectProperties {
    type Error = ApiError;

    fn try_from(builder: DisjointObjectPropertiesBuilder) -> Result<Self, Self::Error> {
        if builder.object_property_expressions.len() < 2 {
            Err(ApiError::MissingField {
                name: "object_property_expressions".to_string(),
            })
        } else {
            Ok(DisjointObjectProperties::new_with_annotations(
                builder.annotations,
                builder.object_property_expressions,
            ))
        }
    }
}

impl_builder_try_from!(DisjointObjectProperties, DisjointObjectPropertiesBuilder);
impl_annotation_builder!(DisjointObjectPropertiesBuilder);

impl DisjointObjectPropertiesBuilder {
    #[inline(always)]
    pub fn object_property_expression<OPE>(mut self, object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        self.object_property_expressions
            .push(object_property_expression.into());
        self
    }

    #[inline(always)]
    pub fn object_property_expressions<IOPE>(mut self, object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        self.object_property_expressions = object_property_expressions.into_iter().collect();
        self
    }

    #[inline(always)]
    pub fn disjoint_with<OPE>(self, object_property_expression: OPE) -> Self
    where
        OPE: Into<ObjectPropertyExpression>,
    {
        self.object_property_expression(object_property_expression)
    }

    #[inline(always)]
    pub fn disjoint_with_all<IOPE>(self, object_property_expressions: IOPE) -> Self
    where
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
    {
        self.object_property_expressions(object_property_expressions)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ SubObjectPropertyOf
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ ObjectPropertyDomain
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ ObjectPropertyRange
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ InverseObjectProperties
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ FunctionalObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ InverseFunctionalObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ ReflexiveObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ IrreflexiveObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ SymmetricObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ AsymmetricObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Object Properties ❯ TransitiveObjectProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ SubDataPropertyOf
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ DisjointDataProperties
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ EquivalentDataProperties
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ FunctionalDataProperty
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ DataPropertyDomain
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Data Properties ❯ EquivalentDataProperties
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Datatype
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Has Key
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ SameIndividual
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ DifferentIndividuals
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ ClassAssertion
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ ObjectPropertyAssertion
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ NegativeObjectPropertyAssertion
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ DataPropertyAssertion
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertions ❯ NegativeDataPropertyAssertion
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotations ❯ SubAnnotationOf
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotations ❯ AnnotationPropertyDomain
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotations ❯ AnnotationPropertyRange
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Annotations ❯ AnnotationAssertion
// ------------------------------------------------------------------------------------------------
