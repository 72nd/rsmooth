use std::error::Error;
use std::fmt;

/// The error type for errors which can occur while running rsmooth.
pub enum SmoothError<'a> {
    /// The pandoc executable wasn't found on the system.
    PandocMissing,
    /// M4 was enabled but executable wasn't found on the system.
    M4Missing,
    /// Working folder couldn't be determined.
    WdNotFound,
    /// Lookup error of shellexpand for paths. First element is the erroneous path, second contains
    /// the cause for the error.
    LookupError(&'a str),
    /// The input file was not found under the given path. 
    InputFileNotFound(&'a str, &'a str),
}

impl fmt::Display for SmoothError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmoothError::PandocMissing => write!(f, "pandoc not found on system"),
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
                given, normalized
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
