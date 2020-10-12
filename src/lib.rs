mod error;
mod file;

/// Converts a given markdown file and saves the result to the same path with the same file name.
pub fn convert(path: &str) -> Result<(), error::SmoothError> {
    Ok(())
}

/// Takes a markdown file and saves the result to the given destination folder.
pub fn convert_to(path: &str, destination_folder: &str) -> Result<(), error::SmoothError<'static>> {
    Ok(())
}
