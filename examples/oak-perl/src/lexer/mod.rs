use crate::{kind::PerlSyntaxKind, language::PerlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, PerlLanguage>;

static PERL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static PERL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static PERL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[derive(Clone)]
pub struct PerlLexer<'config> {
    config: &'config PerlLanguage,
}

impl<'config> PerlLexer<'config> {
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match PERL_WHITESPACE.scan(state.rest(), state.get_position(), PerlSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match PERL_COMMENT.scan(state.rest(), state.get_position(), PerlSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串不能跨行（除非转义）
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PerlSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_variable<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '$' => {
                    state.advance(1);
                    // 读取变量名
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(PerlSyntaxKind::Dollar, start_pos, state.get_position());
                    true
                }
                '@' => {
                    state.advance(1);
                    // 读取数组变量名
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(PerlSyntaxKind::At, start_pos, state.get_position());
                    true
                }
                '%' => {
                    state.advance(1);
                    // 读取哈希变量名
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(PerlSyntaxKind::Percent_, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "if" => PerlSyntaxKind::If,
                    "else" => PerlSyntaxKind::Else,
                    "elsif" => PerlSyntaxKind::Elsif,
                    "unless" => PerlSyntaxKind::Unless,
                    "while" => PerlSyntaxKind::While,
                    "until" => PerlSyntaxKind::Until,
                    "for" => PerlSyntaxKind::For,
                    "foreach" => PerlSyntaxKind::Foreach,
                    "do" => PerlSyntaxKind::Do,
                    "sub" => PerlSyntaxKind::Sub,
                    "package" => PerlSyntaxKind::Package,
                    "use" => PerlSyntaxKind::Use,
                    "require" => PerlSyntaxKind::Require,
                    "my" => PerlSyntaxKind::My,
                    "our" => PerlSyntaxKind::Our,
                    "local" => PerlSyntaxKind::Local,
                    "return" => PerlSyntaxKind::Return,
                    "last" => PerlSyntaxKind::Last,
                    "next" => PerlSyntaxKind::Next,
                    "redo" => PerlSyntaxKind::Redo,
                    "die" => PerlSyntaxKind::Die,
                    "warn" => PerlSyntaxKind::Warn,
                    "eval" => PerlSyntaxKind::Eval,
                    "print" => PerlSyntaxKind::Print,
                    "printf" => PerlSyntaxKind::Printf,
                    "chomp" => PerlSyntaxKind::Chomp,
                    "chop" => PerlSyntaxKind::Chop,
                    "split" => PerlSyntaxKind::Split,
                    "join" => PerlSyntaxKind::Join,
                    "push" => PerlSyntaxKind::Push,
                    "pop" => PerlSyntaxKind::Pop,
                    "shift" => PerlSyntaxKind::Shift,
                    "unshift" => PerlSyntaxKind::Unshift,
                    "keys" => PerlSyntaxKind::Keys,
                    "values" => PerlSyntaxKind::Values,
                    "each" => PerlSyntaxKind::Each,
                    "exists" => PerlSyntaxKind::Exists,
                    "delete" => PerlSyntaxKind::Delete,
                    "defined" => PerlSyntaxKind::Defined,
                    "undef" => PerlSyntaxKind::Undef,
                    "ref" => PerlSyntaxKind::Ref,
                    "bless" => PerlSyntaxKind::Bless,
                    "new" => PerlSyntaxKind::New,
                    "and" => PerlSyntaxKind::And,
                    "or" => PerlSyntaxKind::Or,
                    "not" => PerlSyntaxKind::Not,
                    _ => PerlSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dot = false;

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let kind = PerlSyntaxKind::NumberLiteral;

                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_operators_and_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Increment
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::PlusAssign
                    }
                    else {
                        PerlSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Decrement
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Arrow
                    }
                    else {
                        PerlSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Power
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::MultiplyAssign
                    }
                    else {
                        PerlSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::DivideAssign
                    }
                    else {
                        PerlSyntaxKind::Slash
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Equal
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Match
                    }
                    else {
                        PerlSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::NotEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::NotMatch
                    }
                    else {
                        PerlSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            PerlSyntaxKind::Spaceship
                        }
                        else {
                            PerlSyntaxKind::LessEqual
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::LeftShift
                    }
                    else {
                        PerlSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::RightShift
                    }
                    else {
                        PerlSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::LogicalAnd
                    }
                    else {
                        PerlSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::LogicalOr
                    }
                    else {
                        PerlSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    PerlSyntaxKind::BitwiseXor
                }
                '~' => {
                    state.advance(1);
                    PerlSyntaxKind::BitwiseNot
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PerlSyntaxKind::Range
                    }
                    else {
                        PerlSyntaxKind::Concat
                    }
                }
                '?' => {
                    state.advance(1);
                    PerlSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    PerlSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    PerlSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PerlSyntaxKind::Comma
                }
                '(' => {
                    state.advance(1);
                    PerlSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PerlSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    PerlSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PerlSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PerlSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PerlSyntaxKind::RightBrace
                }
                '\n' => {
                    state.advance(1);
                    PerlSyntaxKind::Newline
                }
                _ => {
                    state.advance(ch.len_utf8());
                    PerlSyntaxKind::Error
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PerlLanguage> for PerlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<PerlLanguage>,
    ) -> LexOutput<PerlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> PerlLexer<'config> {
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(state) {
                continue;
            }

            // 处理注释
            if self.skip_comment(state) {
                continue;
            }

            // 处理字符串
            if self.lex_string(state) {
                continue;
            }

            // 处理变量
            if self.lex_variable(state) {
                continue;
            }

            // 处理标识符和关键字
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // 处理数字
            if self.lex_number(state) {
                continue;
            }

            // 处理操作符和标点符号
            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何模式，创建错误 token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PerlSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        Ok(())
    }
}
