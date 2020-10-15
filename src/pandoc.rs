/// This module contains all functions to call pandoc and handle any errors occurring mine while.
use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

/// Default name of the pandoc executable. Will be used when no other name is defined via the
/// `PANDOC_ENV` constant of this module.
const PANDOC_CMD: &str = "pandoc";

/// Name of the environment variable which will be used to determine the name of the pandoc
/// executable.
const PANDOC_ENV: &str = "PANDOC_CMD";

/// Handles errors in correspondence with the pandoc command.
pub struct PandocError<'a> {
    /// Contains the path to the input file.
    input: Option<PathBuf>,
    /// Contains the path to a pandoc template if any was used.
    template: Option<PathBuf>,
    /// Contains the path to the ouptut file if any was set.
    output: Option<PathBuf>,
    /// Defines the kind of error.
    kind: ErrorKind<'a>,
}

impl<'a> PandocError<'a> {
    /// Returns a new RelativePath error. Learn more in the documentation of
    /// ErrorKind::RelativePath.
    fn relative_path(path: PathBuf, description: &'a str) -> Self {
        Self {
            input: None,
            template: None,
            output: None,
            kind: ErrorKind::RelativePath(path, description),
        }
    }
}

impl fmt::Display for PandocError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::NotFound(executable) => match executable == &PANDOC_CMD {
                true => write!(f, "couldn't find \"pandoc\" on your system, use the env \"{}\" to use an non default executable name", PANDOC_ENV),
                false => write!(f, "couldn't find pandoc with the executable name \"{}\" use env \"{}\" to specify otherwise", executable, PANDOC_ENV),
            },
            ErrorKind::RelativePath(path, purpose) => write!(f, "internal error, pandoc module was called with an relative {} path {}, only absolute paths allowed", purpose, path.display()),
        }
    }
}

/// Defines the different kinds of pandoc errors.
pub enum ErrorKind<'a> {
    /// The pandoc executable wasn't found on the system. Contains the used name for the pandoc
    /// executable.
    NotFound(&'a str),
    /// The methods of Pandoc a instance only accept absolute paths as argument. Hereby the current
    /// working directory cannot infer with the execution of pandoc. All public Pandoc methods have
    /// to check if a path argument is absolute. Contains the erroneous path and a description of
    /// it's purpose.
    RelativePath(PathBuf, &'a str),
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

    // Converts a given file with a template to a given output file. The function
    //pub fn convert_with_template(

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

/// Checks if the given path is absolute and returns the corresponding PandocError.
fn check_path<'a>(path: PathBuf, purpose: &'a str) -> Result<(), PandocError> {
    match path.is_absolute() {
        true => Ok(()),
        false => Err(PandocError::relative_path(path, purpose)),
    }
}
