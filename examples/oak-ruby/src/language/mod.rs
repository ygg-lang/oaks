use crate::kind::RubySyntaxKind;
use oak_core::Language;

/// Ruby 语言实现
#[derive(Default)]
pub struct RubyLanguage;

impl Language for RubyLanguage {
    type SyntaxKind = RubySyntaxKind;
    type TypedRoot = ();
}
