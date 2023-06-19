use oak_core::SyntaxKind;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum XmlSyntaxKind {
    // 基本 kind
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
}

impl SyntaxKind for XmlSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::XmlDeclaration | Self::DoctypeDeclaration | Self::StartTag | Self::EndTag | Self::SelfClosingTag)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::XmlDeclaration | Self::DoctypeDeclaration | Self::StartTag | Self::EndTag | Self::SelfClosingTag)
    }
}
