use oak_core::SyntaxKind;

/// Coq 语法节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CoqSyntaxKind {
    // 琐碎tokens
    Whitespace,
    Newline,
    Comment,

    // 字面量
    StringLiteral,
    NumberLiteral,

    // 标识符和关键字
    Identifier,

    // Coq 关键字
    Theorem,
    Lemma,
    Definition,
    Fixpoint,
    Inductive,
    Record,
    Module,
    Class,
    Instance,
    Proof,
    Qed,
    End,
    Match,
    With,
    Type,
    Set,
    Prop,
    Forall,
    Fun,
    Let,
    In,
    If,
    Then,
    Else,
    Intros,
    Simpl,
    Reflexivity,
    Rewrite,
    Apply,
    Exact,
    Assumption,
    Auto,
    Trivial,
    Discriminate,
    Injection,
    Inversion,
    Destruct,
    Induction,
    Generalize,
    Clear,
    Unfold,
    Fold,
    Compute,
    Eval,
    Check,
    Print,
    Search,
    Locate,
    About,
    Show,
    Goal,
    Goals,
    Undo,
    Restart,
    Abort,
    Admit,
    Admitted,
    Parameter,
    Axiom,
    Variable,
    Hypothesis,
    Section,
    Chapter,
    Require,
    Import,
    Export,
    Open,
    Close,
    Scope,
    Notation,
    Infix,
    Reserved,
    Bind,
    Delimit,
    Arguments,
    Implicit,
    Coercion,
    Identity,
    Canonical,
    Structure,

    // 操作符和分隔符
    Arrow,        // ->
    DoubleArrow,  // =>
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Dot,          // .
    Pipe,         // |
    Underscore,   // _
    Equal,        // =
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    NotEqual,     // <>
    Tilde,        // ~
    At,           // @
    Question,     // ?
    Exclamation,  // !
    Ampersand,    // &
    Hash,         // #
    Dollar,       // $
    Backslash,    // \
    Caret,        // ^

    // 括号
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }

    // 特殊
    Error,
    Eof,
}

impl SyntaxKind for CoqSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    fn is_whitespace(&self) -> bool {
        matches!(self, Self::Whitespace)
    }

    fn is_token_type(&self) -> bool {
        !matches!(self, Self::Error | Self::Eof)
    }

    fn is_element_type(&self) -> bool {
        matches!(self, Self::Error | Self::Eof)
    }
}
