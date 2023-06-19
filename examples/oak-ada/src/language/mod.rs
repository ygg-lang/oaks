use crate::{ast::AdaRoot, kind::AdaSyntaxKind};
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AdaLanguage;

impl Language for AdaLanguage {
    type SyntaxKind = AdaSyntaxKind;
    type TypedRoot = AdaRoot;
}
