use crate::{kind::TypeScriptSyntaxKind, language::TypeScriptLanguage, lexer::TypeScriptLexer};
use oak_core::{Lexer, SourceText, TextEdit, TokenType, UniversalTokenRole};

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
    /// 标识符
    Identifier,
    /// 操作符
    Operator,
    /// 标点符号
    Punctuation,
    /// 错误
    Error,
}

/// 高亮器 trait
pub trait Highlighter {
    /// 对给定的文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

/// TypeScript 语法高亮器
pub struct TypeScriptHighlighter<'config> {
    lexer: TypeScriptLexer<'config>,
}

impl<'config> TypeScriptHighlighter<'config> {
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
                        TypeScriptSyntaxKind::StringLiteral | TypeScriptSyntaxKind::TemplateString | TypeScriptSyntaxKind::RegexLiteral => HighlightKind::String,
                        TypeScriptSyntaxKind::NumericLiteral | TypeScriptSyntaxKind::BigIntLiteral => HighlightKind::Number,
                        _ => HighlightKind::String,
                    },
                    UniversalTokenRole::Comment => HighlightKind::Comment,
                    UniversalTokenRole::Name => HighlightKind::Identifier,
                    UniversalTokenRole::Operator => HighlightKind::Operator,
                    UniversalTokenRole::Punctuation => HighlightKind::Punctuation,
                    _ => continue,
                };
                highlights.push((token.span.start, token.span.end, kind));
            }
        }

        highlights
    }
}
