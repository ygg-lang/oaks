use crate::{ast::DHallRoot, kind::DHallSyntaxKind};
use oak_core::Language;

#[derive(Copy, Clone, Debug)]
pub struct DHallLanguage {
    /// Allow unicode identifiers
    pub unicode_identifiers: bool,
}

impl Default for DHallLanguage {
    fn default() -> Self {
        Self { unicode_identifiers: true }
    }
}

impl Language for DHallLanguage {
    type SyntaxKind = DHallSyntaxKind;
    type TypedRoot = DHallRoot;
}
