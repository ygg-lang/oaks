use oak_core::{Language, LanguageCategory};

/// Ruby 语言实现
#[derive(Default)]
pub struct RubyLanguage;

impl Language for RubyLanguage {
    const NAME: &'static str = "ruby";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::RubySyntaxKind;
    type ElementType = crate::kind::RubySyntaxKind;
    type TypedRoot = crate::ast::ProgramNode;
}
