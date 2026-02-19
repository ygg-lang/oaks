use oak_core::{ElementType, UniversalElementRole};

/// Element types for the reStructuredText language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum RstElementType {
    /// The root of the document.
    Root,
    /// A paragraph of text.
    Paragraph,
    /// A heading level 1.
    Heading1,
    /// A heading level 2.
    Heading2,
    /// A heading level 3.
    Heading3,
    /// A heading level 4.
    Heading4,
    /// A heading level 5.
    Heading5,
    /// A heading level 6.
    Heading6,
    /// A comment.
    Comment,
    /// A directive.
    Directive,
    /// A substitution reference.
    SubstitutionReference,
    /// A role.
    Role,
    /// A footnote reference.
    FootnoteReference,
    /// A footnote definition.
    FootnoteDefinition,
    /// A citation reference.
    CitationReference,
    /// A citation definition.
    CitationDefinition,
    /// A bullet list.
    BulletList,
    /// An enumerated list.
    EnumeratedList,
    /// A list item.
    ListItem,
    /// A definition list.
    DefinitionList,
    /// A definition list term.
    DefinitionTerm,
    /// A definition list definition.
    DefinitionDefinition,
    /// A blockquote.
    Blockquote,
    /// A code block.
    CodeBlock,
    /// A horizontal rule.
    HorizontalRule,
    /// A table.
    Table,
    /// A table row.
    TableRow,
    /// A table cell.
    TableCell,
    /// Emphasized text.
    Emphasis,
    /// Strong text.
    Strong,
    /// Literal text.
    Literal,
    /// A link.
    Link,
    /// A reference name.
    ReferenceName,
    /// A reference target.
    ReferenceTarget,
    /// An admonition.
    Admonition,
    /// Plain text.
    Text,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// An error element.
    Error,
}

impl ElementType for RstElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> <Self as ElementType>::Role {
        match self {
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::RstTokenType> for RstElementType {
    fn from(token: crate::lexer::token_type::RstTokenType) -> Self {
        match token {
            crate::lexer::token_type::RstTokenType::Text => RstElementType::Text,
            crate::lexer::token_type::RstTokenType::Whitespace => RstElementType::Whitespace,
            crate::lexer::token_type::RstTokenType::Newline => RstElementType::Newline,
            crate::lexer::token_type::RstTokenType::Comment => RstElementType::Comment,
            crate::lexer::token_type::RstTokenType::Directive => RstElementType::Directive,
            crate::lexer::token_type::RstTokenType::SubstitutionReference => RstElementType::SubstitutionReference,
            crate::lexer::token_type::RstTokenType::Role => RstElementType::Role,
            crate::lexer::token_type::RstTokenType::FootnoteReference => RstElementType::FootnoteReference,
            crate::lexer::token_type::RstTokenType::FootnoteDefinition => RstElementType::FootnoteDefinition,
            crate::lexer::token_type::RstTokenType::CitationReference => RstElementType::CitationReference,
            crate::lexer::token_type::RstTokenType::CitationDefinition => RstElementType::CitationDefinition,
            crate::lexer::token_type::RstTokenType::BulletListMarker => RstElementType::ListItem,
            crate::lexer::token_type::RstTokenType::EnumeratedListMarker => RstElementType::ListItem,
            crate::lexer::token_type::RstTokenType::DefinitionTerm => RstElementType::DefinitionTerm,
            crate::lexer::token_type::RstTokenType::DefinitionDefinition => RstElementType::DefinitionDefinition,
            crate::lexer::token_type::RstTokenType::BlockquoteMarker => RstElementType::Blockquote,
            crate::lexer::token_type::RstTokenType::CodeBlock => RstElementType::CodeBlock,
            crate::lexer::token_type::RstTokenType::Heading1 => RstElementType::Heading1,
            crate::lexer::token_type::RstTokenType::Heading2 => RstElementType::Heading2,
            crate::lexer::token_type::RstTokenType::Heading3 => RstElementType::Heading3,
            crate::lexer::token_type::RstTokenType::Heading4 => RstElementType::Heading4,
            crate::lexer::token_type::RstTokenType::Heading5 => RstElementType::Heading5,
            crate::lexer::token_type::RstTokenType::Heading6 => RstElementType::Heading6,
            crate::lexer::token_type::RstTokenType::HorizontalRule => RstElementType::HorizontalRule,
            crate::lexer::token_type::RstTokenType::Table => RstElementType::Table,
            crate::lexer::token_type::RstTokenType::TableCell => RstElementType::TableCell,
            crate::lexer::token_type::RstTokenType::Emphasis => RstElementType::Emphasis,
            crate::lexer::token_type::RstTokenType::Strong => RstElementType::Strong,
            crate::lexer::token_type::RstTokenType::Literal => RstElementType::Literal,
            crate::lexer::token_type::RstTokenType::Link => RstElementType::Link,
            crate::lexer::token_type::RstTokenType::ReferenceName => RstElementType::ReferenceName,
            crate::lexer::token_type::RstTokenType::ReferenceTarget => RstElementType::ReferenceTarget,
            _ => RstElementType::Error,
        }
    }
}
