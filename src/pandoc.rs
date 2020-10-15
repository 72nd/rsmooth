/// This module contains all functions to call pandoc and handle any errors occurring mine while.
use std::env;
use std::ffi::OsStr;
use std::process::Command;

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
pub struct Pandoc {
    /// Name of the pandoc executable.
    executable: String,
}

impl Pandoc {
    /// Returns a new instance of the Pandoc struct. Will use the `PANDOC_CMD` environment variable
    /// to determine the name of the pandoc executable. If the variable isn't set the constant
    /// PANDOC_CMD will be used.
    pub fn new() -> Self {
        let env = match env::var(PANDOC_ENV) {
            Ok(x) => x,
            Err(_) => String::from(PANDOC_CMD),
        };
        Self { executable: env }
    }

    /// Internal function to call pandoc itself. Returns an PandocError if anything went wrong. To
    /// ensure consistent error handling all calls to pandoc should be handle with this method.
    fn run<I, S>(&self, args: I) -> Result<(), PandocError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        match Command::new(self.executable.clone()).args(args).spawn() {
            // Ok(_) => Ok(()),
            Ok(_) => {}
            Err(e) => println!("todo: {}", e),
        }
        Ok(())
    }
}
