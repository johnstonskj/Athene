use rdftk_iri::vocab::VOCABULARY_RDF;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Classes
// ------------------------------------------------------------------------------------------------

make_iri_function!(property => VOCABULARY_RDF:Property "RDF Schema 1.1, §2.8 rdf:Property");
make_iri_function!(list => VOCABULARY_RDF:List "RDF Schema 1.1, §5.2.1 rdf:List");
make_iri_function!(bag => VOCABULARY_RDF:Bag "RDF Schema 1.1, §5.1.2 rdf:Bag");
make_iri_function!(seq => VOCABULARY_RDF:Seq "RDF Schema 1.1, §5.1.3 rdf:Seq");
make_iri_function!(alt => VOCABULARY_RDF:Alt "RDF Schema 1.1, §5.1.4 rdf:Alt");
make_iri_function!(statement => VOCABULARY_RDF:Statement "RDF Schema 1.1, §5.3.1 rdf:Statement");

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Datatypes
// ------------------------------------------------------------------------------------------------

make_iri_function!(plain_literal => VOCABULARY_RDF:PlainLiteral "rdf:PlainLiteral: A Datatype for RDF Plain Literals (Second Edition)");

make_iri_function!(lang_string => VOCABULARY_RDF:langString "RDF Schema 1.1, §2.5 rdf:langString");
make_iri_function!(html => VOCABULARY_RDF:HTML "RDF Schema 1.1, §2.6 rdf:HTML");
make_iri_function!(xml_literal => VOCABULARY_RDF:XMLLiteral "RDF Schema 1.1, §2.7 rdf:XMLLiteral");

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(has_type => VOCABULARY_RDF:type "RDF Schema 1.1, §3.3 rdf:type");
make_iri_function!(first => VOCABULARY_RDF:first "RDF Schema 1.1, §5.2.2 rdf:first");
make_iri_function!(rest => VOCABULARY_RDF:rest "RDF Schema 1.1, §5.2.3 rdf:rest");
make_iri_function!(nil => VOCABULARY_RDF:nil "RDF Schema 1.1, §5.2.4 rdf:nil");
make_iri_function!(subject => VOCABULARY_RDF:subject "RDF Schema 1.1, §5.3.2 rdf:subject");
make_iri_function!(predicate => VOCABULARY_RDF:predicate "RDF Schema 1.1, §5.3.3 rdf:predicate");
make_iri_function!(object => VOCABULARY_RDF:object "RDF Schema 1.1, §5.3.4 rdf:object");
make_iri_function!(value => VOCABULARY_RDF:value "RDF Schema 1.1, §5.4.3 rdf:value");
