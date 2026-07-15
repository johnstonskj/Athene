use crate::{
    COMMAND_NAME, OnceCommand, OnceCommandWith,
    command::{
        completions::GenerateCompletions,
        external::RunExternalSubCommand,
        help::{FunctionalSyntaxHelp, HelpSubject},
        simple::{CheckFilesCommand, OntologyStatisticsCommand},
    },
    error::CliError,
};
use clap::{ArgAction, Args, Parser, Subcommand};
use clap_complete::Shell;
use colorchoice::ColorChoice;
use colorchoice_clap::Color;
use rdftk_iri::Name;
use std::{fs::File, io::BufReader, path::PathBuf, process::ExitCode};
use termimad::MadSkin;
use tracing::{instrument, level_filters::LevelFilter, trace};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Parser)]
#[command(name = COMMAND_NAME)]
#[command(about = "Hoot: the quick tool for OWL(s)", long_about = None)]
pub(crate) struct Cli {
    /// Increase logging verbosity by one level per occurance.
    #[arg(
        long,
        short = 'v',
        action = ArgAction::Count,
        global = true,
    )]
    verbose: u8,

    /// Decrease logging verbosity by one level per occurance.
    #[arg(
        long,
        action = ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    quiet: u8,

    #[command(flatten)]
    color: Color,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Check one or more OWL documents for syntax correctness.
    Check(CmdCheckFiles),

    /// Compute statistics for the Ontology in an OWL document.
    Statistics(CmdOntologyStats),

    /// Describe an OWL function.
    Describe(CmdFunctionHelp),

    /// Generate command completions for a number of shells.
    Completions {
        /// Shell to generate completions for (defaults to $SHELL).
        shell: Option<Shell>,
    },

    #[command(external_subcommand)]
    External(#[arg(num_args = 1..)] Vec<String>),
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub(crate) struct CmdCheckFiles {
    /// One or more documents to check.
    documents: Vec<PathBuf>,

    /// If non-zero, process imported ontologies transitively to this depth.
    #[arg(short = 'd', default_value_t = 0)]
    import_depth: u32,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub(crate) struct CmdOntologyStats {
    /// The documents to load.
    document: PathBuf,

    /// If non-zero, process imported ontologies transitively to this depth.
    #[arg(short = 'd', default_value_t = 0)]
    import_depth: u32,
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub(crate) struct CmdFunctionHelp {
    /// If true this will display the raw markdown without using the built-in renderer.
    #[arg(long, default_value_t = false)]
    no_render: bool,

    /// Name of a JSON file containing a termimad serialized skin file.
    #[arg(long, short = 's', env = "TERMIMAD_SKIN_FILE")]
    skin_file: Option<PathBuf>,

    /// The name of the function to describe.
    function_name: Name,
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct Globals {
    verbose_arg_count: u8,
    quiet_arg_count: u8,
    color_choice: ColorChoice,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl OnceCommand for Cli {
    type Output = ExitCode;
    type Error = CliError;

    #[instrument(name = "cli")]
    fn execute(self) -> Result<Self::Output, Self::Error> {
        trace!("Setting color globals to `{:?}`", self.color.as_choice());
        self.color.write_global();
        match self.color.as_choice() {
            ColorChoice::Always | ColorChoice::AlwaysAnsi => colored::control::set_override(true),
            ColorChoice::Never => colored::control::set_override(false),
            ColorChoice::Auto => {}
        }
        self.command.execute_with(Globals {
            verbose_arg_count: self.verbose,
            quiet_arg_count: self.quiet,
            color_choice: self.color.as_choice(),
        })
    }
}

impl Cli {
    pub(crate) fn max_level_filter(&self) -> LevelFilter {
        const DEFAULT_LEVEL_ERROR: u8 = 1;
        let offset = self.verbose as i16 - self.quiet as i16;
        match i16::from(DEFAULT_LEVEL_ERROR).saturating_add(offset) {
            i16::MIN..=0 => LevelFilter::OFF,
            1 => LevelFilter::ERROR,
            2 => LevelFilter::WARN,
            3 => LevelFilter::INFO,
            4 => LevelFilter::DEBUG,
            5..=i16::MAX => LevelFilter::TRACE,
        }
    }
}

impl OnceCommandWith for Commands {
    type Output = ExitCode;
    type Error = CliError;
    type Value = Globals;

    fn execute_with(self, globals: Globals) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Check(cmds) => cmds.execute(),
            Self::Statistics(cmds) => cmds.execute(),
            Self::Describe(cmds) => cmds.execute(),
            // One-shot commands.
            Self::Completions { shell } => GenerateCompletions::new(shell).execute(),
            Self::External(args) => RunExternalSubCommand::new(
                args[0].clone(),
                if args.len() > 1 { &args[1..] } else { &[] },
            )
            .with_verbosity_level(globals.verbose_arg_count)
            .with_quietness_level(globals.quiet_arg_count)
            .with_color_choice(globals.color_choice)
            .execute(),
        }
    }
}

impl OnceCommand for CmdCheckFiles {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        CheckFilesCommand::new(self.documents, self.import_depth).execute()
    }
}

impl OnceCommand for CmdOntologyStats {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        OntologyStatisticsCommand::new(Some(self.document), self.import_depth).execute()
    }
}

impl OnceCommand for CmdFunctionHelp {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let skin: MadSkin = if let Some(skin_file) = self.skin_file {
            let file = File::open(skin_file)?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)?
        } else if terminal_light::luma().map_or(false, |luma| luma > 0.6) {
            MadSkin::default_light()
        } else {
            MadSkin::default_dark()
        };

        FunctionalSyntaxHelp::from(HelpSubject::Function(self.function_name))
            .no_render(self.no_render)
            .skin(skin)
            .execute()
    }
}
