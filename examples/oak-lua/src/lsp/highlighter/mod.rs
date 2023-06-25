#![doc = include_str!("readme.md")]
/// Local definition of highlight kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// Keyword
    Keyword,
    /// String
    String,
    /// Number
    Number,
    /// Comment
    Comment,
    /// Macro
    Macro,
    /// Identifier
    Identifier,
    /// Operator
    Operator,
}

/// Highlighter trait
pub trait Highlighter {
    /// Highlight the given text
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Lua syntax highlighter
pub struct LuaHighlighter;

impl LuaHighlighter {
    /// Creates a new `LuaHighlighter`.
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for LuaHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        // Simple regex or lexer-based highlighting implementation
        // Here we can reuse LuaLexer to get Tokens
        use crate::lexer::{LuaLexer, token_type::LuaTokenType};
        use oak_core::{LexOutput, Lexer, LexerCache, SourceText, Token};

        struct SimpleCache {
            tokens: Vec<Token<LuaTokenType>>,
        }

        impl LexerCache<crate::language::LuaLanguage> for SimpleCache {
            fn set_lex_output(&mut self, output: LexOutput<crate::language::LuaLanguage>) {
                if let Ok(tokens) = output.result {
                    self.tokens = tokens.to_vec()
                }
            }
            fn get_token(&self, index: usize) -> Option<Token<LuaTokenType>> {
                self.tokens.get(index).cloned()
            }
            fn count_tokens(&self) -> usize {
                self.tokens.len()
            }
            fn has_tokens(&self) -> bool {
                !self.tokens.is_empty()
            }
        }

        let config = crate::language::LuaLanguage::default();
        let lexer = LuaLexer::new(&config);
        let mut cache = SimpleCache { tokens: Vec::new() };
        let source = SourceText::new(text);
        let output = lexer.lex(&source, &[], &mut cache);

        if let Ok(tokens) = output.result {
            for token in tokens.iter() {
                let kind = match token.kind {
                    LuaTokenType::And
                    | LuaTokenType::Break
                    | LuaTokenType::Do
                    | LuaTokenType::Else
                    | LuaTokenType::Elseif
                    | LuaTokenType::End
                    | LuaTokenType::False
                    | LuaTokenType::For
                    | LuaTokenType::Function
                    | LuaTokenType::Goto
                    | LuaTokenType::If
                    | LuaTokenType::In
                    | LuaTokenType::Local
                    | LuaTokenType::Nil
                    | LuaTokenType::Not
                    | LuaTokenType::Or
                    | LuaTokenType::Repeat
                    | LuaTokenType::Return
                    | LuaTokenType::Then
                    | LuaTokenType::True
                    | LuaTokenType::Until
                    | LuaTokenType::While => Some(HighlightKind::Keyword),
                    LuaTokenType::String => Some(HighlightKind::String),
                    LuaTokenType::Number => Some(HighlightKind::Number),
                    LuaTokenType::Comment => Some(HighlightKind::Comment),
                    LuaTokenType::Identifier => Some(HighlightKind::Identifier),
                    LuaTokenType::Plus
                    | LuaTokenType::Minus
                    | LuaTokenType::Star
                    | LuaTokenType::Slash
                    | LuaTokenType::Percent
                    | LuaTokenType::Caret
                    | LuaTokenType::Hash
                    | LuaTokenType::EqEq
                    | LuaTokenType::TildeEq
                    | LuaTokenType::LtEq
                    | LuaTokenType::GtEq
                    | LuaTokenType::Lt
                    | LuaTokenType::Gt
                    | LuaTokenType::Eq => Some(HighlightKind::Operator),
                    _ => None,
                };

                if let Some(h_kind) = kind {
                    highlights.push((token.span.start, token.span.end, h_kind))
                }
            }
        }

        highlights
    }
}
