//! GSGL 语言定义
//!
//! 定义GSGL 语言的核心结构体，实现了 oak-core Language trait
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// GSGL 语言 definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct GsglLanguage {}

impl GsglLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for GsglLanguage {
    const NAME: &'static str = "gsgl";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::syntax::GsglSyntaxKind;
    type ElementType = crate::syntax::GsglSyntaxKind;
    type TypedRoot = ();
}
