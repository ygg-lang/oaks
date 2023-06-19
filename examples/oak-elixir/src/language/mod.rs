use oak_core::Language;

use crate::{ast::ElixirRoot, kind::ElixirSyntaxKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ElixirLanguage;

impl Language for ElixirLanguage {
    type SyntaxKind = ElixirSyntaxKind;
    type TypedRoot = ElixirRoot;
}
