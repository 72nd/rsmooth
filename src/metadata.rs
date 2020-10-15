use crate::error::SmoothError;
use crate::pandoc::Pandoc;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;


/// Name of the temporary pandoc template for extracting the header of a markdown file as JSON.
const JSON_TEMPLATE_PATH: &str = "rsmooth-metadata.pandoc-tpl";

/// Content of the metadata as JSON template.
const JSON_TEMPLATE: &str = "$meta-json$";

/// Defines the metadata header of a rsmooth markdown file.
pub struct MetaData<'a> {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    template: &'a str,
}

impl<'a> MetaData<'a> {
    /// Tries to read the YAML header of a given input file to a MetaData struct using pandoc.
    /// Caller has to make sure, pandoc exists on the system.
    pub fn from(file: &PathBuf) -> Result<Self, SmoothError<'a>> {
        let json_tpl = create_template()?;
        match Pandoc::new().convert_with_template_to_str(file, &json_tpl) {
            Ok(x) => println!("{}", String::from_utf8(x).unwrap()),
            Err(_) => {}
        };

        remove_template(json_tpl)?;
        Ok(Self {
            template: "not-implemented",
        })
    }
}

/// Creates the temporary pandoc template for extracting the content of the header as JSON. The
/// template will be created in the systems temporary folder. If the file already exists the
/// execution will halt. Returns the path of the file.
fn create_template<'a>() -> Result<PathBuf, SmoothError<'a>> {
    let path = env::temp_dir().join(PathBuf::from(JSON_TEMPLATE_PATH));
    if path.exists() {
        return Err(SmoothError::JsonTemplateExists(path));
    }
    let mut file = match fs::File::create(&path) {
        Ok(x) => x,
        Err(e) => return Err(SmoothError::CreateJsonTemplateFailed(path, e)),
    };
    match file.write_all(JSON_TEMPLATE.as_bytes()) {
        Ok(_) => Ok(path),
        Err(e) => Err(SmoothError::CreateJsonTemplateFailed(path, e)),
    }
}

/// Removes the temporary metadata as JSON template with the given path.
fn remove_template<'a>(path: PathBuf) -> Result<(), SmoothError<'a>> {
    match fs::remove_file(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(SmoothError::RemoveJsonTemplateFailed(path, e)),
    }
}
