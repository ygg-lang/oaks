use oak_core::{Language, LanguageCategory};

pub struct ObjectiveCLanguage {
    pub arc_enabled: bool,
    pub strict_mode: bool,
}

impl ObjectiveCLanguage {
    pub fn new() -> Self {
        Self { arc_enabled: true, strict_mode: false }
    }

    pub fn with_arc(mut self, enabled: bool) -> Self {
        self.arc_enabled = enabled;
        self
    }

    pub fn with_strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = enabled;
        self
    }
}

impl Default for ObjectiveCLanguage {
    fn default() -> Self {
        Self { arc_enabled: true, strict_mode: false }
    }
}

impl Language for ObjectiveCLanguage {
    const NAME: &'static str = "objective-c";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::kind::ObjectiveCSyntaxKind;
    type ElementType = crate::kind::ObjectiveCSyntaxKind;
    type TypedRoot = crate::ast::ObjectiveCRoot;
}
