use crate::errors::{OakError, OakErrorKind};
use serde_json::Error;

impl From<Error> for OakError {
    fn from(error: Error) -> Self {
        OakErrorKind::SyntaxError { message: error.to_string(), source: Default::default() }.into()
    }
}
