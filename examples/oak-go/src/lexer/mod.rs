use crate::{kind::GoLangSyntaxKind, language::GoLangLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, GoLangLanguage>;

static GO_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GO_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static GO_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static GO_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct GoLexer<'config> {
    config: &'config GoLangLanguage,
}

impl<'config> Lexer<GoLangLanguage> for GoLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<GoLangLanguage>,
    ) -> LexOutput<GoLangLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> GoLexer<'config> {
    pub fn new(config: &'config GoLangLanguage) -> Self {
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

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(GoLangSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match GO_WHITESPACE.scan(state.rest(), state.get_position(), GoLangSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        // 行注释 //
        if let Some(token) = GO_COMMENT.scan(state.rest(), state.get_position(), GoLangSyntaxKind::Comment) {
            state.advance_with(token);
            return true;
        }

        // 块注释 /* */
        if state.rest().starts_with("/*") {
            let start = state.get_position();
            state.advance(2); // 跳过 /*

            while state.not_at_end() {
                if state.rest().starts_with("*/") {
                    state.advance(2); // 跳过 */
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GoLangSyntaxKind::Comment, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        // 普通字符串 "..."
        if let Some(token) = GO_STRING.scan(state.rest(), state.get_position(), GoLangSyntaxKind::StringLiteral) {
            state.advance_with(token);
            return true;
        }

        // 原始字符串 `...`
        if state.rest().starts_with('`') {
            let start = state.get_position();
            state.advance(1); // 跳过开始的 `

            while state.not_at_end() {
                if state.rest().starts_with('`') {
                    state.advance(1); // 跳过结束的 `
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GoLangSyntaxKind::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符字面量
    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(token) = GO_CHAR.scan(state.rest(), state.get_position(), GoLangSyntaxKind::RuneLiteral) {
            state.advance_with(token);
            true
        }
        else {
            false
        }
    }

    /// 词法分析数字字面量
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理十六进制 0x...
        if state.rest().starts_with("0x") || state.rest().starts_with("0X") {
            state.advance(2);
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
                state.add_token(GoLangSyntaxKind::IntLiteral, start, end);
                return true;
            }
            return false;
        }

        // 处理八进制 0...
        if state.rest().starts_with('0') && state.rest().len() > 1 {
            if let Some(next_ch) = state.rest().chars().nth(1) {
                if next_ch.is_ascii_digit() && next_ch != '8' && next_ch != '9' {
                    state.advance(1); // 跳过 0
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
                        state.add_token(GoLangSyntaxKind::IntLiteral, start, end);
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
                if let Some(next_ch) = state.rest().chars().nth(1) {
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
            let kind = if is_float { GoLangSyntaxKind::FloatLiteral } else { GoLangSyntaxKind::IntLiteral };
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 词法分析标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
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
            let kind = self.keyword_or_identifier(&text);
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    /// 判断是关键字还是标识符
    fn keyword_or_identifier(&self, text: &str) -> GoLangSyntaxKind {
        match text {
            // 关键字
            "break" => GoLangSyntaxKind::Break,
            "case" => GoLangSyntaxKind::Case,
            "chan" => GoLangSyntaxKind::Chan,
            "const" => GoLangSyntaxKind::Const,
            "continue" => GoLangSyntaxKind::Continue,
            "default" => GoLangSyntaxKind::Default,
            "defer" => GoLangSyntaxKind::Defer,
            "else" => GoLangSyntaxKind::Else,
            "fallthrough" => GoLangSyntaxKind::Fallthrough,
            "for" => GoLangSyntaxKind::For,
            "func" => GoLangSyntaxKind::Func,
            "go" => GoLangSyntaxKind::Go,
            "goto" => GoLangSyntaxKind::Goto,
            "if" => GoLangSyntaxKind::If,
            "import" => GoLangSyntaxKind::Import,
            "interface" => GoLangSyntaxKind::Interface,
            "map" => GoLangSyntaxKind::Map,
            "package" => GoLangSyntaxKind::Package,
            "range" => GoLangSyntaxKind::Range,
            "return" => GoLangSyntaxKind::Return,
            "select" => GoLangSyntaxKind::Select,
            "struct" => GoLangSyntaxKind::Struct,
            "switch" => GoLangSyntaxKind::Switch,
            "type" => GoLangSyntaxKind::Type,
            "var" => GoLangSyntaxKind::Var,

            // 内置类型
            "bool" => GoLangSyntaxKind::Bool,
            "byte" => GoLangSyntaxKind::Byte,
            "complex64" => GoLangSyntaxKind::Complex64,
            "complex128" => GoLangSyntaxKind::Complex128,
            "error" => GoLangSyntaxKind::ErrorType,
            "float32" => GoLangSyntaxKind::Float32,
            "float64" => GoLangSyntaxKind::Float64,
            "int" => GoLangSyntaxKind::Int,
            "int8" => GoLangSyntaxKind::Int8,
            "int16" => GoLangSyntaxKind::Int16,
            "int32" => GoLangSyntaxKind::Int32,
            "int64" => GoLangSyntaxKind::Int64,
            "rune" => GoLangSyntaxKind::Rune,
            "string" => GoLangSyntaxKind::String,
            "uint" => GoLangSyntaxKind::Uint,
            "uint8" => GoLangSyntaxKind::Uint8,
            "uint16" => GoLangSyntaxKind::Uint16,
            "uint32" => GoLangSyntaxKind::Uint32,
            "uint64" => GoLangSyntaxKind::Uint64,
            "uintptr" => GoLangSyntaxKind::Uintptr,

            // 特殊字面量
            "nil" => GoLangSyntaxKind::NilLiteral,
            "true" | "false" => GoLangSyntaxKind::BoolLiteral,

            // 默认为标识符
            _ => GoLangSyntaxKind::Identifier,
        }
    }

    /// 词法分析操作符
    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 三字符操作符
        if rest.starts_with("<<=") {
            state.advance(3);
            state.add_token(GoLangSyntaxKind::LeftShiftAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with(">>=") {
            state.advance(3);
            state.add_token(GoLangSyntaxKind::RightShiftAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("&^=") {
            state.advance(3);
            state.add_token(GoLangSyntaxKind::AmpersandCaretAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("...") {
            state.advance(3);
            state.add_token(GoLangSyntaxKind::Ellipsis, start, state.get_position());
            return true;
        }

        // 双字符操作符
        if rest.starts_with("++") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::Increment, start, state.get_position());
            return true;
        }
        if rest.starts_with("--") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::Decrement, start, state.get_position());
            return true;
        }
        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::Equal, start, state.get_position());
            return true;
        }
        if rest.starts_with("!=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::NotEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::LessEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::GreaterEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("<<") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::LeftShift, start, state.get_position());
            return true;
        }
        if rest.starts_with(">>") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::RightShift, start, state.get_position());
            return true;
        }
        if rest.starts_with("&&") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::LogicalAnd, start, state.get_position());
            return true;
        }
        if rest.starts_with("||") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::LogicalOr, start, state.get_position());
            return true;
        }
        if rest.starts_with("<-") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::Arrow, start, state.get_position());
            return true;
        }
        if rest.starts_with(":=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::ColonAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("&^") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::AmpersandCaret, start, state.get_position());
            return true;
        }
        if rest.starts_with("+=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::PlusAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("-=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::MinusAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("*=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::StarAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("/=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::SlashAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("%=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::PercentAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("&=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::AmpersandAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("|=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::PipeAssign, start, state.get_position());
            return true;
        }
        if rest.starts_with("^=") {
            state.advance(2);
            state.add_token(GoLangSyntaxKind::CaretAssign, start, state.get_position());
            return true;
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '+' => Some(GoLangSyntaxKind::Plus),
                '-' => Some(GoLangSyntaxKind::Minus),
                '*' => Some(GoLangSyntaxKind::Star),
                '/' => Some(GoLangSyntaxKind::Slash),
                '%' => Some(GoLangSyntaxKind::Percent),
                '&' => Some(GoLangSyntaxKind::Ampersand),
                '|' => Some(GoLangSyntaxKind::Pipe),
                '^' => Some(GoLangSyntaxKind::Caret),
                '<' => Some(GoLangSyntaxKind::Less),
                '>' => Some(GoLangSyntaxKind::Greater),
                '=' => Some(GoLangSyntaxKind::Assign),
                '!' => Some(GoLangSyntaxKind::LogicalNot),
                '(' => Some(GoLangSyntaxKind::LeftParen),
                ')' => Some(GoLangSyntaxKind::RightParen),
                '[' => Some(GoLangSyntaxKind::LeftBracket),
                ']' => Some(GoLangSyntaxKind::RightBracket),
                '{' => Some(GoLangSyntaxKind::LeftBrace),
                '}' => Some(GoLangSyntaxKind::RightBrace),
                ',' => Some(GoLangSyntaxKind::Comma),
                '.' => Some(GoLangSyntaxKind::Period),
                ';' => Some(GoLangSyntaxKind::Semicolon),
                ':' => Some(GoLangSyntaxKind::Colon),
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
