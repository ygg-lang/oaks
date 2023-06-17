use crate::kind::ObjectiveCLanguageSyntaxKind;
use oak_core::Language;

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
        Self::new()
    }
}

impl Language for ObjectiveCLanguage {
    type SyntaxKind = ObjectiveCLanguageSyntaxKind;
}
