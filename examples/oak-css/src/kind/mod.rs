#![allow(non_camel_case_types)]

use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CssSyntaxKind {
    // Trivia
    Whitespace = 0,
    Newline,
    Comment,

    // Literals
    StringLiteral,
    NumberLiteral,
    ColorLiteral,
    UrlLiteral,

    // Identifiers and keywords
    Identifier,
    ClassName,
    IdSelector,
    TagName,
    PseudoClass,
    PseudoElement,
    AttributeName,
    PropertyName,
    FunctionName,

    // CSS Keywords
    Important,
    Inherit,
    Initial,
    Unset,
    Auto,
    None,
    Normal,

    // Units
    Px,
    Em,
    Rem,
    Percent,
    Vh,
    Vw,
    Pt,
    Cm,
    Mm,
    In,
    Pc,
    Ex,
    Ch,
    Vmin,
    Vmax,

    // Operators and punctuation
    Colon,       // :
    Semicolon,   // ;
    Comma,       // ,
    Dot,         // .
    Hash,        // #
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Equals,      // =
    Tilde,       // ~
    Pipe,        // |
    Caret,       // ^
    Dollar,      // $
    GreaterThan, // >

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // At-rules
    AtRule,
    AtImport,
    AtMedia,
    AtKeyframes,
    AtFontFace,
    AtCharset,
    AtNamespace,
    AtSupports,
    AtPage,
    AtDocument,

    // Special tokens
    MediaQuery,
    Selector,
    Declaration,
    Value,
    Function,
    Url,
    CalcExpression,

    // Compound nodes
    SourceFile,
    Error,
    Eof,
}

impl SyntaxKind for CssSyntaxKind {
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
        !matches!(
            self,
            Self::SourceFile
                | Self::MediaQuery
                | Self::Selector
                | Self::Declaration
                | Self::Value
                | Self::Function
                | Self::Url
                | Self::CalcExpression
        )
    }

    fn is_element_type(&self) -> bool {
        matches!(
            self,
            Self::SourceFile
                | Self::MediaQuery
                | Self::Selector
                | Self::Declaration
                | Self::Value
                | Self::Function
                | Self::Url
                | Self::CalcExpression
        )
    }
}
