#![doc = include_str!("readme.md")]
pub mod token_type;
pub use token_type::CppTokenType;

use crate::language::CppLanguage;
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, CppLanguage>;

/// Lexer for the C++ language.
pub struct CppLexer<'config> {
    _config: &'config CppLanguage,
}

/// Type alias for a C lexer.
pub type CLexer<'config> = CppLexer<'config>;

impl<'config> CppLexer<'config> {
    /// Creates a new `CppLexer` with the given configuration.
    pub fn new(config: &'config CppLanguage) -> Self {
        Self { _config: config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(CppTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline sequence.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(CppTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(CppTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a comment (single-line or multi-line).
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                // Single-line comment
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(CppTokenType::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek_next_n(1) {
                // Multi-line comment
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' && state.peek_next_n(1) == Some('/') {
                        state.advance(2);
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(CppTokenType::Comment, start_pos, state.get_position());
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

    /// Lexes a string literal.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            let mut escaped = false;
            while let Some(ch) = state.peek() {
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

                if ch == '"' {
                    state.advance(1);
                    break;
                }

                if ch == '\n' || ch == '\r' {
                    break; // Unclosed string
                }

                state.advance(ch.len_utf8())
            }

            state.add_token(CppTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a character literal.
    fn lex_character<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            let mut escaped = false;
            while let Some(ch) = state.peek() {
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

                if ch == '\'' {
                    state.advance(1);
                    break;
                }

                if ch == '\n' || ch == '\r' {
                    break; // Unclosed character
                }

                state.advance(ch.len_utf8())
            }

            state.add_token(CppTokenType::CharacterLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a numeric literal.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                let mut is_float = false;

                // Handle hex, octal, binary
                if ch == '0' {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch == 'x' || next_ch == 'X' {
                            // Hexadecimal
                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_hexdigit() { state.advance(1) } else { break }
                            }
                        }
                        else if next_ch == 'b' || next_ch == 'B' {
                            // Binary
                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch == '0' || ch == '1' { state.advance(1) } else { break }
                            }
                        }
                        else if next_ch.is_ascii_digit() {
                            // Octal
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() { state.advance(1) } else { break }
                            }
                        }
                        else {
                            state.advance(1); // just '0'
                        }
                    }
                    else {
                        state.advance(1); // just '0'
                    }
                }
                else {
                    // Decimal integer part
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() { state.advance(1) } else { break }
                    }
                }

                // Check for decimal point
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // consume '.'
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() { state.advance(1) } else { break }
                            }
                        }
                    }
                }

