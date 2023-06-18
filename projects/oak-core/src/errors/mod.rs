use crate::source::SourceLocation;

use url::Url;

mod display;
#[cfg(feature = "serde_json")]
mod from_serde_json;
mod from_std;
mod source;

/// Result type for lexical analysis operations.
///
/// This type alias represents the result of tokenization operations,
/// where successful operations return a value of type `T` and failed
/// operations return an [`OakError`].
pub type LexResult<T> = Result<T, OakError>;

/// Result type for parsing operations.
///
/// This type alias represents the result of parsing operations,
/// where successful operations return a value of type `T` and failed
/// operations return an [`OakError`].
pub type ParseResult<T> = Result<T, OakError>;

/// Container for parsing results with associated diagnostics.
///
/// This struct holds both the primary result of a parsing operation
/// and any diagnostic language that were encountered during parsing.
/// This allows for error recovery where parsing can continue even
/// after encountering language, collecting all issues for later analysis.

#[derive(Debug, Clone)]
pub struct OakDiagnostics<T> {
    /// The primary result of the parsing operation.
    /// May contain either a successful value or a fatal error.
    pub result: Result<T, OakError>,
    /// A collection of non-fatal errors or warnings encountered during the operation.
    pub diagnostics: Vec<OakError>,
}

/// The main error type for the Oak Core parsing framework.
///
/// `OakError` represents all possible language that can occur during
/// lexical analysis and parsing operations. It provides detailed
/// error information including error kind and precise source location.
#[derive(Debug, Clone)]
pub struct OakError {
    kind: Box<OakErrorKind>,
}

/// Enumeration of all possible error kinds in the Oak Core framework.
///
/// This enum categorizes different types of language that can occur
/// during parsing operations, each with specific associated data
/// relevant to that error type.
#[derive(Debug)]
pub enum OakErrorKind {
    /// I/O error that occurred while reading source files.
    IoError {
        /// The underlying I/O error.
        error: std::io::Error,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },
    /// Syntax error encountered during parsing.
    SyntaxError {
        /// Human-readable error message describing the kind issue.
        message: String,
        /// Location in the source code where the error occurred.
        source: SourceLocation,
    },
    /// Unexpected character encountered during lexical analysis.
    UnexpectedCharacter {
        /// The character that was not expected at this position.
        character: char,
        /// Location in the source code where the unexpected character was found.
        source: SourceLocation,
    },

    /// Custom error for user-defined error conditions.
    CustomError {
        /// The error message describing the custom error condition.
        message: String,
    },
}

impl OakError {
    /// Creates an I/O error with optional file URL.
    ///
    /// # Arguments
    ///
    /// * `error` - The underlying I/O error
    /// * `url` - URL of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    /// # use std::io;
    ///
    /// let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    /// let error = OakError::io_error(io_err, url::Url::parse("file:///main.rs").unwrap());
    /// ```

    pub fn io_error(error: std::io::Error, url: Url) -> Self {
        OakErrorKind::IoError { error, url: Some(url) }.into()
    }

    /// Creates a kind error with a message and location.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the kind error
    /// * `source` - Location in the source where the error occurred
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::{OakError, SourceLocation};
    ///
    /// let location = SourceLocation { line: 1, column: 5, url: None };
    /// let error = OakError::syntax_error("Unexpected token", location);
    /// ```
    pub fn syntax_error(message: impl Into<String>, source: SourceLocation) -> Self {
        OakErrorKind::SyntaxError { message: message.into(), source }.into()
    }

    /// Creates an unexpected character error.
    ///
    /// # Arguments
    ///
    /// * `character` - The unexpected character
    /// * `source` - Location where the character was found
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::{OakError, SourceLocation};
    ///
    /// let location = SourceLocation { line: 1, column: 0, url: None };
    /// let error = OakError::unexpected_character('$', location);
    /// ```
    pub fn unexpected_character(character: char, source: SourceLocation) -> Self {
        OakErrorKind::UnexpectedCharacter { character, source }.into()
    }

    /// Creates a custom error with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message describing what went wrong
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    ///
    /// let error = OakError::custom_error("Invalid configuration");
    /// ```
    pub fn custom_error(message: impl Into<String>) -> Self {
        OakErrorKind::CustomError { message: message.into() }.into()
    }

    /// Returns a reference to the error kind.
    ///
    /// # Returns
    ///
    /// A reference to the [`OakErrorKind`] enum that categorizes this error.
    pub fn kind(&self) -> &OakErrorKind {
        &self.kind
    }
}

impl Clone for OakErrorKind {
    fn clone(&self) -> Self {
        match self {
            OakErrorKind::IoError { error, url } => {
                // 由于 std::io::Error 不支持 Clone，我们创建一个新的错误
                let new_error = std::io::Error::new(error.kind(), error.to_string());
                OakErrorKind::IoError { error: new_error, url: url.clone() }
            }
            OakErrorKind::SyntaxError { message, source } => {
                OakErrorKind::SyntaxError { message: message.clone(), source: source.clone() }
            }
            OakErrorKind::UnexpectedCharacter { character, source } => {
                OakErrorKind::UnexpectedCharacter { character: *character, source: source.clone() }
            }
            OakErrorKind::CustomError { message } => OakErrorKind::CustomError { message: message.clone() },
        }
    }
}
