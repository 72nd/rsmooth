use crate::error::SmoothError;
use crate::pandoc::Pandoc;
use crate::util;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

/// Name of the temporary pandoc template for extracting the header of a markdown file as JSON.
const JSON_TEMPLATE_PATH: &str = "rsmooth-metadata.pandoc-tpl";

/// Content of the metadata as JSON template.
const JSON_TEMPLATE: &str = "$meta-json$";

/// Defines the metadata header of a rsmooth markdown file.
#[derive(Debug, Deserialize)]
struct Header {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    template: String,
    /// LaTeX engine to be used. Defaults to xelatex.
    #[serde(default = "default_engine")]
    engine: String,
    /// Set additional parameters to pandoc.
    #[serde(default = "default_pandoc_options")]
    pandoc_optons: String,
    /// Whether M4 should be executed on the input file or not.
    #[serde(default = "default_do_m4")]
    do_m4: bool,
    /// Path to bibliography file (JSON CTL).
    #[serde(default = "default_bibliography")]
    bibliography: String,
}

/// Returns the default value (xelatex) for the engine field. Used, when the field is not set in
/// the metadata.
fn default_engine() -> String {
    String::from("xelatex")
}

/// Returns the default value () for the pandoc_optons field. Used, when the field is not set in
/// the metadata.
fn default_pandoc_options() -> String {
    String::new()
}

/// Returns the default value (false) for the do_m4 field. Used, when the field is not set in
/// the metadata.
fn default_do_m4() -> bool {
    false
}

/// Returns the default value () for the bibliography field. Used, when the field is not set in
/// the metadata.
fn default_bibliography() -> String {
    String::new()
}

impl<'a> Header {
    /// Tries to read the YAML header of a given input file to a Metadata struct using pandoc.
    fn from(file: PathBuf) -> Result<Self, SmoothError<'a>> {
        let json_tpl = Header::create_template()?;
        let raw = match Pandoc::new().convert_with_template_to_str(file, json_tpl.clone()) {
            Ok(x) => x,
            Err(x) => {
                Header::remove_template(json_tpl.clone())?;
                return Err(SmoothError::Pandoc(x));
            }
        };
        let mut data: Self = match serde_json::from_str(&raw) {
            Ok(x) => x,
            Err(e) => {
                Header::remove_template(json_tpl.clone())?;
                return Err(SmoothError::MetadataParseFailure(e));
            }
        };
        Header::remove_template(json_tpl.clone())?;

        debug!("parsed {:?}", data);
        Ok(data)
    }

    /// Creates the temporary pandoc template for extracting the content of the header as JSON. The
    /// template will be created in the systems temporary folder. If the file already exists the
    /// execution will halt. Returns the path of the file.
    fn create_template() -> Result<PathBuf, SmoothError<'a>> {
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
    fn remove_template(path: PathBuf) -> Result<(), SmoothError<'a>> {
        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(e) => Err(SmoothError::RemoveJsonTemplateFailed(path, e)),
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    template: PathBuf,
    /// LaTeX engine to be used. Defaults to xelatex.
    engine: String,
    /// Set additional parameters to pandoc.
    pandoc_options: Vec<String>,
    /// Whether M4 should be executed on the input file or not.
    do_m4: bool,
    /// Path to bibliography file (JSON CTL).
    bibliography: PathBuf,
}

impl<'a> Metadata {
    /// Tries to read the YAML header of a given input file to a Metadata struct using pandoc. The
    /// function will test the paths.
    pub fn from(path: PathBuf) -> Result<Self, SmoothError<'a>> {
        let header = Header::from(path)?;
        Ok(Self {
            template: Metadata::normalize_test_template(header.template).unwrap(),
            engine: header.engine,
            pandoc_options: header.pandoc_optons.split_whitespace().map(|x| String::from(x)).collect(),
            do_m4: header.do_m4,
            bibliography: Metadata::normalize_test_bibliography(header.bibliography).unwrap(),
        })
    }

    /// Takes the path to the template file and returns a normalized absolute PathBuf. Also tests if
    /// the file exists.
    fn normalize_test_template(path: String) -> Result<PathBuf, SmoothError<'a>> {
        let rsl = util::normalize_path(&path)?;
        match rsl.exists() {
            true => Ok(rsl),
            false => Err(SmoothError::TemplateNotFound(rsl)),
        }
    }

    /// Takes the path to the default_bibliography file and returns a normalized absolute PathBuf. Also tests if
    /// the file exists.
    fn normalize_test_bibliography(path: String) -> Result<PathBuf, SmoothError<'a>> {
        let rsl = util::normalize_path(&path)?;
        match rsl.exists() {
            true => Ok(rsl),
            false => Err(SmoothError::BibliographyNotFound(rsl)),
        }
    }
}
