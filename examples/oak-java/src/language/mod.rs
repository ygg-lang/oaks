use crate::{ast::JavaRoot, kind::JavaSyntaxKind};
use oak_core::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JavaLanguage;

impl Language for JavaLanguage {
    type SyntaxKind = JavaSyntaxKind;
    type TypedRoot = JavaRoot;
}
