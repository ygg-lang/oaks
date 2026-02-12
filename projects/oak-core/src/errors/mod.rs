use crate::source::SourceId;

mod display;
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

impl<T> OakDiagnostics<T> {
    /// Creates a new OakDiagnostics with the given result and no diagnostics.
    pub fn new(result: Result<T, OakError>) -> Self {
        Self { result, diagnostics: Vec::new() }
    }

    /// Creates a new OakDiagnostics with a successful result.
    pub fn success(value: T) -> Self {
        Self { result: Ok(value), diagnostics: Vec::new() }
    }

    /// Creates a new OakDiagnostics with a fatal error.
    pub fn error(error: OakError) -> Self {
        Self { result: Err(error), diagnostics: Vec::new() }
    }

    /// Returns true if there are any fatal errors or diagnostics.
    pub fn has_errors(&self) -> bool {
        self.result.is_err() || !self.diagnostics.is_empty()
    }
}

impl<'a, L: crate::Language> OakDiagnostics<&'a crate::tree::GreenNode<'a, L>> {
    /// Returns the successful green node result, panicking on error.
    pub fn green(&self) -> &'a crate::tree::GreenNode<'a, L> {
        self.result.as_ref().expect("Failed to get green node from parse output")
    }
}

/// The main error type for the Oak Core parsing framework.
///
/// `OakError` represents all possible language that can occur during
/// lexical analysis and parsing operations. It provides detailed
/// error information including error kind and precise source location.
#[derive(Clone)]
pub struct OakError {
    /// The specific kind of error.
    kind: Box<OakErrorKind>,
}

impl OakError {
    /// Creates a new OakError with the given kind.
    pub fn new(kind: OakErrorKind) -> Self {
        Self { kind: Box::new(kind) }
    }

    /// Creates a new custom error with the given message.
    pub fn custom_error(message: impl Into<String>) -> Self {
        Self::new(OakErrorKind::CustomError { message: message.into() })
    }
}

