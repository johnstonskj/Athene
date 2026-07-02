use rdftk_iri::vocab::VOCABULARY_OWL;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(deprecated_iri => VOCABULARY_OWL:deprecated);
make_iri_function!(backward_compatible_with_iri => VOCABULARY_OWL:deprecated);
make_iri_function!(incompatible_with_iri => VOCABULARY_OWL:incompatibleWith);
make_iri_function!(prior_version_iri => VOCABULARY_OWL:priorVersion);
make_iri_function!(version_info_iri => VOCABULARY_OWL:versionInfo);

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Classes
// ------------------------------------------------------------------------------------------------

make_iri_function!(thing_iri => VOCABULARY_OWL:Thing);
make_iri_function!(nothing_iri => VOCABULARY_OWL:Nothing);

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Datatypes
// ------------------------------------------------------------------------------------------------

make_iri_function!(real_iri => VOCABULARY_OWL:real);
make_iri_function!(rational_iri => VOCABULARY_OWL:rational);
