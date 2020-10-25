use super::{Filter, FilterError};

use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;

use tera::{Context, Error as TeraError, Tera};

/// Contains the errors which can occur while the execution of the template filter
pub enum TemplateError {
    /// Parent element of source file path couldn't be determined.
    NoParentFolder(PathBuf),
    /// Some tera error.
    Tera(TeraError),
}

impl From<TemplateError> for FilterError {
    fn from(item: TemplateError) -> FilterError {
        FilterError {
            name: String::from("template"),
            description: match item {
                TemplateError::NoParentFolder(path) => {
                    format!("no parent folder for path {} found", path.display())
                }
                TemplateError::Tera(err) => format!("tera engine error {}", err),
            },
        }
    }
}

/// The template filter applies the tera template engine on the given string.
pub struct Template {
    /// Path to the parent folder where the data originates. This is used to make relative paths
    /// used in the source reachable.
    wd: PathBuf,
    /// Optional context providing the template engine with additional values.
    context: HashMap<String, String>,
}

impl Template {
    /// Takes the path of the input markdown file and an optional hash map for the template
    /// context. Returns an instance of the template filter.
    pub fn new(
        input_file: &PathBuf,
        context: Option<HashMap<String, String>>,
    ) -> Result<Self, FilterError> {
        Ok(Self {
            wd: match input_file.parent() {
                Some(x) => x.to_path_buf(),
                None => return Err(TemplateError::NoParentFolder(input_file.to_path_buf()).into()),
            },
            context: match context {
                Some(x) => x,
                None => HashMap::new(),
            },
        })
    }
}

impl Filter for Template {
    fn apply(self, data: String) -> Result<String, FilterError> {
        let mut pth = self.wd;
        pth.push("**/*.md");
        let mut tpl = match Tera::new(pth.as_os_str().to_str().unwrap()) {
            Ok(x) => x,
            Err(e) => return Err(TemplateError::Tera(e).into()),
        };
        let ctx = match Context::from_serialize(self.context) {
            Ok(x) => x,
            Err(e) => return Err(TemplateError::Tera(e).into()),
        };
        match tpl.render_str(&data, &ctx) {
            Ok(x) => Ok(x),
            Err(e) => Err(TemplateError::Tera(e).into()),
        }
    }
}
