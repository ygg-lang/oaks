use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SolidityElementType {
    SourceFile,
    Eof,
    Error,
}

impl oak_core::TokenType for SolidityElementType {
    type Role = oak_core::UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn role(&self) -> Self::Role {
        match self {
            _ => oak_core::UniversalTokenRole::None,
        }
    }
}

impl SolidityElementType {
    pub fn is_token_type(&self) -> bool {
        true
    }

    pub fn is_element_type(&self) -> bool {
        false
    }
}

impl ElementType for SolidityElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::SolidityTokenType> for SolidityElementType {
    fn from(token: crate::lexer::token_type::SolidityTokenType) -> Self {
                match token {
            crate::lexer::token_type::SolidityTokenType::Whitespace => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Newline => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LineComment => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BlockComment => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Contract => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Interface => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Library => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Function => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Modifier => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Event => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Struct => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Enum => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Mapping => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Array => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Public => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Private => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Internal => Self::Error,
            crate::lexer::token_type::SolidityTokenType::External => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Pure => Self::Error,
            crate::lexer::token_type::SolidityTokenType::View => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Payable => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Constant => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Bool => Self::Error,
            crate::lexer::token_type::SolidityTokenType::String => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Bytes => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Address => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Uint => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Int => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Fixed => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Ufixed => Self::Error,
            crate::lexer::token_type::SolidityTokenType::If => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Else => Self::Error,
            crate::lexer::token_type::SolidityTokenType::For => Self::Error,
            crate::lexer::token_type::SolidityTokenType::While => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Do => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Break => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Continue => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Return => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Try => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Catch => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Import => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Pragma => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Using => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Is => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Override => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Virtual => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Abstract => Self::Error,
            crate::lexer::token_type::SolidityTokenType::NumberLiteral => Self::Error,
            crate::lexer::token_type::SolidityTokenType::StringLiteral => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BooleanLiteral => Self::Error,
            crate::lexer::token_type::SolidityTokenType::AddressLiteral => Self::Error,
            crate::lexer::token_type::SolidityTokenType::HexLiteral => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Identifier => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Plus => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Minus => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Star => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Slash => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Percent => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Power => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Equal => Self::Error,
            crate::lexer::token_type::SolidityTokenType::NotEqual => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Less => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Greater => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LessEqual => Self::Error,
            crate::lexer::token_type::SolidityTokenType::GreaterEqual => Self::Error,
            crate::lexer::token_type::SolidityTokenType::And => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Or => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Not => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BitAnd => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BitOr => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BitXor => Self::Error,
            crate::lexer::token_type::SolidityTokenType::BitNot => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LeftShift => Self::Error,
            crate::lexer::token_type::SolidityTokenType::RightShift => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Assign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::PlusAssign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::MinusAssign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::StarAssign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::SlashAssign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::PercentAssign => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LeftParen => Self::Error,
            crate::lexer::token_type::SolidityTokenType::RightParen => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LeftBrace => Self::Error,
            crate::lexer::token_type::SolidityTokenType::RightBrace => Self::Error,
            crate::lexer::token_type::SolidityTokenType::LeftBracket => Self::Error,
            crate::lexer::token_type::SolidityTokenType::RightBracket => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Semicolon => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Comma => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Dot => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Arrow => Self::Error,
            crate::lexer::token_type::SolidityTokenType::SourceFile => Self::SourceFile,
            crate::lexer::token_type::SolidityTokenType::Error => Self::Error,
            crate::lexer::token_type::SolidityTokenType::Eof => Self::Eof,
            _ => Self::Error,
        }
    }
}
