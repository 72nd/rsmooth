use super::{Filter, FilterError};

use std::convert::From;
use std::path::PathBuf;

use regex::{Captures, Error as RegexError, Regex};

/// Contains the errors which can occur while the execution expand paths filter
pub enum ExpandPathsError {
    /// Parent element of source file path couldn't be determined.
    NoParentFolder(PathBuf),
    /// Error occurred while compiling a regular expression. Contains the erroneous expression and
    /// the regex error.
    RegexCompileFailed(String, RegexError),
}

impl From<ExpandPathsError> for FilterError {
    fn from(item: ExpandPathsError) -> FilterError {
        FilterError {
            name: String::from("expand_paths"),
            description: match item {
                ExpandPathsError::NoParentFolder(path) => {
                    format!("no parent folder for path {} found", path.display())
                }
                ExpandPathsError::RegexCompileFailed(expression, err) => format!(
                    "compiling \"{}\" to regular expression failed {}",
                    expression, err
                ),
            },
        }
    }
}

/// Defines the element which should be expanded. This way the filter can be called multiple times
/// for different use cases.
pub enum ExpandOn {
    /// Expand on Tera include statements.
    TeraIncludes,
    /// Expand embedded links like images.
    EmbeddedLinks,
}

impl ExpandOn {
    /// Returns the correct regex search and replace pattern for a given document element.
    fn re_pattern(self) -> Result<Regex, ExpandPathsError> {
        match self {
            ExpandOn::TeraIncludes => {
                match Regex::new(r#"(?P<prefix>\{%\s+include\s+")(?P<path>.*?)(?P<infix>"\s+%})"#) {
                    Ok(x) => Ok(x),
                    Err(e) => Err(ExpandPathsError::RegexCompileFailed(String::from(""), e)),
                }
            }
            ExpandOn::EmbeddedLinks => match Regex::new("") {
                Ok(x) => Ok(x),
                Err(e) => Err(ExpandPathsError::RegexCompileFailed(String::from(""), e)),
            },
        }
    }
}

/// Expand paths in the source file to absolute paths and also shellexpand them.
pub struct ExpandPaths {
    /// Path to the parent folder where the data originates. This is used to make relative paths
    /// used in the source reachable.
    wd: PathBuf,
    /// Defines which elements should be expanded.
    expand_on: Vec<ExpandOn>,
}

impl ExpandPaths {
    /// Returns a new expand paths filter. Parameter states on which elements the path should be
    /// expanded.
    pub fn new(input_file: &PathBuf, expand_on: Vec<ExpandOn>) -> Result<Self, FilterError> {
        Ok(Self {
            wd: match input_file.parent() {
                Some(x) => x.to_path_buf(),
                None => {
                    return Err(ExpandPathsError::NoParentFolder(input_file.to_path_buf()).into())
                }
            },
            expand_on: expand_on,
        })
    }
}

impl Filter for ExpandPaths {
    fn apply(self, data: String) -> Result<String, FilterError> {
        let mut rsl = data;
        for element in self.expand_on {
            let re = element.re_pattern()?;
            rsl = re
                .replace_all(&rsl, |caps: &Captures| {
                    format!(
                        "{}HALLO{}",
                        caps.name("prefix").unwrap().as_str(),
                        caps.name("infix").unwrap().as_str()
                    )
                })
                .to_string();
        }
        Ok(rsl)
    }
}

/*
/// Expands a capture containing the `prefix` (all text before the path), `path` and `infix`
/// (all text after the path) to the desired format with the absolute path.
struct Expander;

impl Replacer for Expander {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
    }
}
*/
