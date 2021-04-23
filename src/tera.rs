use crate::error::SmoothError;

use std::collections::HashMap;
use std::path::PathBuf;

use serde_json::value::Value;
use tera::{Context, Tera};

/// The template filter applies the Tera template engine on the given string.
pub struct Template {
    /// Path to the parent folder where the data originates. This is used to make relative paths
    /// used in the source reachable.
    wd: PathBuf,
    /// Optional context providing the template engine with additional values.
    context: HashMap<String, Value>,
}

impl<'a> Template {
    /// Takes the path of the input markdown file and an optional hash map for the template
    /// context. Returns an instance of the template filter.
    pub fn new(
        input_file: &PathBuf,
        context: Option<HashMap<String, Value>>,
    ) -> Result<Self, SmoothError<'a>> {
        Ok(Self {
            wd: match input_file.parent() {
                Some(x) => x.to_path_buf(),
                None => return Err(SmoothError::NoParentFolder(input_file.to_path_buf()).into()),
            },
            context: match context {
                Some(x) => x,
                None => HashMap::new(),
            },
        })
    }

    pub fn apply(self, data: String) -> Result<String, SmoothError<'a>> {
        let mut pth = self.wd;
        pth.push("**/*.md");
        let mut tpl = match Tera::new(pth.as_os_str().to_str().unwrap()) {
            Ok(x) => x,
            Err(e) => return Err(SmoothError::Tera(e)),
        };
        let ctx = match Context::from_serialize(self.context) {
            Ok(x) => x,
            Err(e) => return Err(SmoothError::Tera(e)),
        };
        match tpl.render_str(&data, &ctx) {
            Ok(x) => Ok(x),
            Err(e) => Err(SmoothError::Tera(e)),
        }
    }
}
