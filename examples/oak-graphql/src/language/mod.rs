use crate::{ast::GraphQLRoot, kind::GraphQLSyntaxKind};
use oak_core::Language;

/// GraphQL 语言实现
#[derive(Debug, Clone)]
pub struct GraphQLLanguage {}

impl Language for GraphQLLanguage {
    type SyntaxKind = GraphQLSyntaxKind;
    type TypedRoot = GraphQLRoot;
}
