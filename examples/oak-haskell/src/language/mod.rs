use crate::kind::HaskellSyntaxKind;
use oak_core::{Arc, GreenNode, Language};

#[derive(Debug, Clone, Default)]
pub struct HaskellLanguage;

impl Language for HaskellLanguage {
    type SyntaxKind = HaskellSyntaxKind;
    type TypedRoot = Arc<GreenNode<HaskellSyntaxKind>>;
}
