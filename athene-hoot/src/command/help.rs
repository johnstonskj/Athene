use std::process::ExitCode;

use athene_owlapi::help::get_help_content;
use rdftk_iri::Name;
use termimad::MadSkin;

use crate::{OnceCommand, error::CliError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct FunctionalSyntaxHelp {
    subject: HelpSubject,
    no_render: bool,
    skin: MadSkin,
}

#[derive(Clone, Debug)]
pub(crate) enum HelpSubject {
    Function(Name),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<HelpSubject> for FunctionalSyntaxHelp {
    fn from(subject: HelpSubject) -> Self {
        Self {
            subject,
            no_render: false,
            skin: MadSkin::default_light(),
        }
    }
}

impl OnceCommand for FunctionalSyntaxHelp {
    type Output = ExitCode;
    type Error = CliError;

    fn execute(self) -> Result<Self::Output, Self::Error> {
        let HelpSubject::Function(function_name) = &self.subject;
        let help_content = get_help_content();
        if let Some(help) = help_content.help_for_function(function_name) {
            let markdown = help.to_string();
            if self.no_render {
                println!("{markdown}");
            } else {
                self.skin.print_text(&markdown);
            }
            Ok(ExitCode::SUCCESS)
        } else {
            eprintln!("No help found for function '{}'", function_name);
            Ok(ExitCode::FAILURE)
        }
    }
}

impl FunctionalSyntaxHelp {
    pub(crate) fn no_render(mut self, no_render: bool) -> Self {
        self.no_render = no_render;
        self
    }

    pub(crate) fn skin(mut self, skin: MadSkin) -> Self {
        self.skin = skin;
        self
    }
}
