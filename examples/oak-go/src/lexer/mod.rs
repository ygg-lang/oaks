use crate::{kind::GoSyntaxKind, language::GoLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, GoLanguage>;

static GO_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GO_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: false });
static GO_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static GO_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct GoLexer<'config> {
    pub(crate) _config: &'config GoLanguage,
}

impl<'config> GoLexer<'config> {
    pub fn new(config: &'config GoLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Lexer<GoLanguage> for GoLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<GoLanguage>) -> LexOutput<GoLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> GoLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            if self.lex_char_literal(state) {
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

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GO_WHITESPACE.scan(state, GoSyntaxKind::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GO_COMMENT.scan(state, GoSyntaxKind::Comment, GoSyntaxKind::Comment)
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        // 普通字符串 "..."
        if GO_STRING.scan(state, GoSyntaxKind::StringLiteral) {
            return true;
        }

        // 原始字符串 `...`
        if state.starts_with("`") {
            let start = state.get_position();
            state.advance(1); // 跳过开始的 `

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1); // 跳过结束的 `
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let end = state.get_position();
            state.add_token(GoSyntaxKind::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符字面量
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GO_CHAR.scan(state, GoSyntaxKind::RuneLiteral)
    }

    /// 词法分析数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理十六进制 0x...
        if state.consume_if_starts_with("0x") || state.consume_if_starts_with("0X") {
            while let Some(ch) = state.peek() {
                if ch.is_ascii_hexdigit() {
                    state.advance(ch.len_utf8());
                    has_digits = true;
                }
                else {
                    break;
                }
            }
            if has_digits {
                let end = state.get_position();
                state.add_token(GoSyntaxKind::IntLiteral, start, end);
                return true;
            }
            return false;
        }

        // 处理八进制 0...
        if state.consume_if_starts_with("0") {
            if let Some(next_ch) = state.peek() {
                if next_ch.is_ascii_digit() && next_ch != '8' && next_ch != '9' {
                    while let Some(ch) = state.peek() {
                        if ch >= '0' && ch <= '7' {
                            state.advance(ch.len_utf8());
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                    if has_digits {
                        let end = state.get_position();
                        state.add_token(GoSyntaxKind::IntLiteral, start, end);
                        return true;
                    }
                }
            }
        }

        // 处理十进制数字
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
                has_digits = true;
            }
            else if ch == '.' && !is_float {
                // 检查是否是浮点数
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(ch.len_utf8()); // 跳过 .
                        is_float = true;
                    }
                    else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
            else if (ch == 'e' || ch == 'E') && has_digits {
                // 科学计数法
                state.advance(ch.len_utf8());
                if let Some(sign_ch) = state.peek() {
                    if sign_ch == '+' || sign_ch == '-' {
                        state.advance(sign_ch.len_utf8());
                    }
                }
                let mut exp_digits = false;
                while let Some(exp_ch) = state.peek() {
                    if exp_ch.is_ascii_digit() {
                        state.advance(exp_ch.len_utf8());
                        exp_digits = true;
                    }
                    else {
                        break;
                    }
                }
                if exp_digits {
                    is_float = true;
                }
                break;
            }
            else {
                break;
            }
        }

        if has_digits {
            let end = state.get_position();
            let kind = if is_float { GoSyntaxKind::FloatLiteral } else { GoSyntaxKind::IntLiteral };
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 词法分析标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 标识符必须以字母或下划线开始
        if let Some(first_ch) = state.peek() {
            if !first_ch.is_alphabetic() && first_ch != '_' {
                return false;
            }

            state.advance(first_ch.len_utf8());

            // 后续字符可以是字母、数字或下划线
            while let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            let end = state.get_position();
            let text = state.get_text_in((start..end).into());
            let kind = self.keyword_or_identifier(text.as_ref());
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> GoSyntaxKind {
        match text {
            // 关键字
            "break" => GoSyntaxKind::Break,
            "case" => GoSyntaxKind::Case,
            "chan" => GoSyntaxKind::Chan,
            "const" => GoSyntaxKind::Const,
            "continue" => GoSyntaxKind::Continue,
            "default" => GoSyntaxKind::Default,
            "defer" => GoSyntaxKind::Defer,
            "else" => GoSyntaxKind::Else,
            "fallthrough" => GoSyntaxKind::Fallthrough,
            "for" => GoSyntaxKind::For,
            "func" => GoSyntaxKind::Func,
            "go" => GoSyntaxKind::Go,
            "goto" => GoSyntaxKind::Goto,
            "if" => GoSyntaxKind::If,
            "import" => GoSyntaxKind::Import,
            "interface" => GoSyntaxKind::Interface,
            "map" => GoSyntaxKind::Map,
            "package" => GoSyntaxKind::Package,
            "range" => GoSyntaxKind::Range,
            "return" => GoSyntaxKind::Return,
            "select" => GoSyntaxKind::Select,
            "struct" => GoSyntaxKind::Struct,
            "switch" => GoSyntaxKind::Switch,
            "type" => GoSyntaxKind::Type,
            "var" => GoSyntaxKind::Var,

            // 内置类型
            "bool" => GoSyntaxKind::Bool,
            "byte" => GoSyntaxKind::Byte,
            "complex64" => GoSyntaxKind::Complex64,
            "complex128" => GoSyntaxKind::Complex128,
            "error" => GoSyntaxKind::ErrorType,
            "float32" => GoSyntaxKind::Float32,
            "float64" => GoSyntaxKind::Float64,
            "int" => GoSyntaxKind::Int,
            "int8" => GoSyntaxKind::Int8,
            "int16" => GoSyntaxKind::Int16,
            "int32" => GoSyntaxKind::Int32,
            "int64" => GoSyntaxKind::Int64,
            "rune" => GoSyntaxKind::Rune,
            "string" => GoSyntaxKind::String,
            "uint" => GoSyntaxKind::Uint,
            "uint8" => GoSyntaxKind::Uint8,
            "uint16" => GoSyntaxKind::Uint16,
            "uint32" => GoSyntaxKind::Uint32,
            "uint64" => GoSyntaxKind::Uint64,
            "uintptr" => GoSyntaxKind::Uintptr,

            // 特殊字面量
            "nil" => GoSyntaxKind::NilLiteral,
            "true" | "false" => GoSyntaxKind::BoolLiteral,

            // 默认为标识符
            _ => GoSyntaxKind::Identifier,
        }
    }

    /// 词法分析操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 三字符操作符
        let patterns_3: &[(&str, GoSyntaxKind)] = &[("<<=", GoSyntaxKind::LeftShiftAssign), (">>=", GoSyntaxKind::RightShiftAssign), ("&^=", GoSyntaxKind::AmpersandCaretAssign), ("...", GoSyntaxKind::Ellipsis)];

        for (pat, kind) in patterns_3 {
            if state.starts_with(pat) {
                state.advance(3);
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // 双字符操作符
        let patterns_2: &[(&str, GoSyntaxKind)] = &[
            ("++", GoSyntaxKind::Increment),
            ("--", GoSyntaxKind::Decrement),
            ("==", GoSyntaxKind::Equal),
            ("!=", GoSyntaxKind::NotEqual),
            ("<=", GoSyntaxKind::LessEqual),
            (">=", GoSyntaxKind::GreaterEqual),
            ("<<", GoSyntaxKind::LeftShift),
            (">>", GoSyntaxKind::RightShift),
            ("&&", GoSyntaxKind::LogicalAnd),
            ("||", GoSyntaxKind::LogicalOr),
            ("<-", GoSyntaxKind::Arrow),
            (":=", GoSyntaxKind::ColonAssign),
            ("&^", GoSyntaxKind::AmpersandCaret),
            ("+=", GoSyntaxKind::PlusAssign),
            ("-=", GoSyntaxKind::MinusAssign),
            ("*=", GoSyntaxKind::StarAssign),
            ("/=", GoSyntaxKind::SlashAssign),
            ("%=", GoSyntaxKind::PercentAssign),
            ("&=", GoSyntaxKind::AmpersandAssign),
            ("|=", GoSyntaxKind::PipeAssign),
            ("^=", GoSyntaxKind::CaretAssign),
        ];

        for (pat, kind) in patterns_2 {
            if state.starts_with(pat) {
                state.advance(2);
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '+' => Some(GoSyntaxKind::Plus),
                '-' => Some(GoSyntaxKind::Minus),
                '*' => Some(GoSyntaxKind::Star),
                '/' => Some(GoSyntaxKind::Slash),
                '%' => Some(GoSyntaxKind::Percent),
                '&' => Some(GoSyntaxKind::Ampersand),
                '|' => Some(GoSyntaxKind::Pipe),
                '^' => Some(GoSyntaxKind::Caret),
                '<' => Some(GoSyntaxKind::Less),
                '>' => Some(GoSyntaxKind::Greater),
                '=' => Some(GoSyntaxKind::Assign),
                '!' => Some(GoSyntaxKind::LogicalNot),
                '(' => Some(GoSyntaxKind::LeftParen),
                ')' => Some(GoSyntaxKind::RightParen),
                '[' => Some(GoSyntaxKind::LeftBracket),
                ']' => Some(GoSyntaxKind::RightBracket),
                '{' => Some(GoSyntaxKind::LeftBrace),
                '}' => Some(GoSyntaxKind::RightBrace),
                ',' => Some(GoSyntaxKind::Comma),
                '.' => Some(GoSyntaxKind::Period),
                ';' => Some(GoSyntaxKind::Semicolon),
                ':' => Some(GoSyntaxKind::Colon),
                _ => None,
            };

            if let Some(token_kind) = kind {
                state.advance(ch.len_utf8());
                let end = state.get_position();
                state.add_token(token_kind, start, end);
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
}
