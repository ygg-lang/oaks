use crate::kind::HandlebarsSyntaxKind;
use oak_core::{Arc, GreenNode, Language};

#[derive(Default)]
pub struct HandlebarsLanguage {}

impl Language for HandlebarsLanguage {
    type SyntaxKind = HandlebarsSyntaxKind;
    type TypedRoot = Arc<GreenNode<HandlebarsSyntaxKind>>;
}
