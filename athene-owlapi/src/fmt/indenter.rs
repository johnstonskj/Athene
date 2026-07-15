use core::cell::RefCell;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Indenter {
    indent_step: usize,
    current_indentation: RefCell<usize>,
}

const DEFAULT_INDENTATION: usize = 4;
const INDENTATION_CHAR: char = ' ';

// ------------------------------------------------------------------------------------------------
// Implementations ❯ Indenter
// ------------------------------------------------------------------------------------------------

impl Default for Indenter {
    fn default() -> Self {
        Self::new(DEFAULT_INDENTATION)
    }
}

impl Indenter {
    pub fn new(indent_step: usize) -> Self {
        Self::new_at(indent_step, 0)
    }

    pub fn new_at(indent_step: usize, start_at: usize) -> Self {
        Self {
            indent_step,
            current_indentation: RefCell::new(start_at),
        }
    }

    pub fn indent(&self) {
        self.indent_by(self.indent_step);
    }

    pub fn indent_by(&self, step: usize) {
        let current = *self.current_indentation.borrow();
        self.current_indentation.replace(current + step);
    }

    pub fn outdent(&self) {
        self.outdent_by(self.indent_step);
    }

    pub fn outdent_by(&self, step: usize) {
        let current = *self.current_indentation.borrow();
        self.current_indentation.replace(current - step);
    }

    pub fn indent_prefix_string(&self) -> String {
        self.indent_prefix_string_for(*self.current_indentation.borrow())
    }

    pub fn separator_string(&self, alternate: bool) -> String {
        if alternate {
            format!("\n{}", self.indent_prefix_string())
        } else {
            " ".to_string()
        }
    }

    pub fn indent_prefix_string_for(&self, indentation: usize) -> String {
        core::iter::repeat_n(INDENTATION_CHAR, indentation).collect()
    }
}
