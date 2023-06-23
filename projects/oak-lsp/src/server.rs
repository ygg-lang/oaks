use crate::service::LanguageService;
use std::{
    error::Error,
    fmt::{Display, Formatter},
    sync::Arc,
};
use tokio::io::{AsyncRead, AsyncWrite};

/// Errors that can occur during LSP communication.
#[derive(Debug)]
pub enum LspError {
    /// Input/Output error.
    Io(std::io::Error),
    /// UTF-8 decoding error.
    Utf8(std::string::FromUtf8Error),
    /// JSON serialization/deserialization error.
    Json(String),
    /// Other miscellaneous errors.
    Other(String),
}

impl Display for LspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LspError::Io(e) => write!(f, "IO error: {}", e),
            LspError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            LspError::Json(e) => write!(f, "JSON error: {}", e),
            LspError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl Error for LspError {}

impl From<std::io::Error> for LspError {
    fn from(e: std::io::Error) -> Self {
        LspError::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for LspError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        LspError::Utf8(e)
    }
}

impl From<serde_json::Error> for LspError {
    fn from(e: serde_json::Error) -> Self {
        LspError::Json(e.to_string())
    }
}

/// A language server that handles LSP requests and notifications.
pub struct LspServer<S: LanguageService> {
    _service: Arc<S>,
}

impl<S: LanguageService> LspServer<S> {
    /// Creates a new `LspServer` with the given language service.
    pub fn new(service: Arc<S>) -> Self {
        Self { _service: service }
    }

    /// Runs the language server on the given input and output streams.
    pub async fn run<R, W>(&self, mut _read: R, mut _write: W) -> Result<(), LspError>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        // Basic skeleton for now to satisfy compilation
        Ok(())
    }
}
