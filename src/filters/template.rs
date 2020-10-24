use std::error::Error;
use std::fmt;

use tera::Error as TeraError;

/// Contains the errors which can occur while the execution of the template filter
pub enum TemplatingError {
    /// Some tera error.
    Tera(TeraError),
}

impl fmt::Display for TemplatingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TemplatingError::Tera(err) => write!(f, "tera error {}", err,),
        }
    }
}

impl fmt::Debug for TemplatingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for TemplatingError {}

/// The template filter applies the tera template engine on the given string.
pub struct Template {
    /// Raw data to be processed by the template engine.
    data: String,
    /// Path to the parent folder where the data originates. This is used to make relative paths
    /// used in the source reachable.
}
