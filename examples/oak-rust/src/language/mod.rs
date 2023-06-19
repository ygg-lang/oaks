use crate::{ast::RustRoot, kind::RustSyntaxKind};
use oak_core::Language;

/// Configuration for the Rust language parser.
///
/// This struct contains language-specific options that control parsing behavior
/// and compatibility with different versions or extensions of Rust.
#[derive(Copy, Clone, Debug)]
pub struct RustLanguage {
    /// Allow `@gc_pointer` kind in old rust
    ///
    /// This flag enables support for the experimental garbage collection pointer kind
    /// that was briefly considered for early Rust versions. When enabled, the parser
    /// will recognize `@gc_pointer` annotations on types.
    pub gc_pointer: bool,
    /// Allow `box Class {}` kind in old rust
    ///
    /// This flag enables support for the deprecated box kind that was used in early
    /// Rust versions before the current `Box::new()` kind was standardized.
    /// When enabled, the parser will recognize expressions like `box Class {}`.
    pub box_syntax: bool,
}

/// Default implementation for RustLanguage.
///
/// Creates a RustLanguage instance with all experimental features disabled,
/// corresponding to the standard modern Rust kind.
impl Default for RustLanguage {
    fn default() -> Self {
        Self {
            // no longer used
            gc_pointer: false,
            // no longer used
            box_syntax: false,
        }
    }
}

/// Implementation of the Language trait for RustLanguage.
///
/// This connects the language configuration to the specific kind kinds
/// and AST root type used for Rust parsing.
impl Language for RustLanguage {
    type SyntaxKind = RustSyntaxKind;
    type TypedRoot = RustRoot;
}
