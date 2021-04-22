use super::{Filter, FilterError};
use crate::util;

use std::convert::From;
use std::fmt;
use std::path::PathBuf;

use regex::{Captures, Error as RegexError, Regex};
use serde::Serialize;

/// Contains the errors which can occur while the execution expand paths filter
pub enum ExpandPathsError {
    /// Normalize Error occurred.
    NormalizeError(util::NormalizeError),
    /// Parent element of source file path couldn't be determined.
    NoParentFolder(PathBuf),
    /// Error occurred while compiling a regular expression. Contains the erroneous expression and
    /// the regex error.
    RegexCompileFailed(String, RegexError),
    /// Internal error when one of the three named capturing groups (`prefix`, `path` or `infix`)
    /// isn't found while the path expansion. Contains the name of the missing group and the
    /// document element type.
    GroupNotFound(String, ExpandOn),
}

impl From<ExpandPathsError> for FilterError {
    fn from(item: ExpandPathsError) -> FilterError {
        FilterError {
            name: String::from("expand_paths"),
            description: match item {
                ExpandPathsError::NormalizeError(err) => format!("path normalize error {}", err),
                ExpandPathsError::NoParentFolder(path) => {
                    format!("no parent folder for path {} found", path.display())
                }
                ExpandPathsError::RegexCompileFailed(expression, err) => format!(
                    "compiling \"{}\" to regular expression failed {}",
                    expression, err
                ),
                ExpandPathsError::GroupNotFound(group, element) => format!(
                    "couldn't find match group {} while expanding {}",
                    group, element,
                ),
            },
        }
    }
}

impl From<util::NormalizeError> for ExpandPathsError {
    fn from(item: util::NormalizeError) -> Self {
        ExpandPathsError::NormalizeError(item)
    }
}

/// Defines the element which should be expanded. This way the filter can be called multiple times
/// for different use cases.
#[derive(Clone, Serialize)]
pub enum ExpandOn {
    /// Expand on Tera include statements.
    TeraIncludes,
    /// Expand embedded links like images.
    #[serde(rename = "embedded_links")]
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
            ExpandOn::EmbeddedLinks => match Regex::new(r#"(?P<prefix>!\[.*?\]\()(?P<path>.*?)(?P<infix>\))"#) {
                Ok(x) => Ok(x),
                Err(e) => Err(ExpandPathsError::RegexCompileFailed(String::from(""), e)),
            },
        }
    }
}

impl fmt::Display for ExpandOn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExpandOn::TeraIncludes => "terra includes",
                ExpandOn::EmbeddedLinks => "embedded links",
            }
        )
    }
}

/// Expand paths in the source file to absolute paths and also shellexpand them.
#[derive(Clone)]
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

    /// Expands a given path to it's absolute representation based on the parent directory of the
    /// source file. The function also shellexpands the path.
    fn expand_path<S: Into<String>>(&mut self, path: S) -> String {
        match util::normalize_path(path.into(), Some(self.wd.clone())) {
            Ok(x) => String::from(x.to_str().unwrap()),
            Err(e) => {
                error!("{}", e);
                String::from("ERROR")
            }
        }
    }
}

impl Filter for ExpandPaths {
    fn apply(self, data: String) -> Result<String, FilterError> {
        let mut rsl = data;
        let mut tmp = self.clone();
        for element in self.expand_on {
            let re = element.re_pattern()?;
            rsl = re
                .replace_all(&rsl, |caps: &Captures| {
                    format!(
                        "{}{}{}",
                        caps.name("prefix").unwrap().as_str(),
                        tmp.expand_path(caps.name("path").unwrap().as_str()),
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
