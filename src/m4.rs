/// Module for the usage of the m4 general-purpose macro processor.
use std::fmt;
use std::io::Error as IOError;

/// Default name of the m4 executable. Will be used when no other name is defined via the
/// `M4_ENV` constant of this module.
const M4_CMD: &str = "m4";

/// Name of the environment variable which will be used to determine the name of the m4 executable.
const M4_ENV: &str = "M4_CMD";

/// Handles the errors in correspondence with the m4 command.
pub enum M4Error<'a> {
    /// The m4 executable wasn't found on the system. Contains the name used for the m4 executable.
    NotFound(&'a str),
    /// The execution of m4 failed. Contains the stderr output.
    ExecutionFailed(String),
    /// Calling of m4 failed. Contains the std::io::Error.
    CallingFailed(IOError),
}

impl fmt::Display for M4Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            M4Error::NotFound(executable) => match executable == &M4_CMD {
                true => write!(
                    f,
                    "couldn't find \"m4\" on your system, use the env \"{}\" to use an non default executable name",
                    M4_ENV
                ),
                false => write!(
                    f,
                    "couldn't find m4 with the executable name \"{}\" use env \"{}\" to specify otherwise",
                    executable,
                    M4_ENV
                ),
            },
            M4Error::ExecutionFailed(err) => write!(
                f,
                "m4 failed with {}",
                err
            ),
            M4Error::CallingFailed(err) => write!(
                f,
                "couldn't call m4 {}",
                err
            ),
        }
    }
}
