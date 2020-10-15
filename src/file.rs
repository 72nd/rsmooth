use crate::error::SmoothError;
use crate::metadata::MetaData;

use std::env;
use std::path::{Path, PathBuf};

use shellexpand;

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
        let norm_in_path = normalize_path(in_path)?;
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
        let metadata = MetaData::from(&self.path)?;
        Ok(())
    }
}

/// Returns the absolute path to a given folder. Environment variables (words starting with `$`)
/// will be take into account. A tilde (`~`) at the beginning of a path will be replaced with the
/// home directory of the current user.
fn normalize_path<'a>(path: &'a str) -> Result<PathBuf, SmoothError> {
    let expanded = match shellexpand::full(path) {
        Ok(x) => x,
        Err(_e) => return Err(SmoothError::LookupError(path)),
    };
    let mut expanded_str = String::new();
    expanded_str.push_str(&expanded);
    let p = Path::new(&expanded_str);
    if p.is_absolute() {
        Ok(p.to_path_buf())
    } else {
        match env::current_dir() {
            Ok(x) => {
                let rsl = x.join(p);
                Ok(rsl)
            }
            Err(_) => Err(SmoothError::WdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn normalize_path() {
        let wd = env::current_dir().unwrap();
        let home = env::home_dir().unwrap();

        assert_eq!(
            super::normalize_path("/home/max/documents/document.md")
                .unwrap()
                .to_str()
                .unwrap(),
            "/home/max/documents/document.md"
        );
        assert_eq!(
            super::normalize_path("~/documents/document.md")
                .unwrap()
                .to_str()
                .unwrap(),
            format!("{}/documents/document.md", home.to_str().unwrap())
        );
        assert_eq!(
            super::normalize_path("documents/document.md")
                .unwrap()
                .to_str()
                .unwrap(),
            format!("{}/documents/document.md", wd.to_str().unwrap())
        );

        env::set_var("TEST", "/home/max");
        assert_eq!(
            super::normalize_path("$TEST/documents/document.md")
                .unwrap()
                .to_str()
                .unwrap(),
            "/home/max/documents/document.md"
        );
        env::remove_var("TEST");
    }
}
