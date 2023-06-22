use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum XmlSyntaxKind {
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

impl TokenType for XmlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }
}

impl ElementType for XmlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Element => UniversalElementRole::Container,
            Self::Attribute => UniversalElementRole::Detail,
            Self::Prolog => UniversalElementRole::Detail,
            Self::StartTag | Self::EndTag | Self::SelfClosingTag => UniversalElementRole::Detail,
            Self::Comment => UniversalElementRole::None,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
