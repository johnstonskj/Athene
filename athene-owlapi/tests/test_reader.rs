//! Reader round-trip tests for OWL 2 functional-style syntax.
use athene_owlapi::{axioms::Axiom, reader::parse_str};

fn wrap(axiom: &str) -> String {
    format!(
        "Prefix(:=<http://example.org/>)\n\
         Ontology(<http://example.org/>\n{axiom}\n)",
    )
}

fn parse_one(axiom: &str) -> Axiom {
    let input = wrap(axiom);
    let doc =
        parse_str(&input, false).unwrap_or_else(|e| panic!("parse failed: {e}\n---\n{input}"));
    doc.ontology()
        .axioms()
        .next()
        .cloned()
        .expect("no axiom produced")
}

// ── Declarations ──────────────────────────────────────────────────────────────

#[test]
fn declaration_class() {
    assert!(matches!(
        parse_one("Declaration( Class( :MyClass ) )"),
        Axiom::Declaration(_)
    ));
}

#[test]
fn declaration_datatype() {
    assert!(matches!(
        parse_one("Declaration( Datatype( :MyDatatype ) )"),
        Axiom::Declaration(_)
    ));
}

#[test]
fn declaration_object_property() {
    assert!(matches!(
        parse_one("Declaration( ObjectProperty( :myProp ) )"),
        Axiom::Declaration(_)
    ));
}

#[test]
fn declaration_data_property() {
    assert!(matches!(
        parse_one("Declaration( DataProperty( :myProp ) )"),
        Axiom::Declaration(_)
    ));
}

#[test]
fn declaration_annotation_property() {
    assert!(matches!(
        parse_one("Declaration( AnnotationProperty( :myAnnot ) )"),
        Axiom::Declaration(_)
    ));
}

#[test]
fn declaration_named_individual() {
    assert!(matches!(
        parse_one("Declaration( NamedIndividual( :alice ) )"),
        Axiom::Declaration(_)
    ));
}

// ── Class axioms ──────────────────────────────────────────────────────────────

