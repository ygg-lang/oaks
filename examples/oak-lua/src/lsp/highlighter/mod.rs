#![doc = include_str!("readme.md")]
/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 宏
    Macro,
    /// 标识符
    Identifier,
    /// 运算符
    Operator,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// Lua 语法高亮器
pub struct LuaHighlighter;

impl LuaHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for LuaHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        // 简单的正则表达式或基于词法分析的高亮实现
        // 这里我们可以复用 LuaLexer 来获取 Token
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
