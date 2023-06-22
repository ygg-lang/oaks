use crate::{ast::ValkyrieRoot, kind::ValkyrieSyntaxKind};
use oak_core::{Language, LanguageCategory};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// The Valkyrie programming language definition.
pub struct ValkyrieLanguage {
    /// Allow using `<xml/>` syntax in source code
    pub allow_xml: bool,
    /// Allow using `<$ template $>` syntax in source code
    pub allow_template: bool,
}

impl Default for ValkyrieLanguage {
    fn default() -> Self {
        Self { allow_xml: false, allow_template: false }
    }
}

impl Language for ValkyrieLanguage {
    const NAME: &'static str = "valkyrie";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = ValkyrieSyntaxKind;
    type ElementType = ValkyrieSyntaxKind;
    type TypedRoot = ValkyrieRoot;
}
