use oak_core::{ElementType, UniversalElementRole};

/// LLVM IR element types for the parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum LLvmElementType {
    /// Root node of the program.
    Root,
    /// Identifier.
    Identifier,
    /// Numeric literal.
    Number,
    /// String literal.
    String,
    /// Comment.
    Comment,
    /// Whitespace.
    Whitespace,
    /// Newline.
    Newline,
    /// Error node.
    Error,
    /// End of file.
    Eof,
    // Added variants
    /// Local variable (starts with %).
    LocalVar,
    /// Global variable (starts with @).
    GlobalVar,
    /// Metadata.
    Metadata,
    /// Equal sign (=).
    Equal,
    /// Comma (,).
    Comma,
    /// Left parenthesis (().
    LParen,
    /// Right parenthesis ()).
    RParen,
    /// Left bracket ([).
    LBracket,
    /// Right bracket (]).
    RBracket,
    /// Left brace ({).
    LBrace,
    /// Right brace (}).
    RBrace,
    /// Star (*).
    Star,
    /// Colon (:).
    Colon,
    /// Keyword.
    Keyword,
    // Structured elements
    /// Top-level item.
    Item,
    /// Global variable declaration.
    Global,
    /// Function definition.
    Function,
    /// Function parameter.
    Parameter,
    /// Basic block.
    Block,
    /// Instruction.
    Instruction,
    /// Type annotation.
    Type,
    /// Operand.
    Operand,
}

impl ElementType for LLvmElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Function => UniversalElementRole::Container,
            Self::Block => UniversalElementRole::Container,
            Self::Instruction => UniversalElementRole::Statement,
            Self::Global => UniversalElementRole::Binding,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::LLvmTokenType> for LLvmElementType {
    fn from(token: crate::lexer::token_type::LLvmTokenType) -> Self {
                match token {
            crate::lexer::token_type::LLvmTokenType::Root => Self::Root,
            crate::lexer::token_type::LLvmTokenType::Identifier => Self::Identifier,
            crate::lexer::token_type::LLvmTokenType::Number => Self::Number,
            crate::lexer::token_type::LLvmTokenType::String => Self::String,
            crate::lexer::token_type::LLvmTokenType::Comment => Self::Comment,
            crate::lexer::token_type::LLvmTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::LLvmTokenType::Newline => Self::Newline,
            crate::lexer::token_type::LLvmTokenType::Error => Self::Error,
            crate::lexer::token_type::LLvmTokenType::Eof => Self::Eof,
            crate::lexer::token_type::LLvmTokenType::LocalVar => Self::LocalVar,
            crate::lexer::token_type::LLvmTokenType::GlobalVar => Self::GlobalVar,
            crate::lexer::token_type::LLvmTokenType::Metadata => Self::Metadata,
            crate::lexer::token_type::LLvmTokenType::Equal => Self::Equal,
            crate::lexer::token_type::LLvmTokenType::Comma => Self::Comma,
            crate::lexer::token_type::LLvmTokenType::LParen => Self::LParen,
            crate::lexer::token_type::LLvmTokenType::RParen => Self::RParen,
            crate::lexer::token_type::LLvmTokenType::LBracket => Self::LBracket,
            crate::lexer::token_type::LLvmTokenType::RBracket => Self::RBracket,
            crate::lexer::token_type::LLvmTokenType::LBrace => Self::LBrace,
            crate::lexer::token_type::LLvmTokenType::RBrace => Self::RBrace,
            crate::lexer::token_type::LLvmTokenType::Star => Self::Star,
            crate::lexer::token_type::LLvmTokenType::Colon => Self::Colon,
            crate::lexer::token_type::LLvmTokenType::Keyword => Self::Keyword,
            _ => Self::Error,
        }
    }
}
