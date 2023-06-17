use oak_core::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErlangSyntaxKind {
    // 基本 kind
    Whitespace,
    Newline,
    Comment,

    // 标识符和字面    Identifier,
    Atom,
    Variable,
    Number,
    String,
    Character,

    // Erlang 关键    After,
    And,
    Andalso,
    Band,
    Begin,
    Bnot,
    Bor,
    Bsl,
    Bsr,
    Bxor,
    Case,
    Catch,
    Cond,
    Div,
    End,
    Fun,
    If,
    Let,
    Not,
    Of,
    Or,
    Orelse,
    Query,
    Receive,
    Rem,
    Try,
    When,
    Xor,

    // 操作    Plus,           // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Equal,           // =
    EqualEqual,      // ==
    SlashEqual,      // /=
    EqualColonEqual, // =:=
    EqualSlashEqual, // =/=
    Less,            // <
    Greater,         // >
    LessEqual,       // =<
    GreaterEqual,    // >=
    PlusPlus,        // ++
    MinusMinus,      // --
    Exclamation,     // !
    Question,        // ?

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

impl SyntaxKind for ErlangSyntaxKind {
    fn is_trivia(&self) -> bool {
        matches!(self, Self::Whitespace | Self::Newline | Self::Comment)
    }

    fn is_comment(&self) -> bool {
        todo!()
    }

    fn is_whitespace(&self) -> bool {
        todo!()
    }

    fn is_token_type(&self) -> bool {
        todo!()
    }

    fn is_element_type(&self) -> bool {
        todo!()
    }
}
