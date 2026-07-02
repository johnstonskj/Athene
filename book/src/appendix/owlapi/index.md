<!-- markdownlint-disable-file MD033 -->
# Appendix: OWL API for Rust

The [`athene-owlapi`]() package is a Rust OWL API that provides an implementation
as close as possible to the *OWL 2 Web Ontology Language
[Structural Specification](https://www.w3.org/TR/owl2-syntax/)*.

As the structural specification relies heavily on an object-oriented model with
inheritence of key concepts there has had to be a mapping from the modeling in
the specification to the API herein. That mapping can be describe as follows.

1. Parent types, which are usually abstract, in the OWL 2 specification are
   present in the API as enumerated types.
   1. The `strum` crate provides `is_{variant}` and `trye_as_{variant}`
      methods on the enum to access sub-types.
   2. As all sub-types are distinct each super-type provides implementations
      of `From` between parent and each child type.
2. Attributes present on parent types are pushed down (replicated) on all
   sub-types; however, accessors are present on the enumeration for these
   attributes.
3. Constructors, usually `new(...)`, are provided for simple cases such as
   `Declaration` with more complex cases using a builder pattern.

As the primary goal in the design of the package interface is consistency with the OWL 2
structural specification, rather than writing documentation from scratch we will rely on the
text of the OWL specification. Each section of Rust documentation will have a sub-section
titled **Specification (Section X.Y)** denoting the source location. Examples in the Rust
documentation will reference examples in the same section of the source, with the OWL
/
![The Rusty-Barred Owl: Strix hylophila](./Coruja-listrada_(Strix_hylophila)_no_Parque_Estadual_Intervales.jpg)
<!-- By Nortondefeis - Own work, CC BY-SA 4.0, https://commons.wikimedia.org/w/index.php?curid=49829508 -->

<span class="figure caption">Strix hylophila -- Rusty-Barred Owl (By Nortondefeis - Own work, CC BY-SA 4.0)</span>
