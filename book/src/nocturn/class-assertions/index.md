<!-- markdownlint-disable-file MD033 -->
# Class Assertions

![Overview of Class Assertions](./overview.svg)

<span class="figure caption">Overview of Class Assertions</span>

## rdfs:subClassOf

![rdfs:subClassOf](./rdfs-subClassOf.svg)

<span class="figure caption">An RDF Schema *subClassOf* Edge</span>

### rdfs:subClassOf Rules

1. The source and target of the edge **must** both be classes.

## rdf:type

![rdf:type](./rdf-type.svg)

<span class="figure caption">An RDF *type*-of Edge</span>

### rdf:type Rules

1. The source and target of the edge **must** both be classes.

## owl:equivalentClass

![owl:equivalentClass](./owl-equivalentClass.svg)

<span class="figure caption">An OWL *equivalentClass* Edge</span>

### owl:equivalentClass Rules

1. The source and target of the edge **must** both be classes.

## owl:disjointWith

![owl:disjointWith](./owl-disjointWith.svg)

<span class="figure caption">An OWL *disjointWith* Edge</span>

### owl:disjointWith Rules

1. The source and target of the edge **must** both be classes.
1. The source and target of the edge **must** not be the same.
