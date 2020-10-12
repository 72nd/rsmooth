use crate::error;

use std::path::Path;

/// Defines the metadata header of a rsmooth markdown file.
struct Header<'a> {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    template: &'a str,
}

/// Describes the (root) markdown file which should be converted.
pub struct File<'a> {
    /// Absolute path to the markdown source file.
    path: &'a str,
    /// Destination path for the output file.
    ouput_path: Option<&'a str>,
}

impl<'a> File<'a> {
    /// Returns a new instance of the file object for a given path. The output folder can be
    /// defined, otherwise the same folder and file name as the input file will be used.
    pub fn new<S: Into<&'a str>>(path: S, output_path: Option<S>) -> Self {
        Self {
            path: path.into(),
            ouput_path: match output_path {
                Some(x) => Some(x.into()),
                None => None,
            },
        }
    }
}

/// Checks the existence of file with the given path.
fn check_file_existence(path: &str) -> bool {
    Path::new(path).exists()
}

/// Returns the absolute path to a given folder. Environment variables (words starting with `$`)
/// will be take into account. A tilde (`~`) at the beginning of a path will be replaced with the
/// home directory of the current user.
fn normalize_path(path: &str) -> &str {
    path
}
