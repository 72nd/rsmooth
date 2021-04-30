#[macro_use]
extern crate log;

mod error;
mod example;
mod file;
mod metadata;
mod pandoc;
mod tera;
mod util;

use example::Example;
use file::File;


/// Defines the possible output formats for rsmooth.
pub enum OutputFormat {
    /// Portable Document Format.
    Pdf,
    /// OpenDocument Text format.
    Odt,
    /// Office Open XML Document format.
    Docx
}

/// Converts a given markdown file and saves the result to the same path with the same file name.
/// The keep_temp parameter states whether the temporary pandoc input file should be kept for
/// debugging purposes.
pub fn convert<'a>(
    path: &'a str,
    output: Option<&'a str>,
    keep_temp: bool,
    format: OutputFormat,
) -> Result<(), error::SmoothError<'a>> {
    let f = File::new(path, output, format)?;
    f.convert(keep_temp)?;
    Ok(())
}

/// Provides a example markdown document showcasing the key concepts of rsmooth. If no path is
/// given, the method will return the document as a string otherwise the content will be saved
/// to the given path.
pub fn example<'a>(path: Option<&'a str>) -> Result<Option<&'a str>, error::SmoothError<'a>> {
    match path {
        Some(x) => {
            Example::save_to_file(x)?;
            Ok(None)
        }
        None => Ok(Some(Example::as_string()?)),
    }
}
