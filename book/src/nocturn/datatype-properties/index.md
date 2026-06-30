<!-- markdownlint-disable-file MD033 -->
# Datatype Properties

![Datatype Properties](./overview.svg)

<span class="figure caption">Datatype Properties</span>

## owl:DatatypeProperty

![Class Notation](./owl-DatatypeProperty.svg)

<span class="figure caption">An OWL *DatatypeProperty* Edge</span>

### owl:DatatypeProperty Rules

1. The source of the edge **must** be a class, and the target **must** be a Datatype.
2. The edge **must** be named.

## rdfs:subPropertyOf

![rdfs:subPropertyOf](./rdfs-subPropertyOf.svg)

<span class="figure caption">An RDF Schema *subPropertyOf* Edge</span>

### rdfs:subPropertyOf Rules

1. The source and target of the edge **must** both be Datatype Properties.
2. The edge **must** be named.

## owl:disjointWith

![Class Notation](./owl-disjointWith.svg)

<span class="figure caption">An OWL *disjointWith* Edge</span>

### owl:disjointWith Rules

1. The source and target of the edge **must** both be Datatype Properties.
