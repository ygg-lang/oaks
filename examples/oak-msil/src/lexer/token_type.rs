use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type MsilToken = Token<MsilTokenType>;

impl TokenType for MsilTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Error;

    fn is_ignored(&self) -> bool {
        false
    }

    fn role(&self) -> Self::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MsilTokenType {
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
