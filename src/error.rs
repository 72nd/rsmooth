use crate::pandoc::PandocError;

use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::path::PathBuf;

use serde_json::error::Error as JsonError;

/// The error type for errors which can occur while running rsmooth.
pub enum SmoothError<'a> {
    /// Error occurring while calling pandoc contains an PandocError. For more information on the
    /// handling of pandoc (-errors) see the pandoc module.
    Pandoc(PandocError<'a>),
    /// M4 was enabled but executable wasn't found on the system.
    M4Missing,
    /// Working folder couldn't be determined.
    WdNotFound,
    /// Lookup error of shellexpand for paths. Contains the erroneous path.  
    LookupError(String),
    /// The input file was not found under the given path.
    InputFileNotFound(&'a str, PathBuf),
    /// Couldn't read the Frontmatter YAML Header of the input file. String resembles the path to
    /// the input file.
    MetadataRead(&'a str),
    /// Occurs when the JSON template is already present in the temporary folder. See the matedata
    /// module for more information. Contains the path to the template file.
    JsonTemplateExists(PathBuf),
    /// Error occurring while the creation of the metadata as JSON template. First element contains
    /// path to the file, the second element contains the std::io::Error with the cause.
    CreateJsonTemplateFailed(PathBuf, IOError),
    /// Occurs when the converting metadata JSON cannot be parsed.
    MetadataParseFailure(JsonError),
    /// Error for failing of the metadata as JSON template removal. Contains the path to the
    /// template file and the cause.
    RemoveJsonTemplateFailed(PathBuf, IOError),
    /// The given template path as specified in the metadata header was not found with the given
    /// path.
    TemplateNotFound(PathBuf),
    /// The given bibliography path as specified in the metadata header was not found with the given
    /// path.
    BibliographyNotFound(PathBuf),
}

impl fmt::Display for SmoothError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmoothError::Pandoc(err) => write!(f, "pandoc error: {}", err),
            SmoothError::M4Missing => write!(
                f,
                "m4 was enabled in metadata-header but executable isn't present on system"
            ),
            SmoothError::WdNotFound => write!(f, "working directory couldn't be determined"),
            SmoothError::LookupError(path) => {
                write!(f, "some environment variables not found in path {}", path)
            }
            SmoothError::InputFileNotFound(given, normalized) => write!(
                f,
                "input file \"{}\" couldn't be found under normalized path \"{}\"",
                given,
                normalized.display()
            ),
            SmoothError::MetadataRead(path) => write!(
                f,
                "YAML header for input file \"{}\" couldn't be read",
                path
            ),
            SmoothError::JsonTemplateExists(path) => write!(
                f,
                "pandoc template for extracting the metadata as JSON already present under \"{}\" please remove this file manually before proceeding",
                path.display()
            ),
            SmoothError::CreateJsonTemplateFailed(path, why) => write!(
                f,
                "couldn't write temporary metadata-as-JSON template to {} {}",
                path.display(),
                why
            ),
            SmoothError::MetadataParseFailure(err) => write!(
                f,
                "couldn't parse frontmatter metadata header of document {}",
                err
            ),
            SmoothError::RemoveJsonTemplateFailed(path, why) => write!(
                f,
                "couldn't remove temporary metadata-as-JSON template under {} {}",
                path.display(),
                why
            ),
            SmoothError::TemplateNotFound(path) => write!(
                f,
                "couldn't find template file under {}",
                path.display()
            ),
            SmoothError::BibliographyNotFound(path) => write!(
                f,
                "couldn't find bibliography file under {}",
                path.display()
            ),
        }
    }
}

impl fmt::Debug for SmoothError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for SmoothError<'_> {}
