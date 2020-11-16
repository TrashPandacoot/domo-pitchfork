use std::error::Error;
use std::fmt;
use std::io;

/// used to represent all `domo_pitchfork` errors
#[derive(Debug)]
pub struct PitchforkError {
    pub kind: PitchforkErrorKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[derive(Clone, Debug)]
pub enum PitchforkErrorKind {
    /// errors from the reqwest crate.
    Reqwest,
    /// Errors from csv serialization failures.
    Csv,
    /// Errors from serialization/deserialization of JSON.
    Serde,
    /// Domo Server errors with HTTP response status code and response body.
    DomoBadRequest(u16, String),
    /// Io Error.
    Io,
    Unknown,
}

impl PitchforkError {
    pub fn from(e: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        Self {
            kind: PitchforkErrorKind::Unknown,
            source: Some(e.into()),
        }
    }
    pub fn with_source<E>(mut self, e: E) -> Self
    where
        E: 'static + Error + Send + Sync,
    {
        self.source = Some(Box::new(e));
        self
    }

    pub fn new<T>(e: T) -> Self
    where
        T: Into<Box<dyn Error + Send + Sync>>,
    {
        Self {
            kind: PitchforkErrorKind::Unknown,
            source: Some(e.into())
        }
    }

    /// Change the `kind` for a PitchforkError
    /// This is useful if you're trying to do something like:
    /// Err(PitchforkError::from(e).with_kind(PitchforkErrorKind:Csv)
    pub fn with_kind(&mut self, k: PitchforkErrorKind) {
        self.kind = k;
    }
}

impl Error for PitchforkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|boxed| boxed.as_ref() as &(dyn Error + 'static))
    }
}

impl fmt::Display for PitchforkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PitchforkErrorKind::Reqwest => write!(f, "Reqwest Error in domo_pitchfork"),
            PitchforkErrorKind::Csv => write!(f, "Csv Error in domo_pitchfork"),
            PitchforkErrorKind::Serde => write!(f, "Serde Error in domo_pitchfork"),
            PitchforkErrorKind::DomoBadRequest(status_code, response_body) => write!(f, "HTTP {}: {}", status_code, response_body),
            PitchforkErrorKind::Unknown => write!(f, "Unknown Pitchfork Error"),
            PitchforkErrorKind::Io => write!(f, "io::Error"),
        }
    }
}

impl From<Box<dyn Error + Send + Sync>> for PitchforkError {
    fn from(e: Box<dyn Error + Send + Sync>) -> Self {
        Self {
            kind: PitchforkErrorKind::Unknown,
            source: Some(e),
        }
    }
}

impl From<PitchforkErrorKind> for PitchforkError {
    fn from(kind: PitchforkErrorKind) -> Self {
        Self { kind, source: None }
    }
}

impl From<io::Error> for PitchforkError {
    fn from(err: io::Error) -> Self {
        Self {
            kind: PitchforkErrorKind::Io,
            source: Some(Box::new(err)),
        }
    }
}

// impl From<reqwest::Error> for PitchforkError {
//     fn from(err: reqwest::Error) -> Self {
//         Self {
//             kind: PitchforkErrorKind::Reqwest,
//             source: Some(Box::new(err)),
//         }
//     }
// }

impl From<csv::Error> for PitchforkError {
    fn from(err: csv::Error) -> Self {
        Self {
            kind: PitchforkErrorKind::Csv,
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for PitchforkError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            kind: PitchforkErrorKind::Serde,
            source: Some(Box::new(err)),
        }
    }
}

impl From<()> for PitchforkError {
    fn from(_: ()) -> Self {
        Self {
            kind: PitchforkErrorKind::Unknown,
            source: None,
        }
    }
}




#[derive(Debug)]
pub struct DomoErr(pub String);
impl fmt::Display for DomoErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl std::error::Error for DomoErr{}