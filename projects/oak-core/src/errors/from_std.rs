use crate::errors::{OakError, OakErrorKind};

impl From<OakErrorKind> for OakError {
    fn from(kind: OakErrorKind) -> Self {
        Self { kind: Box::new(kind) }
    }
}

impl From<std::io::Error> for OakError {
    fn from(error: std::io::Error) -> Self {
        OakErrorKind::IoError { error, url: None }.into()
    }
}
