use crate::error::SmoothError;
use crate::metadata::Metadata;
use crate::util;

use std::path::{Path, PathBuf};

/// Describes the (root) markdown file which should be converted.
pub struct File<'a> {
    /// Absolute path to the markdown source file.
    path: PathBuf,
    /// Destination path for the output file.
    ouput_path: Option<&'a str>,
}

impl<'a> File<'a> {
    /// Returns a new instance of the file object for a given path. The output folder can be
    /// defined, otherwise the same folder and file name as the input file will be used.
    pub fn new<S: Into<&'a str>>(path: S, output_path: Option<S>) -> Result<Self, SmoothError<'a>> {
        let in_path = path.into();
        let norm_in_path = util::normalize_path(in_path)?;
        if !norm_in_path.exists() {
            return Err(SmoothError::InputFileNotFound(in_path, norm_in_path));
        }
        Ok(Self {
            path: norm_in_path,
            ouput_path: match output_path {
                Some(x) => Some(x.into()),
                None => None,
            },
        })
    }

    /// Converts the loaded markdown file.
    pub fn convert(self) -> Result<(), SmoothError<'a>> {
        let metadata = Metadata::from(self.path)?;
        Ok(())
    }
}
