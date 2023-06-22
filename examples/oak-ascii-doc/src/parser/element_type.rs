use crate::lexer::AsciiDocTokenType;
use oak_core::{ElementType, UniversalElementRole};
use serde::{Deserialize, Serialize};

/// Represents all possible element kinds in the AsciiDoc markup language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AsciiDocElementType {
    /// A wrapper for tokens
    Token(AsciiDocTokenType),
    /// Section level 1 header
    Header1,
    /// Section level 2 header
    Header2,
    /// Section level 3 header
    Header3,
    /// Section level 4 header
    Header4,
    /// Section level 5 header
    Header5,
    /// Section level 6 header
    Header6,

    /// Bold text
    Bold,
    /// Italic text
    Italic,
    /// Monospace text
    Monospace,
    /// Superscript text
    Superscript,
    /// Subscript text
    Subscript,
    /// Highlight text
    Highlight,
    /// Strikethrough text
    Strikethrough,

    /// Unordered list item
    UnorderedListItem,
    /// Ordered list item
    OrderedListItem,
    /// Description list item
    DescriptionListItem,

    /// Code block
    CodeBlock,
    /// Quote block
    QuoteBlock,
    /// Sidebar block
    SidebarBlock,
    /// Example block
    ExampleBlock,
    /// Literal block
    LiteralBlock,
    /// Passthrough block
    PassthroughBlock,

    /// Link
    Link,
    /// Image
    Image,
    /// Cross reference
    CrossReference,
    /// Anchor
    Anchor,

    /// Table cell
    TableCell,

    /// Attribute
    Attribute,
    /// Macro
    Macro,
    /// Include directive
    Include,

    /// Root node of the document
    SourceFile,
    /// Error element
    Error,
}

impl From<AsciiDocTokenType> for AsciiDocElementType {
    fn from(token: AsciiDocTokenType) -> Self {
        Self::Token(token)
    }
}

impl ElementType for AsciiDocElementType {
    type Role = UniversalElementRole;

    fn is_root(&self) -> bool {
        matches!(self, Self::SourceFile)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::SourceFile => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}
