
use crate::{
    axioms::Axiom,
    annotations::{Annotation, HasAnnotations},
    entities::{ Individual},
    expressions::{ClassExpression, DataPropertyExpression, ObjectPropertyExpression},
    fmt::DisplayPretty,
    literals::Literal,
};
use strum::{EnumIs, EnumTryAs};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;


// ------------------------------------------------------------------------------------------------
// Public Types ❯ Assertion
// ------------------------------------------------------------------------------------------------

///
/// OWL 2 supports a rich set of axioms for stating assertions — axioms about individuals that are
/// often also called facts.
///
/// ## Specification (Section §9.6)
///
/// ```bnf
/// Assertion :=
///     SameIndividual | DifferentIndividuals | ClassAssertion |
///     ObjectPropertyAssertion | NegativeObjectPropertyAssertion |
///     DataPropertyAssertion | NegativeDataPropertyAssertion
///
/// sourceIndividual := Individual
///
/// targetIndividual := Individual
///
/// targetValue := Literal
/// ```
///
#[derive(Clone, Debug, PartialEq, EnumIs, EnumTryAs)]
pub enum Assertion {
    SameIndividual(SameIndividual),
    DifferentIndividuals(DifferentIndividuals),
    ClassAssertion(ClassAssertion),
    ObjectPropertyAssertion(ObjectPropertyAssertion),
    NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
    DataPropertyAssertion(DataPropertyAssertion),
    NegativeDataPropertyAssertion(NegativeDataPropertyAssertion),
}

///
/// An individual equality axiom $SameIndividual( a_1 \cdots a_n )$ states that all of the
/// individuals $a_i, 1 \leq i \leq n$, are equal to each other. This axiom allows one
/// to use each $a_i$ as a synonym for each $a_j$ — that is, in any expression in the
/// ontology containing such an axiom, $a_i$ can be replaced with $a_j$ without affecting
/// the meaning of the ontology.
///
/// ## Specification (Section §9.6.1)
///
/// ```bnf
/// SameIndividual :=
///     'SameIndividual' '('
///         axiomAnnotations Individual Individual { Individual }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct SameIndividual {
    axiom_annotations: Vec<Annotation>,
    individuals: Vec<Individual>, // 2..*
}

///
/// An individual inequality axiom $DifferentIndividuals( a_1 \cdots an )$ states that all of
/// the individuals $a_i, 1 \leq i \leq n$, are different from each other; that is, no
/// individuals $a_i$ and $a_j$ with $i \neq j$ can be derived to be equal. This axiom can be
/// used to axiomatize the unique name assumption — the assumption that all different individual
/// names denote different individuals.
///
/// ## Specification (Section §9.6.2)
///
/// ```bnf
/// DifferentIndividuals :=
///     'DifferentIndividuals' '('
///         axiomAnnotations
///         Individual Individual { Individual }
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DifferentIndividuals {
    axiom_annotations: Vec<Annotation>,
    individuals: Vec<Individual>, // 2..*
}

///
/// A class assertion $ClassAssertion( CE \ a )$ states that the individual $a$ is an instance of
/// the class expression $CE$.
///
/// ## Specification (Section §9.6.3)
///
/// ```bnf
/// ClassAssertion :=
///     'ClassAssertion' '('
///         axiomAnnotations ClassExpression Individual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ClassAssertion {
    axiom_annotations: Vec<Annotation>,
    individual: Individual,
    class_expression: ClassExpression,
}

///
/// A positive object property assertion $ObjectPropertyAssertion( OPE \ a_1 \ a_2 )$ states that the
/// individual $a_1$ is connected by the object property expression $OPE$ to the individual $a_2$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// ObjectPropertyAssertion :=
///     'ObjectPropertyAssertion' '('
///         axiomAnnotations
///         ObjectPropertyExpression sourceIndividual targetIndividual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_individual: Individual,
    object_property_expression: ObjectPropertyExpression,
}

///
/// A negative object property assertion $NegativeObjectPropertyAssertion( OPE \ a_1 \ a_2 )$ states
/// that the individual $a_1$ is not connected by the object property expression $OPE$ to the
/// individual $a_2$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// NegativeObjectPropertyAssertion :=
///     'NegativeObjectPropertyAssertion' '('
///         axiomAnnotations
///         ObjectPropertyExpression sourceIndividual targetIndividual
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NegativeObjectPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_individual: Individual,
    object_property_expression: ObjectPropertyExpression,
}

