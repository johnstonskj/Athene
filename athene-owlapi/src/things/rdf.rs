use rdftk_iri::vocab::VOCABULARY_RDF;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Datatypes
// ------------------------------------------------------------------------------------------------

//
// The IRI for the `rdf:PlainLiteral` datatype.
//
// This datatype was introduced in it's own specification, [rdf:PlainLiteral: A
// Datatype for RDF Plain Literals](https://www.w3.org/TR/rdf-plain-literal/) and
// used in OWL 2.
//
make_iri_function!(plain_literal_iri => VOCABULARY_RDF:PlainLiteral);
