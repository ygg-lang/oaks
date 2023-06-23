pub mod token_type;
pub use token_type::MojoTokenType;

use crate::MojoLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, MojoLanguage>;

/// Mojo 词法分析器
#[derive(Clone, Default)]
pub struct MojoLexer {}

impl Lexer<MojoLanguage> for MojoLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<MojoLanguage>) -> LexOutput<MojoLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl MojoLexer {
    /// 创建新的词法分析器
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let mut indent_stack = vec![0];
        let mut bracket_level: usize = 0;
        let mut at_line_start = true;

        while state.not_at_end() {
            let safe_point = state.get_position();

            if at_line_start && bracket_level == 0 {
                self.handle_indentation(state, &mut indent_stack);
                at_line_start = false;
                continue;
            }

            if let Some(ch) = state.current() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state, bracket_level);
                        at_line_start = true;
                    }
                    '#' => {
                        self.lex_comment(state);
                    }
                    '"' | '\'' => {
                        self.lex_string(state);
                    }
                    '0'..='9' => {
                        self.lex_number(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier_or_keyword(state);
                    }
                    '(' | '[' | '{' => {
                        bracket_level += 1;
                        self.lex_delimiter(state);
                    }
                    ')' | ']' | '}' => {
                        bracket_level = bracket_level.saturating_sub(1);
                        self.lex_delimiter(state);
                    }
                    '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' => {
                        self.lex_operator(state);
                    }
                    ',' | ':' | ';' | '.' => {
                        self.lex_delimiter(state);
                    }
                    _ => {
                        state.advance(ch.len_utf8());
                        state.add_token(MojoTokenType::Error, safe_point, state.get_position())
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }

        // Emit remaining dedents
        while indent_stack.len() > 1 {
            indent_stack.pop();
            let pos = state.get_position();
            state.add_token(MojoTokenType::Dedent, pos, pos)
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8())
            }
            else {
                break;
            }
        }
        if state.get_position() > start_pos {
            state.add_token(MojoTokenType::Whitespace, start_pos, state.get_position());
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, bracket_level: usize) {
        let start_pos = state.get_position();
        let kind = if bracket_level > 0 { MojoTokenType::Whitespace } else { MojoTokenType::Newline };

        if let Some('\n') = state.current() {
            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
        }
        else if let Some('\r') = state.current() {
            state.advance(1);
            if let Some('\n') = state.current() {
                state.advance(1);
            }
            state.add_token(kind, start_pos, state.get_position());
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        state.advance(1); // skip '#'
        while let Some(ch) = state.current() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            state.advance(ch.len_utf8())
        }
        state.add_token(MojoTokenType::Comment, start_pos, state.get_position());
    }

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let quote = state.current().unwrap();
        state.advance(1);
        let mut escaped = false;
        while let Some(ch) = state.current() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8());
                continue;
            }
            if ch == '\\' {
                escaped = true;
                state.advance(1);
                continue;
            }
            if ch == quote {
                state.advance(1);
                break;
            }
            state.advance(ch.len_utf8());
        }
        state.add_token(MojoTokenType::String, start_pos, state.get_position());
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let mut is_float = false;
        while let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else if ch == '.' && !is_float {
                is_float = true;
                state.advance(1);
            }
            else {
                break;
            }
        }
        let kind = if is_float { MojoTokenType::Float } else { MojoTokenType::Integer };
        state.add_token(kind, start_pos, state.get_position());
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        while let Some(ch) = state.current() {
            if ch.is_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        let text = state.get_text_in(oak_core::Range { start: start_pos, end: state.get_position() });
        let kind = match text.as_ref() {
            "fn" => MojoTokenType::Fn,
            "struct" => MojoTokenType::Struct,
            "var" => MojoTokenType::Var,
            "let" => MojoTokenType::Let,
            "if" => MojoTokenType::If,
            "else" => MojoTokenType::Else,
            "while" => MojoTokenType::While,
            "for" => MojoTokenType::For,
            "in" => MojoTokenType::In,
            "return" => MojoTokenType::Return,
            "break" => MojoTokenType::Break,
            "continue" => MojoTokenType::Continue,
            "import" => MojoTokenType::Import,
            "from" => MojoTokenType::From,
            "True" => MojoTokenType::True,
            "False" => MojoTokenType::False,
            "None" => MojoTokenType::None,
            _ => MojoTokenType::Identifier,
        };
        state.add_token(kind, start_pos, state.get_position());
    }

    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let ch = state.current().unwrap();
        state.advance(1);
        let kind = match ch {
            '+' => MojoTokenType::Plus,
            '-' => {
                if let Some('>') = state.current() {
                    state.advance(1);
                    MojoTokenType::Arrow
                }
                else {
                    MojoTokenType::Minus
                }
            }
            '*' => MojoTokenType::Star,
            '/' => MojoTokenType::Slash,
            '%' => MojoTokenType::Percent,
            '=' => {
                if let Some('=') = state.current() {
                    state.advance(1);
                    MojoTokenType::EqualEqual
                }
                else {
                    MojoTokenType::Equal
                }
            }
            '<' => {
                if let Some('=') = state.current() {
                    state.advance(1);
                    MojoTokenType::LessEqual
                }
                else {
                    MojoTokenType::Less
                }
            }
            '>' => {
                if let Some('=') = state.current() {
                    state.advance(1);
                    MojoTokenType::GreaterEqual
                }
                else {
                    MojoTokenType::Greater
                }
            }
            '!' => {
                if let Some('=') = state.current() {
                    state.advance(1);
                    MojoTokenType::NotEqual
                }
                else {
                    MojoTokenType::Error
                }
            }
            _ => MojoTokenType::Error,
        };
        state.add_token(kind, start_pos, state.get_position());
    }

    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        let start_pos = state.get_position();
        let ch = state.current().unwrap();
        state.advance(1);
        let kind = match ch {
            '(' => MojoTokenType::LeftParen,
            ')' => MojoTokenType::RightParen,
            '[' => MojoTokenType::LeftBracket,
            ']' => MojoTokenType::RightBracket,
            '{' => MojoTokenType::LeftBrace,
            '}' => MojoTokenType::RightBrace,
            ',' => MojoTokenType::Comma,
            ':' => MojoTokenType::Colon,
            ';' => MojoTokenType::Semicolon,
            '.' => MojoTokenType::Dot,
            _ => MojoTokenType::Error,
        };
        state.add_token(kind, start_pos, state.get_position());
    }

    fn handle_indentation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<usize>) {
        let start_pos = state.get_position();
        let mut indent = 0;
        let mut temp_pos = start_pos;

        while let Some(ch) = state.get_char_at(temp_pos) {
            if ch == ' ' {
                indent += 1;
            }
            else if ch == '\t' {
                indent += 4; // Mojo usually uses 4 spaces for tabs
            }
            else {
                break;
            }
            temp_pos += ch.len_utf8();
        }

        match state.get_char_at(temp_pos) {
            Some('\n') | Some('\r') | Some('#') => {
                // Empty line or comment, don't change indentation
                return;
            }
            None => return, // EOF
            _ => {}
        }

        state.advance(temp_pos - start_pos);
        if state.get_position() > start_pos {
            state.add_token(MojoTokenType::Whitespace, start_pos, state.get_position());
        }

        let last_indent = *stack.last().unwrap();
        if indent > last_indent {
            stack.push(indent);
            state.add_token(MojoTokenType::Indent, state.get_position(), state.get_position());
        }
        else {
            while indent < *stack.last().unwrap() {
                stack.pop();
                state.add_token(MojoTokenType::Dedent, state.get_position(), state.get_position());
            }
        }
    }
}
