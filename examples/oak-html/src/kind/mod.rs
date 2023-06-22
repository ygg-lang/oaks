use oak_core::{ElementType, TokenType, UniversalElementRole, UniversalTokenRole};
use serde::{Deserialize, Serialize};

/// HTML 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HtmlSyntaxKind {
    // 标签相关
    TagOpen,      // <
    TagClose,     // >
    TagSlashOpen, // </
    TagSelfClose, // />

    // 标签名和属性
    TagName,
    AttributeName,
    AttributeValue,

    // 文本内容
    Text,

    // 注释
    Comment, // <!-- -->

    // 特殊字符
    Equal, // =
    Quote, // " '

    // DOCTYPE
    Doctype, // <!DOCTYPE html>

    // CDATA
    CData, // <![CDATA[...]]>

    // 处理指令
    ProcessingInstruction, // <?xml ... ?>

    // 实体引用
    EntityRef, // &amp; &lt;
    CharRef,   // &#123; &#x1A;

    // 空白和换行
    Whitespace,
    Newline,

    // 特殊
    Document,
    Element,
    Eof,
    Error,
}

impl TokenType for HtmlSyntaxKind {
    const END_OF_STREAM: Self = Self::Eof;
    type Role = UniversalTokenRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace | Self::Newline => UniversalTokenRole::Whitespace,
            Self::Comment => UniversalTokenRole::Comment,
            Self::TagName => UniversalTokenRole::Name,
            Self::AttributeName => UniversalTokenRole::Name,
            Self::AttributeValue | Self::Text | Self::CData | Self::ProcessingInstruction | Self::EntityRef | Self::CharRef => UniversalTokenRole::Literal,
            Self::TagOpen | Self::TagClose | Self::TagSlashOpen | Self::TagSelfClose | Self::Equal | Self::Quote | Self::Doctype => UniversalTokenRole::Punctuation,
            Self::Eof => UniversalTokenRole::Eof,
            _ => UniversalTokenRole::None,
        }
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }
}

impl ElementType for HtmlSyntaxKind {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Document => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::Document)
    }
}
