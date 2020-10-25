use crate::error::SmoothError;

use std::env;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

use shellexpand;

/// Errors occurring while the normalization of paths.
pub enum NormalizeError {
    /// Working folder couldn't be determined.
    WdNotFound,
    /// Lookup error of shellexpand for paths. Contains the erroneous path.  
    LookupError(String),
}

impl fmt::Display for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NormalizeError::WdNotFound => write!(f, "working directory couldn't be determined"),
            NormalizeError::LookupError(path) => {
                write!(f, "some environment variables not found in path {}", path)
            }
        }
    }
}

impl fmt::Debug for NormalizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for NormalizeError {}

/// Returns the absolute path to a given file. Environment variables (words starting with `$`)
/// will be take into account. A tilde (`~`) at the beginning of a path will be replaced with the
/// home directory of the current user. An optional working directory can be specified. Otherwise
/// the value from the system will be used.
pub fn normalize_path<'a, S: Into<String>>(
    path: S,
    wd: Option<PathBuf>,
) -> Result<PathBuf, NormalizeError> {
    let p = path.into();
    let expanded = match shellexpand::full(&p) {
        Ok(x) => x,
        Err(_e) => return Err(NormalizeError::LookupError(p)),
    };
    let mut expanded_str = String::new();
    expanded_str.push_str(&expanded);
    let p = Path::new(&expanded_str);
    if p.is_absolute() {
        return Ok(p.to_path_buf());
    } else {
        match wd {
            Some(x) => Ok(x.join(p)),
            None => match env::current_dir() {
                Ok(x) => Ok(x.join(p)),
                Err(_) => Err(NormalizeError::WdNotFound),
            },
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
            super::normalize_path("/home/max/documents/document.md", None)
                .unwrap()
                .to_str()
                .unwrap(),
            "/home/max/documents/document.md"
        );
        assert_eq!(
            super::normalize_path("~/documents/document.md", None)
                .unwrap()
                .to_str()
                .unwrap(),
            format!("{}/documents/document.md", home.to_str().unwrap())
        );
        assert_eq!(
            super::normalize_path("documents/document.md", None)
                .unwrap()
                .to_str()
                .unwrap(),
            format!("{}/documents/document.md", wd.to_str().unwrap())
        );

        env::set_var("TEST", "/home/max");
        assert_eq!(
            super::normalize_path("$TEST/documents/document.md", None)
                .unwrap()
                .to_str()
                .unwrap(),
            "/home/max/documents/document.md"
        );
        env::remove_var("TEST");
    }
}