                // Check for scientific notation
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1)
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() { state.advance(1) } else { break }
                        }
                    }
                }

                // Check for suffix
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() { state.advance(1) } else { break }
                }

                let token_kind = if is_float { CppTokenType::FloatLiteral } else { CppTokenType::IntegerLiteral };
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

    /// Lexes a keyword or identifier.
    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    // C++ Keywords
                    "alignas" | "alignof" | "and" | "and_eq" | "asm" | "atomic_cancel" | "atomic_commit" | "atomic_noexcept" | "auto" | "bitand" | "bitor" | "bool" | "break" | "case" | "catch" | "char" | "char8_t" | "char16_t" | "char32_t" | "class"
                    | "compl" | "concept" | "const" | "consteval" | "constexpr" | "constinit" | "const_cast" | "continue" | "co_await" | "co_return" | "co_yield" | "decltype" | "default" | "delete" | "do" | "double" | "dynamic_cast" | "else" | "enum"
                    | "explicit" | "export" | "extern" | "float" | "for" | "friend" | "goto" | "if" | "inline" | "int" | "long" | "mutable" | "namespace" | "new" | "noexcept" | "not" | "not_eq" | "nullptr" | "operator" | "or" | "or_eq" | "private"
                    | "protected" | "public" | "reflexpr" | "register" | "reinterpret_cast" | "requires" | "return" | "short" | "signed" | "sizeof" | "static" | "static_assert" | "static_cast" | "struct" | "switch" | "synchronized" | "template"
                    | "this" | "thread_local" | "throw" | "try" | "typedef" | "typeid" | "typename" | "union" | "unsigned" | "using" | "virtual" | "void" | "volatile" | "wchar_t" | "while" | "xor" | "xor_eq" => CppTokenType::Keyword,
                    "true" | "false" => CppTokenType::BooleanLiteral,
                    _ => CppTokenType::Identifier,
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

    /// Lexes an operator.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let (token_kind, advance_count) = match ch {
                '+' => {
                    if let Some('+') = state.peek_next_n(1) {
                        (CppTokenType::Increment, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::PlusAssign, 2)
                    }
                    else {
                        (CppTokenType::Plus, 1)
                    }
                }
                '-' => {
                    if let Some('-') = state.peek_next_n(1) {
                        (CppTokenType::Decrement, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::MinusAssign, 2)
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        (CppTokenType::Arrow, 2)
                    }
                    else {
                        (CppTokenType::Minus, 1)
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::StarAssign, 2)
                    }
                    else {
                        (CppTokenType::Star, 1)
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::SlashAssign, 2)
                    }
                    else {
                        (CppTokenType::Slash, 1)
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::PercentAssign, 2)
                    }
                    else {
                        (CppTokenType::Percent, 1)
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::Equal, 2)
                    }
                    else {
                        (CppTokenType::Assign, 1)
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::NotEqual, 2)
                    }
                    else {
                        (CppTokenType::LogicalNot, 1)
                    }
                }
                '<' => {
                    if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) { (CppTokenType::LeftShiftAssign, 3) } else { (CppTokenType::LeftShift, 2) }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::LessEqual, 2)
                    }
                    else {
                        (CppTokenType::Less, 1)
                    }
                }
                '>' => {
                    if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) { (CppTokenType::RightShiftAssign, 3) } else { (CppTokenType::RightShift, 2) }
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::GreaterEqual, 2)
                    }
                    else {
                        (CppTokenType::Greater, 1)
                    }
                }
                '&' => {
                    if let Some('&') = state.peek_next_n(1) {
                        (CppTokenType::LogicalAnd, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::AndAssign, 2)
                    }
                    else {
                        (CppTokenType::BitAnd, 1)
                    }
                }
                '|' => {
                    if let Some('|') = state.peek_next_n(1) {
                        (CppTokenType::LogicalOr, 2)
                    }
                    else if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::OrAssign, 2)
                    }
                    else {
                        (CppTokenType::BitOr, 1)
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        (CppTokenType::XorAssign, 2)
                    }
                    else {
                        (CppTokenType::BitXor, 1)
                    }
                }
                '~' => (CppTokenType::BitNot, 1),
                '?' => (CppTokenType::Question, 1),
                ':' => {
                    if let Some(':') = state.peek_next_n(1) {
                        (CppTokenType::Scope, 2)
                    }
                    else {
                        (CppTokenType::Colon, 1)
                    }
                }
                '.' => (CppTokenType::Dot, 1),
                _ => return false,
            };

            state.advance(advance_count);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a delimiter.
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CppTokenType::LeftParen,
                ')' => CppTokenType::RightParen,
                '[' => CppTokenType::LeftBracket,
                ']' => CppTokenType::RightBracket,
                '{' => CppTokenType::LeftBrace,
                '}' => CppTokenType::RightBrace,
                ',' => CppTokenType::Comma,
                ';' => CppTokenType::Semicolon,
                _ => return false,
            };

            state.advance(1);
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a preprocessor directive.
    fn lex_preprocessor<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            // Read until end of line
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            state.add_token(CppTokenType::Preprocessor, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<CppLanguage> for CppLexer<'config> {
    /// Tokenizes the input source text.
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<CppLanguage>) -> LexOutput<CppLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> CppLexer<'config> {
    /// Main lexer loop that tokenizes the source text.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            // Try various lexing rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string(state) {
                continue;
            }

            if self.lex_character(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword_or_identifier(state) {
                continue;
            }

            if self.lex_preprocessor(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果都不匹配，跳过当前字符并记录错误
            let start = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(CppTokenType::Error, start, state.get_position())
            }
        }
        Ok(())
    }
}
