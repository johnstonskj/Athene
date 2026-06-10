# Class and Value Expressions

![Introduction](./introduction.svg)

## Logical

![Logical](./logical.svg)

```turtle
:exampleProperty a owl:ObjectProperty ;
  rdfs:subPropertyOf owl:topObjectProperty ;
  rdfs:domain :Class ;
  rdfs:range [
    a owl:Class ;
    owl:unionOf (
      :ClassA
      :ClassB
    )
  ] .
```

### Logical Rules

TBD

## Enumeration

![Enumeration](./enumeration.svg)

### Enumeration Rules

TBD

## All Different

![All Different](./owl-allDifferent.svg)

### All Different Rules

TBD

## Datatype Restrictions

![Datatype Restrictions](./datatype-restriction.svg)

### Datatype Restrictions Rules

TBD
