#![doc = include_str!("readme.md")]
use crate::{language::CoqLanguage, lexer::token_type::CoqTokenType};
use oak_core::{LexOutput, Lexer, LexerCache, SourceText, Token};

/// 高亮类型的本地定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    /// 关键字
    Keyword,
    /// 函数/策略
    Function,
    /// 字符串
    String,
    /// 数字
    Number,
    /// 注释
    Comment,
    /// 标识符
    Identifier,
    /// 运算符
    Operator,
}

/// Coq 语言的高亮器
pub struct CoqHighlighter;

impl CoqHighlighter {
    /// 创建新的 Coq 高亮器
    pub fn new() -> Self {
        Self
    }
}

/// 高亮器接口
pub trait Highlighter {
    /// 对文本进行高亮处理
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)>;
}

impl Highlighter for CoqHighlighter {
    fn highlight(&self, text: &str) -> Vec<(usize, usize, HighlightKind)> {
        let mut highlights = Vec::new();

        struct SimpleCache {
            tokens: Vec<Token<CoqTokenType>>,
        }

        impl LexerCache<CoqLanguage> for SimpleCache {
            fn set_lex_output(&mut self, output: LexOutput<CoqLanguage>) {
                if let Ok(tokens) = output.result {
                    self.tokens = tokens.to_vec();
                }
            }
            fn get_token(&self, index: usize) -> Option<Token<CoqTokenType>> {
                self.tokens.get(index).cloned()
            }
            fn count_tokens(&self) -> usize {
                self.tokens.len()
            }
            fn has_tokens(&self) -> bool {
                !self.tokens.is_empty()
            }
        }

        let language = CoqLanguage::default();
        let lexer = crate::lexer::CoqLexer::new(&language);
        let mut cache = SimpleCache { tokens: Vec::new() };
        let source = SourceText::new(text);
        let output = lexer.lex(&source, &[], &mut cache);

        if let Ok(tokens) = output.result {
            for token in tokens.iter() {
                let kind = match token.kind {
                    CoqTokenType::Theorem
                    | CoqTokenType::Lemma
                    | CoqTokenType::Definition
                    | CoqTokenType::Fixpoint
                    | CoqTokenType::Inductive
                    | CoqTokenType::Record
                    | CoqTokenType::Module
                    | CoqTokenType::Class
                    | CoqTokenType::Instance
                    | CoqTokenType::Proof
                    | CoqTokenType::Qed
                    | CoqTokenType::End
                    | CoqTokenType::Match
                    | CoqTokenType::With
                    | CoqTokenType::Type
                    | CoqTokenType::Set
                    | CoqTokenType::Prop
                    | CoqTokenType::Forall
                    | CoqTokenType::Fun
                    | CoqTokenType::Let
                    | CoqTokenType::In
                    | CoqTokenType::If
                    | CoqTokenType::Then
                    | CoqTokenType::Else => Some(HighlightKind::Keyword),

                    CoqTokenType::Intros
                    | CoqTokenType::Simpl
                    | CoqTokenType::Reflexivity
                    | CoqTokenType::Rewrite
                    | CoqTokenType::Apply
                    | CoqTokenType::Exact
                    | CoqTokenType::Assumption
                    | CoqTokenType::Auto
                    | CoqTokenType::Trivial
                    | CoqTokenType::Discriminate
                    | CoqTokenType::Injection
                    | CoqTokenType::Inversion
                    | CoqTokenType::Destruct
                    | CoqTokenType::Induction
                    | CoqTokenType::Generalize
                    | CoqTokenType::Clear
                    | CoqTokenType::Unfold
                    | CoqTokenType::Fold
                    | CoqTokenType::Compute
                    | CoqTokenType::Eval => Some(HighlightKind::Function),

                    CoqTokenType::Check
                    | CoqTokenType::Print
                    | CoqTokenType::Search
                    | CoqTokenType::Locate
                    | CoqTokenType::About
                    | CoqTokenType::Show
                    | CoqTokenType::Goal
                    | CoqTokenType::Goals
                    | CoqTokenType::Undo
                    | CoqTokenType::Restart
                    | CoqTokenType::Abort
                    | CoqTokenType::Admit
                    | CoqTokenType::Admitted => Some(HighlightKind::Keyword),

                    CoqTokenType::StringLiteral => Some(HighlightKind::String),
                    CoqTokenType::NumberLiteral => Some(HighlightKind::Number),
                    CoqTokenType::Comment => Some(HighlightKind::Comment),
                    CoqTokenType::Identifier => Some(HighlightKind::Identifier),

                    CoqTokenType::Arrow
                    | CoqTokenType::DoubleArrow
                    | CoqTokenType::Colon
                    | CoqTokenType::Semicolon
                    | CoqTokenType::Comma
                    | CoqTokenType::Dot
                    | CoqTokenType::Pipe
                    | CoqTokenType::Equal
                    | CoqTokenType::Plus
                    | CoqTokenType::Minus
                    | CoqTokenType::Star
                    | CoqTokenType::Slash
                    | CoqTokenType::Less
                    | CoqTokenType::Greater
                    | CoqTokenType::LessEqual
                    | CoqTokenType::GreaterEqual
                    | CoqTokenType::NotEqual => Some(HighlightKind::Operator),

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
