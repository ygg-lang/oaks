#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::DjangoLanguage, lexer::token_type::DjangoTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::TextEdit,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, DjangoLanguage>;

static DJANGO_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static _DJANGO_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "{#", block_start: "{#", block_end: "#}", nested_blocks: false });
static DJANGO_STRING_DOUBLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static DJANGO_STRING_SINGLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct DjangoLexer<'config> {
    _config: &'config DjangoLanguage,
}

impl<'config> Lexer<DjangoLanguage> for DjangoLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<DjangoLanguage>) -> LexOutput<DjangoLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> DjangoLexer<'config> {
    pub fn new(config: &'config DjangoLanguage) -> Self {
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

            if self.lex_string(state) || self.lex_string_manual(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_django_tags(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            if self.lex_html_text(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        DJANGO_WHITESPACE.scan(state, DjangoTokenType::Whitespace)
    }

    /// 跳过注释
    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.rest().starts_with("{#") {
            let start = state.get_position();
            state.advance(2); // 跳过 "{#"

            // 查找注释结束标记 "#}"
            while state.not_at_end() {
                if state.rest().starts_with("#}") {
                    state.advance(2); // 跳过 "#}"
                    break;
                }
                state.advance(1)
            }

            state.add_token(DjangoTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    /// 词法分析字符串
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        DJANGO_STRING_DOUBLE.scan(state, DjangoTokenType::String) || DJANGO_STRING_SINGLE.scan(state, DjangoTokenType::String)
    }

    /// 处理换行
    fn _lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(DjangoTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(DjangoTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());

                let token_kind = match text.as_ref() {
                    "if" => DjangoTokenType::If,
                    "elif" => DjangoTokenType::Elif,
                    "else" => DjangoTokenType::Else,
                    "endif" => DjangoTokenType::Endif,
                    "for" => DjangoTokenType::For,
                    "empty" => DjangoTokenType::Empty,
                    "endfor" => DjangoTokenType::Endfor,
                    "block" => DjangoTokenType::Block,
                    "endblock" => DjangoTokenType::Endblock,
                    "extends" => DjangoTokenType::Extends,
                    "include" => DjangoTokenType::Include,
                    "load" => DjangoTokenType::Load,
                    "with" => DjangoTokenType::With,
                    "endwith" => DjangoTokenType::Endwith,
                    "autoescape" => DjangoTokenType::Autoescape,
                    "endautoescape" => DjangoTokenType::Endautoescape,
                    "csrf_token" => DjangoTokenType::Csrf,
                    "url" => DjangoTokenType::Url,
                    "static" => DjangoTokenType::Static,
                    "now" => DjangoTokenType::Now,
                    "cycle" => DjangoTokenType::Cycle,
                    "filter" => DjangoTokenType::Filter,
                    "endfilter" => DjangoTokenType::Endfilter,
                    "spaceless" => DjangoTokenType::Spaceless,
                    "endspaceless" => DjangoTokenType::Endspaceless,
                    "verbatim" => DjangoTokenType::Verbatim,
                    "endverbatim" => DjangoTokenType::Endverbatim,
                    "and" => DjangoTokenType::And,
                    "or" => DjangoTokenType::Or,
                    "not" => DjangoTokenType::Not,
                    "in" => DjangoTokenType::In,
                    _ => DjangoTokenType::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理数字
    /// 词法分析数字
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    let dot_pos = state.get_position();
                    state.advance(1);

                    if let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(ch.len_utf8());
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                    else {
                        // 回退点号
                        state.set_position(dot_pos);
                    }
                }

                state.add_token(DjangoTokenType::Number, start_pos, state.get_position());
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

    /// 处理字符

    fn lex_string_manual<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        state.add_token(DjangoTokenType::String, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if state.peek().is_some() {
                            state.advance(1)
                        }
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                // 未闭合的字符

                state.add_token(DjangoTokenType::Error, start_pos, state.get_position());
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

    /// 处理 Django 标签
    fn lex_django_tags<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            state.advance(1);

            if let Some(next_ch) = state.peek() {
                match next_ch {
                    '{' => {
                        // 变量标签 {{
                        state.advance(1);
                        state.add_token(DjangoTokenType::VariableStart, start_pos, state.get_position());
                        true
                    }
                    '%' => {
                        // 模板标签 {%
                        state.advance(1);
                        state.add_token(DjangoTokenType::TagStart, start_pos, state.get_position());
                        true
                    }
                    '#' => {
                        // 注释标签 {#
                        state.advance(1);
                        state.add_token(DjangoTokenType::CommentStart, start_pos, state.get_position());
                        true
                    }
                    _ => {
                        // 回退
                        state.set_position(start_pos);
                        false
                    }
                }
            }
            else {
                // 回退
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('%') = state.peek() {
            state.advance(1);
            if let Some('}') = state.peek() {
                state.advance(1);
                state.add_token(DjangoTokenType::TagEnd, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('}') = state.peek() {
            state.advance(1);
            if let Some('}') = state.peek() {
                state.advance(1);
                state.add_token(DjangoTokenType::VariableEnd, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作符
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        Some(DjangoTokenType::EqualEqual)
                    }
                    else {
                        Some(DjangoTokenType::Equal)
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        Some(DjangoTokenType::NotEqual)
                    }
                    else {
                        None
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        Some(DjangoTokenType::LessEqual)
                    }
                    else {
                        Some(DjangoTokenType::Less)
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        Some(DjangoTokenType::GreaterEqual)
                    }
                    else {
                        Some(DjangoTokenType::Greater)
                    }
                }
                '|' => {
                    state.advance(1);
                    Some(DjangoTokenType::Pipe)
                }
                ':' => {
                    state.advance(1);
                    Some(DjangoTokenType::Colon)
                }
                '.' => {
                    state.advance(1);
                    Some(DjangoTokenType::Dot)
                }
                ',' => {
                    state.advance(1);
                    Some(DjangoTokenType::Comma)
                }
                '+' => {
                    state.advance(1);
                    Some(DjangoTokenType::Plus)
                }
                '-' => {
                    state.advance(1);
                    Some(DjangoTokenType::Minus)
                }
                '*' => {
                    state.advance(1);
                    Some(DjangoTokenType::Star)
                }
                '/' => {
                    state.advance(1);
                    Some(DjangoTokenType::Slash)
                }
                _ => None,
            };

            if let Some(kind) = kind {
                state.add_token(kind, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => Some(DjangoTokenType::LeftParen),
                ')' => Some(DjangoTokenType::RightParen),
                '[' => Some(DjangoTokenType::LeftBracket),
                ']' => Some(DjangoTokenType::RightBracket),
                ';' => Some(DjangoTokenType::Semicolon),
                _ => None,
            };

            if let Some(kind) = kind {
                state.advance(1);
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

    /// 处理 HTML 文本
    fn lex_html_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 如果遇到 Django 标签的开始，停止
            if ch == '{' {
                let current_pos = state.get_position();
                state.advance(1);
                if let Some(next_ch) = state.peek() {
                    if next_ch == '{' || next_ch == '%' || next_ch == '#' {
                        state.set_position(current_pos);
                        break;
                    }
                }
            }
            state.advance(ch.len_utf8())
        }

        if state.get_position() > start_pos {
            state.add_token(DjangoTokenType::HtmlContent, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
