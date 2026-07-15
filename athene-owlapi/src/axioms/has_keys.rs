
use crate::{
    annotations::{Annotation, HasAnnotations},
    expressions::{ClassExpression, DataPropertyExpression, ObjectPropertyExpression},
    fmt::DisplayPretty,
};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;


// ------------------------------------------------------------------------------------------------
// Public Types ❯ Has Key
// ------------------------------------------------------------------------------------------------

///
/// A key axiom $HasKey( CE ( OPE_1 ... OPE_m ) ( DPE_1 ... DPE_n ) )$ states that each (named)
/// instance of the class expression $CE$ is uniquely identified by the object property expressions
/// $OPE_i$ and/or the data property experssions $DPE_j$ — that is, no two distinct (named)
/// instances of $CE$ can coincide on the values of all object property expressions $OPE_i$ and all
/// data property expressions $DPE_j$. In each such axiom in an OWL ontology, $m$ or $n$ (or both)
/// must be larger than zero. A key axiom of the form $HasKey( owl:Thing ( OPE ) () )$ is similar
/// to the axiom $InverseFunctionalObjectProperty( OPE )$, the main differences being that the
/// former axiom is applicable only to individuals that are explicitly named in an ontology, while
/// the latter axiom is also applicable to anonymous individuals and individuals whose existence is
/// implied by existential quantification.
///
/// ## Specification (Section §9.5)
///
/// ```bnf
/// HasKey :=
///     'HasKey' '(' axiomAnnotations
///         ClassExpression
///         '(' { ObjectPropertyExpression } ')'
///         '(' { DataPropertyExpression } ')'
///     ')'
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct HasKey {
    axiom_annotations: Vec<Annotation>,
    class_expression: ClassExpression,
    object_property_expressions: Vec<ObjectPropertyExpression>,
    data_property_expressions: Vec<DataPropertyExpression>,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ HasKey
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    HasKey(
        @list axiom_annotations,
        class_expression,
        @list object_property_expressions, /* ( ... ) */
        @list data_property_expressions    /* ( ... ) */
    )
);
impl_has_annotations!(HasKey, axiom_annotations);

impl HasKey {
    pub fn new<CE, IOPE, IDPE>(class_expression: CE, object_property_expressions: IOPE, data_property_expressions: IDPE) -> Self
    where
        CE: Into<ClassExpression>,
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self::new_with_annotations(
            Vec::default(),
            class_expression,
            object_property_expressions,
            data_property_expressions,
        )
    }

    pub fn new_with_annotations<IA, CE, IOPE, IDPE>(
        axiom_annotations: IA,
        class_expression: CE,
        object_property_expressions: IOPE,
        data_property_expressions: IDPE,
    ) -> Self
    where
        IA: IntoIterator<Item = Annotation>,
        CE: Into<ClassExpression>,
        IOPE: IntoIterator<Item = ObjectPropertyExpression>,
        IDPE: IntoIterator<Item = DataPropertyExpression>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            class_expression: class_expression.into(),
            object_property_expressions: object_property_expressions.into_iter().collect(),
            data_property_expressions: data_property_expressions.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn class_expression(&self) -> &ClassExpression {
        &self.class_expression
    }

    pub fn object_property_expressions(&self) -> impl Iterator<Item = &ObjectPropertyExpression> {
        self.object_property_expressions.iter()
    }

    pub fn data_property_expressions(&self) -> impl Iterator<Item = &DataPropertyExpression> {
        self.data_property_expressions.iter()
    }
}
