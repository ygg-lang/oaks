use crate::{kind::SassSyntaxKind, language::SassLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, SassLanguage>;

static SASS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SASS_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static SASS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static SASS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct SassLexer<'config> {
    config: &'config SassLanguage,
}

impl<'config> Lexer<SassLanguage> for SassLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<SassLanguage>,
    ) -> LexOutput<SassLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> SassLexer<'config> {
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析循环
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

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_color_literal(state) {
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
        state.add_token(SassSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match SASS_WHITESPACE.scan(state.rest(), state.get_position(), SassSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();
        // line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(SassSyntaxKind::LineComment, start, state.get_position());
            return true;
        }
        // block comment: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(SassSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // normal string: "..." or '...'
        if let Some(quote) = state.current() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if ch == quote && !escaped {
                        state.advance(1); // consume closing quote
                        break;
                    }
                    state.advance(ch.len_utf8());
                    if escaped {
                        escaped = false;
                        continue;
                    }
                    if ch == '\\' {
                        escaped = true;
                        continue;
                    }
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                }
                state.add_token(SassSyntaxKind::StringLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;
        state.advance(1);

        // 读取数字部分
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // fractional part
        if state.peek() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // 单位后缀 (px, em, rem, %, etc.)
        while let Some(c) = state.peek() {
            if c.is_ascii_alphabetic() || c == '%' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { SassSyntaxKind::FloatLiteral } else { SassSyntaxKind::NumberLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '-') {
            return false;
        }
        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                state.advance(1);
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "@import" => SassSyntaxKind::Import,
            "@include" => SassSyntaxKind::Include,
            "@extend" => SassSyntaxKind::Extend,
            "@mixin" => SassSyntaxKind::Mixin,
            "@function" => SassSyntaxKind::Function,
            "@return" => SassSyntaxKind::Return,
            "@if" => SassSyntaxKind::If,
            "@else" => SassSyntaxKind::Else,
            "@elseif" => SassSyntaxKind::ElseIf,
            "@for" => SassSyntaxKind::For,
            "@each" => SassSyntaxKind::Each,
            "@while" => SassSyntaxKind::While,
            "!default" => SassSyntaxKind::Default,
            "!important" => SassSyntaxKind::Important,
            "!optional" => SassSyntaxKind::Optional,
            "!global" => SassSyntaxKind::Global,
            "and" => SassSyntaxKind::And,
            "or" => SassSyntaxKind::Or,
            "not" => SassSyntaxKind::Not,
            _ => SassSyntaxKind::Identifier,
        };
        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_variable<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('$') {
            return false;
        }
        state.advance(1);

        // 变量名必须以字母或下划线开头
        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(1);
                while let Some(c) = state.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(SassSyntaxKind::Variable, start, state.get_position());
                return true;
            }
        }
        state.set_position(start);
        false
    }

    fn lex_color_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('#') {
            return false;
        }
        state.advance(1);

        let mut hex_digits = 0;
        while let Some(c) = state.peek() {
            if c.is_ascii_hexdigit() {
                state.advance(1);
                hex_digits += 1;
            }
            else {
                break;
            }
        }

        // 有效的颜色值长度: 3, 4, 6, 8
        if hex_digits == 3 || hex_digits == 4 || hex_digits == 6 || hex_digits == 8 {
            state.add_token(SassSyntaxKind::ColorLiteral, start, state.get_position());
            return true;
        }

        state.set_position(start);
        false
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 多字符操作符
        let patterns: &[(&str, SassSyntaxKind)] =
            &[("==", SassSyntaxKind::EqEq), ("!=", SassSyntaxKind::Ne), ("<=", SassSyntaxKind::Le), (">=", SassSyntaxKind::Ge)];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(SassSyntaxKind::Plus),
                '-' => Some(SassSyntaxKind::Minus),
                '*' => Some(SassSyntaxKind::Star),
                '/' => Some(SassSyntaxKind::Slash),
                '%' => Some(SassSyntaxKind::Percent),
                '=' => Some(SassSyntaxKind::Eq),
                '<' => Some(SassSyntaxKind::Lt),
                '>' => Some(SassSyntaxKind::Gt),
                _ => None,
            };
            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => SassSyntaxKind::LeftParen,
                ')' => SassSyntaxKind::RightParen,
                '{' => SassSyntaxKind::LeftBrace,
                '}' => SassSyntaxKind::RightBrace,
                '[' => SassSyntaxKind::LeftBracket,
                ']' => SassSyntaxKind::RightBracket,
                ';' => SassSyntaxKind::Semicolon,
                ':' => SassSyntaxKind::Colon,
                ',' => SassSyntaxKind::Comma,
                '.' => SassSyntaxKind::Dot,
                '#' => SassSyntaxKind::Hash,
                '$' => SassSyntaxKind::Dollar,
                '@' => SassSyntaxKind::At,
                '&' => SassSyntaxKind::Ampersand,
                '!' => SassSyntaxKind::Exclamation,
                '?' => SassSyntaxKind::Question,
                '~' => SassSyntaxKind::Tilde,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
