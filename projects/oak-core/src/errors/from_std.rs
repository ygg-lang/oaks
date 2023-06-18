use crate::errors::{OakError, OakErrorKind};

impl From<OakErrorKind> for OakError {
    fn from(kind: OakErrorKind) -> Self {
        Self { kind: Box::new(kind) }
    }
}
