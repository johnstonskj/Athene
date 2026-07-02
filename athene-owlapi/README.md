# Package athene-owlapi

This package provides an OWL API that provides as an implementation as close as possible to
the *OWL 2 Web Ontology Language [Structural Specification](https://www.w3.org/TR/owl2-syntax/)*.

[![Apache-2.0 License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT License](https://img.shields.io/badge/license-mit-118811.svg)](https://opensource.org/license/mit)
[![crates.io](https://img.shields.io/crates/v/athene-owlapi.svg)](https://crates.io/crates/athene-owlapi)
[![docs.rs](https://docs.rs/athene-owlapi/badge.svg)](https://docs.rs/athene-owlapi)

Part of the [Athene project](https://athene.github.io).

As the OWL 2 Structural Specification relies heavily on an object-oriented model with inheritence
of key concepts there has had to be a mapping from the modeling in the specification to the
API herein. That mapping can be describe as follows.

1. Parent types, which are usually abstract, in the OWL 2 specification are present in the
   API as enumerated types.
   1. The `strum` crate provides `is_{variant}` and `trye_as_{variant}` methods on the enum
      to access sub-types.
   2. As all sub-types are distinct each super-type provides implementations of `From`
      between parent and each child type.
2. Attributes present on parent types are pushed down (replicated) on all sub-types; however,
   accessors are present on the enumeration for these attributes.
3. Constructors, usually `new(...)`, are provided for simple cases such as `Declaration`
   with more complex cases using a builder pattern.

As the primary goal in the design of the package interface is consistency with the OWL 2
structural specification, rather than writing documentation from scratch we will rely on the
text of the OWL specification. Each section of Rust documentation will have a sub-section
titled **Specification (Section X.Y)** denoting the source location. Examples in the Rust
documentation will reference examples in the same section of the source, with the OWL
functional syntax shown with Rust equivalent when relevant.

## Examples

Given the following source in the OWL functional syntax.

```owl
Prefix(:=<http://www.example.com/ontology1#>)
Ontology( <http://www.example.com/ontology1>
    Import( <http://www.example.com/ontology2> )
    Annotation( rdfs:label "An example" )
    SubClassOf( :Child owl:Thing )
)
```

The following Rust code

```rust
use athene_owlapi::{
    Ontology, OntologyDocument,
    axioms::SubClassOf,
    builders::{AnnotationBuilder, Builder},
    entities::{Class, EntityTrait},
};
use rdftk_iri::Iri;
use std::str::FromStr;

let document = OntologyDocument::builder()
   .with_default_namespace(Iri::from_str("http://www.example.com/ontology1#").unwrap())
   .with_ontology(Ontology::builder()
       .with_ontology_iri(Iri::from_str("http://www.example.com/ontology1").unwrap())
       .with_direct_import(Iri::from_str("http://www.example.com/ontology2").unwrap())
       .with_rdfs_label("An example")
       .with_class_axiom(SubClassOf::new(
           Class::new(Iri::from_str("http://www.example.com/ontology1#Child").unwrap()).into(),
           Class::new(Iri::from_str("http://www.w3.org/2002/07/owl#Thing").unwrap()).into(),
       ))
       .build()
       .expect("could not build Ontology"))
   .build()
   .expect("could not build OntologyDocument");
```

## Status

TBD

### API Coverage

### Builders/Erognomics

### Reader Robustness

### Documentation

### Test Coverage

## License(s)

The contents of this repository are made available under the following
licenses:

### Apache-2.0

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Licensed under the Apache License, Version 2.0 (the "License");
> you may not use this file except in compliance with the License.
> You may obtain a copy of the License at
> 
>     http://www.apache.org/licenses/LICENSE-2.0
> 
> Unless required by applicable law or agreed to in writing, software
> distributed under the License is distributed on an "AS IS" BASIS,
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
> See the License for the specific language governing permissions and
> limitations under the License.
> ```

See the enclosed file [LICENSE-Apache](https://github.com/johnstonskj/Athene/blob/main/LICENSE-Apache).

### MIT

> ```text
> Copyright 2025 Simon Johnston <johnstonskj@gmail.com>
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the “Software”), to deal
> in the Software without restriction, including without limitation the rights to
> use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
> the Software, and to permit persons to whom the Software is furnished to do so,
> subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
> INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
> PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
> HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
> OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
> SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
> ```

See the enclosed file [LICENSE-MIT](https://github.com/johnstonskj/Athene/blob/main/LICENSE-MIT).
