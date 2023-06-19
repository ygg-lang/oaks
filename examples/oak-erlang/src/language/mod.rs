use crate::{ast::ErlangRoot, kind::ErlangSyntaxKind};
use oak_core::Language;

/// Erlang 语言配置
#[derive(Debug, Clone, Default)]
pub struct ErlangLanguage;

impl Language for ErlangLanguage {
    type SyntaxKind = ErlangSyntaxKind;
    type TypedRoot = ErlangRoot;
}
