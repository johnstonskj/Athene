# Describe

```bash
❯ hoot describe --help
Describe an OWL function

Usage: hoot describe [OPTIONS] <FUNCTION_NAME>

Arguments:
  <FUNCTION_NAME>  The name of the function to describe

Options:
      --no-render              If true this will display the raw markdown without using the built-in renderer.
  -v, --verbose...             Increase logging verbosity by one level per occurance
      --quiet...               Decrease logging verbosity by one level per occurance
  -s, --skin-file <SKIN_FILE>  Name of a JSON file containing a termimad serialized skin file [env: TERMIMAD_SKIN_FILE=]
      --color <WHEN>           Controls when to use color [default: auto] [possible values: auto, always, never]
  -h, --help                   Print help
```

## Example

```bash
❯ hoot describe Annotation

# Function Annotation (Section §10.1)

**Summary.** OWL 2 applications often need ways to associate additional information 
with ontologies, entities, and axioms. To this end, OWL 2 provides for annotations
on ontologies, axioms, and entities.

## Syntax

~~~bnf
Annotation :=                                                    
    'Annotation' '('                                             
        annotationAnnotations AnnotationProperty AnnotationValue 
    ')'                                                          
                                                                 
annotationAnnotations  := { Annotation }                         
AnnotationValue := AnonymousIndividual | IRI | Literal           
~~~

## Description

Ontologies, axioms, and annotations themselves can be annotated using annotations.
Such annotations consist of an annotation property and an annotation value, where
the latter can be anonymous individuals, IRIs, and literals.

## See Also

**AnnotationProperty**

## Source

Content from [OWL 2 Web Ontology Language Structural Specification and 
Functional-Style Syntax (Second Edition)](https://www.w3.org/TR/owl2-syntax/).

Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C liability, 
trademark and document use rules apply.
```

## No-Render Option

TBD

## Skin-File Option

TBD

```json
{
  "bold": "Yellow Bold",
  "italic": "Magenta rgb(30, 30, 40) Italic Underlined OverLined",
  "strikeout": "CrossedOut",
  "inline_code": "ansi(249) ansi(235) Reverse",
  "ellipsis": "",
  "bullet": "⟡ Yellow",
  "quote": "▐ Yellow Bold",
  "horizontal_rule": "― ansi(238)",
  "scrollbar": "▐ ansi(178) ansi(237)",
  "paragraph": "Magenta center 4 4",
  "code_block": "ansi(249) ansi(235) 4",
  "table": "ansi(239) center",
  "headers": [
    "ansi(178) ansi(129) Bold Underlined center",
    "ansi(178) Bold Underlined ",
    "ansi(254) Underlined ",
    "ansi(178) Underlined ",
    "ansi(178) Underlined ",
    "ansi(178) Underlined ",
    "ansi(178) Underlined ",
    "ansi(178) Underlined "
  ],
  "table_border_chars": "rounded"
}
```
