pub mod token_type;

use crate::{language::JinjaLanguage, lexer::token_type::JinjaTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source,
    lexer::{LexOutput, StringConfig, WhitespaceConfig},
    source::TextEdit,
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, JinjaLanguage>;

static JINJA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static JINJA_STRING_DOUBLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static JINJA_STRING_SINGLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct JinjaLexer<'config> {
    config: &'config JinjaLanguage,
}

impl<'config> Lexer<JinjaLanguage> for JinjaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<JinjaLanguage>) -> LexOutput<JinjaLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> JinjaLexer<'config> {
    pub fn new(config: &'config JinjaLanguage) -> Self {
        Self { config }
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

            if self.lex_jinja_tags(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
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

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        JINJA_WHITESPACE.scan(state, JinjaTokenType::Whitespace)
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.rest().starts_with(&self.config.comment_start) {
            let start = state.get_position();
            state.advance(self.config.comment_start.len());

            while state.not_at_end() {
                if state.rest().starts_with(&self.config.comment_end) {
                    state.advance(self.config.comment_end.len());
                    break;
                }
                state.advance(1)
            }

            state.add_token(JinjaTokenType::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_jinja_tags<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        let rest = state.rest();

        if rest.starts_with(&self.config.variable_start) {
            state.advance(self.config.variable_start.len());
            state.add_token(JinjaTokenType::VariableStart, start_pos, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.variable_end) {
            state.advance(self.config.variable_end.len());
            state.add_token(JinjaTokenType::VariableEnd, start_pos, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.tag_start) {
            state.advance(self.config.tag_start.len());
            state.add_token(JinjaTokenType::TagStart, start_pos, state.get_position());
            true
        }
        else if rest.starts_with(&self.config.tag_end) {
            state.advance(self.config.tag_end.len());
            state.add_token(JinjaTokenType::TagEnd, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        JINJA_STRING_DOUBLE.scan(state, JinjaTokenType::String) || JINJA_STRING_SINGLE.scan(state, JinjaTokenType::String)
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(JinjaTokenType::Number, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());
                let kind = match text.as_ref() {
                    "if" => JinjaTokenType::If,
                    "elif" => JinjaTokenType::Elif,
                    "else" => JinjaTokenType::Else,
                    "endif" => JinjaTokenType::Endif,
                    "for" => JinjaTokenType::For,
                    "endfor" => JinjaTokenType::Endfor,
                    "block" => JinjaTokenType::Block,
                    "endblock" => JinjaTokenType::Endblock,
                    "extends" => JinjaTokenType::Extends,
                    "include" => JinjaTokenType::Include,
                    "import" => JinjaTokenType::Import,
                    "from" => JinjaTokenType::From,
                    "macro" => JinjaTokenType::Macro,
                    "endmacro" => JinjaTokenType::Endmacro,
                    "call" => JinjaTokenType::Call,
                    "endcall" => JinjaTokenType::Endcall,
                    "filter" => JinjaTokenType::Filter,
                    "endfilter" => JinjaTokenType::Endfilter,
                    "set" => JinjaTokenType::Set,
                    "endset" => JinjaTokenType::Endset,
                    "with" => JinjaTokenType::With,
                    "endwith" => JinjaTokenType::Endwith,
                    "autoescape" => JinjaTokenType::Autoescape,
                    "endautoescape" => JinjaTokenType::Endautoescape,
                    "do" => JinjaTokenType::Do,
                    "and" => JinjaTokenType::And,
                    "or" => JinjaTokenType::Or,
                    "not" => JinjaTokenType::Not,
                    "in" => JinjaTokenType::In,
                    "is" => JinjaTokenType::Is,
                    "recursive" => JinjaTokenType::Recursive,
                    "scoped" => JinjaTokenType::Scoped,
                    _ => JinjaTokenType::Identifier,
                };
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        Some(JinjaTokenType::EqualEqual)
                    }
                    else {
                        Some(JinjaTokenType::Equal)
                    }
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        Some(JinjaTokenType::NotEqual)
                    }
                    else {
                        None
                    }
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        Some(JinjaTokenType::LessEqual)
                    }
                    else {
                        Some(JinjaTokenType::Less)
                    }
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        Some(JinjaTokenType::GreaterEqual)
                    }
                    else {
                        Some(JinjaTokenType::Greater)
                    }
                }
                '+' => {
                    state.advance(1);
                    Some(JinjaTokenType::Plus)
                }
                '-' => {
                    state.advance(1);
                    Some(JinjaTokenType::Minus)
                }
                '*' => {
                    state.advance(1);
                    if state.peek() == Some('*') {
                        state.advance(1);
                        Some(JinjaTokenType::DoubleStar)
                    }
                    else {
                        Some(JinjaTokenType::Star)
                    }
                }
                '/' => {
                    state.advance(1);
                    if state.peek() == Some('/') {
                        state.advance(1);
                        Some(JinjaTokenType::DoubleSlash)
                    }
                    else {
                        Some(JinjaTokenType::Slash)
                    }
                }
                '%' => {
                    state.advance(1);
                    Some(JinjaTokenType::Percent)
                }
                '|' => {
                    state.advance(1);
                    Some(JinjaTokenType::Pipe)
                }
                ':' => {
                    state.advance(1);
                    Some(JinjaTokenType::Colon)
                }
                '.' => {
                    state.advance(1);
                    Some(JinjaTokenType::Dot)
                }
                ',' => {
                    state.advance(1);
                    Some(JinjaTokenType::Comma)
                }
                '~' => {
                    state.advance(1);
                    Some(JinjaTokenType::Tilde)
                }
                _ => None,
            };
            if let Some(kind) = kind {
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => Some(JinjaTokenType::LeftParen),
                ')' => Some(JinjaTokenType::RightParen),
                '[' => Some(JinjaTokenType::LeftBracket),
                ']' => Some(JinjaTokenType::RightBracket),
                '{' => Some(JinjaTokenType::LeftBrace),
                '}' => Some(JinjaTokenType::RightBrace),
                _ => None,
            };
            if let Some(kind) = kind {
                state.advance(1);
                state.add_token(kind, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_html_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            let rest = state.rest();
            if rest.starts_with(&self.config.variable_start) || rest.starts_with(&self.config.tag_start) || rest.starts_with(&self.config.comment_start) {
                break;
            }
            state.advance(ch.len_utf8());
        }
        if state.get_position() > start_pos {
            state.add_token(JinjaTokenType::HtmlContent, start_pos, state.get_position());
            return true;
        }
        false
    }
}
