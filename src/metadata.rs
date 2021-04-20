use crate::error::SmoothError;
use crate::pandoc::Pandoc;
use crate::util;

use std::collections::HashMap;
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
    template: Option<String>,
    /// LaTeX engine to be used. Defaults to xelatex.
    #[serde(default = "default_engine")]
    engine: String,
    /// Set additional parameters to pandoc.
    // #[serde(default = "default_pandoc_options")]
    pandoc_optons: Option<String>,
    /// Whether templating with the Tera engine should be executed on the input file or not.
    #[serde(default = "default_do_tera")]
    do_tera: bool,
    /// Optional template context aka. variables etc.
    pub tera_context: Option<HashMap<String, String>>,
    /// Whether newline should break text in description texts. This is especially useful when
    /// using description lists for screen- and stageplays.
    #[serde(default = "default_break_description")]
    break_description: bool,
    /// Path to bibliography file (JSON CTL).
    bibliography: Option<String>,
}

/// Returns the default value (xelatex) for the engine field. Used, when the field is not set in
/// the metadata.
fn default_engine() -> String {
    String::from("xelatex")
}

/// Returns the default value (false) for the do_tera field. Used, when the field is not set in
/// the metadata.
fn default_do_tera() -> bool {
    false
}

/// Returns the default value (false) for the break_description field. Used, when the field is
/// not set in the metadata.
fn default_break_description() -> bool {
    false
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
        let data: Self = match serde_json::from_str(&raw) {
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

#[derive(Debug, Clone)]
pub struct Metadata {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    /// If no template is set, pandoc will be called without the --template option thus using the
    /// default template of pandoc.
    pub template: Option<PathBuf>,
    /// LaTeX engine to be used. Defaults to xelatex.
    pub engine: String,
    /// Set additional parameters to pandoc.
    pub pandoc_options: Option<Vec<String>>,
    /// Whether the content of the input file should be feed into the Terra templating engine.
    pub do_tera: bool,
    /// Optional template context aka. variables etc.
    pub tera_context: Option<HashMap<String, String>>,
    /// Whether newline should break text in description texts. This is especially useful when
    /// using description lists for screen- and stageplays.
    pub break_description: bool,
    /// Path to bibliography file (JSON CTL).
    pub bibliography: Option<PathBuf>,
}

impl<'a> Metadata {
    /// Tries to read the YAML header of a given input file to a Metadata struct using pandoc. The
    /// function will test the paths.
    pub fn from(path: PathBuf) -> Result<Self, SmoothError<'a>> {
        let header = Header::from(path)?;
        Ok(Self {
            template: match header.template {
                Some(x) => Some(Metadata::normalize_test_template(x)?),
                None => None,
            },
            engine: header.engine,
            pandoc_options: match header.pandoc_optons {
                Some(x) => Some(x.split_whitespace().map(|y| String::from(y)).collect()),
                None => None,
            },
            do_tera: header.do_tera,
            tera_context: header.tera_context,
            break_description: header.break_description,
            bibliography: match header.bibliography {
                Some(x) => Some(Metadata::normalize_test_bibliography(x)?),
                None => None,
            },
        })
    }

    /// Takes the path to the template file and returns a normalized absolute PathBuf. Also tests if
    /// the file exists.
    fn normalize_test_template(path: String) -> Result<PathBuf, SmoothError<'a>> {
        let rsl = util::normalize_path(&path, None)?;
        match rsl.exists() {
            true => Ok(rsl),
            false => Err(SmoothError::TemplateNotFound(rsl)),
        }
    }

    /// Takes the path to the default_bibliography file and returns a normalized absolute PathBuf. Also tests if
    /// the file exists.
    fn normalize_test_bibliography(path: String) -> Result<PathBuf, SmoothError<'a>> {
        let rsl = util::normalize_path(&path, None)?;
        match rsl.exists() {
            true => Ok(rsl),
            false => Err(SmoothError::BibliographyNotFound(rsl)),
        }
    }
}
