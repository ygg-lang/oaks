#![doc = include_str!("readme.md")]
use oak_core::Source;
pub mod token_type;

use crate::{language::GroovyLanguage, lexer::token_type::GroovyTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::TextEdit,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, GroovyLanguage>;

static GROOVY_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static GROOVY_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: false });
static GROOVY_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static GROOVY_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct GroovyLexer<'config> {
    _config: &'config GroovyLanguage,
}

impl<'config> Lexer<GroovyLanguage> for GroovyLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<GroovyLanguage>) -> LexOutput<GroovyLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> GroovyLexer<'config> {
    pub fn new(config: &'config GroovyLanguage) -> Self {
        Self { _config: config }
    }

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
        GROOVY_WHITESPACE.scan(state, GroovyTokenType::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        // 行注释 // 和 块注释 /* ... */
        if GROOVY_COMMENT.scan(state, GroovyTokenType::Comment, GroovyTokenType::Comment) {
            return true;
        }

        false
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        // 普通字符串 "..."
        if GROOVY_STRING.scan(state, GroovyTokenType::StringLiteral) {
            return true;
        }

        // 三重引号字符串 """..."""
        if state.consume_if_starts_with("\"\"\"") {
            let start = state.get_position() - 3;

            while state.not_at_end() {
                if state.consume_if_starts_with("\"\"\"") {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GroovyTokenType::StringLiteral, start, end);
            return true;
        }

        // GString $/.../$
        if state.consume_if_starts_with("$/") {
            let start = state.get_position() - 2;

            while state.not_at_end() {
                if state.consume_if_starts_with("/$") {
                    break;
                }
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                }
            }

            let end = state.get_position();
            state.add_token(GroovyTokenType::StringLiteral, start, end);
            return true;
        }

        false
    }

