/// This module contains all functions to call pandoc and handle any errors occurring mine while.
use crate::metadata::Metadata;

use std::env;
use std::fmt;
use std::io::{self, Error as IOError, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Output};

/// Default name of the pandoc executable. Will be used when no other name is defined via the
/// `PANDOC_ENV` constant of this module.
const PANDOC_CMD: &str = "pandoc";

/// Name of the environment variable which will be used to determine the name of the pandoc
/// executable.
const PANDOC_ENV: &str = "PANDOC_CMD";

/// Contains information about the calling of the pandoc command. Used to accompany error messages
/// when the pandoc execution fails.
pub struct DebugInfo {
    /// Source file.
    input: String,
    /// Path to the template file.
    template: String,
    /// Output file.
    output: String,
    /// Pandoc stderr.
    err: String,
    /// States whether pandoc was called for extracting the metadata in the header or not. This
    /// alters the error message.
    tried_extracting_header: bool,
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tried_extracting_header {
            true => write!(
                f,
                "pandoc failed to extract header metadata from \"{}\" {}",
                self.input, self.err,
            ),

            false => write!(
                f,
                "pandoc failed to convert \"{}\" to \"{}\" with template \"{}\" {}",
                self.input, self.output, self.template, self.err,
            ),
        }
    }
}

/// Defines the different kinds of pandoc errors.
pub enum PandocError<'a> {
    /// The pandoc executable wasn't found on the system. Contains the used name for the pandoc
    /// executable.
    NotFound(String),
    /// The methods of Pandoc a instance only accept absolute paths as argument. Hereby the current
    /// working directory cannot infer with the execution of pandoc. All public Pandoc methods have
    /// to check if a path argument is absolute. Contains the erroneous path and a description of
    /// it's purpose.
    RelativePath(PathBuf, &'a str),
    /// Couldn't convert the Vec<u8> from the pandoc stdout to a string.
    StringFromUtf8,
    /// The execution of pandoc failed. Contains DebugInfo structure.
    ExecutionFailed(DebugInfo),
    /// Couldn't call the pandoc command but the executable is on the system with the used name,
    /// thus not a NotFound error.
    CallFailed(IOError),
}

impl fmt::Display for PandocError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PandocError::NotFound(executable) => match executable == &PANDOC_CMD {
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
            PandocError::RelativePath(path, purpose) => write!(
                f,
                "internal error, pandoc module was called with an relative {} path {}, only absolute paths allowed",
                purpose,
                path.display()
            ),
            PandocError::StringFromUtf8 => write!(
                f,
                "couldn't convert standard output (stdout) from pandoc"
            ),
            PandocError::ExecutionFailed(info) => write!(
                f,
                "{}",
                info
            ),
            PandocError::CallFailed(err) => write!(
                f,
                "couldn't call pandoc {}",
                err,
            ),
        }
    }
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

        let mut cmd = Command::new(self.0.clone());
        cmd.arg("--template").arg(template).arg(&input);

        Pandoc::output_to_result(
            cmd.output(),
            self.0.clone(),
            true,
            String::from(input.to_str().unwrap()),
            String::new(),
            String::new(),
        )
    }

    /// Converts a given file with a template to a output file. Optionally it's possible to add
    /// parameters to the pandoc call.
    pub fn convert_with_metadata_to_file(
        &self,
        input: &PathBuf,
        metadata: Metadata,
        output: PathBuf,
    ) -> Result<(), PandocError<'a>> {
        let mut cmd = Command::new(self.0.clone());
        cmd.arg(&input)
            .arg("--template")
            .arg(&metadata.template)
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
        cmd.arg("-o").arg(&output);
        match Pandoc::output_to_result(
            cmd.output(),
            self.0.clone(),
            false,
            String::from(input.to_str().unwrap()),
            String::from(output.to_str().unwrap()),
            String::from(metadata.template.to_str().unwrap()),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// Checks the output of a pandoc call and returns the appropriate result.
    fn output_to_result(
        rsl: io::Result<Output>,
        pandoc_bin: String,
        tried_extracting_header: bool,
        input: String,
        output: String,
        temlate: String,
    ) -> Result<String, PandocError<'a>> {
        match rsl {
            Ok(x) => {
                if x.status.success() {
                    match String::from_utf8(x.stdout) {
                        Ok(x) => Ok(x),
                        Err(_) => Err(PandocError::StringFromUtf8),
                    }
                } else {
                    Err(PandocError::ExecutionFailed(DebugInfo {
                        input: input,
                        output: output,
                        template: temlate,
                        err: String::from_utf8(x.stderr).unwrap(),
                        tried_extracting_header: tried_extracting_header,
                    }))
                }
            }
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    Err(PandocError::NotFound(pandoc_bin))
                } else {
                    Err(PandocError::CallFailed(e))
                }
            }
        }
    }
}

/// Checks if the given path is absolute and returns the corresponding PandocError.
fn check_path<'a>(path: PathBuf, purpose: &'a str) -> Result<(), PandocError<'a>> {
    match path.is_absolute() {
        true => Ok(()),
        false => Err(PandocError::RelativePath(path, purpose)),
    }
}
