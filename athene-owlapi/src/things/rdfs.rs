use rdftk_iri::vocab::VOCABULARY_RDF_SCHEMA;

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Classes
// ------------------------------------------------------------------------------------------------

make_iri_function!(resource => VOCABULARY_RDF_SCHEMA:Resource "RDF Schema 1.1, §2.1 rdfs:Resource");
make_iri_function!(class => VOCABULARY_RDF_SCHEMA:Class "RDF Schema 1.1, §2.2 rdfs:Class");
make_iri_function!(literal => VOCABULARY_RDF_SCHEMA:Literal "RDF Schema 1.1, §2.3 rdfs:Literal");
make_iri_function!(datatype => VOCABULARY_RDF_SCHEMA:Datatype "RDF Schema 1.1, §2.4 rdfs:Datatype");
make_iri_function!(container => VOCABULARY_RDF_SCHEMA:Container "RDF Schema 1.1, §5.1.1 rdfs:Container");
make_iri_function!(container_membership_property => VOCABULARY_RDF_SCHEMA:ContainerMembershipProperty "RDF Schema 1.1, §5.1.5 rdfs:ContainerMembershipProperty");

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(range => VOCABULARY_RDF_SCHEMA:range "RDF Schema 1.1, §3.1 rdfs:range");
make_iri_function!(domain => VOCABULARY_RDF_SCHEMA:domain "RDF Schema 1.1, §3.2 rdfs:domain");
make_iri_function!(sub_class_of => VOCABULARY_RDF_SCHEMA:subClassOf "RDF Schema 1.1, §3.4 rdfs:subClassOf");
make_iri_function!(sub_property_of => VOCABULARY_RDF_SCHEMA:subPropertyOf "RDF Schema 1.1, §3.5 rdfs:subPropertyOf");
make_iri_function!(member => VOCABULARY_RDF_SCHEMA:member "RDF Schema 1.1, §5.1.6 rdfs:member");

// ------------------------------------------------------------------------------------------------
// Public Functions ❯ Annotation Properties
// ------------------------------------------------------------------------------------------------

make_iri_function!(label => VOCABULARY_RDF_SCHEMA:label "RDF Schema 1.1, §3.6 rdfs:label");
make_iri_function!(comment => VOCABULARY_RDF_SCHEMA:comment "RDF Schema 1.1, §3.7 rdfs:comment");
make_iri_function!(see_also => VOCABULARY_RDF_SCHEMA:seeAlso "RDF Schema 1.1, §5.4.1 rdfs:seeAlso");
make_iri_function!(is_defined_by => VOCABULARY_RDF_SCHEMA:isDefinedBy "RDF Schema 1.1, §5.4.2 rdfs:isDefinedBy");
