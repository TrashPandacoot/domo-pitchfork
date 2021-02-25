use std::fmt;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct DomoErr(pub String);
impl fmt::Display for DomoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl std::error::Error for DomoErr{}

/// Domo returns this on non-ok status codes.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomoApiError {
    pub status: u16,
    pub status_reason: Option<String>,
    pub message: String,
    pub path: Option<String>,
    pub toe: Option<String>,
}

impl std::error::Error for DomoApiError {}
impl std::fmt::Display for DomoApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Domo API Error: {} {} {:?}",
            self.status,
            self.message,
            self.toe.as_ref()
        )
    }
}
