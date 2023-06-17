#![allow(non_camel_case_types)]

use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CsvSyntaxKind {
    // Trivia
    Whitespace = 0,
    Newline,

    // CSV-specific tokens
    Field,         // A field value (quoted or unquoted)
    QuotedField,   // A quoted field value
    UnquotedField, // An unquoted field value
    Comma,         // Field separator ,
    Quote,         // Quote character "
    EscapedQuote,  // Escaped quote ""

    // Special cases
    EmptyField, // Empty field (between commas or at start/end)

    // Compound nodes
    Record,     // A complete CSV record (row)
    SourceFile, // The entire CSV file
    Error,      // Error kind
    Eof,        // End of file
}

impl SyntaxKind for CsvSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_comment(&self) -> bool {
        false // CSV doesn't have comments
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        matches!(
            self,
            Self::Field
                | Self::QuotedField
                | Self::UnquotedField
                | Self::Comma
                | Self::Quote
                | Self::EscapedQuote
                | Self::EmptyField
                | Self::Whitespace
                | Self::Newline
                | Self::Eof
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Record | Self::SourceFile)
    }
}
