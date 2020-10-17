use crate::error::SmoothError;
use crate::metadata::Metadata;
use crate::pandoc::Pandoc;
use crate::util;

use std::path::{Path, PathBuf};

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
        // TODO: Do M4
        // TODO: Do Verse break
        match Pandoc::new().convert_with_metadata_to_file(self.path, metadata, self.ouput_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(SmoothError::Pandoc(e)),
        }
    }

    /// Takes the input path of a markdown document and returns the same path with the .pdf
    /// extension. Used when no output path is specified. This function will be useful when rsmooth
    /// also allows the export to other files than PDFs.
    fn out_path_from_input(input: PathBuf) -> PathBuf {
        input.with_extension("pdf")
    }
}
