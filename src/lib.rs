#[macro_use]
extern crate log;

mod error;
mod file;
mod metadata;
mod pandoc;
mod templating;
mod util;

use file::File;

/// Converts a given markdown file and saves the result to the same path with the same file name.
pub fn convert<'a>(path: &'a str, output: Option<&'a str>) -> Result<(), error::SmoothError<'a>> {
    let f = File::new(path, output)?;
    f.convert()?;
    Ok(())
}
