use crate::error::SmoothError;

use std::env;
use std::path::{Path, PathBuf};

use shellexpand;

/// Returns the absolute path to a given file. Environment variables (words starting with `$`)
/// will be take into account. A tilde (`~`) at the beginning of a path will be replaced with the
/// home directory of the current user.
pub fn normalize_path<'a>(path: &'a str) -> Result<PathBuf, SmoothError> {
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
