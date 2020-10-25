#[macro_use]
extern crate log;

mod error;
mod file;
mod metadata;
mod pandoc;
mod filters;
mod util;

use file::File;

/// Converts a given markdown file and saves the result to the same path with the same file name.
/// The keep_temp parameter states whether the temporary pandoc input file should be kept for
/// debugging purposes.
pub fn convert<'a>(path: &'a str, output: Option<&'a str>, keep_temp: bool) -> Result<(), error::SmoothError<'a>> {
    let f = File::new(path, output)?;
    f.convert(keep_temp)?;
    Ok(())
}