impl std::fmt::Debug for OakError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for OakError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        OakError::serde_error(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for OakError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        OakError::deserialize_error(msg.to_string())
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
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },
    /// Syntax error encountered during parsing.
    SyntaxError {
        /// The error message.
        message: String,
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },
    /// Unexpected character encountered during lexical analysis.
    UnexpectedCharacter {
        /// The character that was not expected at this position.
        character: char,
        /// The byte offset where the unexpected character was found.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },

    /// Unexpected token encountered during parsing.
    UnexpectedToken {
        /// The token that was not expected.
        token: String,
        /// The byte offset where the unexpected token was found.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },

    /// Unexpected end of file encountered during parsing.
    UnexpectedEof {
        /// The byte offset where the EOF was encountered.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },

    /// Custom error for user-defined error conditions.
    CustomError {
        /// The error message.
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
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },

    /// Expected a name (identifier).
    ExpectedName {
        /// The kind of name that was expected (e.g., "function name").
        name_kind: String,
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
    },

    /// Trailing comma is not allowed.
    TrailingCommaNotAllowed {
        /// The byte offset where the error occurred.
        offset: usize,
        /// Optional source ID of the file that caused the error.
        source_id: Option<SourceId>,
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

    /// Test regenerated.
    TestRegenerated {
        /// The file that was regenerated.
        path: std::path::PathBuf,
    },

    /// Serde error.
    SerdeError {
        /// The error message.
        message: String,
    },

    /// Serde deserialization error.
    DeserializeError {
        /// The error message.
        message: String,
    },

    /// XML error.
    XmlError {
        /// The error message.
        message: String,
    },

    /// Zip error.
    ZipError {
        /// The error message.
        message: String,
    },

    /// Parse error.
    ParseError {
        /// The error message.
        message: String,
    },

    /// Internal error.
    InternalError {
        /// The error message.
        message: String,
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
            OakErrorKind::SerdeError { .. } => "error.serde",
            OakErrorKind::DeserializeError { .. } => "error.deserialize",
            OakErrorKind::XmlError { .. } => "error.xml",
            OakErrorKind::ZipError { .. } => "error.zip",
            OakErrorKind::ParseError { .. } => "error.parse",
            OakErrorKind::InternalError { .. } => "error.internal",
        }
    }
}

impl OakError {
    /// Gets the kind of this error.
    pub fn kind(&self) -> &OakErrorKind {
        &self.kind
    }

    /// Creates a test failure error.
    pub fn test_failure(path: std::path::PathBuf, expected: String, actual: String) -> Self {
        OakErrorKind::TestFailure { path, expected, actual }.into()
    }

    /// Creates a test regenerated error.
    pub fn test_regenerated(path: std::path::PathBuf) -> Self {
        OakErrorKind::TestRegenerated { path }.into()
    }

    /// Creates an I/O error with optional Source ID.
    ///
    /// # Arguments
    ///
    /// * `error` - The underlying I/O error
    /// * `source_id` - Source ID of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    /// # use std::io;
    ///
    /// let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    /// let error = OakError::io_error(io_err, 1);
    /// ```

    pub fn io_error(error: std::io::Error, source_id: SourceId) -> Self {
        OakErrorKind::IoError { error, source_id: Some(source_id) }.into()
    }

    /// Creates a kind error with a message and location.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the kind error
    /// * `offset` - The byte offset where the error occurred
    /// * `source_id` - Optional source ID of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    ///
    /// let error = OakError::syntax_error("Unexpected token", 5, None);
    /// ```
    pub fn syntax_error(message: impl Into<String>, offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::SyntaxError { message: message.into(), offset, source_id }.into()
    }

    /// Creates an unexpected character error.
    ///
    /// # Arguments
    ///
    /// * `character` - The unexpected character
    /// * `offset` - The byte offset where the character was found
    /// * `source_id` - Optional source ID of the file that caused the error
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use oak_core::OakError;
    ///
    /// let error = OakError::unexpected_character('$', 0, None);
    /// ```
    pub fn unexpected_character(character: char, offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::UnexpectedCharacter { character, offset, source_id }.into()
    }

    /// Creates an unexpected token error.
    pub fn unexpected_token(token: impl Into<String>, offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::UnexpectedToken { token: token.into(), offset, source_id }.into()
    }

    /// Creates an unexpected end of file error.
    pub fn unexpected_eof(offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::UnexpectedEof { offset, source_id }.into()
    }

    /// Creates an expected token error.
    pub fn expected_token(expected: impl Into<String>, offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::ExpectedToken { expected: expected.into(), offset, source_id }.into()
    }

    /// Creates an expected name error.
    pub fn expected_name(name_kind: impl Into<String>, offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::ExpectedName { name_kind: name_kind.into(), offset, source_id }.into()
    }

    /// Creates a trailing comma not allowed error.
    pub fn trailing_comma_not_allowed(offset: usize, source_id: Option<SourceId>) -> Self {
        OakErrorKind::TrailingCommaNotAllowed { offset, source_id }.into()
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

    /// Creates a serde error.
    pub fn serde_error(message: impl Into<String>) -> Self {
        OakErrorKind::SerdeError { message: message.into() }.into()
    }

    /// Creates a serde deserialization error.
    pub fn deserialize_error(message: impl Into<String>) -> Self {
        OakErrorKind::DeserializeError { message: message.into() }.into()
    }

    /// Creates an XML error.
    pub fn xml_error(message: impl Into<String>) -> Self {
        OakErrorKind::XmlError { message: message.into() }.into()
    }

    /// Creates a zip error.
    pub fn zip_error(message: impl Into<String>) -> Self {
        OakErrorKind::ZipError { message: message.into() }.into()
    }

    /// Creates a parse error.
    pub fn parse_error(message: impl Into<String>) -> Self {
        OakErrorKind::ParseError { message: message.into() }.into()
    }

    /// Creates an internal error.
    pub fn internal_error(message: impl Into<String>) -> Self {
        OakErrorKind::InternalError { message: message.into() }.into()
    }

    /// Attach a source ID to the error context.
    pub fn with_source_id(mut self, source_id: SourceId) -> Self {
        match self.kind.as_mut() {
            OakErrorKind::IoError { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::SyntaxError { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::UnexpectedCharacter { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::UnexpectedToken { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::ExpectedToken { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::ExpectedName { source_id: u, .. } => *u = Some(source_id),
            OakErrorKind::TrailingCommaNotAllowed { source_id: u, .. } => *u = Some(source_id),
            _ => {}
        }
        self
    }
}

impl Clone for OakErrorKind {
    fn clone(&self) -> Self {
        match self {
            OakErrorKind::IoError { error, source_id } => {
                // Since std::io::Error doesn't support Clone, we create a new error
                let new_error = std::io::Error::new(error.kind(), error.to_string());
                OakErrorKind::IoError { error: new_error, source_id: *source_id }
            }
            OakErrorKind::SyntaxError { message, offset, source_id } => OakErrorKind::SyntaxError { message: message.clone(), offset: *offset, source_id: *source_id },
            OakErrorKind::UnexpectedCharacter { character, offset, source_id } => OakErrorKind::UnexpectedCharacter { character: *character, offset: *offset, source_id: *source_id },
            OakErrorKind::UnexpectedToken { token, offset, source_id } => OakErrorKind::UnexpectedToken { token: token.clone(), offset: *offset, source_id: *source_id },
            OakErrorKind::UnexpectedEof { offset, source_id } => OakErrorKind::UnexpectedEof { offset: *offset, source_id: *source_id },
            OakErrorKind::ExpectedToken { expected, offset, source_id } => OakErrorKind::ExpectedToken { expected: expected.clone(), offset: *offset, source_id: *source_id },
            OakErrorKind::ExpectedName { name_kind, offset, source_id } => OakErrorKind::ExpectedName { name_kind: name_kind.clone(), offset: *offset, source_id: *source_id },
            OakErrorKind::TrailingCommaNotAllowed { offset, source_id } => OakErrorKind::TrailingCommaNotAllowed { offset: *offset, source_id: *source_id },
            OakErrorKind::CustomError { message } => OakErrorKind::CustomError { message: message.clone() },
            OakErrorKind::InvalidTheme { message } => OakErrorKind::InvalidTheme { message: message.clone() },
            OakErrorKind::UnsupportedFormat { format } => OakErrorKind::UnsupportedFormat { format: format.clone() },
            OakErrorKind::ColorParseError { color } => OakErrorKind::ColorParseError { color: color.clone() },
            OakErrorKind::FormatError { message } => OakErrorKind::FormatError { message: message.clone() },
            OakErrorKind::SemanticError { message } => OakErrorKind::SemanticError { message: message.clone() },
            OakErrorKind::ProtocolError { message } => OakErrorKind::ProtocolError { message: message.clone() },
            OakErrorKind::TestFailure { path, expected, actual } => OakErrorKind::TestFailure { path: path.clone(), expected: expected.clone(), actual: actual.clone() },
            OakErrorKind::TestRegenerated { path } => OakErrorKind::TestRegenerated { path: path.clone() },
            OakErrorKind::SerdeError { message } => OakErrorKind::SerdeError { message: message.clone() },
            OakErrorKind::DeserializeError { message } => OakErrorKind::DeserializeError { message: message.clone() },
            OakErrorKind::XmlError { message } => OakErrorKind::XmlError { message: message.clone() },
            OakErrorKind::ZipError { message } => OakErrorKind::ZipError { message: message.clone() },
            OakErrorKind::ParseError { message } => OakErrorKind::ParseError { message: message.clone() },
            OakErrorKind::InternalError { message } => OakErrorKind::InternalError { message: message.clone() },
        }
    }
}
