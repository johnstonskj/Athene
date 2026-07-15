use rdftk_iri::vocab::VOCABULARY_SKOS;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(alt_label => VOCABULARY_SKOS:altLabel);
make_iri_function!(pref_label => VOCABULARY_SKOS:prefLabel);
make_iri_function!(hidden_label => VOCABULARY_SKOS:hiddenLabel);
make_iri_function!(note => VOCABULARY_SKOS:note);
make_iri_function!(change_note => VOCABULARY_SKOS:changeNote);
make_iri_function!(editorial_note => VOCABULARY_SKOS:editorialNote);
make_iri_function!(history_note => VOCABULARY_SKOS:historyNote);
make_iri_function!(scope_note => VOCABULARY_SKOS:scopeNote);
make_iri_function!(definition => VOCABULARY_SKOS:definition);
make_iri_function!(example => VOCABULARY_SKOS:example);
