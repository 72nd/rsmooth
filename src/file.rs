use crate::error::SmoothError;
use crate::metadata::Metadata;
use crate::pandoc::Pandoc;
use crate::util;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use tempfile::NamedTempFile;

/// Describes the (root) markdown file which should be converted.
pub struct File {
    /// Absolute path to the markdown source file.
    path: PathBuf,
    /// Destination path for the output file.
    ouput_path: PathBuf,
}

impl<'a> File {
    /// Returns a new instance of the file object for a given path. The output folder can be
    /// defined, otherwise the same folder and file name as the input file will be used.
    pub fn new<S: Into<&'a str>>(path: S, output_path: Option<S>) -> Result<Self, SmoothError<'a>> {
        let in_path = path.into();
        let norm_in_path = util::normalize_path(in_path)?;
        if !norm_in_path.exists() {
            return Err(SmoothError::InputFileNotFound(in_path, norm_in_path));
        }
        Ok(Self {
            path: norm_in_path.clone(),
            ouput_path: match output_path {
                Some(x) => util::normalize_path(x.into())?,
                None => File::out_path_from_input(norm_in_path),
            },
        })
    }

    /// Converts the loaded markdown file.
    pub fn convert(self) -> Result<(), SmoothError<'a>> {
        let metadata = Metadata::from(self.path.clone())?;

        let mut content = self.read_source()?;
        let mut current = File::temporary_file_from_source(self.path.clone())?;
        current.write_all(content.as_bytes());

        // TODO: Do Verse break
        match Pandoc::new().convert_with_metadata_to_file(current, metadata, self.ouput_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(SmoothError::Pandoc(e)),
        }
    }

    /// Reads the input file and returns the content as a string. This is used to apply all
    /// internal filters.
    fn read_source(self) -> Result<String, SmoothError<'a>> {
        match fs::read_to_string(self.path.clone()) {
            Ok(x) => Ok(x),
            Err(e) => Err(SmoothError::ReadSourceFailed(self.path, e)),
        }
    }

    /// Takes the input path of a markdown document and returns the same path with the .pdf
    /// extension. Used when no output path is specified. This function will be useful when rsmooth
    /// also allows the export to other files than PDFs.
    fn out_path_from_input(input: PathBuf) -> PathBuf {
        input.with_extension("pdf")
    }

    /// Encapsulates the instantiating of a new NamedTempFile and returns the appropriate smooth
    /// error on error.
    fn new_named_tempfile() -> Result<NamedTempFile, SmoothError<'a>> {
        match NamedTempFile::new() {
            Ok(x) => Ok(x),
            Err(e) => Err(SmoothError::TemporaryFile(e)),
        }
    }

    /// Creates a temporary file with the content of the source file.
    fn temporary_file_from_source(path: PathBuf) -> Result<NamedTempFile, SmoothError<'a>> {
        let mut file = File::new_named_tempfile()?;
        match file.write_all(&match fs::read(path.clone()) {
            Ok(x) => x,
            Err(e) => return Err(SmoothError::ReadSourceFailed(path, e)),
        }) {
            Ok(_) => Ok(file),
            Err(e) => return Err(SmoothError::TemporaryFile(e)),
        }
    }
}
