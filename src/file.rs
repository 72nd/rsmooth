use crate::error::SmoothError;
use crate::metadata::Metadata;
use crate::pandoc::Pandoc;
use crate::tera::Template;
use crate::util;
use crate::OutputFormat;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use tempfile::NamedTempFile;

/// Describes the (root) markdown file which should be converted.
pub struct File {
    /// Absolute path to the markdown source file.
    path: PathBuf,
    /// Destination path for the output file.
    ouput_path: PathBuf,
    /// Desired format of the output file.
    output_format: OutputFormat,
}

impl<'a> File {
    /// Returns a new instance of the file object for a given path. The output folder can be
    /// defined, otherwise the same folder and file name as the input file will be used.
    pub fn new<S: Into<&'a str>>(
        path: S,
        output_path: Option<S>,
        output_format: OutputFormat,
    ) -> Result<Self, SmoothError<'a>> {
        let in_path = path.into();
        let norm_in_path = util::normalize_path(in_path, None)?;
        if !norm_in_path.exists() {
            return Err(SmoothError::InputFileNotFound(in_path, norm_in_path));
        }
        Ok(Self {
            path: norm_in_path.clone(),
            ouput_path: match output_path {
                Some(x) => util::normalize_path(x.into(), None)?,
                None => File::out_path_from_input(norm_in_path, &output_format),
            },
            output_format: output_format,
        })
    }

    /// Converts the loaded markdown file. The keep_temp parameter states whether the temporary
    /// pandoc input file should be kept for debugging purposes.
    pub fn convert(self, output_raw: bool) -> Result<(), SmoothError<'a>> {
        let metadata = Metadata::from(&self.path, &self.parent_folder()?)?;

        let mut content = self.read_source()?;
        // content = ExpandPaths::new(&self.path, vec![ExpandOn::TeraIncludes])?.apply(content)?;

        if metadata.do_tera {
            content = Template::new(&self.path, metadata.clone().tera_context)?.apply(content)?;
        }

        let mut current = File::new_named_tempfile()?;
        match current.write_all(content.as_bytes()) {
            Ok(_) => {}
            Err(e) => return Err(SmoothError::WriteFailed(current.path().to_path_buf(), e)),
        };
        let prepared_input = current.path().to_path_buf();

        if output_raw {
            println!("{}", content)
        }

        let result = match self.output_format {
            OutputFormat::Pdf => Pandoc::new().convert_with_metadata_to_pdf(
                &prepared_input,
                metadata,
                &self.ouput_path,
                Some(&self.parent_folder()?),
            ),
            OutputFormat::Odt | OutputFormat::Docx => Pandoc::new().convert_with_metadata_to_office(
                &prepared_input,
                metadata,
                &self.ouput_path,
                Some(&self.parent_folder()?),
            ),
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(SmoothError::Pandoc(e)),
        }
    }

    /// Reads the input file and returns the content as a string. This is used to apply all
    /// internal filters.
    fn read_source(&self) -> Result<String, SmoothError<'a>> {
        match fs::read_to_string(self.path.clone()) {
            Ok(x) => Ok(x),
            Err(e) => Err(SmoothError::ReadSourceFailed(self.path.clone(), e)),
        }
    }

    fn parent_folder(&self) -> Result<PathBuf, SmoothError<'a>> {
        match self.path.parent() {
            Some(x) => Ok(x.to_path_buf()),
            None => Err(SmoothError::NoParentFolder(self.path.to_path_buf()).into()),
        }
    }

    /// Takes the input path of a markdown document and returns the same path with the .pdf
    /// extension. Used when no output path is specified. This function will be useful when rsmooth
    /// also allows the export to other files than PDFs.
    fn out_path_from_input(input: PathBuf, format: &OutputFormat) -> PathBuf {
        match format {
            OutputFormat::Pdf => input.with_extension("pdf"),
            OutputFormat::Odt => input.with_extension("odt"),
            OutputFormat::Docx => input.with_extension("docx"),
        }
    }

    /// Encapsulates the instantiating of a new NamedTempFile and returns the appropriate smooth
    /// error on error.
    fn new_named_tempfile() -> Result<NamedTempFile, SmoothError<'a>> {
        match NamedTempFile::new() {
            Ok(x) => Ok(x),
            Err(e) => Err(SmoothError::TemporaryFile(e)),
        }
    }
}
