use std::path::PathBuf;

/// Defines the metadata header of a rsmooth markdown file.
struct MetaData<'a> {
    /// Path to the pandoc template file can be absolute or relative to the markdown file. Tilde
    /// (`~`) can be used to refer to the home folder of the current user. It's also possible to
    /// use to use environment variables by prefixing the name with a dollar sign (ex.: `$PATH`).
    template: &'a str,
}

impl MetaData {
    /// Tries to read the YAML header of a given input file to a MetaData struct using pandoc.
    /// Caller has to make sure, pandoc exists on the system.
    pub fn from(file: &PathBuf) -> Result<Self {

    }
}
