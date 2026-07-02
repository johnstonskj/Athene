use athene_owlapi::{
    Ontology, OntologyDocument,
    axioms::{Declaration, SubClassOf},
    builders::{AnnotationBuilder, Builder},
    entities::{Class, EntityTrait},
};
use rdftk_iri::Iri;
use std::str::FromStr;

#[test]
fn test_section_3p4_example_ontology() {
    let example = Ontology::builder()
        .with_ontology_iri(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
        .with_direct_import(Iri::from_str("http://www.example.com/my/2.0").unwrap())
        .build()
        .unwrap();

    assert_eq!(
        "Ontology( <http://www.example.com/importing-ontology> Import( <http://www.example.com/my/2.0> ) )",
        format!("{example}")
    );

    assert_eq!(
        "Ontology(
    <http://www.example.com/importing-ontology>
    Import(
        <http://www.example.com/my/2.0>
    )
)",
        format!("{example:#}")
    );
}

#[test]
fn test_section_3p4_example_ontology_document() {
    let example = Ontology::builder()
        .with_ontology_iri(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
        .with_direct_import(Iri::from_str("http://www.example.com/my/2.0").unwrap())
        .build()
        .unwrap();

    let doc = OntologyDocument::builder()
        .with_default_namespace(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
        .with_ontology(example)
        .build()
        .unwrap();

    assert_eq!(
        "Prefix( : = <http://www.example.com/importing-ontology> ) Ontology( <http://www.example.com/importing-ontology> Import( <http://www.example.com/my/2.0> ) )",
        format!("{doc}")
    );

    assert_eq!(
        "Prefix(
    : = <http://www.example.com/importing-ontology>
)
Ontology(
    <http://www.example.com/importing-ontology>
    Import(
        <http://www.example.com/my/2.0>
    )
)",
        format!("{doc:#}")
    );
}

#[test]
fn test_section_3p7_example_ontology_document() {
    let example = Ontology::builder()
        .with_ontology_iri(Iri::from_str("http://www.example.com/ontology1").unwrap())
        .with_direct_import(Iri::from_str("http://www.example.com/ontology2").unwrap())
        .with_rdfs_label("An example")
        .with_class_axiom(SubClassOf::new(
            Class::new(Iri::from_str("http://www.example.com/ontology1#Child").unwrap()),
            Class::new(Iri::from_str("http://www.w3.org/2002/07/owl#Thing").unwrap()),
        ))
        .build()
        .unwrap();

    let doc = OntologyDocument::builder()
        .with_default_namespace(Iri::from_str("http://www.example.com/ontology1#").unwrap())
        .with_ontology(example)
        .build()
        .unwrap();

    assert_eq!(
        "Prefix( : = <http://www.example.com/ontology1#> ) Ontology( <http://www.example.com/ontology1> Import( <http://www.example.com/ontology2> ) Annotation( rdfs:label \"An example\" ) SubClassOf( :Child owl:Thing ) )",
        format!("{doc}")
    );

    assert_eq!(
        "Prefix(
    : = <http://www.example.com/ontology1#>
)
Ontology(
    <http://www.example.com/ontology1>
    Import(
        <http://www.example.com/ontology2>
    )
    Annotation(
        rdfs:label
        \"An example\"
    )
    SubClassOf(
        :Child
        owl:Thing
    )
)",
        format!("{doc:#}")
    );
}

#[test]
fn test_ontology_version_without_ontology_iri_error() {
    let result = Ontology::builder()
        .with_version_iri(Iri::from_str("http://www.example.com/importing-ontology").unwrap())
        .with_direct_import(Iri::from_str("http://www.example.com/my/2.0").unwrap())
        .build();
    assert!(result.is_err());
}

#[test]
fn test_simple_ontology_with_declaration() {
    let example = Ontology::builder()
        .with_ontology_iri(Iri::from_str("http://www.example.com/ex-ontology/").unwrap())
        .with_declaration(Declaration::new(
            Class::new(Iri::from_str("http://www.example.com/ex-ontology/Car").unwrap()).into(),
        ))
        .build()
        .unwrap();

    assert_eq!(
        r"Ontology( <http://www.example.com/ex-ontology/> Declaration( Class( <http://www.example.com/ex-ontology/Car> ) ) )",
        format!("{example}")
    );

    assert_eq!(
        r"Ontology(
    <http://www.example.com/ex-ontology/>
    Declaration(
        Class(
            <http://www.example.com/ex-ontology/Car>
        )
    )
)",
        format!("{example:#}")
    );
}
