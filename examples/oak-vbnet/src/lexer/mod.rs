#![doc = include_str!("readme.md")]

use crate::language::VbNetLanguage;

/// Token types and definitions for the VB.NET lexer.
pub mod token_type;

use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};
pub use token_type::VbNetTokenType;

pub(crate) type State<'a, S> = LexerState<'a, S, VbNetLanguage>;

/// VB.NET lexer
pub struct VbNetLexer<'config> {
    config: &'config VbNetLanguage,
}

impl<'config> VbNetLexer<'config> {
    /// Creates a new VB.NET lexer
    pub fn new(config: &'config VbNetLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace characters (spaces and tabs).
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VbNetTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline character (LF or CRLF).
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VbNetTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VbNetTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a comment (single-line `'` or multi-line `''' ... '''`).
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            // Check if it's a block comment (three single quotes)
            if let Some('\'') = state.peek() {
                state.advance(1);
                if let Some('\'') = state.peek() {
                    state.advance(1);
                    // Parse until closing three single quotes
                    while state.not_at_end() {
                        if let Some('\'') = state.peek() {
                            state.advance(1);
                            if let Some('\'') = state.peek() {
                                state.advance(1);
                                if let Some('\'') = state.peek() {
                                    state.advance(1);
                                    break;
                                }
                            }
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    state.add_token(VbNetTokenType::BlockComment, start_pos, state.get_position());
                    return true;
                }
                else {
                    // Backtrack, not a block comment
                    state.set_position(start_pos);
                    return false;
                }
            }
            else {
                // It's a line comment
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(VbNetTokenType::LineComment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes a string literal (`"..."`).
    /// Handles basic escape sequences.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    // Check for double quote escape
                    state.advance(1);
                    if let Some('"') = state.peek() {
                        state.advance(1);
                        continue;
                    }
                    break;
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(VbNetTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a character literal (`'...'`).
    fn lex_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch != '\'' {
                    state.advance(ch.len_utf8());
                    // Check for escape sequence
                    if let Some('\\') = state.peek() {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1)
                        }
                    }
                    // Check for closing single quote
                    if let Some('\'') = state.peek() {
                        state.advance(1);
                        state.add_token(VbNetTokenType::CharLiteral, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            // If we get here, it's not a valid char literal, backtrack
            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    /// Lexes a date literal (`#...#`).
    fn lex_date<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '#' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(VbNetTokenType::DateLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a number literal.
    ///
    /// Supports:
    /// - Decimal integers (`123`)
    /// - Floating-point numbers (`123.45`, `1.2e3`)
    /// - Underscore separators (`1_000_000`)
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_numeric() {
                state.advance(ch.len_utf8());

                let mut has_dot = false;
                let mut has_e = false;

                while let Some(ch) = state.peek() {
                    if ch.is_numeric() {
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else if (ch == 'e' || ch == 'E') && !has_e {
                        has_e = true;
                        state.advance(1);
                        // Check for optional sign after e
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }
                    }
                    else if ch == '_' {
                        // Underscore separator
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let token_type = if has_dot || has_e { VbNetTokenType::FloatLiteral } else { VbNetTokenType::IntegerLiteral };

                state.add_token(token_type, start_pos, state.get_position());
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
    ///
    /// Identifiers can start with a letter or underscore.
    /// Subsequent characters can be letters, digits, or underscores.
    ///
    /// Keywords are matched against the standard VB.NET keyword list. If a match is found,
    /// the specific keyword token is returned; otherwise, it is treated as an identifier.
    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_type = match text.as_ref() {
                    "Namespace" => VbNetTokenType::Namespace,
                    "Imports" => VbNetTokenType::Imports,
                    "Class" => VbNetTokenType::Class,
                    "Interface" => VbNetTokenType::Interface,
                    "Structure" => VbNetTokenType::Structure,
                    "Enum" => VbNetTokenType::Enum,
                    "Module" => VbNetTokenType::Module,
                    "Delegate" => VbNetTokenType::Delegate,
                    "Event" => VbNetTokenType::Event,
                    "Function" => VbNetTokenType::Function,
                    "Sub" => VbNetTokenType::Sub,
                    "Property" => VbNetTokenType::Property,
                    "Dim" => VbNetTokenType::Dim,
                    "Const" => VbNetTokenType::Const,
                    "As" => VbNetTokenType::As,
                    "In" => VbNetTokenType::In,
                    "If" => VbNetTokenType::If,
                    "Then" => VbNetTokenType::Then,
                    "Else" => VbNetTokenType::Else,
                    "ElseIf" => VbNetTokenType::ElseIf,
                    "End" => VbNetTokenType::End,
                    "For" => VbNetTokenType::For,
                    "Each" => VbNetTokenType::Each,
                    "To" => VbNetTokenType::To,
                    "Step" => VbNetTokenType::Step,
                    "While" => VbNetTokenType::While,
                    "Do" => VbNetTokenType::Do,
                    "Loop" => VbNetTokenType::Loop,
                    "Until" => VbNetTokenType::Until,
                    "Select" => VbNetTokenType::Select,
                    "Case" => VbNetTokenType::Case,
                    "Default" => VbNetTokenType::Default,
                    "With" => VbNetTokenType::With,
                    "Try" => VbNetTokenType::Try,
                    "Catch" => VbNetTokenType::Catch,
                    "Finally" => VbNetTokenType::Finally,
                    "Throw" => VbNetTokenType::Throw,
                    "Exit" => VbNetTokenType::Exit,
                    "Continue" => VbNetTokenType::Continue,
                    "Return" => VbNetTokenType::Return,
                    "Me" => VbNetTokenType::Me,
                    "MyBase" => VbNetTokenType::MyBase,
                    "MyClass" => VbNetTokenType::MyClass,
                    "New" => VbNetTokenType::New,
                    "Of" => VbNetTokenType::Of,
                    "ByVal" => VbNetTokenType::ByVal,
                    "ByRef" => VbNetTokenType::ByRef,
                    "Optional" => VbNetTokenType::Optional,
                    "ParamArray" => VbNetTokenType::ParamArray,
                    "Public" => VbNetTokenType::Public,
                    "Private" => VbNetTokenType::Private,
                    "Protected" => VbNetTokenType::Protected,
                    "Friend" => VbNetTokenType::Friend,
                    "ProtectedFriend" => VbNetTokenType::ProtectedFriend,
                    "Shared" => VbNetTokenType::Shared,
                    "MustInherit" => VbNetTokenType::MustInherit,
                    "NotInheritable" => VbNetTokenType::NotInheritable,
                    "MustOverride" => VbNetTokenType::MustOverride,
                    "Overridable" => VbNetTokenType::Overridable,
                    "Overrides" => VbNetTokenType::Overrides,
                    "NotOverridable" => VbNetTokenType::NotOverridable,
                    "MustOverrideReadOnly" => VbNetTokenType::MustOverrideReadOnly,
                    "ReadOnly" => VbNetTokenType::ReadOnly,
                    "WriteOnly" => VbNetTokenType::WriteOnly,
                    "Static" => VbNetTokenType::Static,
                    "Partial" => VbNetTokenType::Partial,
                    "Async" => VbNetTokenType::Async,
                    "Await" => VbNetTokenType::Await,
                    "From" => VbNetTokenType::From,
                    "Where" => VbNetTokenType::Where,
                    "Order" => VbNetTokenType::Order,
                    "By" => VbNetTokenType::By,
                    "Group" => VbNetTokenType::Group,
                    "Join" => VbNetTokenType::Join,
                    "On" => VbNetTokenType::On,
                    "Into" => VbNetTokenType::Into,
                    "Let" => VbNetTokenType::Let,
                    "And" => VbNetTokenType::And,
                    "Or" => VbNetTokenType::Or,
                    "Not" => VbNetTokenType::Not,
                    "Xor" => VbNetTokenType::Xor,
                    "AndAlso" => VbNetTokenType::AndAlso,
                    "OrElse" => VbNetTokenType::OrElse,
                    "Is" => VbNetTokenType::Is,
                    "IsNot" => VbNetTokenType::IsNot,
                    "Like" => VbNetTokenType::Like,
                    "TypeOf" => VbNetTokenType::TypeOf,
                    "True" => VbNetTokenType::BooleanLiteral,
                    "False" => VbNetTokenType::BooleanLiteral,
                    "Nothing" => VbNetTokenType::NothingLiteral,
                    _ => VbNetTokenType::Identifier,
                };

                state.add_token(token_type, start_pos, state.get_position());
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
    ///
    /// Handles single-character and multi-character operators.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_type = match ch {
                '+' => {
                    state.advance(1);
                    VbNetTokenType::Plus
                }
                '-' => {
                    state.advance(1);
                    VbNetTokenType::Minus
                }
                '*' => {
                    state.advance(1);
                    VbNetTokenType::Star
                }
                '/' => {
                    state.advance(1);
                    VbNetTokenType::Slash
                }
                '\\' => {
                    state.advance(1);
                    VbNetTokenType::Backslash
                }
                '%' => {
                    state.advance(1);
                    VbNetTokenType::Percent
                }
                '^' => {
                    state.advance(1);
                    VbNetTokenType::Caret
                }
                '=' => {
                    state.advance(1);
                    VbNetTokenType::Equal
                }
                '<' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        VbNetTokenType::NotEqual
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        VbNetTokenType::LessEqual
                    }
                    else {
                        VbNetTokenType::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        VbNetTokenType::GreaterEqual
                    }
                    else {
                        VbNetTokenType::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    VbNetTokenType::Ampersand
                }
                '!' => {
                    state.advance(1);
                    VbNetTokenType::Exclamation
                }
                _ => return false,
            };

            state.add_token(token_type, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a delimiter.
    ///
    /// Handles structural characters such as:
    /// - Parentheses: `(`, `)`
    /// - Brackets: `[`, `]`
    /// - Braces: `{`, `}`
    /// - Punctuation: `;`, `,`, `.`, `:`
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_type = match ch {
                '(' => VbNetTokenType::LeftParen,
                ')' => VbNetTokenType::RightParen,
                '[' => VbNetTokenType::LeftBracket,
                ']' => VbNetTokenType::RightBracket,
                '{' => VbNetTokenType::LeftBrace,
                '}' => VbNetTokenType::RightBrace,
                ';' => VbNetTokenType::Semicolon,
                ',' => VbNetTokenType::Comma,
                '.' => VbNetTokenType::Dot,
                ':' => VbNetTokenType::Colon,
                '#' => VbNetTokenType::Hash,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_type, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Runs the lexer on the input state.
    ///
    /// This method performs the main lexing loop, attempting to match various
    /// token types (whitespace, comments, literals, keywords, etc.) until the
    /// end of the input is reached.
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            if self.lex_char(state) {
                continue;
            }

            if self.lex_date(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_keyword_or_identifier(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // If no pattern matches, handle the error character and advance
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VbNetTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }
}

impl<'config> Lexer<VbNetLanguage> for VbNetLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], mut cache: &'a mut impl LexerCache<VbNetLanguage>) -> LexOutput<VbNetLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, &mut cache)
    }
}
