#![doc = include_str!("readme.md")]
pub mod token_type;

pub use self::token_type::PythonTokenType;
use crate::language::PythonLanguage;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, PythonLanguage>;

/// Python lexer implementation.
#[derive(Clone)]
pub struct PythonLexer<'config> {
    _config: &'config PythonLanguage,
}

impl<'config> Lexer<PythonLanguage> for PythonLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<PythonLanguage>) -> LexOutput<PythonLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PythonLexer<'config> {
    /// Creates a new Python lexer.
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { _config: config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(PythonTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newline characters.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, bracket_level: usize) -> bool {
        let start_pos = state.get_position();
        let kind = if bracket_level > 0 { PythonTokenType::Whitespace } else { PythonTokenType::Newline };

        if let Some('\n') = state.current() {
            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.current() {
            state.advance(1);
            if let Some('\n') = state.current() {
                state.advance(1);
            }
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('#') = state.current() {
            let start_pos = state.get_position();
            state.advance(1); // Skip '#'

            // Read until end of line
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            state.add_token(PythonTokenType::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles string literals.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Check for prefixes (f, r, b, u, etc.)
        let mut prefix = None;
        if let Some(ch) = state.current() {
            if "frbuFRBU".contains(ch) {
                // Check if next char is a quote
                if let Some(next_ch) = state.peek_next_n(ch.len_utf8()) {
                    if next_ch == '"' || next_ch == '\'' {
                        prefix = Some(ch.to_ascii_lowercase());
                        state.advance(ch.len_utf8());
                    }
                }
            }
        }

        // Check if it's the start of a string
        let quote_char = match state.current() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => {
                if prefix.is_some() {
                    // This shouldn't happen if we checked correctly above
                    return false;
                }
                return false;
            }
        };

        state.advance(1); // Skip first quote

        // Check if it's a triple-quoted string
        let is_triple = if let (Some(c1), Some(c2)) = (state.peek_next_n(0), state.peek_next_n(1)) { c1 == quote_char && c2 == quote_char } else { false };

        if is_triple {
            state.advance(2); // Skip remaining two quotes
        }

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

            if ch == quote_char {
                if is_triple {
                    if let (Some(c1), Some(c2)) = (state.peek_next_n(1), state.peek_next_n(2)) {
                        if c1 == quote_char && c2 == quote_char {
                            state.advance(3); // Skip three quotes
                            break;
                        }
                    }
                    state.advance(1);
                    continue;
                }
                else {
                    state.advance(1); // Skip closing quote
                    break;
                }
            }
            else if (ch == '\n' || ch == '\r') && !is_triple {
                // Single-line strings cannot contain newlines
                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        let kind = match prefix {
            Some('f') => PythonTokenType::FString,
            Some('b') => PythonTokenType::Bytes,
            _ => PythonTokenType::String,
        };
        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// Handles number literals.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        // Simple implementation: only handles basic decimal numbers
        while let Some(ch) = state.current() {
            if ch.is_ascii_digit() || ch == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(PythonTokenType::Number, start_pos, state.get_position());
        true
    }

    /// Handles identifiers or keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Check first character
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // Read identifier
        let mut text = String::new();
        while let Some(ch) = state.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                text.push(ch);
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        // Check if it's a keyword
        let kind = match text.as_str() {
            "and" => PythonTokenType::AndKeyword,
            "as" => PythonTokenType::AsKeyword,
            "assert" => PythonTokenType::AssertKeyword,
            "async" => PythonTokenType::AsyncKeyword,
            "await" => PythonTokenType::AwaitKeyword,
            "break" => PythonTokenType::BreakKeyword,
            "class" => PythonTokenType::ClassKeyword,
            "continue" => PythonTokenType::ContinueKeyword,
            "def" => PythonTokenType::DefKeyword,
            "del" => PythonTokenType::DelKeyword,
            "elif" => PythonTokenType::ElifKeyword,
            "else" => PythonTokenType::ElseKeyword,
            "except" => PythonTokenType::ExceptKeyword,
            "False" => PythonTokenType::FalseKeyword,
            "finally" => PythonTokenType::FinallyKeyword,
            "for" => PythonTokenType::ForKeyword,
            "from" => PythonTokenType::FromKeyword,
            "global" => PythonTokenType::GlobalKeyword,
            "if" => PythonTokenType::IfKeyword,
            "import" => PythonTokenType::ImportKeyword,
            "in" => PythonTokenType::InKeyword,
            "is" => PythonTokenType::IsKeyword,
            "lambda" => PythonTokenType::LambdaKeyword,
            "None" => PythonTokenType::NoneKeyword,
            "nonlocal" => PythonTokenType::NonlocalKeyword,
            "not" => PythonTokenType::NotKeyword,
            "or" => PythonTokenType::OrKeyword,
            "pass" => PythonTokenType::PassKeyword,
            "raise" => PythonTokenType::RaiseKeyword,
            "return" => PythonTokenType::ReturnKeyword,
            "True" => PythonTokenType::TrueKeyword,
            "try" => PythonTokenType::TryKeyword,
            "while" => PythonTokenType::WhileKeyword,
            "with" => PythonTokenType::WithKeyword,
            "yield" => PythonTokenType::YieldKeyword,
            _ => PythonTokenType::Identifier,
        };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// Handles operators.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::PlusAssign
                    }
                    else {
                        PythonTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::MinusAssign
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        PythonTokenType::Arrow
                    }
                    else {
                        PythonTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::StarAssign
                    }
                    else if let Some('*') = state.current() {
                        state.advance(1);
                        if let Some('=') = state.current() {
                            state.advance(1);
                            PythonTokenType::DoubleStarAssign
                        }
                        else {
                            PythonTokenType::DoubleStar
                        }
                    }
                    else {
                        PythonTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::SlashAssign
                    }
                    else if let Some('/') = state.current() {
                        state.advance(1);
                        if let Some('=') = state.current() {
                            state.advance(1);
                            PythonTokenType::DoubleSlashAssign
                        }
                        else {
                            PythonTokenType::DoubleSlash
                        }
                    }
                    else {
                        PythonTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::PercentAssign
                    }
                    else {
                        PythonTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::Equal
                    }
                    else {
                        PythonTokenType::Assign
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::LessEqual
                    }
                    else if let Some('<') = state.current() {
                        state.advance(1);
                        if let Some('=') = state.current() {
                            state.advance(1);
                            PythonTokenType::LeftShiftAssign
                        }
                        else {
                            PythonTokenType::LeftShift
                        }
                    }
                    else {
                        PythonTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        if let Some('=') = state.current() {
                            state.advance(1);
                            PythonTokenType::RightShiftAssign
                        }
                        else {
                            PythonTokenType::RightShift
                        }
                    }
                    else {
                        PythonTokenType::Greater
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::NotEqual
                    }
                    else {
                        return false;
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::AmpersandAssign
                    }
                    else {
                        PythonTokenType::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::PipeAssign
                    }
                    else {
                        PythonTokenType::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::CaretAssign
                    }
                    else {
                        PythonTokenType::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    PythonTokenType::Tilde
                }
                '@' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        PythonTokenType::AtAssign
                    }
                    else {
                        PythonTokenType::At
                    }
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// Handles delimiters.
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => PythonTokenType::LeftParen,
                ')' => PythonTokenType::RightParen,
                '[' => PythonTokenType::LeftBracket,
                ']' => PythonTokenType::RightBracket,
                '{' => PythonTokenType::LeftBrace,
                '}' => PythonTokenType::RightBrace,
                ',' => PythonTokenType::Comma,
                ':' => PythonTokenType::Colon,
                ';' => PythonTokenType::Semicolon,
                '.' => PythonTokenType::Dot, // Simple handling, ellipses not supported
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }
}

impl<'config> PythonLexer<'config> {
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

            if let Some(ch) = state.peek() {
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
                    'f' | 'r' | 'b' | 'u' | 'F' | 'R' | 'B' | 'U' => {
                        if !self.lex_string(state) {
                            self.lex_identifier_or_keyword(state);
                        }
                    }
                    'a'..='e' | 'g'..='q' | 's' | 't' | 'v'..='z' | 'A'..='E' | 'G'..='Q' | 'S' | 'T' | 'V'..='Z' | '_' => {
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
                    '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '&' | '|' | '^' | '~' | '@' => {
                        self.lex_operator(state);
                    }
                    ',' | ':' | ';' | '.' => {
                        self.lex_delimiter(state);
                    }
                    _ => {
                        // Fallback to error
                        state.advance(ch.len_utf8());
                        state.add_token(PythonTokenType::Error, safe_point, state.get_position())
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }

        // Emit remaining dedents
        while indent_stack.len() > 1 {
            indent_stack.pop();
            let pos = state.get_position();
            state.add_token(PythonTokenType::Dedent, pos, pos)
        }

        Ok(())
    }

    fn handle_indentation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<usize>) {
        let start_pos = state.get_position();
        let current_indent;

        // Skip comments and empty lines at start of line
        let mut temp_state = state.get_position();
        loop {
            let mut indent = 0;
            while let Some(ch) = state.get_char_at(temp_state) {
                if ch == ' ' {
                    indent += 1
                }
                else if ch == '\t' {
                    indent += 8
                }
                // Standard Python tab width
                else {
                    break;
                }
                temp_state += 1
            }

            match state.get_char_at(temp_state) {
                Some('\n') | Some('\r') | Some('#') => {
                    // This is an empty line or comment-only line, ignore indentation change
                    return;
                }
                None => return, // EOF
                _ => {
                    current_indent = indent;
                    break;
                }
            }
        }

        // Advance state to skip the indentation we just measured
        if current_indent > 0 {
            let end_pos = state.get_position() + (temp_state - state.get_position());
            state.add_token(PythonTokenType::Whitespace, start_pos, end_pos);
            state.set_position(end_pos);
        }

        let last_indent = *stack.last().unwrap();
        if current_indent > last_indent {
            stack.push(current_indent);
            state.add_token(PythonTokenType::Indent, state.get_position(), state.get_position())
        }
        else {
            while current_indent < *stack.last().unwrap() {
                stack.pop();
                state.add_token(PythonTokenType::Dedent, state.get_position(), state.get_position())
            }
            // If current_indent doesn't match any previous level, it's an indentation error,
            // but for now we just stop at the closest level.
        }
    }
}
