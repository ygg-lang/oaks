use oak_core::{ElementType, Parser, UniversalElementRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MsilElementType {
    // 节点种类
    Root,
    Assembly,
    AssemblyExtern,
    Module,
    Class,
    Method,
    Instruction,
    Label,
    Directive,
    Type,
    Identifier,
    Number,
    String,
    Comment,
    ErrorNode,

    // 词法种类 - 关键
    AssemblyKeyword, // .assembly
    ExternKeyword,   // extern
    ModuleKeyword,   // .module
    ClassKeyword,    // .class
    MethodKeyword,   // .method
    PublicKeyword,   // public
    PrivateKeyword,  // private
    StaticKeyword,   // static
    Keyword,         // other keywords
    // 词法种类 - 符号
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    Dot,          // .
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Equal,        // =
    Slash,        // /

    // 词法种类 - 字面
    IdentifierToken,
    NumberToken,
    StringToken,

    // 词法种类 - 其他
    Whitespace,
    CommentToken,
    Eof,
    Error,
}

impl ElementType for MsilElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::MsilTokenType> for MsilElementType {
    fn from(token: crate::lexer::token_type::MsilTokenType) -> Self {
        match token {
            crate::lexer::token_type::MsilTokenType::Root => MsilElementType::Root,
            crate::lexer::token_type::MsilTokenType::Assembly => MsilElementType::Assembly,
            crate::lexer::token_type::MsilTokenType::AssemblyExtern => MsilElementType::AssemblyExtern,
            crate::lexer::token_type::MsilTokenType::Module => MsilElementType::Module,
            crate::lexer::token_type::MsilTokenType::Class => MsilElementType::Class,
            crate::lexer::token_type::MsilTokenType::Method => MsilElementType::Method,
            crate::lexer::token_type::MsilTokenType::Instruction => MsilElementType::Instruction,
            crate::lexer::token_type::MsilTokenType::Label => MsilElementType::Label,
            crate::lexer::token_type::MsilTokenType::Directive => MsilElementType::Directive,
            crate::lexer::token_type::MsilTokenType::Type => MsilElementType::Type,
            crate::lexer::token_type::MsilTokenType::Identifier => MsilElementType::Identifier,
            crate::lexer::token_type::MsilTokenType::Number => MsilElementType::Number,
            crate::lexer::token_type::MsilTokenType::String => MsilElementType::String,
            crate::lexer::token_type::MsilTokenType::Comment => MsilElementType::Comment,
            crate::lexer::token_type::MsilTokenType::ErrorNode => MsilElementType::ErrorNode,
            crate::lexer::token_type::MsilTokenType::AssemblyKeyword => MsilElementType::AssemblyKeyword,
            crate::lexer::token_type::MsilTokenType::ExternKeyword => MsilElementType::ExternKeyword,
            crate::lexer::token_type::MsilTokenType::ModuleKeyword => MsilElementType::ModuleKeyword,
            crate::lexer::token_type::MsilTokenType::ClassKeyword => MsilElementType::ClassKeyword,
            crate::lexer::token_type::MsilTokenType::MethodKeyword => MsilElementType::MethodKeyword,
            crate::lexer::token_type::MsilTokenType::PublicKeyword => MsilElementType::PublicKeyword,
            crate::lexer::token_type::MsilTokenType::PrivateKeyword => MsilElementType::PrivateKeyword,
            crate::lexer::token_type::MsilTokenType::StaticKeyword => MsilElementType::StaticKeyword,
            crate::lexer::token_type::MsilTokenType::Keyword => MsilElementType::Keyword,
            crate::lexer::token_type::MsilTokenType::LeftBrace => MsilElementType::LeftBrace,
            crate::lexer::token_type::MsilTokenType::RightBrace => MsilElementType::RightBrace,
            crate::lexer::token_type::MsilTokenType::LeftParen => MsilElementType::LeftParen,
            crate::lexer::token_type::MsilTokenType::RightParen => MsilElementType::RightParen,
            crate::lexer::token_type::MsilTokenType::LeftBracket => MsilElementType::LeftBracket,
            crate::lexer::token_type::MsilTokenType::RightBracket => MsilElementType::RightBracket,
            crate::lexer::token_type::MsilTokenType::Dot => MsilElementType::Dot,
            crate::lexer::token_type::MsilTokenType::Colon => MsilElementType::Colon,
            crate::lexer::token_type::MsilTokenType::Semicolon => MsilElementType::Semicolon,
            crate::lexer::token_type::MsilTokenType::Comma => MsilElementType::Comma,
            crate::lexer::token_type::MsilTokenType::Equal => MsilElementType::Equal,
            crate::lexer::token_type::MsilTokenType::Slash => MsilElementType::Slash,
            crate::lexer::token_type::MsilTokenType::IdentifierToken => MsilElementType::IdentifierToken,
            crate::lexer::token_type::MsilTokenType::NumberToken => MsilElementType::NumberToken,
            crate::lexer::token_type::MsilTokenType::StringToken => MsilElementType::StringToken,
            crate::lexer::token_type::MsilTokenType::Whitespace => MsilElementType::Whitespace,
            crate::lexer::token_type::MsilTokenType::CommentToken => MsilElementType::CommentToken,
            crate::lexer::token_type::MsilTokenType::Eof => MsilElementType::Eof,
            crate::lexer::token_type::MsilTokenType::Error => MsilElementType::Error,
        }
    }
}
