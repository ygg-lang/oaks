#![doc = include_str!("readme.md")]
use oak_core::{Language, LanguageCategory};

/// GraphQL 语言实现
#[derive(Debug, Clone)]
pub struct GraphQLLanguage {}

impl Language for GraphQLLanguage {
    const NAME: &'static str = "graphql";
    const CATEGORY: LanguageCategory = LanguageCategory::Dsl;

    type TokenType = crate::lexer::token_type::GraphQLTokenType;
    type ElementType = crate::parser::element_type::GraphQLElementType;
    type TypedRoot = ();
}
