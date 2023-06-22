use oak_core::{Language, LanguageCategory};

/// GraphQL 语言实现
#[derive(Debug, Clone)]
pub struct GraphQLLanguage {}

impl Language for GraphQLLanguage {
    const NAME: &'static str = "graphql";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::kind::GraphQLSyntaxKind;
    type ElementType = crate::kind::GraphQLSyntaxKind;
    type TypedRoot = ();
}
