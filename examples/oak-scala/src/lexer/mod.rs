#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::ScalaLanguage, lexer::token_type::ScalaTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, ScalaLanguage>;

static SCALA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCALA_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static SCALA_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static SCALA_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone, Debug)]
pub struct ScalaLexer<'config> {
    _config: &'config ScalaLanguage,
}

impl<'config> Lexer<ScalaLanguage> for ScalaLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ScalaLanguage>) -> LexOutput<ScalaLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ScalaLexer<'config> {
    pub fn new(config: &'config ScalaLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
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

            // 错误处理：如果没有匹配任何规则，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ScalaTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SCALA_WHITESPACE.scan(state, ScalaTokenType::Whitespace)
    }

    /// 处理换行
    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(ScalaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ScalaTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        // 行注释 & 块注释
        if SCALA_COMMENT.scan(state, ScalaTokenType::LineComment, ScalaTokenType::BlockComment) {
            return true;
        }

        false
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SCALA_STRING.scan(state, ScalaTokenType::StringLiteral)
    }

    fn lex_char_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        SCALA_CHAR.scan(state, ScalaTokenType::CharLiteral)
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        let start = state.get_position();
        let mut len = 0;

        // 跳过数字
        while let Some(ch) = state.source().get_char_at(start + len) {
            if ch.is_ascii_digit() {
                len += ch.len_utf8();
            }
            else if ch == '.' {
                // 浮点数
                len += ch.len_utf8();
                while let Some(ch) = state.source().get_char_at(start + len) {
                    if ch.is_ascii_digit() {
                        len += ch.len_utf8();
                    }
                    else {
                        break;
                    }
                }
                break;
            }
            else {
                break;
            }
        }

        state.advance(len);
        let end = state.get_position();
        state.add_token(ScalaTokenType::IntegerLiteral, start, end);
        true
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let first_char = match state.current() {
            Some(c) if c.is_alphabetic() || c == '_' => c,
            _ => return false,
        };

        let start = state.get_position();
        let mut len = first_char.len_utf8();

        while let Some(ch) = state.source().get_char_at(start + len) {
            if ch.is_alphanumeric() || ch == '_' {
                len += ch.len_utf8();
            }
            else {
                break;
            }
        }

        let text = state.source().get_text_in((start..start + len).into());
        state.advance(len);
        let end = state.get_position();

        let kind = match text.as_ref() {
            "abstract" => ScalaTokenType::Abstract,
            "case" => ScalaTokenType::Case,
            "catch" => ScalaTokenType::Catch,
            "class" => ScalaTokenType::Class,
            "def" => ScalaTokenType::Def,
            "do" => ScalaTokenType::Do,
            "else" => ScalaTokenType::Else,
            "extends" => ScalaTokenType::Extends,
            "false" => ScalaTokenType::False,
            "final" => ScalaTokenType::Final,
            "finally" => ScalaTokenType::Finally,
            "for" => ScalaTokenType::For,
            "if" => ScalaTokenType::If,
            "implicit" => ScalaTokenType::Implicit,
            "import" => ScalaTokenType::Import,
            "lazy" => ScalaTokenType::Lazy,
            "match" => ScalaTokenType::Match,
            "new" => ScalaTokenType::New,
            "null" => ScalaTokenType::Null,
            "object" => ScalaTokenType::Object,
            "override" => ScalaTokenType::Override,
            "package" => ScalaTokenType::Package,
            "private" => ScalaTokenType::Private,
            "protected" => ScalaTokenType::Protected,
            "return" => ScalaTokenType::Return,
            "sealed" => ScalaTokenType::Sealed,
            "super" => ScalaTokenType::Super,
            "this" => ScalaTokenType::This,
            "throw" => ScalaTokenType::Throw,
            "trait" => ScalaTokenType::Trait,
            "true" => ScalaTokenType::True,
            "try" => ScalaTokenType::Try,
            "type" => ScalaTokenType::Type,
            "val" => ScalaTokenType::Val,
            "var" => ScalaTokenType::Var,
            "while" => ScalaTokenType::While,
            "with" => ScalaTokenType::With,
            "yield" => ScalaTokenType::Yield,
            _ => ScalaTokenType::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // 多字符操作符
        if state.starts_with("=>") {
            state.advance(2);
            state.add_token(ScalaTokenType::Arrow, start, state.get_position());
            return true;
        }
        if state.starts_with("<=") {
            state.advance(2);
            state.add_token(ScalaTokenType::LessEqual, start, state.get_position());
            return true;
        }
        if state.starts_with(">=") {
            state.advance(2);
            state.add_token(ScalaTokenType::GreaterEqual, start, state.get_position());
            return true;
        }
        if state.starts_with("==") {
            state.advance(2);
            state.add_token(ScalaTokenType::EqualEqual, start, state.get_position());
            return true;
        }
        if state.starts_with("!=") {
            state.advance(2);
            state.add_token(ScalaTokenType::NotEqual, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };
        let start = state.get_position();
        state.advance(ch.len_utf8());
        let end = state.get_position();

        let kind = match ch {
            '(' => ScalaTokenType::LeftParen,
            ')' => ScalaTokenType::RightParen,
            '[' => ScalaTokenType::LeftBracket,
            ']' => ScalaTokenType::RightBracket,
            '{' => ScalaTokenType::LeftBrace,
            '}' => ScalaTokenType::RightBrace,
            ',' => ScalaTokenType::Comma,
            ';' => ScalaTokenType::Semicolon,
            ':' => ScalaTokenType::Colon,
            '.' => ScalaTokenType::Dot,
            '+' => ScalaTokenType::Plus,
            '-' => ScalaTokenType::Minus,
            '*' => ScalaTokenType::Star,
            '/' => ScalaTokenType::Slash,
            '%' => ScalaTokenType::Percent,
            '=' => ScalaTokenType::Eq,
            '<' => ScalaTokenType::Lt,
            '>' => ScalaTokenType::Gt,
            '!' => ScalaTokenType::Not,
            '&' => ScalaTokenType::And,
            '|' => ScalaTokenType::Or,
            '^' => ScalaTokenType::Xor,
            '~' => ScalaTokenType::Tilde,
            '?' => ScalaTokenType::Question,
            '@' => ScalaTokenType::At,
            '#' => ScalaTokenType::Hash,
            _ => {
                return false;
            }
        };

        state.add_token(kind, start, end);
        true
    }
}
