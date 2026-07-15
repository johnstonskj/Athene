//!
//! Stage 2 semantic converter: [`SyntaxDocument`] → [`OntologyDocument`].
//!

use crate::{
    Import, Ontology, OntologyDocument,
    annotations::{Annotation, AnnotationValue},
    axioms::{
        Axiom,
        annotations::{
            AnnotationAssertion, AnnotationAxiom, AnnotationPropertyDomain,
            AnnotationPropertyRange, AnnotationSubject, SubAnnotationOf,
        },
        assertions::{
            Assertion, ClassAssertion, DataPropertyAssertion, DifferentIndividuals,
            NegativeDataPropertyAssertion, NegativeObjectPropertyAssertion,
            ObjectPropertyAssertion, SameIndividual,
        },
        classes::{ClassAxiom, DisjointClasses, DisjointUnion, EquivalentClass, SubClassOf},
        data_properties::{
            DataPropertyAxiom, DataPropertyDomain, DataPropertyRange, DisjointDataProperties,
            EquivalentDataProperties, FunctionalDataProperty, SubDataPropertyOf,
        },
        datatypes::DatatypeDefinition,
        declarations::Declaration,
        has_keys::HasKey,
        object_properties::{
            AsymmetricObjectProperty, DisjointObjectProperties, EquivalentObjectProperties,
            FunctionalObjectProperty, InverseFunctionalObjectProperty, InverseObjectProperties,
            IrreflexiveObjectProperty, ObjectPropertyAxiom, ObjectPropertyDomain,
            ObjectPropertyRange, PropertyExpressionChain, ReflexiveObjectProperty,
            SubObjectPropertyExpression, SubObjectPropertyOf, SymmetricObjectProperty,
            TransitiveObjectProperty,
        },
    },
    builders::{AnnotationBuilder, Builder, HasBuilder},
    entities::{
        AnnotationProperty, AnonymousIndividual, Class, DataProperty, Datatype, Entity,
        EntityTrait, Individual, NamedIndividual, ObjectProperty,
    },
    expressions::{
        ClassExpression, DataAllValuesFrom, DataExactCardinality, DataHasValue, DataMaxCardinality,
        DataMinCardinality, DataPropertyExpression, DataSomeValuesFrom,
        InverseObjectProperty as InverseOPExpr, ObjectAllValuesFrom, ObjectComplementOf,
        ObjectExactCardinality, ObjectHasSelf, ObjectHasValue, ObjectIntersectionOf,
        ObjectMaxCardinality, ObjectMinCardinality, ObjectOneOf, ObjectPropertyExpression,
        ObjectSomeValuesFrom, ObjectUnionOf,
    },
    literals::Literal,
    ranges::{
        DataComplementOf, DataIntersectionOf, DataOneOf, DataRange, DataUnionOf,
        DatatypeRestriction, FacetRestriction,
    },
    reader::{
        ast::{
            IriRef, LiteralSyntax, PrefixedIriRef, Span, SyntaxDocument, SyntaxNode, SyntaxNodeKind,
        },
        error::ReaderError,
        reporter::Reporter,
    },
    reserved_prefix_map,
    syntax::FN_PREFIX,
};
use core::str::FromStr;
use rdftk_iri::{Iri, IriPrefixMap, Namespace};

#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) struct Converter {
    prefixes: IriPrefixMap,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Converter {
    fn default() -> Self {
        Self {
            prefixes: reserved_prefix_map(),
        }
    }
}
impl Converter {
    pub(super) fn convert(
        mut self,
        doc: SyntaxDocument,
        reporter: &dyn Reporter,
    ) -> Result<OntologyDocument, ReaderError> {
        let mut document: Option<OntologyDocument> = None;
        for node in &doc.nodes {
            if node.kind.is_comment() {
                // ignore
            } else if document.is_some() {
                return Err(reporter.unexpected_ast_node(node, &["comment"]));
            } else {
                if node.function_name() == Some("Prefix") {
                    self.process_prefix_decl(node, reporter)?;
                } else if node.function_name() == Some("Ontology") {
                    document = Some(self.build_document(node, reporter)?);
                } else {
                    return Err(
                        reporter.unexpected_ast_node(node, &["comment", "Prefix", "Ontology"])
                    );
                }
            }
        }
        if let Some(document) = document {
            Ok(document)
        } else {
            Err(reporter.missing_ast_node(&Span::default(), &["Ontology"]))
        }
    }

    // ── Prefix handling ────────────────────────────────────────────────────────

