use oak_core::{ElementType, Token, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

pub type MsilToken = Token<MsilSyntaxKind>;

/// 统一MSIL 语法种类（包含节点与词法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MsilSyntaxKind {
    // 节点种类
    Root,
    Assembly,
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

impl TokenType for MsilSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::CommentToken | Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken | Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

impl ElementType for MsilSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::ErrorNode | Self::Error => UniversalElementRole::Error,
            Self::Root => UniversalElementRole::Root,
            Self::Method | Self::Class => UniversalElementRole::Detail,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Root)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error | Self::ErrorNode)
    }
}
