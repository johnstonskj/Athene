<!-- markdownlint-disable-file MD033 -->
# Datatype Assertions

![Datatype Assertions](./overview.svg)

<span class="figure caption">Datatype Assertions</span>

## rdf:type

![rdf:type](./rdf-type.svg)

<span class="figure caption">An RDF *type*-of Edge</span>

### rdf:type Rules

1. The source and target of the edge **must** both be datatypes.

## Datatype owl:Restriction Notation

![Class Notation](./owl-Restriction.svg)

<span class="figure caption">An OWL *Restriction* Edge and Axiom Node</span>

### Datatype owl:Restriction Rules

1. The source and target of the edge **must** both be datatypes.
2. The edge **may** have an attached axiom node.
   1. Axiom details TBD
