use rdftk_iri::vocab::VOCABULARY_OWL;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Classes
// ------------------------------------------------------------------------------------------------

make_iri_function!(thing => VOCABULARY_OWL:Thing);
make_iri_function!(nothing => VOCABULARY_OWL:Nothing);

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Datatypes
// ------------------------------------------------------------------------------------------------

make_iri_function!(real => VOCABULARY_OWL:real);
make_iri_function!(rational => VOCABULARY_OWL:rational);

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(deprecated => VOCABULARY_OWL:deprecated);
make_iri_function!(backward_compatible_with => VOCABULARY_OWL:deprecated);
make_iri_function!(incompatible_with => VOCABULARY_OWL:incompatibleWith);
make_iri_function!(prior_version => VOCABULARY_OWL:priorVersion);
make_iri_function!(version_info => VOCABULARY_OWL:versionInfo);