///
/// A positive data property assertion $DataPropertyAssertion( DPE \ a \ lt )$ states that the
/// individual $a$ is connected by the data property expression $DPE$ to the literal $lt$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// DataPropertyAssertion :=
///     'DataPropertyAssertion' '('
///         axiomAnnotations
///         DataPropertyExpression sourceIndividual targetValue
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DataPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_value: Literal,
    data_property_expression: DataPropertyExpression,
}

///
/// A negative data property assertion $NegativeDataPropertyAssertion( DPE \ a \ lt )$ states
/// that the individual $a$ is not connected by the data property expression $DPE$ to the
/// literal $lt$.
///
/// ## Specification (Section §)
///
/// ```bnf
/// NegativeDataPropertyAssertion :=
///     'NegativeDataPropertyAssertion' '('
///         axiomAnnotations
///         DataPropertyExpression sourceIndividual targetValue
///     ')'
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct NegativeDataPropertyAssertion {
    axiom_annotations: Vec<Annotation>,
    source_individual: Individual,
    target_value: Literal,
    data_property_expression: DataPropertyExpression,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Assertion
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    Assertion enum SameIndividual,
    DifferentIndividuals,
    ClassAssertion,
    ObjectPropertyAssertion,
    NegativeObjectPropertyAssertion,
    DataPropertyAssertion,
    NegativeDataPropertyAssertion);

impl_has_annotations!(
    Assertion enum SameIndividual,
    DifferentIndividuals,
    ClassAssertion,
    ObjectPropertyAssertion,
    NegativeObjectPropertyAssertion,
    DataPropertyAssertion,
    NegativeDataPropertyAssertion);

impl_from_for_variant!(Assertion, SameIndividual);
impl_from_for_variant!(Assertion, DifferentIndividuals);
impl_from_for_variant!(Assertion, ClassAssertion);
impl_from_for_variant!(Assertion, ObjectPropertyAssertion);
impl_from_for_variant!(Assertion, NegativeObjectPropertyAssertion);
impl_from_for_variant!(Assertion, DataPropertyAssertion);
impl_from_for_variant!(Assertion, NegativeDataPropertyAssertion);

impl_from_for_variant!(Axiom, Assertion ( from SameIndividual ) );
impl_from_for_variant!(Axiom, Assertion ( from DifferentIndividuals ) );
impl_from_for_variant!(Axiom, Assertion ( from ClassAssertion ) );
impl_from_for_variant!(Axiom, Assertion ( from ObjectPropertyAssertion ) );
impl_from_for_variant!(Axiom, Assertion ( from NegativeObjectPropertyAssertion ) );
impl_from_for_variant!(Axiom, Assertion ( from DataPropertyAssertion ) );
impl_from_for_variant!(Axiom, Assertion ( from NegativeDataPropertyAssertion ) );

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(SameIndividual( @list axiom_annotations, @list individuals ));
impl_has_annotations!(SameIndividual, axiom_annotations);

impl SameIndividual {
    pub fn new<II>(individuals: II) -> Self 
    where
        II: IntoIterator<Item = Individual>,
    {
        Self::new_with_annotations(
            Vec::default(),
            individuals,
        )
    }

