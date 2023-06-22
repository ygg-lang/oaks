use crate::{kind::SassSyntaxKind, language::SassLanguage};
use oak_core::{
    Lexer, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, SassLanguage>;

static SASS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SASS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: false });
static SASS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static SASS_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct SassLexer<'config> {
    _config: &'config SassLanguage,
}

impl<'config> Lexer<SassLanguage> for SassLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<SassLanguage>) -> LexOutput<SassLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SassLexer<'config> {
    pub fn new(config: &'config SassLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SASS_WHITESPACE.scan(state, SassSyntaxKind::Whitespace)
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SASS_COMMENT.scan(state, SassSyntaxKind::LineComment, SassSyntaxKind::BlockComment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if SASS_STRING.scan(state, SassSyntaxKind::StringLiteral) {
            return true;
        }
        if SASS_CHAR.scan(state, SassSyntaxKind::StringLiteral) {
            return true;
        }
        false
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_float = false;
        state.advance(first.len_utf8());

        // 读取数字部分
        while let Some(c) = state.current() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        // fractional part
        if state.current() == Some('.') {
            let n1 = state.source().get_char_at(state.get_position() + 1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_float = true;
                state.advance(1); // consume '.'
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(c.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // 单位后缀 (px, em, rem, %, etc.)
        while let Some(c) = state.current() {
            if c.is_ascii_alphabetic() || c == '%' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        state.add_token(if is_float { SassSyntaxKind::FloatLiteral } else { SassSyntaxKind::NumberLiteral }, start, end);
        true
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };
        if !(ch.is_ascii_alphabetic() || ch == '_' || ch == '-' || ch == '@' || ch == '!') {
            return false;
        }
        state.advance(ch.len_utf8());
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }
        let end = state.get_position();
        let text = state.source().get_text_in(core::range::Range { start, end });
        let kind = match text.as_ref() {
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
        state.add_token(kind, start, end);
        true
    }

    fn lex_variable<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('$') {
            return false;
        }
        state.advance(1);

        // 变量名必须以字母或下划线开头
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(c) = state.current() {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                        state.advance(c.len_utf8());
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

    fn lex_color_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if state.current() != Some('#') {
            return false;
        }
        state.advance(1);

        let mut hex_digits = 0;
        while let Some(c) = state.current() {
            if c.is_ascii_hexdigit() {
                state.advance(c.len_utf8());
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

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // 多字符操作符
        let patterns: &[(&str, SassSyntaxKind)] = &[("==", SassSyntaxKind::EqEq), ("!=", SassSyntaxKind::Ne), ("<=", SassSyntaxKind::Le), (">=", SassSyntaxKind::Ge)];

        for (pat, kind) in patterns {
            if state.source().get_text_from(start).as_ref().starts_with(pat) {
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

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
