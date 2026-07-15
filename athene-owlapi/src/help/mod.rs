use rdftk_iri::Name;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct ItemHelp {
    #[serde(skip, default)]
    function_name: String,
    section: String,
    description: String,
    bnf: Option<String>,
    long_description: Option<String>,
    #[serde(default)]
    see_also: Vec<Name>,
}

#[derive(Debug, Deserialize)]
pub struct HelpContent {
    functions: HashMap<Name, ItemHelp>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_help_content() -> HelpContent {
    let mut content: HelpContent = toml::from_str(FUNCTION_HELP_CONTENT).unwrap();
    for (function_name, help) in content.functions.iter_mut() {
        help.function_name = function_name.to_string();
    }
    content
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ HelpContent
// ------------------------------------------------------------------------------------------------

impl HelpContent {
    pub fn help_for_function(&self, function_name: &Name) -> Option<&ItemHelp> {
        self.functions.get(function_name)
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❯ ItemHelp
// ------------------------------------------------------------------------------------------------

const FUNCTION_HELP_CONTENT: &str = include_str!("functions.toml");

impl Display for ItemHelp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        const HASH: char = '#';

        write!(
            f,
            "{HASH} Function {} (Section §{})

**Summary.** {}
",
            self.function_name,
            self.section_in_specification(),
            self.description()
        )?;

        if let Some(bnf) = self.syntax_in_bnf() {
            write!(
                f,
                "
{HASH}{HASH} Syntax

```bnf
{bnf}
```
"
            )?;
        }

        if let Some(long_description) = self.long_description() {
            write!(
                f,
                "
{HASH}{HASH} Description

{long_description}
"
            )?;
        }

        if self.has_cross_references() {
            write!(
                f,
                "
{HASH}{HASH} See Also

{}
",
                self.see_also()
                    .map(|s| format!("**{s}**"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        write!(f, "
{HASH}{HASH} Source

Content from [OWL 2 Web Ontology Language Structural Specification and Functional-Style Syntax (Second Edition)](https://www.w3.org/TR/owl2-syntax/).

Copyright © 2012 W3C® (MIT, ERCIM, Keio), All Rights Reserved. W3C liability, trademark and document use rules apply.
"
            )
    }
}

impl ItemHelp {
    pub fn section_in_specification(&self) -> &str {
        &self.section
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn syntax_in_bnf(&self) -> Option<&str> {
        self.bnf.as_deref()
    }

    pub fn long_description(&self) -> Option<&str> {
        self.long_description.as_deref()
    }

    pub fn see_also(&self) -> impl Iterator<Item = &Name> {
        self.see_also.iter()
    }

    pub fn has_cross_references(&self) -> bool {
        !self.see_also.is_empty()
    }
}
