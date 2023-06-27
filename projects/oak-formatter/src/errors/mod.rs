use oak_core::errors::OakError;

/// Format result type
///
/// This type represents the result of a formatting operation.
pub type FormatResult<T> = Result<T, OakError>;
