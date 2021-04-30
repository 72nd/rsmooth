use crate::pandoc::PandocError;
use crate::util::NormalizeError;

use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::path::PathBuf;

use serde_json::error::Error as JsonError;
use tera::Error as TeraError;

/// The error type for errors which can occur while running rsmooth.
pub enum SmoothError<'a> {
    /// Normalize Error occurred.
    NormalizeError(NormalizeError),
    /// Error occurring while calling pandoc contains an PandocError. For more information on the
    /// handling of pandoc (-errors) see the pandoc module.
    Pandoc(PandocError<'a>),
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
    /// The given reference path as specified in the metadata header was not found with the given
    /// path.
    ReferenceNotFound(PathBuf),
    /// The given bibliography path as specified in the metadata header was not found with the given
    /// path.
    BibliographyNotFound(PathBuf),
    /// The given citation style file path as specified in the metadata header was not found with the given
    /// path.
    CitationStyleNotFound(PathBuf),
    /// Error while creating a temporary file. Contains the error.
    TemporaryFile(IOError),
    /// Couldn't read source file.
    ReadSourceFailed(PathBuf, IOError),
    /// Error occurring while the creation of a file. Should be used when no more specific error is
    /// necessary. First element contains path to the file, the second element contains the
    /// std::io::Error with the cause.
    FileCreateFailed(PathBuf, IOError),
    /// Write file failed.
    WriteFailed(PathBuf, IOError),
    /// Parent element of given path couldn't be determined.
    NoParentFolder(PathBuf),
    /// Some tera error.
    Tera(TeraError),
    /// Given reference file isn't compatible with the requested output format. E.g. using a Odt
    /// reference file for a docx export. First parameter contains the path to the faulty reference
    /// file the second describes the output format.
    IncompatibleReferenceFile(PathBuf, &'a str),
}


impl From<NormalizeError> for SmoothError<'_> {
    fn from(item: NormalizeError) -> Self {
        Self::NormalizeError(item)
    }
}

impl fmt::Display for SmoothError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmoothError::NormalizeError(err) => write!(f, "path normalize error {}", err),
            SmoothError::Pandoc(err) => write!(f, "{}", err),
            SmoothError::InputFileNotFound(given, normalized) => match given == &normalized.as_os_str() {
                true => write!(
                    f,
                    "input file \"{}\" couldn't be found",
                    given,
                ),
                false => write!(
                    f,
                    "input file \"{}\" couldn't be found under normalized path \"{}\"",
                    given,
                    normalized.display(),
                ),
            },
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
            SmoothError::ReferenceNotFound(path) => write!(
                f,
                "couldn't find reference file under {}",
                path.display()
            ),
            SmoothError::BibliographyNotFound(path) => write!(
                f,
                "couldn't find bibliography file under {}",
                path.display()
            ),
            SmoothError::CitationStyleNotFound(path) => write!(
                f,
                "couldn't find citation style (csl) file under {}",
                path.display()
            ),
            SmoothError::TemporaryFile(err) => write!(
                f,
                "couldn't create temporary file {}",
                err
            ),
            SmoothError::ReadSourceFailed(file, err) => write!(
                f,
                "couldn't read the content of the given markdown file {} {}",
                file.display(),
                err
            ),
            SmoothError::FileCreateFailed(file, err) => write!(
                f,
                "couldn't create file {} {}",
                file.display(),
                err,
            ),
            SmoothError::WriteFailed(file, err) => write!(
                f,
                "couldn't write to file {} {}",
                file.display(),
                err,
            ),
            SmoothError::NoParentFolder(file) => write!(
                f,
                "no parent folder for path {} found",
                file.display(),
            ),
            SmoothError::Tera(error) => write!(
                f,
                "error in Tera templating engine {}",
                error,
            ),
            SmoothError::IncompatibleReferenceFile(file, format) => write!(
                f,
                "reference file {} isn't compatible to output format {}",
                file.display(),
                format
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