#[test]
fn sub_class_of() {
    assert!(matches!(
        parse_one("SubClassOf( :Child :Parent )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn equivalent_classes() {
    assert!(matches!(
        parse_one("EquivalentClasses( :A :B )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn disjoint_classes() {
    assert!(matches!(
        parse_one("DisjointClasses( :A :B )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn disjoint_union() {
    assert!(matches!(
        parse_one("DisjointUnion( :A :B :C )"),
        Axiom::ClassAxiom(_)
    ));
}

// ── Object property axioms ────────────────────────────────────────────────────

#[test]
fn sub_object_property_of() {
    assert!(matches!(
        parse_one("SubObjectPropertyOf( :op1 :op2 )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn sub_object_property_chain() {
    assert!(matches!(
        parse_one("SubObjectPropertyOf( ObjectPropertyChain( :op1 :op2 ) :op3 )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn equivalent_object_properties() {
    assert!(matches!(
        parse_one("EquivalentObjectProperties( :op1 :op2 )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn disjoint_object_properties() {
    assert!(matches!(
        parse_one("DisjointObjectProperties( :op1 :op2 )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn inverse_object_properties() {
    assert!(matches!(
        parse_one("InverseObjectProperties( :op1 :op2 )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn object_property_domain() {
    assert!(matches!(
        parse_one("ObjectPropertyDomain( :op :MyClass )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn object_property_range() {
    assert!(matches!(
        parse_one("ObjectPropertyRange( :op :MyClass )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn functional_object_property() {
    assert!(matches!(
        parse_one("FunctionalObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn inverse_functional_object_property() {
    assert!(matches!(
        parse_one("InverseFunctionalObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn reflexive_object_property() {
    assert!(matches!(
        parse_one("ReflexiveObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn irreflexive_object_property() {
    assert!(matches!(
        parse_one("IrreflexiveObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn symmetric_object_property() {
    assert!(matches!(
        parse_one("SymmetricObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn asymmetric_object_property() {
    assert!(matches!(
        parse_one("AsymmetricObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

#[test]
fn transitive_object_property() {
    assert!(matches!(
        parse_one("TransitiveObjectProperty( :op )"),
        Axiom::ObjectPropertyAxiom(_)
    ));
}

// ── Data property axioms ──────────────────────────────────────────────────────

#[test]
fn sub_data_property_of() {
    assert!(matches!(
        parse_one("SubDataPropertyOf( :dp1 :dp2 )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn equivalent_data_properties() {
    assert!(matches!(
        parse_one("EquivalentDataProperties( :dp1 :dp2 )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn disjoint_data_properties() {
    assert!(matches!(
        parse_one("DisjointDataProperties( :dp1 :dp2 )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn data_property_domain() {
    assert!(matches!(
        parse_one("DataPropertyDomain( :dp :MyClass )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn data_property_range() {
    assert!(matches!(
        parse_one("DataPropertyRange( :dp xsd:string )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn functional_data_property() {
    assert!(matches!(
        parse_one("FunctionalDataProperty( :dp )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

// ── Datatype definition ───────────────────────────────────────────────────────

#[test]
fn datatype_definition() {
    assert!(matches!(
        parse_one(
            "DatatypeDefinition( :MyType \
             DatatypeRestriction( xsd:integer xsd:minInclusive \"18\"^^xsd:integer ) )"
        ),
        Axiom::DatatypeDefinition(_)
    ));
}

// ── HasKey ────────────────────────────────────────────────────────────────────

#[test]
fn has_key_with_opes_and_dpes() {
    assert!(matches!(
        parse_one("HasKey( :MyClass ( :op1 :op2 ) ( :dp1 ) )"),
        Axiom::HasKey(_)
    ));
}

#[test]
fn has_key_empty_opes() {
    assert!(matches!(
        parse_one("HasKey( :MyClass ( ) ( :dp1 ) )"),
        Axiom::HasKey(_)
    ));
}

#[test]
fn has_key_only_opes() {
    assert!(matches!(
        parse_one("HasKey( :MyClass ( :op1 ) ( ) )"),
        Axiom::HasKey(_)
    ));
}

// ── Assertions ────────────────────────────────────────────────────────────────

#[test]
fn same_individual() {
    assert!(matches!(
        parse_one("SameIndividual( :alice :bob )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn different_individuals() {
    assert!(matches!(
        parse_one("DifferentIndividuals( :alice :bob )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn class_assertion() {
    assert!(matches!(
        parse_one("ClassAssertion( :MyClass :alice )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn object_property_assertion() {
    assert!(matches!(
        parse_one("ObjectPropertyAssertion( :op :alice :bob )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn negative_object_property_assertion() {
    assert!(matches!(
        parse_one("NegativeObjectPropertyAssertion( :op :alice :bob )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn data_property_assertion() {
    assert!(matches!(
        parse_one("DataPropertyAssertion( :dp :alice \"42\"^^xsd:integer )"),
        Axiom::Assertion(_)
    ));
}

#[test]
fn negative_data_property_assertion() {
    assert!(matches!(
        parse_one("NegativeDataPropertyAssertion( :dp :alice \"no\" )"),
        Axiom::Assertion(_)
    ));
}

// ── Annotation axioms ─────────────────────────────────────────────────────────

#[test]
fn annotation_assertion() {
    assert!(matches!(
        parse_one("AnnotationAssertion( rdfs:label :alice \"Alice\" )"),
        Axiom::AnnotationAxiom(_)
    ));
}

#[test]
fn sub_annotation_property_of() {
    assert!(matches!(
        parse_one("SubAnnotationPropertyOf( :myAnnot rdfs:label )"),
        Axiom::AnnotationAxiom(_)
    ));
}

#[test]
fn annotation_property_domain() {
    assert!(matches!(
        parse_one("AnnotationPropertyDomain( rdfs:label :MyClass )"),
        Axiom::AnnotationAxiom(_)
    ));
}

#[test]
fn annotation_property_range() {
    assert!(matches!(
        parse_one("AnnotationPropertyRange( rdfs:label :MyClass )"),
        Axiom::AnnotationAxiom(_)
    ));
}

// ── Class expressions ─────────────────────────────────────────────────────────

#[test]
fn ce_object_intersection_of() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectIntersectionOf( :A :B ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_union_of() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectUnionOf( :A :B ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_complement_of() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectComplementOf( :A ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_one_of() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectOneOf( :alice :bob ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_some_values_from() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectSomeValuesFrom( :op :A ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_all_values_from() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectAllValuesFrom( :op :A ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_has_value() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectHasValue( :op :alice ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_has_self() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectHasSelf( :op ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_min_cardinality() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectMinCardinality( 2 :op ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_max_cardinality_qualified() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectMaxCardinality( 3 :op :A ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_exact_cardinality() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectExactCardinality( 1 :op ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_some_values_from() {
    assert!(matches!(
        parse_one("SubClassOf( DataSomeValuesFrom( :dp xsd:string ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_all_values_from() {
    assert!(matches!(
        parse_one("SubClassOf( DataAllValuesFrom( :dp xsd:integer ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_has_value() {
    assert!(matches!(
        parse_one("SubClassOf( DataHasValue( :dp \"hello\" ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_min_cardinality() {
    assert!(matches!(
        parse_one("SubClassOf( DataMinCardinality( 1 :dp ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_max_cardinality_qualified() {
    assert!(matches!(
        parse_one("SubClassOf( DataMaxCardinality( 5 :dp xsd:string ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_data_exact_cardinality() {
    assert!(matches!(
        parse_one("SubClassOf( DataExactCardinality( 2 :dp ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

#[test]
fn ce_object_inverse_of() {
    assert!(matches!(
        parse_one("SubClassOf( ObjectSomeValuesFrom( ObjectInverseOf( :op ) :A ) :C )"),
        Axiom::ClassAxiom(_)
    ));
}

// ── Data ranges ───────────────────────────────────────────────────────────────

#[test]
fn dr_data_intersection_of() {
    assert!(matches!(
        parse_one("DataPropertyRange( :dp DataIntersectionOf( xsd:integer xsd:string ) )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn dr_data_union_of() {
    assert!(matches!(
        parse_one("DataPropertyRange( :dp DataUnionOf( xsd:integer xsd:string ) )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn dr_data_complement_of() {
    assert!(matches!(
        parse_one("DataPropertyRange( :dp DataComplementOf( xsd:integer ) )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn dr_data_one_of() {
    assert!(matches!(
        parse_one("DataPropertyRange( :dp DataOneOf( \"1\"^^xsd:integer \"2\"^^xsd:integer ) )"),
        Axiom::DataPropertyAxiom(_)
    ));
}

#[test]
fn dr_datatype_restriction() {
    assert!(matches!(
        parse_one(
            "DataPropertyRange( :dp DatatypeRestriction( xsd:integer \
             xsd:minInclusive \"0\"^^xsd:integer \
             xsd:maxInclusive \"100\"^^xsd:integer ) )"
        ),
        Axiom::DataPropertyAxiom(_)
    ));
}

// ── Annotations on axioms ─────────────────────────────────────────────────────

#[test]
fn annotated_subclass_of() {
    assert!(matches!(
        parse_one("SubClassOf( Annotation( rdfs:comment \"a note\" ) :A :B )"),
        Axiom::ClassAxiom(_)
    ));
}
