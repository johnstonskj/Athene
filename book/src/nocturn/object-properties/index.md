<!-- markdownlint-disable-file MD033 -->
# Object Properties

Object Properties, in general, are shown as directed edges between class nodes.

![Object Properties](./overview.svg)

<span class="figure caption">Object Properties</span>

The following sections outline the notation for common property types, and
patterns.

## owl:ObjectProperty

![owl:ObjectProperty](./owl-ObjectProperty.svg)

<span class="figure caption">An OWL *ObjectProperty* Edge</span>

### owl:ObjectProperty Rules

1. The source and target of the edge **must** both be classes.
2. The edge **must** be named.

## rdfs:subPropertyOf

![rdfs:subPropertyOf](./rdfs-subPropertyOf.svg)

<span class="figure caption">An RDF Schema *subPropertyOf* Edge</span>

### rdfs:subPropertyOf Rules

1. The source and target of the edge **must** both be Object Properties.
2. The edge **must** be named.

## owl:inverseOf

![owl:inverseOf](./owl-inverseOf.svg)

<span class="figure caption">An OWL *inverseOf* Edge</span>

### owl:inverseOf Rules

1. The source and target of the edge **must** both be Object Properties.

## owl:SymmetricProperty

![owl:SymmetricProperty](./owl-SymmetricProperty.svg)

<span class="figure caption">A OWL *SymmetricProperty* Edge</span>

### owl:SymmetricProperty Rules

1. The source and target of the edge **must** both be classes.
2. The edge **must** be named.

## owl:TransitiveProperty

![owl:TransitiveProperty](./owl-TransitiveProperty.svg)

<span class="figure caption">A OWL *TransitiveProperty* Edge</span>

### owl:TransitiveProperty Rules

1. The source and target of the edge **must** both be classes.
2. The edge **must** be named.

## owl:SymmetricProperty *and* owl:TransitiveProperty

![owl:SymmetricProperty and owl:TransitiveProperty](./owl-SymmetricTransitiveProperty.svg)

<span class="figure caption">An OWL *SymmetricProperty* and OWL *TransitiveProperty* Combined Edge</span>

### owl:Symmetric/TransitiveProperty Rules

1. The source and target of the edge **must** both be classes.
2. The edge **must** be named.
