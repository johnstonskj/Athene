use rdftk_iri::vocab::VOCABULARY_SKOS;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(alt_label_iri => VOCABULARY_SKOS:altLabel);
make_iri_function!(pref_label_iri => VOCABULARY_SKOS:prefLabel);
make_iri_function!(hidden_label_iri => VOCABULARY_SKOS:hiddenLabel);
make_iri_function!(note_iri => VOCABULARY_SKOS:note);
make_iri_function!(change_note_iri => VOCABULARY_SKOS:changeNote);
make_iri_function!(editorial_note_iri => VOCABULARY_SKOS:editorialNote);
make_iri_function!(history_note_iri => VOCABULARY_SKOS:historyNote);
make_iri_function!(scope_note_iri => VOCABULARY_SKOS:scopeNote);
make_iri_function!(definition_iri => VOCABULARY_SKOS:definition);
make_iri_function!(example_iri => VOCABULARY_SKOS:example);
