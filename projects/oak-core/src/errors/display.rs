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
        match self {
            OakErrorKind::IoError { error, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "I/O error at source {}: {}", id, error)
                }
                else {
                    write!(f, "I/O error: {}", error)
                }
            }
            OakErrorKind::SyntaxError { message, offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Syntax error at source {}:{}: {}", id, offset, message)
                }
                else {
                    write!(f, "Syntax error at {}: {}", offset, message)
                }
            }
            OakErrorKind::UnexpectedCharacter { character, offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Unexpected character '{}' at source {}:{}", character, id, offset)
                }
                else {
                    write!(f, "Unexpected character '{}' at {}", character, offset)
                }
            }
            OakErrorKind::UnexpectedToken { token, offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Unexpected token '{}' at source {}:{}", token, id, offset)
                }
                else {
                    write!(f, "Unexpected token '{}' at {}", token, offset)
                }
            }
            OakErrorKind::UnexpectedEof { offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Unexpected end of file at source {}:{}", id, offset)
                }
                else {
                    write!(f, "Unexpected end of file at {}", offset)
                }
            }
            OakErrorKind::CustomError { message } => {
                write!(f, "{}", message)
            }
            OakErrorKind::InvalidTheme { message } => {
                write!(f, "Invalid theme: {}", message)
            }
            OakErrorKind::UnsupportedFormat { format } => {
                write!(f, "Unsupported format: {}", format)
            }
            OakErrorKind::ColorParseError { color } => {
                write!(f, "Color parsing error: {}", color)
            }
            OakErrorKind::FormatError { message } => {
                write!(f, "Format error: {}", message)
            }
            OakErrorKind::SemanticError { message } => {
                write!(f, "Semantic error: {}", message)
            }
            OakErrorKind::ProtocolError { message } => {
                write!(f, "Protocol error: {}", message)
            }
            OakErrorKind::ExpectedToken { expected, offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Expected '{}' at source {}:{}", expected, id, offset)
                }
                else {
                    write!(f, "Expected '{}' at {}", expected, offset)
                }
            }
            OakErrorKind::ExpectedName { name_kind, offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Expected {} at source {}:{}", name_kind, id, offset)
                }
                else {
                    write!(f, "Expected {} at {}", name_kind, offset)
                }
            }
            OakErrorKind::TrailingCommaNotAllowed { offset, source_id } => {
                if let Some(id) = source_id {
                    write!(f, "Trailing comma not allowed at source {}:{}", id, offset)
                }
                else {
                    write!(f, "Trailing comma not allowed at {}", offset)
                }
            }
            OakErrorKind::TestFailure { path, expected, actual } => {
                write!(f, "\x1b[31;1mFAIL\x1b[0m \x1b[36m{}\x1b[0m\n\x1b[32m- Exp:\x1b[0m {}\n\x1b[31m- Act:\x1b[0m {}", path.display(), expected, actual)
            }
            OakErrorKind::TestRegenerated { path } => {
                write!(f, "\x1b[33;1mREGEN\x1b[0m \x1b[36m{}\x1b[0m\n(Please verify and rerun)", path.display())
            }
            OakErrorKind::SerdeError { message } => {
                write!(f, "Serde error: {}", message)
            }
            OakErrorKind::DeserializeError { message } => {
                write!(f, "Deserialize error: {}", message)
            }
            OakErrorKind::XmlError { message } => {
                write!(f, "XML error: {}", message)
            }
            OakErrorKind::ZipError { message } => {
                write!(f, "Zip error: {}", message)
            }
            OakErrorKind::ParseError { message } => {
                write!(f, "Parse error: {}", message)
            }
            OakErrorKind::InternalError { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}
