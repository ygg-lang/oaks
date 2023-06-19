use crate::{kind::HandlebarsSyntaxKind, language::HandlebarsLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError, Token,
    lexer::{LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, HandlebarsLanguage>;

// Scanner configurations
static HB_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static HB_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[derive(Clone)]
pub struct HandlebarsLexer<'config> {
    config: &'config HandlebarsLanguage,
}

impl<'config> Lexer<HandlebarsLanguage> for HandlebarsLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<HandlebarsLanguage>,
    ) -> LexOutput<HandlebarsLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> HandlebarsLexer<'config> {
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_handlebars_expression(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_content(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // Add EOF token
        let eof_pos = state.get_position();
        state.add_token(HandlebarsSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match HB_WHITESPACE.scan(state.rest(), state.get_position(), HandlebarsSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        if state.current() == Some('\n') || state.current() == Some('\r') {
            let start = state.get_position();
            state.advance(1);
            if state.current() == Some('\n') && state.peek() == Some('\r') {
                state.advance(1);
            }
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Newline, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        if rest.starts_with("{{!--") {
            let start = state.get_position();
            state.advance(5); // Skip "{{!--"

            // Find the end of the comment
            while state.not_at_end() {
                if state.rest().starts_with("--}}") {
                    state.advance(4); // Skip "--}}"
                    break;
                }
                state.advance(1);
            }

            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Comment, start, end);
            true
        }
        else if rest.starts_with("{{!") {
            let start = state.get_position();
            state.advance(3); // Skip "{{!"

            // Find the end of the comment
            while state.not_at_end() {
                if state.rest().starts_with("}}") {
                    state.advance(2); // Skip "}}"
                    break;
                }
                state.advance(1);
            }

            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Comment, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_handlebars_expression<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        let start = state.get_position();

        if rest.starts_with("{{{") {
            state.advance(3);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenUnescaped, start, end);
            true
        }
        else if rest.starts_with("{{#") {
            state.advance(3);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenBlock, start, end);
            true
        }
        else if rest.starts_with("{{/") {
            state.advance(3);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::CloseBlock, start, end);
            true
        }
        else if rest.starts_with("{{>") {
            state.advance(3);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenPartial, start, end);
            true
        }
        else if rest.starts_with("{{") {
            state.advance(2);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Open, start, end);
            true
        }
        else if rest.starts_with("}}}") {
            state.advance(3);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::CloseUnescaped, start, end);
            true
        }
        else if rest.starts_with("}}") {
            state.advance(2);
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Close, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match HB_STRING.scan(state.rest(), 0, HandlebarsSyntaxKind::StringLiteral) {
            Some(token) => {
                // 创建新的 token 并调整位置为绝对位置
                use std::range::Range;
                let adjusted_token = Token {
                    kind: token.kind,
                    span: Range { start: token.span.start + state.get_position(), end: token.span.end + state.get_position() },
                };
                state.advance_with(adjusted_token);
                true
            }
            None => false,
        }
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_ascii_digit() {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                state.add_token(HandlebarsSyntaxKind::NumberLiteral, start, end);
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

    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_alphabetic() || c == '_' {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                state.add_token(HandlebarsSyntaxKind::Identifier, start, end);
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

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(c) = state.current() {
            let start = state.get_position();
            let kind = match c {
                '(' => HandlebarsSyntaxKind::LeftParen,
                ')' => HandlebarsSyntaxKind::RightParen,
                '[' => HandlebarsSyntaxKind::LeftBracket,
                ']' => HandlebarsSyntaxKind::RightBracket,
                '=' => HandlebarsSyntaxKind::Equal,
                '|' => HandlebarsSyntaxKind::Pipe,
                '#' => HandlebarsSyntaxKind::Hash,
                '.' => HandlebarsSyntaxKind::Dot,
                '/' => HandlebarsSyntaxKind::Slash,
                _ => return false,
            };
            state.advance(1);
            let end = state.get_position();
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_content<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let mut has_content = false;

        while state.not_at_end() {
            let rest = state.rest();
            // Stop if we encounter Handlebars kind
            if rest.starts_with("{{") {
                break;
            }
            state.advance(1);
            has_content = true;
        }

        if has_content {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Content, start, end);
            true
        }
        else {
            false
        }
    }
}