    fn process_prefix_decl(
        &mut self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<(), ReaderError> {
        let reserved_values = reserved_prefix_map();
        let args = self.semantic_args_of(node);
        if args.len() != 3 {
            return Err(reporter.argument_arity(FN_PREFIX, args.len(), 3.into(), node.span));
        }
        // Prefix( <Namespace> = <FullIri> ) → 3 args
        let prefix_key: Option<String> = match args.first().and_then(|n| n.try_as_iri_ref()) {
            Some(IriRef::Namespace(Some(prefix_string))) => {
                if reserved_values
                    .prefixes()
                    .find(|p| p.to_string() == format!("{prefix_string}:"))
                    .is_some()
                {
                    return Err(reporter.using_reserved_prefix(
                        prefix_string.as_str(),
                        self.semantic_args_of(node)[0].span,
                    ));
                } else {
                    Some(prefix_string.clone())
                }
            }
            Some(IriRef::Namespace(None)) => None,
            _ => return Ok(()), /* TODO: this has to be an error */
        };
        // TODO: check for "="!
        let iri: Iri = match args.get(2).and_then(|n| n.try_as_iri_ref()) {
            Some(IriRef::Full(iri_str)) => {
                let iri = self.parse_iri(iri_str, node.span, reporter)?;
                println!(
                    "MATCH? {iri:?} in {:?}",
                    reserved_values.iris().collect::<Vec<_>>()
                );
                if reserved_values.iris().find(|s| *s == &iri).is_some() {
                    return Err(
                        reporter.using_reserved_iri(iri_str, self.semantic_args_of(node)[2].span)
                    );
                } else {
                    iri.clone()
                }
            }
            _ => return Ok(()), /* TODO: this has to be an error */
        };
        match prefix_key {
            None => self.prefixes.set_default_namespace(iri),
            Some(p) => {
                let ns = Namespace::from_str(&format!("{}:", p))
                    .map_err(|e| reporter.invalid_namespace(&p, e.to_string(), args[0].span))?;
                self.prefixes.insert(ns, iri);
            }
        }
        Ok(())
    }

    // ── Document/Ontology building ─────────────────────────────────────────────

    fn build_document(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<OntologyDocument, ReaderError> {
        let ontology = self.convert_ontology(node, reporter)?;
        let mut doc_builder = OntologyDocument::builder();

        for (ns, iri) in self.prefixes.mappings() {
            if ns.is_default() {
                doc_builder = doc_builder.default_prefix(iri.clone());
            } else {
                doc_builder = doc_builder.prefix(ns.clone(), iri.clone());
            }
        }

        doc_builder
            .ontology(ontology)
            .build()
            .map_err(|e| ReaderError::InvalidArgument {
                message: e.to_string(),
                span: node.span,
            })
    }

    fn convert_ontology(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Ontology, ReaderError> {
        let args = self.semantic_args_of(node);
        let mut builder = Ontology::builder();
        let mut idx = 0;

        // optional ontologyIRI
        if idx < args.len() && args[idx].try_as_iri_ref().is_some() {
            builder = builder.ontology_iri(self.resolve_iri(args[idx], reporter)?);
            idx += 1;
            // optional versionIRI
            if idx < args.len() && args[idx].try_as_iri_ref().is_some() {
                builder = builder.version_iri(self.resolve_iri(args[idx], reporter)?);
                idx += 1;
            }
        }

        for arg in &args[idx..] {
            match arg.function_name() {
                Some("Import") => {
                    let iri = self.convert_import(arg, reporter)?;
                    builder = builder.import(iri);
                }
                Some("Annotation") => {
                    builder = builder.annotation(self.convert_annotation(arg, reporter)?);
                }
                Some(_) => {
                    if let Some(axiom) = self.convert_axiom(arg, reporter)? {
                        builder = builder.axiom(axiom);
                    }
                }
                None => {}
            }
        }

        builder.build().map_err(|e| ReaderError::InvalidArgument {
            message: e.to_string(),
            span: node.span,
        })
    }

    fn convert_import(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Import, ReaderError> {
        let args = self.semantic_args_of(node);
        let iri = self.resolve_iri(
            args.first()
                .ok_or_else(|| reporter.argument_arity("Import", 0, 1.into(), node.span))?,
            reporter,
        )?;
        Ok(Import::from(iri))
    }

    // ── IRI resolution ─────────────────────────────────────────────────────────

    fn parse_iri(&self, s: &str, span: Span, reporter: &dyn Reporter) -> Result<Iri, ReaderError> {
        Iri::from_str(s).map_err(|e| reporter.invalid_iri(s, e.into(), span))
    }

    fn opt_str_to_namespace(
        &self,
        prefix: &Option<String>,
        span: Span,
        reporter: &dyn Reporter,
    ) -> Result<Namespace, ReaderError> {
        match prefix {
            None => Ok(Namespace::new_default()),
            Some(p) => Namespace::from_str(&format!("{}:", p))
                .map_err(|e| reporter.invalid_namespace(p, e.to_string(), span)),
        }
    }

    fn expand_iri_ref(
        &self,
        iri_ref: &IriRef,
        span: Span,
        reporter: &dyn Reporter,
    ) -> Result<Iri, ReaderError> {
        match iri_ref {
            IriRef::Full(s) => self.parse_iri(s, span, reporter),
            IriRef::Prefixed(PrefixedIriRef { prefix, local }) => {
                let ns = self.opt_str_to_namespace(prefix, span, reporter)?;
                let base = self.prefixes.get_namespace(&ns).ok_or_else(|| {
                    reporter.unknown_prefix(prefix.as_deref().unwrap_or(""), span)
                })?;
                self.parse_iri(&format!("{:#}{}", base, local), span, reporter)
            }
            IriRef::Namespace(opt_prefix) => {
                let ns = self.opt_str_to_namespace(opt_prefix, span, reporter)?;
                let iri = self.prefixes.get_namespace(&ns).ok_or_else(|| {
                    reporter.unknown_prefix(opt_prefix.as_deref().unwrap_or(""), span)
                })?;
                Ok(iri.clone())
            }
        }
    }

    fn resolve_iri(&self, node: &SyntaxNode, reporter: &dyn Reporter) -> Result<Iri, ReaderError> {
        match node.try_as_iri_ref() {
            Some(iri_ref) => self.expand_iri_ref(iri_ref, node.span, reporter),
            None => Err(reporter.unexpected_ast_node(node, &["IRI"])),
        }
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    fn semantic_args_of<'a>(&self, node: &'a SyntaxNode) -> Vec<&'a SyntaxNode> {
        node.function_args()
            .map(|args| {
                args.iter()
                    .filter(|n| !matches!(n.kind, SyntaxNodeKind::Comment(_)))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Splits leading `Annotation(...)` nodes from the rest.
    fn split_annotations<'a>(
        &self,
        args: &[&'a SyntaxNode],
        reporter: &dyn Reporter,
    ) -> Result<(Vec<Annotation>, Vec<&'a SyntaxNode>), ReaderError> {
        let mut annotations = Vec::new();
        let mut rest_start = 0;
        for (i, node) in args.iter().enumerate() {
            if node.function_name() == Some("Annotation") {
                annotations.push(self.convert_annotation(node, reporter)?);
                rest_start = i + 1;
            } else {
                break;
            }
        }
        Ok((annotations, args[rest_start..].to_vec()))
    }

    fn arg_err(
        &self,
        function: &str,
        got: usize,
        span: Span,
        reporter: &dyn Reporter,
    ) -> ReaderError {
        reporter.argument_arity(function, got, (1..).into(), span)
    }

    // ── Annotations ───────────────────────────────────────────────────────────

    fn convert_annotation(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Annotation, ReaderError> {
        let args = self.semantic_args_of(node);
        let (nested, rest) = self.split_annotations(&args, reporter)?;
        if rest.len() != 2 {
            return Err(reporter.argument_arity("Annotation", args.len(), 2.into(), node.span));
        }
        let prop_iri = self.resolve_iri(rest[0], reporter)?;
        let value = self.convert_annotation_value(rest[1], reporter)?;
        if nested.is_empty() {
            Ok(Annotation::new(prop_iri, value))
        } else {
            Ok(Annotation::new_with_annotations(nested, prop_iri, value))
        }
    }

    fn convert_annotation_value(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<AnnotationValue, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            return Ok(AnnotationValue::Iri(
                self.expand_iri_ref(iri_ref, node.span, reporter)?,
            ));
        }
        if let Some(ls) = node.as_literal_syntax() {
            return Ok(AnnotationValue::Literal(
                self.convert_literal(ls, reporter)?,
            ));
        }
        if let Some(id) = node.as_node_id() {
            let anon =
                AnonymousIndividual::from_str(id).map_err(|e| ReaderError::InvalidArgument {
                    message: e.to_string(),
                    span: node.span,
                })?;
            return Ok(AnnotationValue::AnonymousIndividual(anon));
        }
        Err(reporter.unexpected_ast_node(node, &["AnnotationValue"]))
    }

    // ── Literals ──────────────────────────────────────────────────────────────

    fn convert_literal(
        &self,
        ls: &LiteralSyntax,
        reporter: &dyn Reporter,
    ) -> Result<Literal, ReaderError> {
        match (&ls.lang_tag, &ls.datatype) {
            (Some(lang), _) => {
                // language-tagged: "text@lang"^^rdf:PlainLiteral (abbreviated form)
                Ok(Literal::plain(format!("{}@{}", ls.lexical_form, lang)))
            }
            (None, Some(dt_iri_ref)) => {
                let dt_iri = self.expand_iri_ref(dt_iri_ref, ls.span, reporter)?;
                Ok(Literal::new(ls.lexical_form.clone(), Datatype::new(dt_iri)))
            }
            (None, None) => Ok(Literal::plain(ls.lexical_form.clone())),
        }
    }

    // ── Individuals ───────────────────────────────────────────────────────────

    fn convert_individual(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Individual, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            let iri = self.expand_iri_ref(iri_ref, node.span, reporter)?;
            return Ok(Individual::NamedIndividual(NamedIndividual::new(iri)));
        }
        if let Some(id) = node.as_node_id() {
            let anon =
                AnonymousIndividual::from_str(id).map_err(|e| ReaderError::InvalidArgument {
                    message: e.to_string(),
                    span: node.span,
                })?;
            return Ok(Individual::AnonymousIndividual(anon));
        }
        Err(reporter.unexpected_ast_node(node, &["Individual"]))
    }

    // ── Property expressions ──────────────────────────────────────────────────

    fn convert_ope(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<ObjectPropertyExpression, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            let iri = self.expand_iri_ref(iri_ref, node.span, reporter)?;
            return Ok(ObjectPropertyExpression::ObjectProperty(
                ObjectProperty::new(iri),
            ));
        }
        if node.function_name() == Some("ObjectInverseOf") {
            let args = self.semantic_args_of(node);
            let iri = self.resolve_iri(
                args.first().ok_or_else(|| {
                    reporter.argument_arity("ObjectInverseOf", 0, 1.into(), node.span)
                })?,
                reporter,
            )?;
            return Ok(ObjectPropertyExpression::InverseObjectProperty(
                InverseOPExpr::new(ObjectProperty::new(iri)),
            ));
        }
        Err(reporter.unexpected_ast_node(node, &["ObjectPropertyExpression"]))
    }

    fn convert_dpe(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<DataPropertyExpression, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            let iri = self.expand_iri_ref(iri_ref, node.span, reporter)?;
            return Ok(DataPropertyExpression::DataProperty(DataProperty::new(iri)));
        }
        Err(reporter.unexpected_ast_node(node, &["DataPropertyExpression"]))
    }

    // ── Class expressions ─────────────────────────────────────────────────────

    fn convert_class_expression(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<ClassExpression, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            let iri = self.expand_iri_ref(iri_ref, node.span, reporter)?;
            return Ok(ClassExpression::Class(Class::new(iri)));
        }
        let name = node
            .function_name()
            .ok_or_else(|| reporter.unexpected_ast_node(node, &["ClassExpression"]))?;
        let args = self.semantic_args_of(node);
        let span = node.span;

        match name {
            "ObjectIntersectionOf" => {
                let ces = self.collect_class_expressions(&args, reporter)?;
                Ok(ClassExpression::ObjectIntersectionOf(
                    ObjectIntersectionOf::new(ces),
                ))
            }
            "ObjectUnionOf" => {
                let ces = self.collect_class_expressions(&args, reporter)?;
                Ok(ClassExpression::ObjectUnionOf(ObjectUnionOf::new(ces)))
            }
            "ObjectComplementOf" => {
                let ce = self.convert_class_expression(
                    args.first()
                        .ok_or_else(|| self.arg_err("ObjectComplementOf", 0, span, reporter))?,
                    reporter,
                )?;
                Ok(ClassExpression::ObjectComplementOf(
                    ObjectComplementOf::new(ce),
                ))
            }
            "ObjectOneOf" => {
                let inds: Result<Vec<_>, _> = args
                    .iter()
                    .map(|a| self.convert_individual(a, reporter))
                    .collect();
                Ok(ClassExpression::ObjectOneOf(ObjectOneOf::new(inds?)))
            }
            "ObjectSomeValuesFrom" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectSomeValuesFrom", args.len(), span, reporter));
                }
                let ope = self.convert_ope(args[0], reporter)?;
                let ce = self.convert_class_expression(args[1], reporter)?;
                Ok(ClassExpression::ObjectSomeValuesFrom(
                    ObjectSomeValuesFrom::new(ope, ce),
                ))
            }
            "ObjectAllValuesFrom" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectAllValuesFrom", args.len(), span, reporter));
                }
                let ope = self.convert_ope(args[0], reporter)?;
                let ce = self.convert_class_expression(args[1], reporter)?;
                Ok(ClassExpression::ObjectAllValuesFrom(
                    ObjectAllValuesFrom::new(ope, ce),
                ))
            }
            "ObjectHasValue" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectHasValue", args.len(), span, reporter));
                }
                let ope = self.convert_ope(args[0], reporter)?;
                let ind = self.convert_individual(args[1], reporter)?;
                Ok(ClassExpression::ObjectHasValue(ObjectHasValue::new(
                    ope, ind,
                )))
            }
            "ObjectHasSelf" => {
                let ope = self.convert_ope(
                    args.first()
                        .ok_or_else(|| self.arg_err("ObjectHasSelf", 0, span, reporter))?,
                    reporter,
                )?;
                Ok(ClassExpression::ObjectHasSelf(ObjectHasSelf::new(ope)))
            }
            "ObjectMinCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectMinCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let ope = self.convert_ope(args[1], reporter)?;
                let ce = if args.len() > 2 {
                    Some(self.convert_class_expression(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::ObjectMinCardinality(
                    ObjectMinCardinality::new(n, ope, ce),
                ))
            }
            "ObjectMaxCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectMaxCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let ope = self.convert_ope(args[1], reporter)?;
                let ce = if args.len() > 2 {
                    Some(self.convert_class_expression(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::ObjectMaxCardinality(
                    ObjectMaxCardinality::new(n, ope, ce),
                ))
            }
            "ObjectExactCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("ObjectExactCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let ope = self.convert_ope(args[1], reporter)?;
                let ce = if args.len() > 2 {
                    Some(self.convert_class_expression(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::ObjectExactCardinality(
                    ObjectExactCardinality::new(n, ope, ce),
                ))
            }
            "DataSomeValuesFrom" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataSomeValuesFrom", args.len(), span, reporter));
                }
                let dpes: Result<Vec<_>, _> = args[..args.len() - 1]
                    .iter()
                    .map(|a| self.convert_dpe(a, reporter))
                    .collect();
                let dr = self.convert_data_range(args[args.len() - 1], reporter)?;
                Ok(ClassExpression::DataSomeValuesFrom(
                    DataSomeValuesFrom::new(dpes?, dr),
                ))
            }
            "DataAllValuesFrom" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataAllValuesFrom", args.len(), span, reporter));
                }
                let dpes: Result<Vec<_>, _> = args[..args.len() - 1]
                    .iter()
                    .map(|a| self.convert_dpe(a, reporter))
                    .collect();
                let dr = self.convert_data_range(args[args.len() - 1], reporter)?;
                Ok(ClassExpression::DataAllValuesFrom(DataAllValuesFrom::new(
                    dpes?, dr,
                )))
            }
            "DataHasValue" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataHasValue", args.len(), span, reporter));
                }
                let dpe = self.convert_dpe(args[0], reporter)?;
                let ls = args[1]
                    .as_literal_syntax()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[1], &["Literal"]))?;
                Ok(ClassExpression::DataHasValue(DataHasValue::new(
                    dpe,
                    self.convert_literal(ls, reporter)?,
                )))
            }
            "DataMinCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataMinCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let dpe = self.convert_dpe(args[1], reporter)?;
                let dr = if args.len() > 2 {
                    Some(self.convert_data_range(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::DataMinCardinality(
                    DataMinCardinality::new(n, dpe, dr),
                ))
            }
            "DataMaxCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataMaxCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let dpe = self.convert_dpe(args[1], reporter)?;
                let dr = if args.len() > 2 {
                    Some(self.convert_data_range(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::DataMaxCardinality(
                    DataMaxCardinality::new(n, dpe, dr),
                ))
            }
            "DataExactCardinality" => {
                if args.len() < 2 {
                    return Err(self.arg_err("DataExactCardinality", args.len(), span, reporter));
                }
                let n = args[0]
                    .try_as_integer()
                    .ok_or_else(|| reporter.unexpected_ast_node(args[0], &["integer"]))?;
                let dpe = self.convert_dpe(args[1], reporter)?;
                let dr = if args.len() > 2 {
                    Some(self.convert_data_range(args[2], reporter)?)
                } else {
                    None
                };
                Ok(ClassExpression::DataExactCardinality(
                    DataExactCardinality::new(n, dpe, dr),
                ))
            }
            _ => Err(ReaderError::UnknownFunction {
                name: name.to_owned(),
                span,
            }),
        }
    }

    fn collect_class_expressions(
        &self,
        args: &[&SyntaxNode],
        reporter: &dyn Reporter,
    ) -> Result<Vec<ClassExpression>, ReaderError> {
        args.iter()
            .map(|a| self.convert_class_expression(a, reporter))
            .collect()
    }

    // ── Data ranges ───────────────────────────────────────────────────────────

    fn convert_data_range(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<DataRange, ReaderError> {
        if let Some(iri_ref) = node.try_as_iri_ref() {
            let iri = self.expand_iri_ref(iri_ref, node.span, reporter)?;
            return Ok(DataRange::Datatype(Datatype::new(iri)));
        }
        let name = node
            .function_name()
            .ok_or_else(|| ReaderError::UnexpectedToken {
                got: format!("{:?}", node.kind),
                expected: vec!["DataRange".to_string()],
                span: node.span,
            })?;
        let args = self.semantic_args_of(node);
        let span = node.span;

        match name {
            "DataIntersectionOf" => {
                let drs: Result<Vec<_>, _> = args
                    .iter()
                    .map(|a| self.convert_data_range(a, reporter))
                    .collect();
                DataIntersectionOf::new(drs?)
                    .map(DataRange::DataIntersectionOf)
                    .map_err(|e| ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    })
            }
            "DataUnionOf" => {
                let drs: Result<Vec<_>, _> = args
                    .iter()
                    .map(|a| self.convert_data_range(a, reporter))
                    .collect();
                DataUnionOf::new(drs?)
                    .map(DataRange::DataUnionOf)
                    .map_err(|e| ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    })
            }
            "DataComplementOf" => {
                let dr = self.convert_data_range(
                    args.first()
                        .ok_or_else(|| self.arg_err("DataComplementOf", 0, span, reporter))?,
                    reporter,
                )?;
                Ok(DataRange::DataComplementOf(DataComplementOf::new(dr)))
            }
            "DataOneOf" => {
                let lits: Result<Vec<_>, _> = args
                    .iter()
                    .map(|a| {
                        let ls =
                            a.as_literal_syntax()
                                .ok_or_else(|| ReaderError::UnexpectedToken {
                                    got: format!("{:?}", a.kind),
                                    expected: vec!["Literal".to_string()],
                                    span: a.span,
                                })?;
                        self.convert_literal(ls, reporter)
                    })
                    .collect();
                DataOneOf::new(lits?)
                    .map(DataRange::DataOneOf)
                    .map_err(|e| ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    })
            }
            "DatatypeRestriction" => {
                if args.is_empty() {
                    return Err(self.arg_err("DatatypeRestriction", 0, span, reporter));
                }
                let dt = Datatype::new(self.resolve_iri(args[0], reporter)?);
                let mut restrictions = Vec::new();
                let mut i = 1;
                while i + 1 < args.len() {
                    let facet_iri = self.resolve_iri(args[i], reporter)?;
                    let ls = args[i + 1].as_literal_syntax().ok_or_else(|| {
                        ReaderError::UnexpectedToken {
                            got: format!("{:?}", args[i + 1].kind),
                            expected: vec!["Literal".to_string()],
                            span: args[i + 1].span,
                        }
                    })?;
                    restrictions.push(FacetRestriction::new(
                        facet_iri,
                        self.convert_literal(ls, reporter)?,
                    ));
                    i += 2;
                }
                DatatypeRestriction::new(dt, restrictions)
                    .map(DataRange::DatatypeRestriction)
                    .map_err(|e| ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    })
            }
            _ => Err(reporter.unknown_function(node)),
        }
    }

    // ── Entities ──────────────────────────────────────────────────────────────

    fn convert_entity(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Entity, ReaderError> {
        let span = node.span;
        let name = node
            .function_name()
            .ok_or_else(|| ReaderError::UnexpectedToken {
                got: format!("{:?}", node.kind),
                expected: vec!["Entity".to_string()],
                span,
            })?;
        let name_owned = name.to_owned();
        let args = self.semantic_args_of(node);
        let iri = self.resolve_iri(
            args.first()
                .ok_or_else(|| self.arg_err(&name_owned, 0, span, reporter))?,
            reporter,
        )?;
        match name {
            "Class" => Ok(Entity::Class(Class::new(iri))),
            "Datatype" => Ok(Entity::Datatype(Datatype::new(iri))),
            "ObjectProperty" => Ok(Entity::ObjectProperty(ObjectProperty::new(iri))),
            "DataProperty" => Ok(Entity::DataProperty(DataProperty::new(iri))),
            "AnnotationProperty" => Ok(Entity::AnnotationProperty(AnnotationProperty::new(iri))),
            "NamedIndividual" => Ok(Entity::NamedIndividual(NamedIndividual::new(iri))),
            _ => Err(reporter.unexpected_function(
                node,
                &[
                    "Class",
                    "Datatype",
                    "ObjectProperty",
                    "DataProperty",
                    "AnnotationProperty",
                    "NamedIndividual",
                ],
            )),
        }
    }

    // ── Axioms ────────────────────────────────────────────────────────────────

    fn convert_axiom(
        &self,
        node: &SyntaxNode,
        reporter: &dyn Reporter,
    ) -> Result<Option<Axiom>, ReaderError> {
        let name = match node.function_name() {
            Some(n) => n,
            None => return Ok(None),
        };
        let args = self.semantic_args_of(node);
        let span = node.span;
        let (ann, rest) = self.split_annotations(&args, reporter)?;

        macro_rules! ope {
            ($idx:expr) => {
                self.convert_ope(
                    rest.get($idx)
                        .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?,
                    reporter,
                )?
            };
        }
        macro_rules! dpe {
            ($idx:expr) => {
                self.convert_dpe(
                    rest.get($idx)
                        .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?,
                    reporter,
                )?
            };
        }
        macro_rules! ce {
            ($idx:expr) => {
                self.convert_class_expression(
                    rest.get($idx)
                        .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?,
                    reporter,
                )?
            };
        }
        macro_rules! iri {
            ($idx:expr) => {
                self.resolve_iri(
                    rest.get($idx)
                        .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?,
                    reporter,
                )?
            };
        }
        macro_rules! ind {
            ($idx:expr) => {
                self.convert_individual(
                    rest.get($idx)
                        .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?,
                    reporter,
                )?
            };
        }
        macro_rules! lit {
            ($idx:expr) => {{
                let n = rest
                    .get($idx)
                    .ok_or_else(|| self.arg_err(name, rest.len(), span, reporter))?;
                let ls = n
                    .as_literal_syntax()
                    .ok_or_else(|| ReaderError::UnexpectedToken {
                        got: format!("{:?}", n.kind),
                        expected: vec!["Literal".to_string()],
                        span: n.span,
                    })?;
                self.convert_literal(ls, reporter)?
            }};
        }

        let axiom: Axiom = match name {
            // ── Declarations ─────────────────────────────────────────────────
            "Declaration" => {
                let entity = self.convert_entity(
                    rest.first()
                        .ok_or_else(|| self.arg_err("Declaration", 0, span, reporter))?,
                    reporter,
                )?;
                if ann.is_empty() {
                    Declaration::new(entity).into()
                } else {
                    Declaration::new_with_annotations(ann, entity).into()
                }
            }

            // ── Class axioms ──────────────────────────────────────────────────
            "SubClassOf" => {
                let sub = ce!(0);
                let sup = ce!(1);
                ClassAxiom::from(if ann.is_empty() {
                    SubClassOf::new(sub, sup)
                } else {
                    SubClassOf::new_with_annotations(ann, sub, sup)
                })
                .into()
            }
            "EquivalentClasses" => {
                let ces = self.collect_class_expressions(&rest, reporter)?;
                ClassAxiom::from(EquivalentClass::new(ces).map_err(|e| {
                    ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    }
                })?)
                .into()
            }
            "DisjointClasses" => {
                let ces = self.collect_class_expressions(&rest, reporter)?;
                ClassAxiom::from(DisjointClasses::new(ces).map_err(|e| {
                    ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    }
                })?)
                .into()
            }
            "DisjointUnion" => {
                let class_iri = iri!(0);
                let class = Class::new(class_iri);
                let disjoint: Result<Vec<_>, _> = rest[1..]
                    .iter()
                    .map(|a| self.convert_class_expression(a, reporter))
                    .collect();
                ClassAxiom::from(DisjointUnion::new(class, disjoint?).map_err(|e| {
                    ReaderError::InvalidArgument {
                        message: e.to_string(),
                        span,
                    }
                })?)
                .into()
            }

            // ── Object property axioms ────────────────────────────────────────
            "SubObjectPropertyOf" => {
                // sub can be ObjectPropertyExpression or ObjectPropertyChain(...)
                let sub_node = rest
                    .first()
                    .ok_or_else(|| self.arg_err("SubObjectPropertyOf", 0, span, reporter))?;
                let sub = if sub_node.function_name() == Some("ObjectPropertyChain") {
                    let chain_args = self.semantic_args_of(sub_node);
                    let opes: Result<Vec<_>, _> = chain_args
                        .iter()
                        .map(|a| self.convert_ope(a, reporter))
                        .collect();
                    SubObjectPropertyExpression::PropertyExpressionChain(
                        PropertyExpressionChain::new(opes?),
                    )
                } else {
                    SubObjectPropertyExpression::ObjectPropertyExpression(
                        self.convert_ope(sub_node, reporter)?,
                    )
                };
                let sup = ope!(1);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    SubObjectPropertyOf::new(sub, sup)
                } else {
                    SubObjectPropertyOf::new_with_annotations(ann, sub, sup)
                })
                .into()
            }
            "EquivalentObjectProperties" => {
                let opes: Result<Vec<_>, _> =
                    rest.iter().map(|a| self.convert_ope(a, reporter)).collect();
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    EquivalentObjectProperties::new(opes?)
                } else {
                    EquivalentObjectProperties::new_with_annotations(ann, opes?)
                })
                .into()
            }
            "DisjointObjectProperties" => {
                let opes: Result<Vec<_>, _> =
                    rest.iter().map(|a| self.convert_ope(a, reporter)).collect();
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    DisjointObjectProperties::new(opes?)
                } else {
                    DisjointObjectProperties::new_with_annotations(ann, opes?)
                })
                .into()
            }
            "InverseObjectProperties" => {
                let ope1 = ope!(0);
                let ope2 = ope!(1);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    InverseObjectProperties::new(ope1, ope2)
                } else {
                    InverseObjectProperties::new_with_annotations(ann, ope1, ope2)
                })
                .into()
            }
            "ObjectPropertyDomain" => {
                let ope = ope!(0);
                let domain = ce!(1);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    ObjectPropertyDomain::new(ope, domain)
                } else {
                    ObjectPropertyDomain::new_with_annotations(ann, ope, domain)
                })
                .into()
            }
            "ObjectPropertyRange" => {
                let ope = ope!(0);
                let range = ce!(1);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    ObjectPropertyRange::new(ope, range)
                } else {
                    ObjectPropertyRange::new_with_annotations(ann, ope, range)
                })
                .into()
            }
            "FunctionalObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    FunctionalObjectProperty::new(ope)
                } else {
                    FunctionalObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "InverseFunctionalObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    InverseFunctionalObjectProperty::new(ope)
                } else {
                    InverseFunctionalObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "ReflexiveObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    ReflexiveObjectProperty::new(ope)
                } else {
                    ReflexiveObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "IrreflexiveObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    IrreflexiveObjectProperty::new(ope)
                } else {
                    IrreflexiveObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "SymmetricObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    SymmetricObjectProperty::new(ope)
                } else {
                    SymmetricObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "AsymmetricObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    AsymmetricObjectProperty::new(ope)
                } else {
                    AsymmetricObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }
            "TransitiveObjectProperty" => {
                let ope = ope!(0);
                ObjectPropertyAxiom::from(if ann.is_empty() {
                    TransitiveObjectProperty::new(ope)
                } else {
                    TransitiveObjectProperty::new_with_annotations(ann, ope)
                })
                .into()
            }

            // ── Data property axioms ──────────────────────────────────────────
            "SubDataPropertyOf" => {
                let sub = dpe!(0);
                let sup = dpe!(1);
                DataPropertyAxiom::from(if ann.is_empty() {
                    SubDataPropertyOf::new(sub, sup)
                } else {
                    SubDataPropertyOf::new_with_annotations(ann, sub, sup)
                })
                .into()
            }
            "EquivalentDataProperties" => {
                let dpes: Result<Vec<_>, _> =
                    rest.iter().map(|a| self.convert_dpe(a, reporter)).collect();
                DataPropertyAxiom::from(if ann.is_empty() {
                    EquivalentDataProperties::new(dpes?)
                } else {
                    EquivalentDataProperties::new_with_annotations(ann, dpes?)
                })
                .into()
            }
            "DisjointDataProperties" => {
                let dpes: Result<Vec<_>, _> =
                    rest.iter().map(|a| self.convert_dpe(a, reporter)).collect();
                DataPropertyAxiom::from(if ann.is_empty() {
                    DisjointDataProperties::new(dpes?)
                } else {
                    DisjointDataProperties::new_with_annotations(ann, dpes?)
                })
                .into()
            }
            "DataPropertyDomain" => {
                let dpe = dpe!(0);
                let domain = ce!(1);
                DataPropertyAxiom::from(if ann.is_empty() {
                    DataPropertyDomain::new(dpe, domain)
                } else {
                    DataPropertyDomain::new_with_annotations(ann, dpe, domain)
                })
                .into()
            }
            "DataPropertyRange" => {
                let dpe = dpe!(0);
                let dr = self.convert_data_range(
                    rest.get(1).ok_or_else(|| {
                        self.arg_err("DataPropertyRange", rest.len(), span, reporter)
                    })?,
                    reporter,
                )?;
                DataPropertyAxiom::from(if ann.is_empty() {
                    DataPropertyRange::new(dpe, dr)
                } else {
                    DataPropertyRange::new_with_annotations(ann, dpe, dr)
                })
                .into()
            }
            "FunctionalDataProperty" => {
                let dpe = dpe!(0);
                DataPropertyAxiom::from(if ann.is_empty() {
                    FunctionalDataProperty::new(dpe)
                } else {
                    FunctionalDataProperty::new_with_annotations(ann, dpe)
                })
                .into()
            }

            // ── Datatype definition ───────────────────────────────────────────
            "DatatypeDefinition" => {
                let dt = Datatype::new(iri!(0));
                let dr = self.convert_data_range(
                    rest.get(1).ok_or_else(|| {
                        self.arg_err("DatatypeDefinition", rest.len(), span, reporter)
                    })?,
                    reporter,
                )?;
                if ann.is_empty() {
                    DatatypeDefinition::new(dt, dr).into()
                } else {
                    DatatypeDefinition::new_with_annotations(ann, dt, dr).into()
                }
            }

            // ── Assertions ────────────────────────────────────────────────────
            "SameIndividual" => {
                let inds: Result<Vec<_>, _> = rest
                    .iter()
                    .map(|a| self.convert_individual(a, reporter))
                    .collect();
                Assertion::from(if ann.is_empty() {
                    SameIndividual::new(inds?)
                } else {
                    SameIndividual::new_with_annotations(ann, inds?)
                })
                .into()
            }
            "DifferentIndividuals" => {
                let inds: Result<Vec<_>, _> = rest
                    .iter()
                    .map(|a| self.convert_individual(a, reporter))
                    .collect();
                Assertion::from(if ann.is_empty() {
                    DifferentIndividuals::new(inds?)
                } else {
                    DifferentIndividuals::new_with_annotations(ann, inds?)
                })
                .into()
            }
            "ClassAssertion" => {
                let class_expr = ce!(0);
                let individual = ind!(1);
                Assertion::from(if ann.is_empty() {
                    ClassAssertion::new(class_expr, individual)
                } else {
                    ClassAssertion::new_with_annotations(ann, class_expr, individual)
                })
                .into()
            }
            "ObjectPropertyAssertion" => {
                let ope = ope!(0);
                let src = ind!(1);
                let tgt = ind!(2);
                Assertion::from(if ann.is_empty() {
                    ObjectPropertyAssertion::new(ope, src, tgt)
                } else {
                    ObjectPropertyAssertion::new_with_annotations(ann, ope, src, tgt)
                })
                .into()
            }
            "NegativeObjectPropertyAssertion" => {
                let ope = ope!(0);
                let src = ind!(1);
                let tgt = ind!(2);
                Assertion::from(if ann.is_empty() {
                    NegativeObjectPropertyAssertion::new(ope, src, tgt)
                } else {
                    NegativeObjectPropertyAssertion::new_with_annotations(ann, ope, src, tgt)
                })
                .into()
            }
            "DataPropertyAssertion" => {
                let dpe = dpe!(0);
                let src = ind!(1);
                let val = lit!(2);
                Assertion::from(if ann.is_empty() {
                    DataPropertyAssertion::new(dpe, src, val)
                } else {
                    DataPropertyAssertion::new_with_annotations(ann, dpe, src, val)
                })
                .into()
            }
            "NegativeDataPropertyAssertion" => {
                let dpe = dpe!(0);
                let src = ind!(1);
                let val = lit!(2);
                Assertion::from(if ann.is_empty() {
                    NegativeDataPropertyAssertion::new(dpe, src, val)
                } else {
                    NegativeDataPropertyAssertion::new_with_annotations(ann, dpe, src, val)
                })
                .into()
            }

            // ── Annotation axioms ─────────────────────────────────────────────
            "AnnotationAssertion" => {
                let prop_iri = iri!(0);
                let ap = AnnotationProperty::new(prop_iri);
                let subject_node = rest.get(1).ok_or_else(|| {
                    self.arg_err("AnnotationAssertion", rest.len(), span, reporter)
                })?;
                let subject = if let Some(iri_ref) = subject_node.try_as_iri_ref() {
                    AnnotationSubject::Iri(self.expand_iri_ref(
                        iri_ref,
                        subject_node.span,
                        reporter,
                    )?)
                } else if let Some(id) = subject_node.as_node_id() {
                    AnnotationSubject::AnonymousIndividual(
                        AnonymousIndividual::from_str(id).map_err(|e| {
                            ReaderError::InvalidArgument {
                                message: e.to_string(),
                                span,
                            }
                        })?,
                    )
                } else {
                    return Err(ReaderError::UnexpectedToken {
                        got: format!("{:?}", subject_node.kind),
                        expected: vec!["AnnotationSubject".to_string()],
                        span: subject_node.span,
                    });
                };
                let value = self.convert_annotation_value(
                    rest.get(2).ok_or_else(|| {
                        self.arg_err("AnnotationAssertion", rest.len(), span, reporter)
                    })?,
                    reporter,
                )?;
                AnnotationAxiom::from(if ann.is_empty() {
                    AnnotationAssertion::new(ap, subject, value)
                } else {
                    AnnotationAssertion::new_with_annotations(ann, ap, subject, value)
                })
                .into()
            }
            "SubAnnotationPropertyOf" => {
                let sub = AnnotationProperty::new(iri!(0));
                let sup = AnnotationProperty::new(iri!(1));
                AnnotationAxiom::from(if ann.is_empty() {
                    SubAnnotationOf::new(sub, sup)
                } else {
                    SubAnnotationOf::new_with_annotations(ann, sub, sup)
                })
                .into()
            }
            "AnnotationPropertyDomain" => {
                let ap = AnnotationProperty::new(iri!(0));
                let domain = iri!(1);
                AnnotationAxiom::from(if ann.is_empty() {
                    AnnotationPropertyDomain::new(ap, domain)
                } else {
                    AnnotationPropertyDomain::new_with_annotations(ann, ap, domain)
                })
                .into()
            }
            "AnnotationPropertyRange" => {
                let ap = AnnotationProperty::new(iri!(0));
                let range = iri!(1);
                AnnotationAxiom::from(if ann.is_empty() {
                    AnnotationPropertyRange::new(ap, range)
                } else {
                    AnnotationPropertyRange::new_with_annotations(ann, ap, range)
                })
                .into()
            }

            // ── HasKey ────────────────────────────────────────────────────────
            "HasKey" => {
                let ce = ce!(0);
                let opes = if let Some(grp) = rest.get(1) {
                    if grp.function_name() == Some("") {
                        let grp_args = self.semantic_args_of(grp);
                        grp_args
                            .iter()
                            .map(|a| self.convert_ope(a, reporter))
                            .collect::<Result<Vec<_>, _>>()?
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                let dpes = if let Some(grp) = rest.get(2) {
                    if grp.function_name() == Some("") {
                        let grp_args = self.semantic_args_of(grp);
                        grp_args
                            .iter()
                            .map(|a| self.convert_dpe(a, reporter))
                            .collect::<Result<Vec<_>, _>>()?
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };
                if ann.is_empty() {
                    HasKey::new(ce, opes, dpes).into()
                } else {
                    HasKey::new_with_annotations(ann, ce, opes, dpes).into()
                }
            }

            // Unknown function: skip silently
            _ => return Ok(None),
        };

        Ok(Some(axiom))
    }
}
