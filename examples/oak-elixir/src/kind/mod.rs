#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElixirSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,

    // 标识符和字面量
    Identifier,
    Atom,
    Variable,
    Number,
    String,
    Character,
    Sigil,

    // Elixir 关键字
    After,
    And,
    Case,
    Catch,
    Cond,
    Def,
    Defp,
    Defmodule,
    Defstruct,
    Defprotocol,
    Defimpl,
    Defmacro,
    Defmacrop,
    Do,
    Else,
    Elsif,
    End,
    False,
    Fn,
    If,
    In,
    Not,
    Or,
    Receive,
    Rescue,
    True,
    Try,
    Unless,
    When,
    With,

    // 操作符
    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Equal,           // =
    EqualEqual,      // ==
    NotEqual,        // !=
    EqualEqualEqual, // ===
    NotEqualEqual,   // !==
    Less,            // <
    Greater,         // >
    LessEqual,       // <=
    GreaterEqual,    // >=
    PlusPlus,        // ++
    MinusMinus,      // --
    StarStar,        // **
    Exclamation,     // !
    Question,        // ?
    Ampersand,       // &
    At,              // @
    Caret,           // ^
    Tilde,           // ~
    LeftShift,       // <<
    RightShift,      // >>
    MatchOp,         // =~
    PipeRight,       // |>

    // 分隔    LeftParen,      // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Comma,        // ,
    Semicolon,    // ;
    Dot,          // .
    Colon,        // :
    Arrow,        // ->
    Pipe,         // |
    PipePipe,     // ||
    Hash,         // #

    // 特殊
    Error,
    Eof,
}

impl ElixirSyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            Self::After
                | Self::And
                | Self::Case
                | Self::Catch
                | Self::Cond
                | Self::Def
                | Self::Defp
                | Self::Defmodule
                | Self::Defstruct
                | Self::Defprotocol
                | Self::Defimpl
                | Self::Defmacro
                | Self::Defmacrop
                | Self::Do
                | Self::Else
                | Self::Elsif
                | Self::End
                | Self::False
                | Self::Fn
                | Self::If
                | Self::In
                | Self::Not
                | Self::Or
                | Self::Receive
                | Self::Rescue
                | Self::True
                | Self::Try
                | Self::Unless
                | Self::When
                | Self::With
        )
    }
}
