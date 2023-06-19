use std::range::Range;

use oak_core::{SyntaxKind, Token};
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

impl SyntaxKind for MsilSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::CommentToken)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentToken)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(
            self,
            Self::Root
                | Self::Assembly
                | Self::Module
                | Self::Class
                | Self::Method
                | Self::Instruction
                | Self::Label
                | Self::Directive
                | Self::Type
                | Self::ErrorNode
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::Root
                | Self::Assembly
                | Self::Module
                | Self::Class
                | Self::Method
                | Self::Instruction
                | Self::Label
                | Self::Directive
                | Self::Type
                | Self::ErrorNode
        )
    }
}
