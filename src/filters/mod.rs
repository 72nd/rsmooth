mod expand_paths;
mod template;

pub use expand_paths::*;
pub use template::{Template, TemplateError};

/// Describes errors from a filter.
pub struct FilterError {
    pub name: String,
    pub description: String,
}

/// Defines the constitution of an filter.
pub trait Filter {
    /// Applies the filter to a given markdown source and returns the result as a string.
    fn apply(self, data: String) -> Result<String, FilterError>;
}
