use std::error::Error;
use std::fmt;

use tera::Error as TeraError;

/// Contains the errors which can occur while the execution of
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
