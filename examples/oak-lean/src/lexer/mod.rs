#![doc = include_str!("readme.md")]
pub mod token_type;

/// Lean language lexer
///
/// Provides lexical analysis for the Lean language, converting source text into a token stream
use crate::{language::LeanLanguage, lexer::token_type::LeanTokenType};
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, LeanLanguage>;

/// Lean lexer
#[derive(Debug, Clone)]
pub struct LeanLexer<'config> {
    config: &'config LeanLanguage,
}

impl<'config> Lexer<LeanLanguage> for LeanLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<LeanLanguage>) -> LexOutput<LeanLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> LeanLexer<'config> {
    /// Creates a new Lean lexer
    pub fn new(config: &'config LeanLanguage) -> Self {
        Self { config }
    }

    /// Skips whitespace
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' { state.advance(ch.len_utf8()) } else { break }
        }

        if state.get_position() > start_pos {
            state.add_token(LeanTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(LeanTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1)
            }
            state.add_token(LeanTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // Single-line comment
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
                state.add_token(LeanTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack, this is a minus operator
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // Block comment start
                state.advance(1);
                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('-') = state.peek() {
                            state.advance(1);
                            depth += 1
                        }
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8())
                    }
                    else {
                        break;
                    }
                }
                state.add_token(LeanTokenType::Comment, start_pos, state.get_position());
                true
            }
            else {
                // Backtrack, this is a division operator
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// Handles string literals
    fn lex_string<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // String cannot span lines
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(LeanTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles character literals
    fn lex_char<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8())
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(LeanTokenType::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles number literals.
    fn lex_number<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // Scan digits
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                }

                // Check decimal point
                if let Some('.') = state.peek() {
                    state.advance(1); // Skip decimal point
                    if let Some(next_char) = state.peek() {
                        if next_char.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                            }
                        }
                    }
                }

                // Check exponent part
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1)
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() { state.advance(ch.len_utf8()) } else { break }
                        }
                    }
                }

                state.add_token(LeanTokenType::IntegerLiteral, start_pos, state.get_position());
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

    /// Handles identifiers and keywords.
    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '\'' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in(oak_core::Range { start: start_pos, end: state.get_position() });
                let kind = match text.as_ref() {
                    "axiom" => LeanTokenType::Axiom,
                    "constant" => LeanTokenType::Constant,
                    "def" => LeanTokenType::Def,
                    "example" => LeanTokenType::Example,
                    "inductive" => LeanTokenType::Inductive,
                    "lemma" => LeanTokenType::Lemma,
                    "namespace" => LeanTokenType::Namespace,
                    "open" => LeanTokenType::Open,
                    "private" => LeanTokenType::Private,
                    "protected" => LeanTokenType::Protected,
                    "section" => LeanTokenType::Section,
                    "structure" => LeanTokenType::Structure,
                    "theorem" => LeanTokenType::Theorem,
                    "universe" => LeanTokenType::Universe,
                    "variable" => LeanTokenType::Variable,
                    "variables" => LeanTokenType::Variables,
                    "end" => LeanTokenType::End,
                    "import" => LeanTokenType::Import,
                    "export" => LeanTokenType::Export,
                    "prelude" => LeanTokenType::Prelude,
                    "noncomputable" => LeanTokenType::Noncomputable,
                    "partial" => LeanTokenType::Partial,
                    "unsafe" => LeanTokenType::Unsafe,
                    "mutual" => LeanTokenType::Mutual,
                    "where" => LeanTokenType::Where,
                    "have" => LeanTokenType::Have,
                    "show" => LeanTokenType::Show,
                    "suffices" => LeanTokenType::Suffices,
                    "let" => LeanTokenType::Let,
                    "in" => LeanTokenType::In,
                    "if" => LeanTokenType::If,
                    "then" => LeanTokenType::Then,
                    "else" => LeanTokenType::Else,
                    "match" => LeanTokenType::Match,
                    "with" => LeanTokenType::With,
                    "fun" => LeanTokenType::Fun,
                    "do" => LeanTokenType::Do,
                    "for" => LeanTokenType::For,
                    "while" => LeanTokenType::While,
                    "break" => LeanTokenType::Break,
                    "continue" => LeanTokenType::Continue,
                    "return" => LeanTokenType::Return,
                    "try" => LeanTokenType::Try,
                    "catch" => LeanTokenType::Catch,
                    "finally" => LeanTokenType::Finally,
                    "throw" => LeanTokenType::Throw,
                    _ => LeanTokenType::Identifier,
                };

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

    /// Handles operators and delimiters.
    fn lex_operator_or_delimiter<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '(' => LeanTokenType::LeftParen,
                ')' => LeanTokenType::RightParen,
                '{' => LeanTokenType::LeftBrace,
                '}' => LeanTokenType::RightBrace,
                '[' => LeanTokenType::LeftBracket,
                ']' => LeanTokenType::RightBracket,
                ',' => LeanTokenType::Comma,
                ';' => LeanTokenType::Semicolon,
                '+' => LeanTokenType::Plus,
                '*' => LeanTokenType::Star,
                '/' => LeanTokenType::Slash,
                '%' => LeanTokenType::Percent,
                '^' => LeanTokenType::Caret,
                '#' => LeanTokenType::Hash,
                '&' => LeanTokenType::Ampersand,
                '|' => LeanTokenType::Pipe,
                '~' => LeanTokenType::Tilde,
                '!' => LeanTokenType::Bang,
                '?' => LeanTokenType::Question,
                '@' => LeanTokenType::At,
                '$' => LeanTokenType::Dollar,
                '<' => LeanTokenType::Lt,
                '>' => LeanTokenType::Gt,
                '=' => LeanTokenType::Eq,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) || self.lex_newline(state) || self.lex_comment(state) || self.lex_string(state) || self.lex_char(state) || self.lex_number(state) || self.lex_identifier_or_keyword(state) || self.lex_operator_or_delimiter(state) {
                continue;
            }

            // If no rules match, skip current character and mark as error.
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(LeanTokenType::Error, start_pos, state.get_position());
            }
        }

        Ok(())
    }
}
