//! Display implementations for error types.

use crate::errors::{OakError, OakErrorKind};

use core::fmt::{Display, Formatter};
use std::error::Error;

impl Error for OakError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.kind.source()
    }
}

impl Display for OakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Error for OakErrorKind {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OakErrorKind::IoError { error, .. } => Some(error),
            _ => None,
        }
    }
}

impl Display for OakErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            OakErrorKind::IoError { error, url } => {
                if let Some(url) = url {
                    write!(f, "I/O error at {}: {}", url, error)
                }
                else {
                    write!(f, "I/O error: {}", error)
                }
            }
            OakErrorKind::SyntaxError { message, offset, url } => {
                if let Some(url) = url {
                    write!(f, "Syntax error at {}:{}: {}", url, offset, message)
                }
                else {
                    write!(f, "Syntax error at {}: {}", offset, message)
                }
            }
            OakErrorKind::UnexpectedCharacter { character, offset, url } => {
                if let Some(url) = url {
                    write!(f, "Unexpected character '{}' at {}:{}", character, url, offset)
                }
                else {
                    write!(f, "Unexpected character '{}' at {}", character, offset)
                }
            }
            OakErrorKind::UnexpectedToken { token, offset, url } => {
                if let Some(url) = url {
                    write!(f, "Unexpected token '{}' at {}:{}", token, url, offset)
                }
                else {
                    write!(f, "Unexpected token '{}' at {}", token, offset)
                }
            }
            OakErrorKind::UnexpectedEof { offset, url } => {
                if let Some(url) = url {
                    write!(f, "Unexpected end of file at {}:{}", url, offset)
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
            OakErrorKind::ExpectedToken { expected, offset, .. } => {
                write!(f, "Expected '{}' at {}", expected, offset)
            }
            OakErrorKind::ExpectedName { name_kind, offset, .. } => {
                write!(f, "Expected {} at {}", name_kind, offset)
            }
            OakErrorKind::TrailingCommaNotAllowed { offset, .. } => {
                write!(f, "Trailing comma not allowed at {}", offset)
            }
            OakErrorKind::TestFailure { path, expected, actual } => {
                write!(f, "\x1b[31;1mFAIL\x1b[0m \x1b[36m{}\x1b[0m\n\x1b[32m- Exp:\x1b[0m {}\n\x1b[31m- Act:\x1b[0m {}", path.display(), expected, actual)
            }
            OakErrorKind::TestRegenerated { path } => {
                write!(f, "\x1b[33;1mREGEN\x1b[0m \x1b[36m{}\x1b[0m\n(Please verify and rerun)", path.display())
            }
        }
    }
}
