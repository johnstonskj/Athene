use rdftk_iri::vocab::VOCABULARY_RDF_SCHEMA;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(comment_iri => VOCABULARY_RDF_SCHEMA:comment);
make_iri_function!(is_defined_by_iri => VOCABULARY_RDF_SCHEMA:isDefinedBy);
make_iri_function!(label_iri => VOCABULARY_RDF_SCHEMA:label);
make_iri_function!(see_also_iri => VOCABULARY_RDF_SCHEMA:seeAlso);
