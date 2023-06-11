//! Display implementations for error types.

use crate::errors::{OakError, OakErrorKind};
use crate::source::SourceLocation;
use core::fmt::{Debug, Display, Formatter};

impl Display for OakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for OakErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "std")]
            OakErrorKind::IoError { error, url } => {
                if let Some(url) = url {
                    write!(f, "I/O error at {}: {}", url, error)
                }
                else {
                    write!(f, "I/O error: {}", error)
                }
            }
            OakErrorKind::SyntaxError { message, source } => {
                write!(f, "Syntax error at {}: {}", source, message)
            }
            OakErrorKind::UnexpectedCharacter { character, source } => {
                write!(f, "Unexpected character '{}' at {}", character, source)
            }
            OakErrorKind::CustomError { message } => {
                write!(f, "{}", message)
            }
        }
     }
}
