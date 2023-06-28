/// Fluent language definition module.
use oak_core::Language;

/// Fluent language definition.
pub struct FluentLanguage;

impl Language for FluentLanguage {
    /// The name of the language.
    const NAME: &'static str = "fluent";

    /// The category of the language.
    const CATEGORY: oak_core::LanguageCategory = oak_core::LanguageCategory::Dsl;

    /// The token type used in the language.
    type TokenType = super::lexer::token_type::FluentTokenKind;

    /// The element type used in the language.
    type ElementType = super::parser::element_type::FluentElementType;

    /// The root type of the parsed tree.
    type TypedRoot = super::ast::FluentRoot;
}

/// Deserializes a Fluent string into a Fluent AST.
pub fn from_str(input: &str) -> Result<super::ast::FluentRoot, oak_core::OakError> {
    use super::parser::parse;

    let root = parse(input)?;
    Ok(root)
}

/// Serializes a Fluent AST into a Fluent string.
pub fn to_string(root: &super::ast::FluentRoot) -> String {
    // TODO: Implement serialization
    format!("{:?}", root)
}
