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

/// Handlebars 语法高亮器
pub struct HandlebarsHighlighter;

impl HandlebarsHighlighter {
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for HandlebarsHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        use crate::{kind::HandlebarsSyntaxKind, lexer::HandlebarsLexer};
        use oak_core::{LexOutput, Lexer, LexerCache, SourceText, Token};

        struct SimpleCache {
            tokens: Vec<Token<HandlebarsSyntaxKind>>,
        }

        impl LexerCache<crate::language::HandlebarsLanguage> for SimpleCache {
            fn set_lex_output(&mut self, output: LexOutput<crate::language::HandlebarsLanguage>) {
                if let Ok(tokens) = output.result {
                    self.tokens = tokens.to_vec();
                }
            }
            fn get_token(&self, index: usize) -> Option<Token<HandlebarsSyntaxKind>> {
                self.tokens.get(index).cloned()
            }
            fn count_tokens(&self) -> usize {
                self.tokens.len()
            }
            fn has_tokens(&self) -> bool {
                !self.tokens.is_empty()
            }
        }

        let language = crate::language::HandlebarsLanguage::default();
        let lexer = HandlebarsLexer::new(&language);
        let mut cache = SimpleCache { tokens: Vec::new() };
        let source = SourceText::new(text);
        let output = lexer.lex(&source, &[], &mut cache);

        if let Ok(tokens) = output.result {
            for token in tokens.iter() {
                let kind = match token.kind {
                    HandlebarsSyntaxKind::Else => Some(HighlightKind::Keyword),
                    HandlebarsSyntaxKind::StringLiteral => Some(HighlightKind::String),
                    HandlebarsSyntaxKind::NumberLiteral => Some(HighlightKind::Number),
                    HandlebarsSyntaxKind::Comment => Some(HighlightKind::Comment),
                    HandlebarsSyntaxKind::Identifier => Some(HighlightKind::Identifier),
                    HandlebarsSyntaxKind::Open
                    | HandlebarsSyntaxKind::Close
                    | HandlebarsSyntaxKind::OpenUnescaped
                    | HandlebarsSyntaxKind::CloseUnescaped
                    | HandlebarsSyntaxKind::OpenRawBlock
                    | HandlebarsSyntaxKind::CloseRawBlock
                    | HandlebarsSyntaxKind::OpenEndRawBlock
                    | HandlebarsSyntaxKind::OpenBlock
                    | HandlebarsSyntaxKind::OpenInverseBlock
                    | HandlebarsSyntaxKind::CloseBlock
                    | HandlebarsSyntaxKind::OpenPartial
                    | HandlebarsSyntaxKind::OpenComment
                    | HandlebarsSyntaxKind::OpenCommentBlock
                    | HandlebarsSyntaxKind::CloseCommentBlock => Some(HighlightKind::Operator),
                    _ => None,
                };

                if let Some(h_kind) = kind {
                    highlights.push((token.span.start, token.span.end, h_kind));
                }
            }
        }

        highlights
    }
}