    /// 词法分析字符字面量
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        GROOVY_CHAR.scan(state, GroovyTokenType::CharLiteral)
    }

    /// 词法分析数字字面量
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut has_digits = false;
        let mut is_float = false;

        // 处理负号
        if state.consume_if_starts_with("-") {
            // Negative sign
        }

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
        }
        // 处理八进制 0...
        else if state.peek() == Some('0') {
            state.advance(1);
            has_digits = true;
            while let Some(ch) = state.peek() {
                if ch >= '0' && ch <= '7' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
        }
        // 处理十进制
        else {
            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                    has_digits = true;
                }
                else {
                    break;
                }
            }

            // 处理小数部分
            if state.peek() == Some('.') && has_digits {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过 .
                        is_float = true;

                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 处理指数部分
            if let Some(ch) = state.peek() {
                if (ch == 'e' || ch == 'E') && has_digits {
                    state.advance(1);
                    is_float = true;

                    // 处理指数符号
                    if let Some(next) = state.peek() {
                        if next == '+' || next == '-' {
                            state.advance(1);
                        }
                    }

                    // 处理指数数字
                    let mut exp_digits = false;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                            exp_digits = true;
                        }
                        else {
                            break;
                        }
                    }

                    if !exp_digits {
                        // 指数部分必须有数字
                        return false;
                    }
                }
            }
        }

        // 处理数字后缀 (G, L, F, D)
        if has_digits {
            if let Some(ch) = state.peek() {
                if matches!(ch, 'G' | 'g' | 'L' | 'l' | 'F' | 'f' | 'D' | 'd') {
                    state.advance(ch.len_utf8());
                    is_float = matches!(ch, 'F' | 'f' | 'D' | 'd' | 'G' | 'g');
                }
            }
        }

        if has_digits {
            let end = state.get_position();
            let kind = if is_float { GroovyTokenType::FloatLiteral } else { GroovyTokenType::IntLiteral };
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
            if !first_ch.is_alphabetic() && first_ch != '_' && first_ch != '$' {
                return false;
            }

            state.advance(first_ch.len_utf8());

            // 后续字符可以是字母、数字或下划线
            while let Some(ch) = state.peek() {
                if ch.is_alphanumeric() || ch == '_' || ch == '$' { state.advance(ch.len_utf8()) } else { break }
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
    fn keyword_or_identifier(&self, text: &str) -> GroovyTokenType {
        match text {
            // 关键字
            "abstract" => GroovyTokenType::AbstractKeyword,
            "as" => GroovyTokenType::AsKeyword,
            "assert" => GroovyTokenType::AssertKeyword,
            "break" => GroovyTokenType::BreakKeyword,
            "case" => GroovyTokenType::CaseKeyword,
            "catch" => GroovyTokenType::CatchKeyword,
            "class" => GroovyTokenType::ClassKeyword,
            "const" => GroovyTokenType::ConstKeyword,
            "continue" => GroovyTokenType::ContinueKeyword,
            "def" => GroovyTokenType::DefKeyword,
            "default" => GroovyTokenType::DefaultKeyword,
            "do" => GroovyTokenType::DoKeyword,
            "else" => GroovyTokenType::ElseKeyword,
            "enum" => GroovyTokenType::EnumKeyword,
            "extends" => GroovyTokenType::ExtendsKeyword,
            "final" => GroovyTokenType::FinalKeyword,
            "finally" => GroovyTokenType::FinallyKeyword,
            "for" => GroovyTokenType::ForKeyword,
            "goto" => GroovyTokenType::GotoKeyword,
            "if" => GroovyTokenType::IfKeyword,
            "implements" => GroovyTokenType::ImplementsKeyword,
            "import" => GroovyTokenType::ImportKeyword,
            "in" => GroovyTokenType::InKeyword,
            "instanceof" => GroovyTokenType::InstanceofKeyword,
            "interface" => GroovyTokenType::InterfaceKeyword,
            "native" => GroovyTokenType::NativeKeyword,
            "new" => GroovyTokenType::NewKeyword,
            "package" => GroovyTokenType::PackageKeyword,
            "private" => GroovyTokenType::PrivateKeyword,
            "protected" => GroovyTokenType::ProtectedKeyword,
            "public" => GroovyTokenType::PublicKeyword,
            "return" => GroovyTokenType::ReturnKeyword,
            "static" => GroovyTokenType::StaticKeyword,
            "strictfp" => GroovyTokenType::StrictfpKeyword,
            "super" => GroovyTokenType::SuperKeyword,
            "switch" => GroovyTokenType::SwitchKeyword,
            "synchronized" => GroovyTokenType::SynchronizedKeyword,
            "this" => GroovyTokenType::ThisKeyword,
            "throw" => GroovyTokenType::ThrowKeyword,
            "throws" => GroovyTokenType::ThrowsKeyword,
            "trait" => GroovyTokenType::TraitKeyword,
            "transient" => GroovyTokenType::TransientKeyword,
            "try" => GroovyTokenType::TryKeyword,
            "void" => GroovyTokenType::VoidKeyword,
            "volatile" => GroovyTokenType::VolatileKeyword,
            "while" => GroovyTokenType::WhileKeyword,

            // 特殊字面量
            "true" | "false" => GroovyTokenType::BooleanLiteral,
            "null" => GroovyTokenType::NullLiteral,

            // 默认为标识符
            _ => GroovyTokenType::Identifier,
        }
    }

    /// 词法分析操作符
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // 三字符操作符
        if state.consume_if_starts_with(">>>") {
            state.add_token(GroovyTokenType::UnsignedRightShift, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<=>") {
            state.add_token(GroovyTokenType::Spaceship, start, state.get_position());
            return true;
        }

        // 两字符操作符
        if state.consume_if_starts_with("**") {
            state.add_token(GroovyTokenType::Power, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("+=") {
            state.add_token(GroovyTokenType::PlusAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("-=") {
            state.add_token(GroovyTokenType::MinusAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("*=") {
            state.add_token(GroovyTokenType::StarAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("/=") {
            state.add_token(GroovyTokenType::SlashAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("%=") {
            state.add_token(GroovyTokenType::PercentAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("**=") {
            state.add_token(GroovyTokenType::PowerAssign, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("==") {
            state.add_token(GroovyTokenType::Equal, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("!=") {
            state.add_token(GroovyTokenType::NotEqual, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<=") {
            state.add_token(GroovyTokenType::LessEqual, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with(">=") {
            state.add_token(GroovyTokenType::GreaterEqual, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("&&") {
            state.add_token(GroovyTokenType::LogicalAnd, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("||") {
            state.add_token(GroovyTokenType::LogicalOr, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("<<") {
            state.add_token(GroovyTokenType::LeftShift, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with(">>") {
            state.add_token(GroovyTokenType::RightShift, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("++") {
            state.add_token(GroovyTokenType::Increment, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("--") {
            state.add_token(GroovyTokenType::Decrement, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("?:") {
            state.add_token(GroovyTokenType::Elvis, start, state.get_position());
            return true;
        }
        if state.consume_if_starts_with("?.") {
            state.add_token(GroovyTokenType::SafeNavigation, start, state.get_position());
            return true;
        }

        false
    }

    /// 词法分析单字符 token
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();
            let kind = match ch {
                '+' => Some(GroovyTokenType::Plus),
                '-' => Some(GroovyTokenType::Minus),
                '*' => Some(GroovyTokenType::Star),
                '/' => Some(GroovyTokenType::Slash),
                '%' => Some(GroovyTokenType::Percent),
                '=' => Some(GroovyTokenType::Assign),
                '<' => Some(GroovyTokenType::Less),
                '>' => Some(GroovyTokenType::Greater),
                '!' => Some(GroovyTokenType::LogicalNot),
                '&' => Some(GroovyTokenType::BitAnd),
                '|' => Some(GroovyTokenType::BitOr),
                '^' => Some(GroovyTokenType::BitXor),
                '~' => Some(GroovyTokenType::BitNot),
                '?' => Some(GroovyTokenType::Question),
                ':' => Some(GroovyTokenType::Colon),
                '(' => Some(GroovyTokenType::LeftParen),
                ')' => Some(GroovyTokenType::RightParen),
                '[' => Some(GroovyTokenType::LeftBracket),
                ']' => Some(GroovyTokenType::RightBracket),
                '{' => Some(GroovyTokenType::LeftBrace),
                '}' => Some(GroovyTokenType::RightBrace),
                ',' => Some(GroovyTokenType::Comma),
                '.' => Some(GroovyTokenType::Period),
                ';' => Some(GroovyTokenType::Semicolon),
                '@' => Some(GroovyTokenType::At),
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
