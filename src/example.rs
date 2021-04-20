use crate::error::SmoothError;
use crate::util;

use std::fs;
use std::include_str;
use std::io::prelude::*;

/// Provides an example markdown file showcasing the key features of rsmooth.
pub struct Example;

impl<'a> Example {
    /// Returns the example file as a string content.
    pub fn as_string() -> Result<&'a str, SmoothError<'a>> {
        Ok(Self::content())
    }

    /// Saves the example markdown file to the given path.
    pub fn save_to_file<S: Into<String>>(path: S) -> Result<(), SmoothError<'a>> {
        let norm_path = util::normalize_path(path, None)?;
        let mut file = match fs::File::create(&norm_path) {
            Ok(x) => x,
            Err(e) => return Err(SmoothError::FileCreateFailed(norm_path, e)),
        };
        let content = String::from(Self::content());
        match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(SmoothError::WriteFailed(norm_path, e)),
        }
    }

    /// Includes the content of the example file into the binary and returns
    /// it's content.
    fn content() -> &'static str {
        include_str!("example.md")
    }
}
