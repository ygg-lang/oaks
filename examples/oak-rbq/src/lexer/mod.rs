use crate::{kind::RbqSyntaxKind, language::RbqLanguage};
use oak_core::{
    errors::OakError,
    lexer::{CommentConfig, LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, RbqLanguage>;

static RBQ_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });

#[derive(Clone, Debug)]
pub struct RbqLexer<'config> {
    _config: &'config RbqLanguage,
}

impl<'config> Lexer<RbqLanguage> for RbqLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<RbqLanguage>) -> LexOutput<RbqLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> RbqLexer<'config> {
    pub fn new(config: &'config RbqLanguage) -> Self {
        Self { _config: config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            let Some(ch) = state.peek()
            else {
                break;
            };

            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    let start = state.get_position();
                    state.skip_ascii_whitespace();
                    state.add_token(RbqSyntaxKind::Whitespace, start, state.get_position());
                }
                '#' => {
                    RBQ_COMMENT.scan(state, RbqSyntaxKind::Comment, RbqSyntaxKind::Comment);
                }
                '"' => {
                    self.lex_string(state);
                }
                '0'..='9' => {
                    self.lex_number(state);
                }
                '{' | '}' | '[' | ']' | '(' | ')' | ':' | ';' | ',' | '.' | '?' | '@' | '$' => {
                    self.lex_punctuation(state);
                }
                '=' | '!' | '>' | '<' | '&' | '|' | '+' | '-' | '*' | '/' => {
                    self.lex_operator(state);
                }
                _ if ch.is_alphabetic() || ch == '_' => {
                    self.lex_ident_or_keyword(state);
                }
                _ => {
                    state.advance(ch.len_utf8());
                    state.add_token(RbqSyntaxKind::Error, safe_point, state.get_position());
                }
            }
            state.advance_if_dead_lock(safe_point);
        }
        Ok(())
    }

    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();
        state.advance(1); // "
        while state.not_at_end() {
            if state.consume_if_starts_with("\"") {
                state.add_token(RbqSyntaxKind::StringLiteral, start, state.get_position());
                return;
            }
            if state.consume_if_starts_with("\\") {
                state.advance(1);
            }
            else {
                state.advance(1);
            }
        }
        state.add_token(RbqSyntaxKind::Error, start, state.get_position());
    }

    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) {
        let start = state.get_position();
        state.take_while(|c| c.is_ascii_digit());
        if state.consume_if_starts_with(".") {
            state.take_while(|c| c.is_ascii_digit());
        }
        state.add_token(RbqSyntaxKind::NumberLiteral, start, state.get_position());
    }

    fn lex_punctuation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        let ch = state.peek().unwrap();
        let kind = match ch {
            '{' => RbqSyntaxKind::LeftBrace,
            '}' => RbqSyntaxKind::RightBrace,
            '[' => RbqSyntaxKind::LeftBracket,
            ']' => RbqSyntaxKind::RightBracket,
            '(' => RbqSyntaxKind::LeftParen,
            ')' => RbqSyntaxKind::RightParen,
            ':' => RbqSyntaxKind::Colon,
            ';' => RbqSyntaxKind::Semicolon,
            ',' => RbqSyntaxKind::Comma,
            '.' => RbqSyntaxKind::Dot,
            '?' => RbqSyntaxKind::Question,
            '@' => RbqSyntaxKind::At,
            '$' => RbqSyntaxKind::Dollar,
            _ => unreachable!(),
        };
        state.advance(1);
        state.add_token(kind, start, state.get_position());
    }

    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        if state.consume_if_starts_with("==") {
            state.add_token(RbqSyntaxKind::EqEq, start, state.get_position());
        }
        else if state.consume_if_starts_with("!=") {
            state.add_token(RbqSyntaxKind::NotEq, start, state.get_position());
        }
        else if state.consume_if_starts_with(">=") {
            state.add_token(RbqSyntaxKind::GtEq, start, state.get_position());
        }
        else if state.consume_if_starts_with("<=") {
            state.add_token(RbqSyntaxKind::LtEq, start, state.get_position());
        }
        else if state.consume_if_starts_with("&&") {
            state.add_token(RbqSyntaxKind::AndAnd, start, state.get_position());
        }
        else if state.consume_if_starts_with("||") {
            state.add_token(RbqSyntaxKind::OrOr, start, state.get_position());
        }
        else if state.consume_if_starts_with("->") {
            state.add_token(RbqSyntaxKind::Arrow, start, state.get_position());
        }
        else if state.consume_if_starts_with("=") {
            state.add_token(RbqSyntaxKind::Eq, start, state.get_position());
        }
        else if state.consume_if_starts_with("!") {
            state.add_token(RbqSyntaxKind::Not, start, state.get_position());
        }
        else if state.consume_if_starts_with(">") {
            state.add_token(RbqSyntaxKind::Gt, start, state.get_position());
        }
        else if state.consume_if_starts_with("<") {
            state.add_token(RbqSyntaxKind::Lt, start, state.get_position());
        }
        else if state.consume_if_starts_with("+") {
            state.add_token(RbqSyntaxKind::Plus, start, state.get_position());
        }
        else if state.consume_if_starts_with("-") {
            state.add_token(RbqSyntaxKind::Minus, start, state.get_position());
        }
        else if state.consume_if_starts_with("*") {
            state.add_token(RbqSyntaxKind::Star, start, state.get_position());
        }
        else if state.consume_if_starts_with("/") {
            state.add_token(RbqSyntaxKind::Slash, start, state.get_position());
        }
        else if state.consume_if_starts_with("&") {
            state.add_token(RbqSyntaxKind::Ampersand, start, state.get_position());
        }
    }

    fn lex_ident_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start = state.get_position();
        state.take_while(|c| c.is_alphanumeric() || c == '_');
        let text = state.get_text_in(oak_core::Range { start, end: state.get_position() });
        let kind = match text.as_ref() {
            "struct" => RbqSyntaxKind::StructKw,
            "class" => RbqSyntaxKind::ClassKw,
            "enum" => RbqSyntaxKind::EnumKw,
            "union" => RbqSyntaxKind::UnionKw,
            "trait" => RbqSyntaxKind::TraitKw,
            "using" => RbqSyntaxKind::UsingKw,
            "namespace" => RbqSyntaxKind::NamespaceKw,
            "use" => RbqSyntaxKind::UseKw,
            "type" => RbqSyntaxKind::TypeKw,
            "micro" => RbqSyntaxKind::MicroKw,
            "utf8" => RbqSyntaxKind::Utf8Kw,
            "true" => RbqSyntaxKind::TrueKw,
            "false" => RbqSyntaxKind::FalseKw,
            _ => RbqSyntaxKind::Ident,
        };
        state.add_token(kind, start, state.get_position());
    }
}
