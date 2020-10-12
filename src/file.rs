use crate::error::SmoothError;

use std::env;
use std::path::{Path, PathBuf};

use shellexpand;

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

    /// Starts the conversion.
    pub fn convert(&self) -> Result<(), SmoothError> {
        Ok(())
    }
}

/// Checks the existence of file with the given path.
fn check_file_existence(path: &str) -> bool {
    Path::new(path).exists()
}

/// Returns the absolute path to a given folder. Environment variables (words starting with `$`)
/// will be take into account. A tilde (`~`) at the beginning of a path will be replaced with the
/// home directory of the current user.
fn normalize_path<'a>(path: &'a str) -> Result<PathBuf, SmoothError> {
    let expanded = match shellexpand::full(path) {
        Ok(x) => x,
        Err(e) => return Err(SmoothError::LookupError(path)),
    };
    let mut expanded_str = String::new();
    expanded_str.push_str(&expanded);
    println!("{}", expanded);
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
