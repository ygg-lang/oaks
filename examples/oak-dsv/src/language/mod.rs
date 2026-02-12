#![doc = include_str!("readme.md")]
use crate::{lexer::token_type::DsvTokenType, parser::element_type::DsvElementType};
use oak_core::{Language, LanguageCategory};
#[cfg(feature = "serde")]
mod deserializer;
#[cfg(feature = "serde")]
mod serializer;
#[cfg(feature = "serde")]
pub use deserializer::from_value;
#[cfg(feature = "serde")]
pub use serializer::to_value;

/// DSV language implementation for the Oaks framework.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, std::marker::ConstParamTy)]
pub struct DsvLanguage {
    /// Field separator, defaults to comma (`,`).
    pub field_separator: char,
    /// Quote character, defaults to double quote (`"`).
    pub quote_char: char,
}

impl Default for DsvLanguage {
    fn default() -> Self {
        Self { field_separator: ',', quote_char: '"' }
    }
}

impl DsvLanguage {
    /// Sets the field separator.
    pub const fn with_separator(mut self, separator: char) -> Self {
        self.field_separator = separator;
        self
    }

    /// Sets the quote character.
    pub const fn with_quote_char(mut self, quote: char) -> Self {
        self.quote_char = quote;
        self
    }
}

/// DSV language marker with configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dsv<const L: DsvLanguage>;

impl<const L: DsvLanguage> Language for Dsv<L> {
    const NAME: &'static str = "dsv";
    const CATEGORY: LanguageCategory = LanguageCategory::Config;

    type TokenType = DsvTokenType;
    type ElementType = DsvElementType;
    type TypedRoot = crate::ast::DsvRoot<L>;
}