    pub fn new_with_annotations<IA, II>(
        axiom_annotations: IA,
        individuals: II,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        II: IntoIterator<Item = Individual>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn individuals(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(DifferentIndividuals( @list axiom_annotations, @list individuals ));
impl_has_annotations!(DifferentIndividuals, axiom_annotations);

impl DifferentIndividuals {
    pub fn new<II>(individuals: II) -> Self 
    where
        II: IntoIterator<Item = Individual>,
    {
        Self::new_with_annotations(
            Vec::default(),
            individuals,
        )
    }

    pub fn new_with_annotations<IA, II>(
        axiom_annotations: IA,
        individuals: II,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        II: IntoIterator<Item = Individual>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            individuals: individuals.into_iter().collect(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }

    pub fn individuals(&self) -> impl Iterator<Item = &Individual> {
        self.individuals.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(ClassAssertion( @list axiom_annotations, individual, class_expression ));
impl_has_annotations!(ClassAssertion, axiom_annotations);

impl ClassAssertion {
    pub fn new<CE, I>(class_expression: CE, individual: I) -> Self 
    where
        CE: Into<ClassExpression>,
        I: Into<Individual>,
    {
        Self::new_with_annotations(
            Vec::default(),
            class_expression,
            individual,
        )
    }

    pub fn new_with_annotations<IA, CE, I>(
        axiom_annotations: IA,
        class_expression: CE,
        individual: I,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        CE: Into<ClassExpression>,
        I: Into<Individual>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            class_expression: class_expression.into(),
            individual: individual.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    ObjectPropertyAssertion(
        @list axiom_annotations, source_individual, target_individual, object_property_expression
    )
);
impl_has_annotations!(ObjectPropertyAssertion, axiom_annotations);

impl ObjectPropertyAssertion {
    pub fn new<OPE, I1, I2>(object_property_expression: OPE, source_individual: I1, target_individual: I2) -> Self 
    where
        OPE: Into<ObjectPropertyExpression>,
        I1: Into<Individual>,
        I2: Into<Individual>,
    {
        Self::new_with_annotations(
            Vec::default(),
            object_property_expression,
            source_individual,
            target_individual,
        )
    }

    pub fn new_with_annotations<IA, OPE, I1, I2>(
        axiom_annotations: IA,
        object_property_expression: OPE,
        source_individual: I1,
        target_individual: I2,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
        I1: Into<Individual>,
        I2: Into<Individual>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
            source_individual: source_individual.into(),
            target_individual: target_individual.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    NegativeObjectPropertyAssertion(
        @list axiom_annotations, source_individual, target_individual, object_property_expression
    )
);
impl_has_annotations!(NegativeObjectPropertyAssertion, axiom_annotations);

impl NegativeObjectPropertyAssertion {
    pub fn new<OPE, I1, I2>(object_property_expression: OPE, source_individual: I1, target_individual: I2) -> Self 
    where
        OPE: Into<ObjectPropertyExpression>,
        I1: Into<Individual>,
        I2: Into<Individual>,
    {
        Self::new_with_annotations(
            Vec::default(),
            object_property_expression,
            source_individual,
            target_individual,
        )
    }

    pub fn new_with_annotations<IA, OPE, I1, I2>(
        axiom_annotations: IA,
        object_property_expression: OPE,
        source_individual: I1,
        target_individual: I2,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        OPE: Into<ObjectPropertyExpression>,
        I1: Into<Individual>,
        I2: Into<Individual>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            object_property_expression: object_property_expression.into(),
            source_individual: source_individual.into(),
            target_individual: target_individual.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DataPropertyAssertion(
        @list axiom_annotations, source_individual, target_value, data_property_expression
    )
);
impl_has_annotations!(DataPropertyAssertion, axiom_annotations);

impl DataPropertyAssertion {
    pub fn new<DPE, I, V>(data_property_expression: DPE, source_individual: I, target_value: V) -> Self 
    where
        DPE: Into<DataPropertyExpression>,
        I: Into<Individual>,
        V: Into<Literal>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expression,
            source_individual,
            target_value,
        )
    }

    pub fn new_with_annotations<IA, DPE, I, V>(
        axiom_annotations: IA,
        data_property_expression: DPE,
        source_individual: I,
        target_value: V,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE: Into<DataPropertyExpression>,
        I: Into<Individual>,
        V: Into<Literal>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expression: data_property_expression.into(),
            source_individual: source_individual.into(),
            target_value: target_value.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}

// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    NegativeDataPropertyAssertion(
        @list axiom_annotations, source_individual, target_value, data_property_expression
    )
);
impl_has_annotations!(NegativeDataPropertyAssertion, axiom_annotations);

impl NegativeDataPropertyAssertion {
    pub fn new<DPE, I, V>(data_property_expression: DPE, source_individual: I, target_value: V) -> Self 
    where
        DPE: Into<DataPropertyExpression>,
        I: Into<Individual>,
        V: Into<Literal>,
    {
        Self::new_with_annotations(
            Vec::default(),
            data_property_expression,
            source_individual,
            target_value,
        )
    }

    pub fn new_with_annotations<IA, DPE, I, V>(
        axiom_annotations: IA,
        data_property_expression: DPE,
        source_individual: I,
        target_value: V,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DPE: Into<DataPropertyExpression>,
        I: Into<Individual>,
        V: Into<Literal>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            data_property_expression: data_property_expression.into(),
            source_individual: source_individual.into(),
            target_value: target_value.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}
