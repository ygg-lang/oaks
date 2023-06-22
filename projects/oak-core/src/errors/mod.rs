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
/// error information including error kind and/// precise source location.
#[derive(Clone)]
pub struct OakError {
    kind: Box<OakErrorKind>,
}

impl std::fmt::Debug for OakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
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
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },
    /// Unexpected character encountered during lexical analysis.
    UnexpectedCharacter {
        /// The character that was not expected at this position.
        character: char,
        /// The byte offset where the unexpected character was found.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Unexpected token encountered during parsing.
    UnexpectedToken {
        /// The token that was not expected.
        token: String,
        /// The byte offset where the unexpected token was found.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Unexpected end of file encountered during parsing.
    UnexpectedEof {
        /// The byte offset where the EOF was encountered.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Custom error for user-defined error conditions.
    CustomError {
        /// The error message describing the custom error condition.
        message: String,
    },

    /// Invalid theme error for highlighting.
    InvalidTheme {
        /// The error message.
        message: String,
    },

    /// Unsupported format error for exporting.
    UnsupportedFormat {
        /// The unsupported format.
        format: String,
    },

    /// Color parsing error for themes.
    ColorParseError {
        /// The invalid color string.
        color: String,
    },

    /// Formatting error.
    FormatError {
        /// The error message.
        message: String,
    },

    /// Semantic error.
    SemanticError {
        /// The error message.
        message: String,
    },

    /// Protocol error (e.g., MCP, LSP).
    ProtocolError {
        /// The error message.
        message: String,
    },

    /// Expected a specific token.
    ExpectedToken {
        /// The token that was expected.
        expected: String,
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Expected a name (identifier).
    ExpectedName {
        /// The kind of name that was expected (e.g., "function name").
        name_kind: String,
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Trailing comma is not allowed.
    TrailingCommaNotAllowed {
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional URL of the file that caused the error.
        url: Option<Url>,
    },

    /// Test failure error.
    TestFailure {
        /// The file that failed the test.
        path: std::path::PathBuf,
        /// The expected output.
        expected: String,
        /// The actual output.
        actual: String,
    },

    /// Test expected result file was missing or regenerated.
    TestRegenerated {
        /// The file that was regenerated.
        path: std::path::PathBuf,
    },
}

impl OakErrorKind {
    /// Gets the i18n key for this error kind.
    pub fn key(&self) -> &'static str {
        match self {
            OakErrorKind::IoError { .. } => "error.io",
            OakErrorKind::SyntaxError { .. } => "error.syntax",
            OakErrorKind::UnexpectedCharacter { .. } => "error.unexpected_character",
            OakErrorKind::UnexpectedToken { .. } => "error.unexpected_token",
            OakErrorKind::UnexpectedEof { .. } => "error.unexpected_eof",
            OakErrorKind::CustomError { .. } => "error.custom",
            OakErrorKind::InvalidTheme { .. } => "error.invalid_theme",
            OakErrorKind::UnsupportedFormat { .. } => "error.unsupported_format",
            OakErrorKind::ColorParseError { .. } => "error.color_parse",
            OakErrorKind::FormatError { .. } => "error.format",
            OakErrorKind::SemanticError { .. } => "error.semantic",
            OakErrorKind::ProtocolError { .. } => "error.protocol",
            OakErrorKind::ExpectedToken { .. } => "error.expected_token",
            OakErrorKind::ExpectedName { .. } => "error.expected_name",
            OakErrorKind::TrailingCommaNotAllowed { .. } => "error.trailing_comma_not_allowed",
            OakErrorKind::TestFailure { .. } => "error.test_failure",
            OakErrorKind::TestRegenerated { .. } => "error.test_regenerated",
        }
    }
}

impl OakError {
    /// Creates a test failure error.
    pub fn test_failure(path: std::path::PathBuf, expected: String, actual: String) -> Self {
        OakErrorKind::TestFailure { path, expected, actual }.into()
    }

    /// Creates a test regenerated error.
    pub fn test_regenerated(path: std::path::PathBuf) -> Self {
        OakErrorKind::TestRegenerated { path }.into()
    }

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
    /// * `offset` - The byte offset where the error occurred
    /// * `url` - Optional URL of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    ///
    /// let error = OakError::syntax_error("Unexpected token", 5, None);
    /// ```
    pub fn syntax_error(message: impl Into<String>, offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::SyntaxError { message: message.into(), offset, url }.into()
    }

    /// Creates an unexpected character error.
    ///
    /// # Arguments
    ///
    /// * `character` - The unexpected character
    /// * `offset` - The byte offset where the character was found
    /// * `url` - Optional URL of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    ///
    /// let error = OakError::unexpected_character('$', 0, None);
    /// ```
    pub fn unexpected_character(character: char, offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::UnexpectedCharacter { character, offset, url }.into()
    }

    /// Creates an unexpected token error.
    pub fn unexpected_token(token: impl Into<String>, offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::UnexpectedToken { token: token.into(), offset, url }.into()
    }

    /// Creates an unexpected end of file error.
    pub fn unexpected_eof(offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::UnexpectedEof { offset, url }.into()
    }

    /// Creates an expected token error.
    pub fn expected_token(expected: impl Into<String>, offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::ExpectedToken { expected: expected.into(), offset, url }.into()
    }

    /// Creates an expected name error.
    pub fn expected_name(name_kind: impl Into<String>, offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::ExpectedName { name_kind: name_kind.into(), offset, url }.into()
    }

    /// Creates a trailing comma not allowed error.
    pub fn trailing_comma_not_allowed(offset: usize, url: Option<Url>) -> Self {
        OakErrorKind::TrailingCommaNotAllowed { offset, url }.into()
    }

    /// Creates a custom error for user-defined error conditions.with a message.
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

    /// Creates an invalid theme error.
    pub fn invalid_theme(message: impl Into<String>) -> Self {
        OakErrorKind::InvalidTheme { message: message.into() }.into()
    }

    /// Creates an unsupported format error.
    pub fn unsupported_format(format: impl Into<String>) -> Self {
        OakErrorKind::UnsupportedFormat { format: format.into() }.into()
    }

    /// Creates a color parsing error.
    pub fn color_parse_error(color: impl Into<String>) -> Self {
        OakErrorKind::ColorParseError { color: color.into() }.into()
    }

    /// Creates a formatting error.
    pub fn format_error(message: impl Into<String>) -> Self {
        OakErrorKind::FormatError { message: message.into() }.into()
    }

    /// Creates a semantic error.
    pub fn semantic_error(message: impl Into<String>) -> Self {
        OakErrorKind::SemanticError { message: message.into() }.into()
    }

    /// Creates a protocol error.
    pub fn protocol_error(message: impl Into<String>) -> Self {
        OakErrorKind::ProtocolError { message: message.into() }.into()
    }

    /// Returns a reference to the error kind.
    ///
    /// # Returns
    ///
    /// A reference to the [`OakErrorKind`] enum that categorizes this error.
    pub fn kind(&self) -> &OakErrorKind {
        &self.kind
    }

    /// Attach a URL to the error context.
    pub fn with_url(mut self, url: Url) -> Self {
        match self.kind.as_mut() {
            OakErrorKind::IoError { url: u, .. } => *u = Some(url),
            OakErrorKind::SyntaxError { url: u, .. } => *u = Some(url),
            OakErrorKind::UnexpectedCharacter { url: u, .. } => *u = Some(url),
            OakErrorKind::UnexpectedToken { url: u, .. } => *u = Some(url),
            OakErrorKind::ExpectedToken { url: u, .. } => *u = Some(url),
            OakErrorKind::ExpectedName { url: u, .. } => *u = Some(url),
            OakErrorKind::TrailingCommaNotAllowed { url: u, .. } => *u = Some(url),
            _ => {}
        }
        self
    }
}

impl Clone for OakErrorKind {
    fn clone(&self) -> Self {
        match self {
            OakErrorKind::IoError { error, url } => {
                // Since std::io::Error doesn't support Clone, we create a new error
                let new_error = std::io::Error::new(error.kind(), error.to_string());
                OakErrorKind::IoError { error: new_error, url: url.clone() }
            }
            OakErrorKind::SyntaxError { message, offset, url } => OakErrorKind::SyntaxError { message: message.clone(), offset: *offset, url: url.clone() },
            OakErrorKind::UnexpectedCharacter { character, offset, url } => OakErrorKind::UnexpectedCharacter { character: *character, offset: *offset, url: url.clone() },
            OakErrorKind::UnexpectedToken { token, offset, url } => OakErrorKind::UnexpectedToken { token: token.clone(), offset: *offset, url: url.clone() },
            OakErrorKind::UnexpectedEof { offset, url } => OakErrorKind::UnexpectedEof { offset: *offset, url: url.clone() },
            OakErrorKind::ExpectedToken { expected, offset, url } => OakErrorKind::ExpectedToken { expected: expected.clone(), offset: *offset, url: url.clone() },
            OakErrorKind::ExpectedName { name_kind, offset, url } => OakErrorKind::ExpectedName { name_kind: name_kind.clone(), offset: *offset, url: url.clone() },
            OakErrorKind::TrailingCommaNotAllowed { offset, url } => OakErrorKind::TrailingCommaNotAllowed { offset: *offset, url: url.clone() },
            OakErrorKind::CustomError { message } => OakErrorKind::CustomError { message: message.clone() },
            OakErrorKind::InvalidTheme { message } => OakErrorKind::InvalidTheme { message: message.clone() },
            OakErrorKind::UnsupportedFormat { format } => OakErrorKind::UnsupportedFormat { format: format.clone() },
            OakErrorKind::ColorParseError { color } => OakErrorKind::ColorParseError { color: color.clone() },
            OakErrorKind::FormatError { message } => OakErrorKind::FormatError { message: message.clone() },
            OakErrorKind::SemanticError { message } => OakErrorKind::SemanticError { message: message.clone() },
            OakErrorKind::ProtocolError { message } => OakErrorKind::ProtocolError { message: message.clone() },
            OakErrorKind::TestFailure { path, expected, actual } => OakErrorKind::TestFailure { path: path.clone(), expected: expected.clone(), actual: actual.clone() },
            OakErrorKind::TestRegenerated { path } => OakErrorKind::TestRegenerated { path: path.clone() },
        }
    }
}
