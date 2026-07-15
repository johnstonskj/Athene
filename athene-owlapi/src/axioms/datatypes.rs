
use crate::{
    annotations::{Annotation, HasAnnotations},
    entities::{ Datatype},
    fmt::DisplayPretty,
    ranges::DataRange,
};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;


// ------------------------------------------------------------------------------------------------
// Public Types ❯ Datatype Definition
// ------------------------------------------------------------------------------------------------

///
/// A datatype definition $DatatypeDefinition( DT \ DR )$ defines a new datatype $DT$ as being
/// semantically equivalent to the data range $DR$; the latter must be a unary data range.
/// This axiom allows one to use the defined datatype $DT$ as a synonym for $DR$ — that is,
/// in any expression in the ontology containing such an axiom, $DT$ can be replaced with $DR$
/// without affecting the meaning of the ontology.
///
/// ## Specification (Section §9.4)
///
/// ```bnf
/// DatatypeDefinition :=
///     'DatatypeDefinition' '('
///         axiomAnnotations
///         Datatype DataRange
///     ')'
/// ```
///
/// ## Example
///
/// ```owl
/// Declaration( Datatype( a:SSN ) )
/// DatatypeDefinition(
///     a:SSN
///     DatatypeRestriction( xsd:string xsd:pattern "[0-9]{3}-[0-9]{2}-[0-9]{4}" )
/// )
/// DataPropertyRange( a:hasSSN a:SSN )
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct DatatypeDefinition {
    axiom_annotations: Vec<Annotation>,
    datatype: Datatype,
    data_range: DataRange,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ DatatypeDefinition
// ------------------------------------------------------------------------------------------------

impl_display_pretty!(
    DatatypeDefinition( @list axiom_annotations, datatype, data_range )
);
impl_has_annotations!(DatatypeDefinition, axiom_annotations);

impl DatatypeDefinition {
    pub fn new<DT, DR>(datatype: DT, data_range: DR) -> Self 
    where
        DT: Into<Datatype>,
        DR: Into<DataRange>,
    {
        Self::new_with_annotations(
            Vec::default(),
            datatype,
            data_range,
        )
    }

    pub fn new_with_annotations<IA, DT, DR>(
        axiom_annotations: IA,
        datatype: DT,
        data_range: DR,
    ) -> Self 
    where
        IA: IntoIterator<Item = Annotation>,
        DT: Into<Datatype>,
        DR: Into<DataRange>,
    {
        Self {
            axiom_annotations: axiom_annotations.into_iter().collect(),
            datatype: datatype.into(),
            data_range: data_range.into(),
        }
    }

    pub fn axiom_annotations(&self) -> impl Iterator<Item = &Annotation> {
        self.annotations()
    }
}
