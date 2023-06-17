use core::fmt;
use oak_core::SyntaxKind;

/// Lean 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum LeanSyntaxKind {
    // 根节点
    Root,
    SourceFile,

    // 关键字
    Axiom,
    Constant,
    Def,
    Example,
    Inductive,
    Lemma,
    Namespace,
    Open,
    Private,
    Protected,
    Section,
    Structure,
    Theorem,
    Universe,
    Variable,
    Variables,
    End,
    Import,
    Export,
    Prelude,
    Noncomputable,
    Partial,
    Unsafe,
    Mutual,
    Where,
    Have,
    Show,
    Suffices,
    Let,
    In,
    If,
    Then,
    Else,
    Match,
    With,
    Fun,
    Do,
    For,
    While,
    Break,
    Continue,
    Return,
    Try,
    Catch,
    Finally,
    Throw,

    // 标识符和字面量
    Identifier,
    Number,
    String,
    Char,

    // 操作符
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Hash,
    Ampersand,
    Pipe,
    Tilde,
    Bang,
    Question,
    At,
    Dollar,
    Arrow,
    FatArrow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Append,
    Cons,

    // 分隔符
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    Semicolon,
    Colon,
    Comma,
    Dot,
    DotDot,
    ColonEq,
    ColonColon,

    // 空白和注释
    Comment,
    Whitespace,
    Newline,

    // 错误和结束
    Error,
    Eof,
}

impl LeanSyntaxKind {
    /// 检查是否为关键字
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::Axiom
                | Self::Constant
                | Self::Def
                | Self::Example
                | Self::Inductive
                | Self::Lemma
                | Self::Namespace
                | Self::Open
                | Self::Private
                | Self::Protected
                | Self::Section
                | Self::Structure
                | Self::Theorem
                | Self::Universe
                | Self::Variable
                | Self::Variables
                | Self::End
                | Self::Import
                | Self::Export
                | Self::Prelude
                | Self::Noncomputable
                | Self::Partial
                | Self::Unsafe
                | Self::Mutual
                | Self::Where
                | Self::Have
                | Self::Show
                | Self::Suffices
                | Self::Let
                | Self::In
                | Self::If
                | Self::Then
                | Self::Else
                | Self::Match
                | Self::With
                | Self::Fun
                | Self::Do
                | Self::For
                | Self::While
                | Self::Break
                | Self::Continue
                | Self::Return
                | Self::Try
                | Self::Catch
                | Self::Finally
                | Self::Throw
        )
    }

    /// 检查是否为操作    pub
    fn is_operator(self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Caret
                | Self::Hash
                | Self::Ampersand
                | Self::Pipe
                | Self::Tilde
                | Self::Bang
                | Self::Question
                | Self::At
                | Self::Dollar
                | Self::Arrow
                | Self::FatArrow
                | Self::Eq
                | Self::Ne
                | Self::Lt
                | Self::Le
                | Self::Gt
                | Self::Ge
                | Self::And
                | Self::Or
                | Self::Not
                | Self::Append
                | Self::Cons
        )
    }

    /// 检查是否为标点符号
    pub fn is_punctuation(self) -> bool {
        matches!(
            self,
            Self::LeftParen
                | Self::RightParen
                | Self::LeftBrace
                | Self::RightBrace
                | Self::LeftBracket
                | Self::RightBracket
                | Self::LeftAngle
                | Self::RightAngle
                | Self::Semicolon
                | Self::Colon
                | Self::Comma
                | Self::Dot
                | Self::DotDot
                | Self::ColonEq
                | Self::ColonColon
        )
    }

    /// 检查是否为字面    pub
    fn is_literal(self) -> bool {
        matches!(self, Self::Number | Self::String | Self::Char)
    }

    /// 检查是否为空白字符
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Comment | Self::Whitespace | Self::Newline)
    }
}

impl fmt::Display for LeanSyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Root => "ROOT",
            Self::SourceFile => "SOURCE_FILE",
            Self::Axiom => "axiom",
            Self::Constant => "constant",
            Self::Def => "def",
            Self::Example => "example",
            Self::Inductive => "inductive",
            Self::Lemma => "lemma",
            Self::Namespace => "namespace",
            Self::Open => "open",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Section => "section",
            Self::Structure => "structure",
            Self::Theorem => "theorem",
            Self::Universe => "universe",
            Self::Variable => "variable",
            Self::Variables => "variables",
            Self::End => "end",
            Self::Import => "import",
            Self::Export => "export",
            Self::Prelude => "prelude",
            Self::Noncomputable => "noncomputable",
            Self::Partial => "partial",
            Self::Unsafe => "unsafe",
            Self::Mutual => "mutual",
            Self::Where => "where",
            Self::Have => "have",
            Self::Show => "show",
            Self::Suffices => "suffices",
            Self::Let => "let",
            Self::In => "in",
            Self::If => "if",
            Self::Then => "then",
            Self::Else => "else",
            Self::Match => "match",
            Self::With => "with",
            Self::Fun => "fun",
            Self::Do => "do",
            Self::For => "for",
            Self::While => "while",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Return => "return",
            Self::Try => "try",
            Self::Catch => "catch",
            Self::Finally => "finally",
            Self::Throw => "throw",
            Self::Identifier => "IDENTIFIER",
            Self::Number => "NUMBER",
            Self::String => "STRING",
            Self::Char => "CHAR",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Star => "*",
            Self::Slash => "/",
            Self::Percent => "%",
            Self::Caret => "^",
            Self::Hash => "#",
            Self::Ampersand => "&",
            Self::Pipe => "|",
            Self::Tilde => "~",
            Self::Bang => "!",
            Self::Question => "?",
            Self::At => "@",
            Self::Dollar => "$",
            Self::Arrow => "->",
            Self::FatArrow => "=>",
            Self::Eq => "=",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::And => "&&",
            Self::Or => "||",
            Self::Not => "¬",
            Self::Append => "++",
            Self::Cons => "::",
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::LeftAngle => "<",
            Self::RightAngle => ">",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::DotDot => "..",
            Self::ColonEq => ":=",
            Self::ColonColon => "::",
            Self::Comment => "COMMENT",
            Self::Whitespace => "WHITESPACE",
            Self::Newline => "NEWLINE",
            Self::Error => "ERROR",
            Self::Eof => "EOF",
        };
        write!(f, "{}", name)
    }
}

impl SyntaxKind for LeanSyntaxKind {
    fn is_trivia(&self) -> bool {
        self.is_trivia()
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Root | Self::SourceFile)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Root | Self::SourceFile)
    }
}
