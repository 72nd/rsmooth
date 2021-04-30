/// The module handles the calls to Libre-Office.
use std::fmt;
use std::io::{Error as IOError, ErrorKind};
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Default name of the Libre-Office executable. Will be used when no other name is defined
/// via the `LIBREOFFICE_CMD` constant of this module.
const LIBREOFFICE_CMD: &str = "soffice";

/// Name of the environment variable which will be used to determine the name of the Libre-Office
/// executable.
const LIBREOFFICE_ENV: &str = "LIBREOFFICE_CMD";

/// Different errors occurring while the execution of Libre-Office.
pub enum LibreOfficeError {
    /// The executable for Libre-Office wasn't found on the system. Contains the used name.
    NotFound(String),
    /// The execution of Libre-Office failed. Contains the input and output path as well as the
    /// error message.
    ExecutionFailed(PathBuf, PathBuf, String),
    /// The executable was found but calling failed.
    CallFailed(IOError),
}

impl fmt::Display for LibreOfficeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            LibreOfficeError::NotFound(executable) => match executable == &LIBREOFFICE_CMD {
                true => write!(
                    f, 
                    "couldn't find \"{}\" on your system, use the env \"{}\" to define a non default executable name",
                    LIBREOFFICE_CMD,
                    LIBREOFFICE_ENV,
                ),
                false => write!(
                    f,
                    "couldn't find libreoffice with the executable name \"{}\" use env \"{}\" to specify otherwise",
                    executable,
                    LIBREOFFICE_ENV,
                ),

            },
            LibreOfficeError::ExecutionFailed(input, output, err) => write!(
                f,
                "couldn't convert {} to {}, {}",
                input.display(),
                output.display(),
                err,
            ),
            LibreOfficeError::CallFailed(err) => write!(
                f,
                "couldn't call libreoffice {}",
                err,
            ),
        }
    }
}

/// Wrapps Libre-Office.
pub struct LibreOffice(String);

impl LibreOffice {
    /// Returns a new instance of the LibreOffice struct. Determines the name of the executable
    /// based on the content of the LIBREOFFICE_ENV environment variable, defaults to
    /// LIBREOFFICE_CMD.
    pub fn new() -> Self {
        Self(match env::var(LIBREOFFICE_ENV) {
            Ok(x) => x,
            Err(_) => String::from(LIBREOFFICE_CMD),
        })
    }

    /// Calls the actual conversion from a office document into a PDF file.
    pub fn convert_to_pdf(
        &self,
        input: &PathBuf,
    ) -> Result<(), LibreOfficeError> {
        let mut cmd = Command::new(self.0.clone());
        cmd.arg("--headless")
            .arg("--convert-to")
            .arg("pdf:writer_pdf_Export")
            .arg("-env:UserInstallation=file:///tmp/LibreOffice_Conversion_${USER}")
            .arg(input);
        match cmd.output() {
            Ok(x) => {
                if x.status.success() {
                    Ok(())
                } else {
                    Err(LibreOfficeError::ExecutionFailed(input.to_path_buf(), PathBuf::from("ho"), String::from_utf8(x.stderr).unwrap()))
                }
            }
            Err(e) => {
                if let ErrorKind::NotFound = e.kind() {
                    Err(LibreOfficeError::NotFound(self.0.clone()))
                } else {
                    Err(LibreOfficeError::CallFailed(e))
                }
            }
        }
    }
}
