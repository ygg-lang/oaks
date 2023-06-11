use crate::errors::{OakError, OakErrorKind};
use alloc::boxed::Box;

impl From<OakErrorKind> for OakError {
    fn from(kind: OakErrorKind) -> Self {
        Self { kind: Box::new(kind) }
    }
}
