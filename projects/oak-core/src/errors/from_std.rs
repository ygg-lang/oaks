use crate::errors::{OakError, OakErrorKind};

impl From<OakErrorKind> for OakError {
    fn from(kind: OakErrorKind) -> Self {
        Self { kind: Box::new(kind) }
    }
}

impl From<std::io::Error> for OakError {
    fn from(error: std::io::Error) -> Self {
        OakErrorKind::IoError { error, source_id: None }.into()
    }
}

impl From<std::num::ParseIntError> for OakError {
    fn from(error: std::num::ParseIntError) -> Self {
        OakErrorKind::ParseError { message: error.to_string() }.into()
    }
}

impl From<std::num::ParseFloatError> for OakError {
    fn from(error: std::num::ParseFloatError) -> Self {
        OakErrorKind::ParseError { message: error.to_string() }.into()
    }
}
