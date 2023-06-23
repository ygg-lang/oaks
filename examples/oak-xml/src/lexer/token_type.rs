use oak_core::{Source, Token, TokenType, UniversalElementRole, UniversalTokenRole};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub type XmlToken = Token<XmlTokenType>;

impl TokenType for XmlTokenType {
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum XmlTokenType {
    // 基本 kind
    Root,
    Whitespace,
    Newline,
    Comment,
    Text,
    Error,
    Eof,

    // XML 特定
    XmlDeclaration,
    DoctypeDeclaration,
    ProcessingInstruction,
    CData,

    // 标签
    StartTag,
    EndTag,
    SelfClosingTag,
    TagName,

    AttributeName,
    AttributeValue,

    // 字面量
    StringLiteral,

    // 标点符号
    LeftAngle,       // <
    RightAngle,      // >
    LeftAngleSlash,  // </
    SlashRightAngle, // />
    Equals,          // =
    Quote,           // "
    SingleQuote,     // '
    Exclamation,     // !
    Question,        // ?
    Ampersand,       // &
    Semicolon,       // ;

    // 实体引用
    EntityReference,
    CharacterReference,

    // 标识符
    Identifier,

    // 非终结符
    SourceFile,
    Element,
    Attribute,
    Prolog,
}
