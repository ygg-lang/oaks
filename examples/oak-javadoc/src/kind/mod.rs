#![allow(unused)]

use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum JavadocSyntaxKind {
    // 基础词法
    Whitespace,
    Newline,
    CommentStart,
    CommentEnd,

    // Javadoc 标签
    JavadocTag,
    Tag,
    ParamTag,
    ReturnTag,
    ThrowsTag,
    SeeTag,
    SinceTag,
    AuthorTag,
    VersionTag,
    DeprecatedTag,
    InheritDocTag,
    CodeTag,
    LiteralTag,
    ValueTag,
    ExceptionTag,
    LinkTag,
    LinkPlainTag,

    // HTML 标签
    HtmlTag,
    HtmlEndTag,
    HtmlPTag,
    HtmlBrTag,
    HtmlCodeTag,
    HtmlPreTag,
    HtmlUlTag,
    HtmlOlTag,
    HtmlLiTag,
    HtmlATag,
    HtmlImgTag,
    HtmlTableTag,
    HtmlTrTag,
    HtmlTdTag,
    HtmlThTag,
    HtmlBlockquoteTag,
    HtmlH1Tag,
    HtmlH2Tag,
    HtmlH3Tag,
    HtmlH4Tag,
    HtmlH5Tag,
    HtmlH6Tag,
    HtmlBTag,
    HtmlITag,
    HtmlEmTag,
    HtmlStrongTag,
    HtmlSpanTag,
    HtmlDivTag,
    HtmlTtTag,
    HtmlKbdTag,
    HtmlVarTag,
    HtmlSampTag,
    HtmlSubTag,
    HtmlSupTag,
    HtmlSmallTag,
    HtmlBigTag,
    HtmlDelTag,
    HtmlInsTag,
    HtmlCiteTag,
    HtmlDfnTag,
    HtmlAbbrTag,
    HtmlAcronymTag,
    HtmlQTag,

    // 文本内容
    Text,

    // 星号
    Asterisk,

    // 特殊字符
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    At,
    Hash,

    // 文件结束
    Eof,

    // 错误
    Error,

    // 根节点
    Root,
}

impl TokenType for JavadocSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::CommentStart | Self::CommentEnd => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for JavadocSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
