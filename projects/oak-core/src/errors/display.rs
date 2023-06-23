//! Display implementations for error types.

use crate::errors::{OakError, OakErrorKind};

use core::fmt::{Display, Formatter};
use std::error::Error;

impl Error for OakError {}

impl Display for OakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Error for OakErrorKind {}

impl Display for OakErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.key())?;
        match self {
            OakErrorKind::IoError { error, source_id } => {
                write!(f, "({:?})", source_id)?;
                write!(f, ": {}", error)
            }
            OakErrorKind::SyntaxError { message, offset, source_id } => {
                write!(f, "{}: (at {:?}:{})", message, source_id, offset)
            }
            OakErrorKind::UnexpectedCharacter { character, offset, source_id } => {
                write!(f, "unexpected character '{}' at {:?}:{}", character, source_id, offset)
            }
            OakErrorKind::UnexpectedToken { token, offset, source_id } => {
                write!(f, "unexpected token '{}' at {:?}:{}", token, source_id, offset)
            }
            OakErrorKind::UnexpectedEof { offset, source_id } => {
                write!(f, "unexpected end of file at {:?}:{}", source_id, offset)
            }
            OakErrorKind::CustomError { message } => write!(f, "{}", message),
            OakErrorKind::InvalidTheme { message } => write!(f, "{}", message),
            OakErrorKind::UnsupportedFormat { format } => write!(f, "unsupported format: {}", format),
            OakErrorKind::ColorParseError { color } => write!(f, "invalid color: {}", color),
            OakErrorKind::FormatError { message } => write!(f, "{}", message),
            OakErrorKind::SemanticError { message } => write!(f, "{}", message),
            OakErrorKind::ProtocolError { message } => write!(f, "{}", message),
            OakErrorKind::ExpectedToken { expected, offset, source_id } => {
                write!(f, "expected token '{}' at {:?}:{}", expected, source_id, offset)
            }
            OakErrorKind::ExpectedName { name_kind, offset, source_id } => {
                write!(f, "expected {} at {:?}:{}", name_kind, source_id, offset)
            }
            OakErrorKind::TrailingCommaNotAllowed { offset, source_id } => {
                write!(f, "trailing comma not allowed at {:?}:{}", source_id, offset)
            }
            OakErrorKind::TestFailure { path, expected, actual } => {
                write!(f, "test failure in {:?}: expected {}, found {}", path, expected, actual)
            }
            OakErrorKind::TestRegenerated { path } => {
                write!(f, "test regenerated: {:?}", path)
            }
            OakErrorKind::SerdeError { message } => write!(f, "serde error: {}", message),
            OakErrorKind::DeserializeError { message } => write!(f, "deserialization error: {}", message),
            OakErrorKind::XmlError { message } => write!(f, "XML error: {}", message),
            OakErrorKind::ZipError { message } => write!(f, "zip error: {}", message),
            OakErrorKind::ParseError { message } => write!(f, "parse error: {}", message),
            OakErrorKind::InternalError { message } => write!(f, "internal error: {}", message),
        }
    }
}
