use crate::{kind::DartSyntaxKind, language::DartLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, DartLanguage>;

static DART_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static DART_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static DART_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[derive(Clone)]
pub struct DartLexer<'config> {
    config: &'config DartLanguage,
}

impl<'config> Lexer<DartLanguage> for DartLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<DartLanguage>,
    ) -> LexOutput<DartLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> DartLexer<'config> {
    pub fn new(config: &'config DartLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(DartSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match DART_WHITESPACE.scan(state.rest(), state.get_position(), DartSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DartSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    state.add_token(DartSyntaxKind::BlockComment, start, state.get_position());
                    return true;
                }
                state.advance(ch.len_utf8());
            }
            // 未闭合的块注释
            state.add_token(DartSyntaxKind::Error, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(DartSyntaxKind::StringLiteral, start, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨行
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符串
                state.add_token(DartSyntaxKind::Error, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 消费数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_double = false;

                // 检查小数点
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // 跳过 '.'
                    is_double = true;

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查科学计数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        is_double = true;

                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }

                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                let kind = if is_double { DartSyntaxKind::DoubleLiteral } else { DartSyntaxKind::IntegerLiteral };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = match text {
                    "abstract" => DartSyntaxKind::Abstract,
                    "as" => DartSyntaxKind::As,
                    "assert" => DartSyntaxKind::Assert,
                    "async" => DartSyntaxKind::Async,
                    "await" => DartSyntaxKind::Await,
                    "break" => DartSyntaxKind::Break,
                    "case" => DartSyntaxKind::Case,
                    "catch" => DartSyntaxKind::Catch,
                    "class" => DartSyntaxKind::Class,
                    "const" => DartSyntaxKind::Const,
                    "continue" => DartSyntaxKind::Continue,
                    "covariant" => DartSyntaxKind::Covariant,
                    "default" => DartSyntaxKind::Default,
                    "deferred" => DartSyntaxKind::Deferred,
                    "do" => DartSyntaxKind::Do,
                    "dynamic" => DartSyntaxKind::Dynamic,
                    "else" => DartSyntaxKind::Else,
                    "enum" => DartSyntaxKind::Enum,
                    "export" => DartSyntaxKind::Export,
                    "extends" => DartSyntaxKind::Extends,
                    "extension" => DartSyntaxKind::Extension,
                    "external" => DartSyntaxKind::External,
                    "factory" => DartSyntaxKind::Factory,
                    "false" => DartSyntaxKind::False,
                    "final" => DartSyntaxKind::Final,
                    "finally" => DartSyntaxKind::Finally,
                    "for" => DartSyntaxKind::For,
                    "function" => DartSyntaxKind::Function,
                    "get" => DartSyntaxKind::Get,
                    "hide" => DartSyntaxKind::Hide,
                    "if" => DartSyntaxKind::If,
                    "implements" => DartSyntaxKind::Implements,
                    "import" => DartSyntaxKind::Import,
                    "in" => DartSyntaxKind::In,
                    "interface" => DartSyntaxKind::Interface,
                    "is" => DartSyntaxKind::Is,
                    "late" => DartSyntaxKind::Late,
                    "library" => DartSyntaxKind::Library,
                    "mixin" => DartSyntaxKind::Mixin,
                    "new" => DartSyntaxKind::New,
                    "null" => DartSyntaxKind::Null,
                    "on" => DartSyntaxKind::On,
                    "operator" => DartSyntaxKind::Operator,
                    "part" => DartSyntaxKind::Part,
                    "required" => DartSyntaxKind::Required,
                    "rethrow" => DartSyntaxKind::Rethrow,
                    "return" => DartSyntaxKind::Return,
                    "set" => DartSyntaxKind::Set,
                    "show" => DartSyntaxKind::Show,
                    "static" => DartSyntaxKind::Static,
                    "super" => DartSyntaxKind::Super,
                    "switch" => DartSyntaxKind::Switch,
                    "sync" => DartSyntaxKind::Sync,
                    "this" => DartSyntaxKind::This,
                    "throw" => DartSyntaxKind::Throw,
                    "true" => DartSyntaxKind::True,
                    "try" => DartSyntaxKind::Try,
                    "typedef" => DartSyntaxKind::Typedef,
                    "var" => DartSyntaxKind::Var,
                    "void" => DartSyntaxKind::Void,
                    "while" => DartSyntaxKind::While,
                    "with" => DartSyntaxKind::With,
                    "yield" => DartSyntaxKind::Yield,
                    _ => DartSyntaxKind::Identifier,
                };

                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    match state.peek() {
                        Some('+') => {
                            state.advance(1);
                            DartSyntaxKind::PlusPlus
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::PlusEqual
                        }
                        _ => DartSyntaxKind::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            DartSyntaxKind::MinusMinus
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::MinusEqual
                        }
                        _ => DartSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        DartSyntaxKind::StarEqual
                    }
                    else {
                        DartSyntaxKind::Star
                    }
                }
                '/' => {
                    // 注释已经在前面处理了
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        DartSyntaxKind::SlashEqual
                    }
                    else {
                        DartSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        DartSyntaxKind::PercentEqual
                    }
                    else {
                        DartSyntaxKind::Percent
                    }
                }
                '~' => {
                    state.advance(1);
                    match state.peek() {
                        Some('/') => {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                DartSyntaxKind::TildeSlashEqual
                            }
                            else {
                                DartSyntaxKind::TildeSlash
                            }
                        }
                        _ => DartSyntaxKind::Tilde,
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::EqualEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            DartSyntaxKind::Arrow
                        }
                        _ => DartSyntaxKind::Equal,
                    }
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        DartSyntaxKind::BangEqual
                    }
                    else {
                        DartSyntaxKind::Bang
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::LessEqual
                        }
                        Some('<') => {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                DartSyntaxKind::LeftShiftEqual
                            }
                            else {
                                DartSyntaxKind::LeftShift
                            }
                        }
                        _ => DartSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::GreaterEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                DartSyntaxKind::RightShiftEqual
                            }
                            else {
                                DartSyntaxKind::RightShift
                            }
                        }
                        _ => DartSyntaxKind::Greater,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            DartSyntaxKind::AmpersandAmpersand
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::AmpersandEqual
                        }
                        _ => DartSyntaxKind::Ampersand,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            DartSyntaxKind::PipePipe
                        }
                        Some('=') => {
                            state.advance(1);
                            DartSyntaxKind::PipeEqual
                        }
                        _ => DartSyntaxKind::Pipe,
                    }
                }
                '^' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        DartSyntaxKind::CaretEqual
                    }
                    else {
                        DartSyntaxKind::Caret
                    }
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if state.peek() == Some('=') {
                                state.advance(1);
                                DartSyntaxKind::QuestionQuestionEqual
                            }
                            else {
                                DartSyntaxKind::QuestionQuestion
                            }
                        }
                        Some('.') => {
                            state.advance(1);
                            DartSyntaxKind::QuestionDot
                        }
                        _ => DartSyntaxKind::Question,
                    }
                }
                '.' => {
                    state.advance(1);
                    match state.peek() {
                        Some('.') => {
                            state.advance(1);
                            if state.peek() == Some('.') {
                                state.advance(1);
                                DartSyntaxKind::DotDotDot
                            }
                            else {
                                DartSyntaxKind::DotDot
                            }
                        }
                        _ => DartSyntaxKind::Dot,
                    }
                }
                _ => return false,
            };

            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => DartSyntaxKind::LeftParen,
                ')' => DartSyntaxKind::RightParen,
                '[' => DartSyntaxKind::LeftBracket,
                ']' => DartSyntaxKind::RightBracket,
                '{' => DartSyntaxKind::LeftBrace,
                '}' => DartSyntaxKind::RightBrace,
                ';' => DartSyntaxKind::Semicolon,
                ',' => DartSyntaxKind::Comma,
                ':' => DartSyntaxKind::Colon,
                '@' => DartSyntaxKind::At,
                '#' => DartSyntaxKind::Hash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }
}
