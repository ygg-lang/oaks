#[doc = include_str!("readme.md")]
use crate::{ast::RustRoot, lexer::RustTokenType, parser::RustElementType};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Rust 语言配置和元数据。
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RustLanguage {
    /// Allow using `unsafe` blocks and functions
    pub allow_unsafe: bool,
    /// Allow using `async` functions and blocks
    pub allow_async: bool,
    /// Enable experimental features
    pub experimental_features: bool,
}

impl Default for RustLanguage {
    fn default() -> Self {
        Self { allow_unsafe: true, allow_async: true, experimental_features: false }
    }
}

impl Language for RustLanguage {
    const NAME: &'static str = "rust";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = RustTokenType;
    type ElementType = RustElementType;
    type TypedRoot = RustRoot;
}
