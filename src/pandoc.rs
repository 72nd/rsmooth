/// This module contains all functions to call pandoc and handle any errors occurring mine while.
use std::env;

/// Default name of the pandoc executable. Will be used when no other name is defined via the
/// `PANDOC_ENV` constant of this module.
const PANDOC_CMD: &str = "pandoc";

/// Name of the environment variable which will be used to determine the name of the pandoc
/// executable.
const PANDOC_ENV: &str = "PANDOC_CMD";

/// Handles errors in correspondence with the pandoc command.
pub struct PandocError {
    kind: PandocErrorKind,
}

/// Defines the different kinds of pandoc errors.
pub enum PandocErrorKind {
    /// The pandoc executable wasn't found on the system.
    NotFound,
}

/// Wrapper for calling pandoc. Exposes all needed functionality via it's method.
pub struct Pandoc<'a> {
    /// Name of the pandoc executable.
    executable: &'a str,
}

impl<'a> Pandoc<'a> {
    /// Returns a new instance of the Pandoc struct. Will use the `PANDOC_CMD` environment variable
    /// to determine the name of the pandoc executable. If the variable isn't set the constant
    /// PANDOC_CMD will be used.
    pub fn new() -> Self {
        Self {
            executable: match env::var(PANDOC_ENV) {
                Ok(x) => x,
                Err(_) => PANDOC_CMD,
            },
        }
    }
}

