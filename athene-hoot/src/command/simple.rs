use crate::{OnceCommand, error::CliError};
use athene_owlapi::{
    Ontology,
    annotations::HasAnnotations,
    axioms::{
        AnnotationAxiom, ClassAxiom, DataPropertyAxiom, DatatypeDefinition, Declaration, HasKey,
        ObjectPropertyAxiom,
    },
    entities::Entity,
    reader,
    syntax::{
        FN_ANNOTATION, FN_ANNOTATION_PROPERTY, FN_CLASS, FN_DATA_PROPERTY, FN_DATATYPE,
        FN_DECLARATION, FN_IMPORT, FN_NAMED_INDIVIDUAL, FN_OBJECT_PROPERTY,
    },
};
use rdftk_iri::Iri;
use std::{collections::HashSet, path::PathBuf, process::ExitCode};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct CheckFilesCommand {
    #[allow(dead_code)]
    files: Vec<PathBuf>,
    #[allow(dead_code)]
    import_depth: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct OntologyStatisticsCommand {
    file: Option<PathBuf>,
    #[allow(dead_code)]
    import_depth: u32,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for CheckFilesCommand {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let mut files_with_errors = 0;
        for file in &self.files {
            match reader::parse_file(file, true) {
                Ok(_) => {}
                Err(_) => files_with_errors += 1,
            }
        }
        if files_with_errors == 0 {
            println!("Congratulations, none of your OWL ontologies have (known) issues.");
            Ok(ExitCode::SUCCESS)
        } else {
            eprintln!(
                "Sorry but {files_with_errors} of {} files have issues that need addressing.",
                self.files.len()
            );
            Ok(ExitCode::FAILURE)
        }
    }
}

impl CheckFilesCommand {
    pub(crate) fn new(files: Vec<PathBuf>, import_depth: u32) -> Self {
        Self {
            files,
            import_depth,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl OnceCommand for OntologyStatisticsCommand {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let document =
            match reader::parse_file(self.file.as_ref().expect("need a file for now"), true) {
                Ok(document) => document,
                Err(_) => return Ok(ExitCode::FAILURE),
            };
        println!("Document");
        println!(
            "└─ Number of `Prefix` declarations: {}",
            document.prefix_mapping_count()
        );
        println!(
            "   └─ Number user-defined: {}",
            document.user_prefix_mapping_count()
        );
        let ontology = document.ontology();
        println!("Ontology");
        if let Some(ontology_iri) = ontology.ontology_iri() {
            println!("├─ IRI: {ontology_iri}");
        }
        if let Some(version_iri) = ontology.version_iri() {
            println!("├─ Version IRI: {version_iri}");
        }
        if ontology.has_direct_imports() {
            let distinct_imports: HashSet<&Iri> =
                ontology.direct_imports().map(|di| di.iri()).collect();
            println!(
                "├─ Number of direct `{}`: {}",
                FN_IMPORT,
                ontology.direct_import_count()
            );
            println!("   └─ Number distinct: {}", distinct_imports.len());
        }
        if ontology.has_annotations() {
            self.annotated_stats("├─", "   └─", ontology)?;
        }
        if ontology.has_axioms() {
            self.axiom_stats("├─", "   └─", ontology)?;
        }
        Ok(ExitCode::SUCCESS)
    }
}

impl OntologyStatisticsCommand {
    pub(crate) fn new(file: Option<PathBuf>, import_depth: u32) -> Self {
        Self { file, import_depth }
    }

    fn annotated_stats(
        &self,
        prefix_1: &str,
        prefix_2: &str,
        annotated: &impl HasAnnotations,
    ) -> Result<ExitCode, CliError> {
        let distinct_properties: HashSet<&Iri> =
            annotated.annotations().map(|ann| ann.property()).collect();
        println!(
            "{prefix_1} Number of `{}`s: {}",
            FN_ANNOTATION,
            annotated.annotation_count()
        );
        println!(
            "{prefix_2} Number of distinct `{}`s: {}",
            FN_ANNOTATION_PROPERTY,
            distinct_properties.len()
        );
        Ok(ExitCode::SUCCESS)
    }

    fn axiom_stats(
        &self,
        prefix_1: &str,
        prefix_2: &str,
        ontology: &Ontology,
    ) -> Result<ExitCode, CliError> {
        let declarations: Vec<&Declaration> = ontology.declarations().collect();
        if !declarations.is_empty() {
            println!(
                "{prefix_1} Number of entity `{}`s: {}",
                FN_DECLARATION,
                declarations.len()
            );
            let counts = (0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32);
            let counts = declarations
                .iter()
                .map(|decl| decl.entity())
                .fold(counts, |c, entity| match entity {
                    Entity::AnnotationProperty(_) => (c.0 + 1, c.1, c.2, c.3, c.4, c.5),
                    Entity::Class(_) => (c.0, c.1 + 1, c.2, c.3, c.4, c.5),
                    Entity::DataProperty(_) => (c.0, c.1, c.2 + 1, c.3, c.4, c.5),
                    Entity::Datatype(_) => (c.0, c.1, c.2, c.3 + 1, c.4, c.5),
                    Entity::ObjectProperty(_) => (c.0, c.1, c.2, c.3, c.4 + 1, c.5),
                    Entity::NamedIndividual(_) => (c.0, c.1, c.2, c.3, c.4, c.5 + 1),
                });
            let counts = vec![counts.0, counts.1, counts.2, counts.3, counts.4, counts.5];
            const LABELS: &[&str] = &[
                FN_ANNOTATION_PROPERTY,
                FN_CLASS,
                FN_DATA_PROPERTY,
                FN_DATATYPE,
                FN_OBJECT_PROPERTY,
                FN_NAMED_INDIVIDUAL,
            ];

            for i in 0..=5 {
                if counts[i] > 0 {
                    println!("{prefix_2} Number of `{}`s: {}", LABELS[i], counts[i]);
                }
            }
        }
        let classes: Vec<&ClassAxiom> = ontology.classes().collect();
        if !classes.is_empty() {}
        let object_properties: Vec<&ObjectPropertyAxiom> = ontology.object_properties().collect();
        if !object_properties.is_empty() {}
        let data_properties: Vec<&DataPropertyAxiom> = ontology.data_properties().collect();
        if !data_properties.is_empty() {}
        let datatypes: Vec<&DatatypeDefinition> = ontology.datatypes().collect();
        if !datatypes.is_empty() {}
        let has_keys: Vec<&HasKey> = ontology.has_keys().collect();
        if !has_keys.is_empty() {}
        let annotation_axioms: Vec<&AnnotationAxiom> = ontology.annotation_axioms().collect();
        if !annotation_axioms.is_empty() {}
        Ok(ExitCode::SUCCESS)
    }
}
