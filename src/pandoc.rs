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
    fn relative_path(path: &'a PathBuf, description: &'a str) -> Self {
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
    RelativePath(&'a PathBuf, &'a str),
}

/// Wrapper for calling pandoc. Exposes all needed functionality via it's method.
pub struct Pandoc {
    /// Name of the pandoc executable.
    executable: String,
}

impl<'a> Pandoc {
    /// Returns a new instance of the Pandoc struct. Will use the `PANDOC_CMD` environment variable
    /// to determine the name of the pandoc executable. If the variable isn't set the constant
    /// PANDOC_CMD will be used.
    pub fn new() -> Self {
        Self {
            executable: match env::var(PANDOC_ENV) {
                Ok(x) => x,
                Err(_) => String::from(PANDOC_CMD),
            },
        }
    }

    /// Converts a given file with a template and returns the result as a string. This function is
    /// mainly used for the extraction of the frontmatter header as a JSON file.
    pub fn convert_with_template_to_str(
        &self,
        input: &'a PathBuf,
        template: &'a PathBuf,
    ) -> Result<Vec<u8>, PandocError<'a>> {
        check_path(input, "input")?;
        check_path(template, "template")?;

        match Command::new(self.executable.clone())
            .arg("--template")
            .arg(input)
            .arg(template)
            .output() {
                Ok(x) => Ok(x.stdout),
                Err(e) => {
                    println!("{}", e);
                    Ok(vec![])
                }
            }
        // self.run(&["--template", input.as_path().to_str().unwrap(), template.as_path().to_str().unwrap()]);
    }
}

/// Checks if the given path is absolute and returns the corresponding PandocError.
fn check_path<'a>(path: &'a PathBuf, purpose: &'a str) -> Result<(), PandocError<'a>> {
    match path.is_absolute() {
        true => Ok(()),
        false => Err(PandocError::relative_path(path, purpose)),
    }
}
