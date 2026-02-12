#![doc = include_str!("readme.md")]
use oak_core::language::{Language, LanguageCategory};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerilogLanguage {}

impl VerilogLanguage {
    pub fn new() -> Self {
        Self {}
    }
}

// Define Verilog root node type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogRoot {
    // Here can contain the top-level structure of Verilog modules
    // Temporarily use a simple placeholder
}

impl Language for VerilogLanguage {
    const NAME: &'static str = "verilog";
    const CATEGORY: LanguageCategory = LanguageCategory::Programming;

    type TokenType = crate::lexer::token_type::VerilogTokenType;
    type ElementType = crate::parser::element_type::VerilogElementType;
    type TypedRoot = crate::ast::VerilogRoot;
}
