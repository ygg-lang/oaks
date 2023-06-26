use oak_core::{TokenType, UniversalTokenRole};

/// Token types for the reStructuredText language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum RstTokenType {
    /// Plain text.
    Text,
    /// Whitespace.
    Whitespace,
    /// A newline.
    Newline,
    /// A comment.
    Comment,
    /// A heading level 1 underline or overline.
    Heading1,
    /// A heading level 2 underline or overline.
    Heading2,
    /// A heading level 3 underline or overline.
    Heading3,
    /// A heading level 4 underline or overline.
    Heading4,
    /// A heading level 5 underline or overline.
    Heading5,
    /// A heading level 6 underline or overline.
    Heading6,
    /// A directive.
    Directive,
    /// A directive argument.
    DirectiveArgument,
    /// A directive option.
    DirectiveOption,
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
    /// A bullet list item marker.
    BulletListMarker,
    /// An enumerated list item marker.
    EnumeratedListMarker,
    /// A definition list term.
    DefinitionTerm,
    /// A definition list definition.
    DefinitionDefinition,
    /// A blockquote marker.
    BlockquoteMarker,
    /// A code block.
    CodeBlock,
    /// A code block language.
    CodeBlockLanguage,
    /// A horizontal rule.
    HorizontalRule,
    /// A table.
    Table,
    /// A table cell.
    TableCell,
    /// A table separator.
    TableSeparator,
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
    /// An error token.
    Error,
    /// End of stream.
    EndOfStream,
}

impl TokenType for RstTokenType {
    const END_OF_STREAM: Self = Self::EndOfStream;
    type Role = UniversalTokenRole;

    fn role(&self) -> <Self as TokenType>::Role {
        match self {
            _ => UniversalTokenRole::None,
        }
    }

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}
