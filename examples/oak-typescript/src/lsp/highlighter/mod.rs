#![doc = include_str!("readme.md")]
use crate::{
    language::TypeScriptLanguage,
    lexer::{TypeScriptLexer, token_type::TypeScriptTokenType},
};
use oak_core::{Lexer, SourceText, TextEdit, TokenType, UniversalTokenRole};

/// Local definition of highlight kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// A keyword.
    Keyword,
    /// A string literal.
    String,
    /// A numeric literal.
    Number,
    /// A comment.
    Comment,
    /// An identifier.
    Identifier,
    /// An operator.
    Operator,
    /// A punctuation mark.
    Punctuation,
    /// An error or unknown token.
    Error,
}

/// Highlighter trait for syntax highlighting.
pub trait Highlighter {
    /// Highlights the given text and returns a list of spans with their corresponding highlight kinds.
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Syntax highlighter for TypeScript.
pub struct TypeScriptHighlighter<'config> {
    lexer: TypeScriptLexer<'config>,
}

impl<'config> TypeScriptHighlighter<'config> {
    /// Creates a new `TypeScriptHighlighter` with the given language configuration.
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
        Self { lexer: TypeScriptLexer::new(config) }
    }
}

impl<'config> Highlighter for TypeScriptHighlighter<'config> {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut session = oak_core::parser::session::ParseSession::<TypeScriptLanguage>::default();
        let edits = Vec::<TextEdit>::new();
        let source = SourceText::new(text);
        let output = self.lexer.lex(&source, &edits, &mut session);

        let mut highlights = Vec::new();
        if let Ok(tokens) = output.result {
            for token in tokens.iter() {
                let kind = match token.kind.role() {
                    UniversalTokenRole::Keyword => HighlightKind::Keyword,
                    UniversalTokenRole::Literal => match token.kind {
                        TypeScriptTokenType::StringLiteral | TypeScriptTokenType::TemplateString | TypeScriptTokenType::RegexLiteral => HighlightKind::String,
                        TypeScriptTokenType::NumericLiteral | TypeScriptTokenType::BigIntLiteral => HighlightKind::Number,
                        _ => HighlightKind::String,
                    },
                    UniversalTokenRole::Comment => HighlightKind::Comment,
                    UniversalTokenRole::Name => HighlightKind::Identifier,
                    UniversalTokenRole::Operator => HighlightKind::Operator,
                    UniversalTokenRole::Punctuation => HighlightKind::Punctuation,
                    _ => HighlightKind::Error,
                };
                highlights.push((token.span.start, token.span.end, kind));
            }
        }

        highlights
    }
}
