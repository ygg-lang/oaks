#![allow(unused)]

use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    // 根节    Root,
}

impl SyntaxKind for JavadocSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::CommentStart | Self::CommentEnd)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root)
    }
}
