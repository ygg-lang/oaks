use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

/// Ruby 语言实现
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RubyLanguage {}

impl RubyLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for RubyLanguage {
    const NAME: &'static str = "ruby";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::RubySyntaxKind;
    type ElementType = crate::kind::RubySyntaxKind;
    type TypedRoot = crate::ast::ProgramNode;
}
