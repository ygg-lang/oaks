use crate::{kind::CoqSyntaxKind, language::CoqLanguage};
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
            tokens: Vec<Token<CoqSyntaxKind>>,
        }

        impl LexerCache<CoqLanguage> for SimpleCache {
            fn set_lex_output(&mut self, output: LexOutput<CoqLanguage>) {
                if let Ok(tokens) = output.result {
                    self.tokens = tokens.to_vec();
                }
            }
            fn get_token(&self, index: usize) -> Option<Token<CoqSyntaxKind>> {
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
                    CoqSyntaxKind::Theorem
                    | CoqSyntaxKind::Lemma
                    | CoqSyntaxKind::Definition
                    | CoqSyntaxKind::Fixpoint
                    | CoqSyntaxKind::Inductive
                    | CoqSyntaxKind::Record
                    | CoqSyntaxKind::Module
                    | CoqSyntaxKind::Class
                    | CoqSyntaxKind::Instance
                    | CoqSyntaxKind::Proof
                    | CoqSyntaxKind::Qed
                    | CoqSyntaxKind::End
                    | CoqSyntaxKind::Match
                    | CoqSyntaxKind::With
                    | CoqSyntaxKind::Type
                    | CoqSyntaxKind::Set
                    | CoqSyntaxKind::Prop
                    | CoqSyntaxKind::Forall
                    | CoqSyntaxKind::Fun
                    | CoqSyntaxKind::Let
                    | CoqSyntaxKind::In
                    | CoqSyntaxKind::If
                    | CoqSyntaxKind::Then
                    | CoqSyntaxKind::Else => Some(HighlightKind::Keyword),

                    CoqSyntaxKind::Intros
                    | CoqSyntaxKind::Simpl
                    | CoqSyntaxKind::Reflexivity
                    | CoqSyntaxKind::Rewrite
                    | CoqSyntaxKind::Apply
                    | CoqSyntaxKind::Exact
                    | CoqSyntaxKind::Assumption
                    | CoqSyntaxKind::Auto
                    | CoqSyntaxKind::Trivial
                    | CoqSyntaxKind::Discriminate
                    | CoqSyntaxKind::Injection
                    | CoqSyntaxKind::Inversion
                    | CoqSyntaxKind::Destruct
                    | CoqSyntaxKind::Induction
                    | CoqSyntaxKind::Generalize
                    | CoqSyntaxKind::Clear
                    | CoqSyntaxKind::Unfold
                    | CoqSyntaxKind::Fold
                    | CoqSyntaxKind::Compute
                    | CoqSyntaxKind::Eval => Some(HighlightKind::Function),

                    CoqSyntaxKind::Check
                    | CoqSyntaxKind::Print
                    | CoqSyntaxKind::Search
                    | CoqSyntaxKind::Locate
                    | CoqSyntaxKind::About
                    | CoqSyntaxKind::Show
                    | CoqSyntaxKind::Goal
                    | CoqSyntaxKind::Goals
                    | CoqSyntaxKind::Undo
                    | CoqSyntaxKind::Restart
                    | CoqSyntaxKind::Abort
                    | CoqSyntaxKind::Admit
                    | CoqSyntaxKind::Admitted => Some(HighlightKind::Keyword),

                    CoqSyntaxKind::StringLiteral => Some(HighlightKind::String),
                    CoqSyntaxKind::NumberLiteral => Some(HighlightKind::Number),
                    CoqSyntaxKind::Comment => Some(HighlightKind::Comment),
                    CoqSyntaxKind::Identifier => Some(HighlightKind::Identifier),

                    CoqSyntaxKind::Arrow
                    | CoqSyntaxKind::DoubleArrow
                    | CoqSyntaxKind::Colon
                    | CoqSyntaxKind::Semicolon
                    | CoqSyntaxKind::Comma
                    | CoqSyntaxKind::Dot
                    | CoqSyntaxKind::Pipe
                    | CoqSyntaxKind::Equal
                    | CoqSyntaxKind::Plus
                    | CoqSyntaxKind::Minus
                    | CoqSyntaxKind::Star
                    | CoqSyntaxKind::Slash
                    | CoqSyntaxKind::Less
                    | CoqSyntaxKind::Greater
                    | CoqSyntaxKind::LessEqual
                    | CoqSyntaxKind::GreaterEqual
                    | CoqSyntaxKind::NotEqual => Some(HighlightKind::Operator),

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
