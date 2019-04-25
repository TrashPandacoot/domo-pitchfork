use std::fmt;
use std::io;
#[derive(Debug)]
/// DomoPitchfork Error definitions.
pub enum DomoError {
    /// Errors from the reqwest crate when HTTP fails
    Reqwest(reqwest::Error),
    /// Errors from the csv crate when csv serialization fails.
    Csv(csv::Error),
    /// These are errors with ser/de JSON in RustyPitchfork.
    Serde(serde_json::Error),
    /// These are errors for RustyPitchfork. Not sure what I intended to
    /// use the `usize` for. TODO: figure out why I made it a usize. If it
    /// wasn't just something picked arbitrarily while I was learning error
    /// handling, update the docs here to explain what/why.
    Pitchfork(usize),
    /// Catch-all for errors
    Other(String),
}

impl fmt::Display for DomoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomoError::Reqwest(r) => write!(f, "Reqwest Error: {}", &r),
            DomoError::Csv(c) => write!(f, "Csv Error: {}", &c),
            DomoError::Serde(s) => write!(f, "Serde Error: {}", &s),
            DomoError::Pitchfork(u) => write!(f, "Pitchfork Error: {}", u),
            DomoError::Other(s) => write!(f, "Pitchfork Error Other: {}", &s),
        }
    }
}

impl From<csv::IntoInnerError<csv::Writer<std::vec::Vec<u8>>>> for DomoError {
    fn from(_err: csv::IntoInnerError<csv::Writer<std::vec::Vec<u8>>>) -> Self {
        // TODO: figure out why I would leave this error like this.
        // and change it to a more appropriate err type if appropriate.
        DomoError::Pitchfork(2)
    }
}

impl From<std::string::FromUtf8Error> for DomoError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        // TODO: figure out why I would leave this error like this.
        // and change it to a more appropriate err type if appropriate.
        // learning effort maybe?
        DomoError::Pitchfork(2)
    }
}

/// Convert JSON serde errors to `DomoError` type.
impl From<serde_json::Error> for DomoError {
    fn from(err: serde_json::Error) -> Self {
        DomoError::Serde(err)
    }
}

/// Convert csv crate errors into `DomoError` type.
// This would likely be an error serializing to csv.
impl From<csv::Error> for DomoError {
    fn from(err: csv::Error) -> Self {
        if !err.is_io_error() {
            return DomoError::Csv(err);
        }
        DomoError::Pitchfork(2)
    }
}

/// Convert reqwest errors to `DomoError` type.
impl From<reqwest::Error> for DomoError {
    fn from(err: reqwest::Error) -> Self {
        DomoError::Reqwest(err)
    }
}

impl From<String> for DomoError {
    fn from(err: String) -> Self {
        DomoError::Other(err)
    }
}

impl<'a> From<&'a str> for DomoError {
    fn from(err: &'a str) -> Self {
        DomoError::Other(err.to_owned())
    }
}

impl<'a> From<io::Error> for DomoError {
    fn from(err: io::Error) -> Self {
        DomoError::Other(err.to_string())
    }
}

impl<'a> From<()> for DomoError {
    fn from(_err: ()) -> Self {
        DomoError::Other("() error".to_owned())
    }
}
