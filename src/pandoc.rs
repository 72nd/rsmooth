/// This module contains all functions to call pandoc and handle any errors occurring mine while.
use crate::metadata::Metadata;

use std::env;
use std::fmt;
use std::io::Error as IOError;
use std::path::PathBuf;
use std::process::Command;

use tempfile::NamedTempFile;

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

    /// Returns a new StringFromUtf8 error instance.
    fn string_from_utf8() -> Self {
        Self {
            input: None,
            template: None,
            output: None,
            kind: ErrorKind::StringFromUtf8,
        }
    }

    /// Returns a new ExecutionFailed error instance. Takes the stderr output of the pandoc
    /// process.
    fn execution_failed(err: Vec<u8>) -> Self {
        Self {
            input: None,
            template: None,
            output: None,
            kind: ErrorKind::ExecutionFailed(String::from_utf8(err).unwrap()),
        }
    }

    /// Returns a new CallingFailed error instance.
    fn calling_failed(err: IOError) -> Self {
        Self {
            input: None,
            template: None,
            output: None,
            kind: ErrorKind::CallingFailed(err),
        }
    }
}

impl fmt::Display for PandocError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::NotFound(executable) => match executable == &PANDOC_CMD {
                true => write!(
                    f,
                    "couldn't find \"pandoc\" on your system, use the env \"{}\" to use an non default executable name",
                    PANDOC_ENV
                ),
                false => write!(
                    f,
                    "couldn't find pandoc with the executable name \"{}\" use env \"{}\" to specify otherwise",
                    executable,
                    PANDOC_ENV
                ),
            },
            ErrorKind::RelativePath(path, purpose) => write!(
                f,
                "internal error, pandoc module was called with an relative {} path {}, only absolute paths allowed",
                purpose,
                path.display()
            ),
            ErrorKind::StringFromUtf8 => write!(
                f,
                "couldn't convert standard output (stdout) from pandoc"
            ),
            ErrorKind::ExecutionFailed(err) => write!(
                f,
                "pandoc failed with {}",
                err
            ),
            ErrorKind::CallingFailed(err) => write!(
                f,
                "couldn't call pandoc {}",
                err
            ),
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
    /// Couldn't convert the Vec<u8> from the pandoc stdout to a string.
    StringFromUtf8,
    /// The execution of pandoc failed. Contains the output of the stderr.
    ExecutionFailed(String),
    /// Calling pandoc failed. Contains the std::io::Error.
    CallingFailed(IOError),
}

/// Wrapper for calling pandoc. Exposes all needed functionality via it's method. Contains the
/// executable name for pandoc.
pub struct Pandoc(String);

impl<'a> Pandoc {
    /// Returns a new instance of the Pandoc struct. Will use the `PANDOC_CMD` environment variable
    /// to determine the name of the pandoc executable. If the variable isn't set the constant
    /// PANDOC_CMD will be used.
    pub fn new() -> Self {
        Self(match env::var(PANDOC_ENV) {
            Ok(x) => x,
            Err(_) => String::from(PANDOC_CMD),
        })
    }

    /// Converts a given file with a template and returns the result as a string. This function is
    /// mainly used for the extraction of the frontmatter header as a JSON file.
    pub fn convert_with_template_to_str(
        &self,
        input: PathBuf,
        template: PathBuf,
    ) -> Result<String, PandocError<'a>> {
        check_path(input.clone(), "input")?;
        check_path(template.clone(), "template")?;
        debug!(
            "input: {}, template: {}",
            input.display(),
            template.display()
        );

        match Command::new(self.0.clone())
            .arg("--template")
            .arg(template)
            .arg(input)
            .output()
        {
            Ok(x) => match String::from_utf8(x.stdout) {
                Ok(x) => Ok(x),
                Err(_) => Err(PandocError::string_from_utf8()),
            },
            Err(e) => {
                // TODO finish
                println!("{}", e);
                Ok(String::new())
            }
        }
    }

    /// Converts a given file with a template to a output file. Optionally it's possible to add
    /// parameters to the pandoc call.
    pub fn convert_with_metadata_to_file(
        &self,
        input: NamedTempFile,
        metadata: Metadata,
        output: PathBuf,
    ) -> Result<(), PandocError<'a>> {
        let mut cmd = Command::new(self.0.clone());
        cmd.arg(input.path())
            .arg("--template")
            .arg(metadata.template)
            .arg("--pdf-engine")
            .arg(metadata.engine)
            .arg("--wrap=preserve");
        if let Some(options) = metadata.pandoc_options {
            cmd.args(options);
        }
        if let Some(bibliography) = metadata.bibliography {
            cmd.arg("--filter")
                .arg("pandoc-citeproc")
                .arg("--bibliography")
                .arg(bibliography);
        }
        cmd.arg("-o").arg(output);
        match cmd.output() {
            Ok(x) => {
                if x.status.success() {
                    Ok(())
                } else {
                    Err(PandocError::execution_failed(x.stderr))
                }
            }
            Err(e) => {
                println!("{}", e);
                Err(PandocError::calling_failed(e))
            }
        }
    }
}

/// Checks if the given path is absolute and returns the corresponding PandocError.
fn check_path<'a>(path: PathBuf, purpose: &'a str) -> Result<(), PandocError<'a>> {
    match path.is_absolute() {
        true => Ok(()),
        false => Err(PandocError::relative_path(path, purpose)),
    }
}
