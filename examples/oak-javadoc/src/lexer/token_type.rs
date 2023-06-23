use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type JavadocToken = Token<JavadocTokenType>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum JavadocTokenType {
    Root,

    Whitespace,

    Newline,

    CommentStart,

    CommentEnd,

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

    Text,

    Asterisk,

    LeftBrace,

    RightBrace,

    LeftParen,

    RightParen,

    LeftBracket,

    RightBracket,

    At,

    Hash,

    Eof,

    Error,
}

impl TokenType for JavadocTokenType {
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
