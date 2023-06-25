#![doc = include_str!("readme.md")]
//! Lexer implementation for the C# language.

use crate::language::CSharpLanguage;

/// Token types and definitions for the C# lexer.
pub mod token_type;

use oak_core::{
    Lexer, LexerCache, LexerState,
    lexer::LexOutput,
    source::{Source, TextEdit},
};
pub use token_type::CSharpTokenType;

pub(crate) type State<'a, S> = LexerState<'a, S, CSharpLanguage>;

/// A lexer for the C# language.
pub struct CSharpLexer<'config> {
    config: &'config CSharpLanguage,
}

impl<'config> CSharpLexer<'config> {
    /// Creates a new C# lexer.
    pub fn new(config: &'config CSharpLanguage) -> Self {
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
            state.add_token(CSharpTokenType::Whitespace, start_pos, state.get_position());
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
            state.add_token(CSharpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(CSharpTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a comment (single-line `//` or multi-line `/* ... */`).
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // Single-line comment
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(CSharpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else if let Some('*') = state.peek() {
                // Multi-line comment
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(CSharpTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            else {
                // Backtrack, not a comment
                state.set_position(start_pos);
                return false;
            }
        }
        false
    }

    /// Lexes a string literal (`"..."`) or character literal (`'...'`).
    /// Handles basic escape sequences.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                    if let Some(_) = state.peek() {
                        state.advance(1)
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CSharpTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else if let Some('\'') = state.peek() {
            // Char literal
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1)
                    }
                }
                else {
                    state.advance(ch.len_utf8())
                }
            }
            state.add_token(CSharpTokenType::CharLiteral, start_pos, state.get_position());
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
    /// - Type suffixes (`f`, `d`, `m`, `l`, `ul`, etc.)
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                // Handle suffixes (f, d, m, l, ul, etc.)
                if let Some(ch) = state.peek() {
                    if ch.is_ascii_alphabetic() {
                        state.advance(ch.len_utf8());
                        if let Some(ch2) = state.peek() {
                            if ch2.is_ascii_alphabetic() {
                                state.advance(ch2.len_utf8())
                            }
                        }
                    }
                }

                state.add_token(CSharpTokenType::NumberLiteral, start_pos, state.get_position());
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
    /// Identifiers can start with a letter, underscore, or `@` (for verbatim identifiers).
    /// Subsequent characters can be letters, digits, or underscores.
    ///
    /// Keywords are matched against the standard C# keyword list. If a match is found,
    /// the specific keyword token is returned; otherwise, it is treated as an identifier.
    fn lex_keyword_or_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '@' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    // C# Keywords
                    "abstract" => CSharpTokenType::Abstract,
                    "as" => CSharpTokenType::As,
                    "async" => CSharpTokenType::AsyncKeyword,
                    "await" => CSharpTokenType::AwaitKeyword,
                    "base" => CSharpTokenType::Base,
                    "bool" => CSharpTokenType::Bool,
                    "break" => CSharpTokenType::Break,
                    "byte" => CSharpTokenType::Byte,
                    "case" => CSharpTokenType::Case,
                    "catch" => CSharpTokenType::Catch,
                    "char" => CSharpTokenType::Char,
                    "checked" => CSharpTokenType::Checked,
                    "class" => CSharpTokenType::Class,
                    "const" => CSharpTokenType::Const,
                    "continue" => CSharpTokenType::Continue,
                    "decimal" => CSharpTokenType::Decimal,
                    "default" => CSharpTokenType::Default,
                    "delegate" => CSharpTokenType::Delegate,
                    "do" => CSharpTokenType::Do,
                    "double" => CSharpTokenType::Double,
                    "else" => CSharpTokenType::Else,
                    "enum" => CSharpTokenType::Enum,
                    "event" => CSharpTokenType::Event,
                    "explicit" => CSharpTokenType::Explicit,
                    "extern" => CSharpTokenType::Extern,
                    "false" => CSharpTokenType::False,
                    "finally" => CSharpTokenType::Finally,
                    "fixed" => CSharpTokenType::Fixed,
                    "float" => CSharpTokenType::Float,
                    "for" => CSharpTokenType::For,
                    "foreach" => CSharpTokenType::Foreach,
                    "goto" => CSharpTokenType::Goto,
                    "if" => CSharpTokenType::If,
                    "implicit" => CSharpTokenType::Implicit,
                    "in" => CSharpTokenType::In,
                    "int" => CSharpTokenType::Int,
                    "interface" => CSharpTokenType::Interface,
                    "internal" => CSharpTokenType::Internal,
                    "is" => CSharpTokenType::Is,
                    "lock" => CSharpTokenType::Lock,
                    "long" => CSharpTokenType::Long,
                    "namespace" => CSharpTokenType::Namespace,
                    "new" => CSharpTokenType::New,
                    "null" => CSharpTokenType::Null,
                    "object" => CSharpTokenType::Object,
                    "operator" => CSharpTokenType::Operator,
                    "out" => CSharpTokenType::Out,
                    "override" => CSharpTokenType::Override,
                    "params" => CSharpTokenType::Params,
                    "private" => CSharpTokenType::Private,
                    "protected" => CSharpTokenType::Protected,
                    "public" => CSharpTokenType::Public,
                    "readonly" => CSharpTokenType::Readonly,
                    "record" => CSharpTokenType::Record,
                    "ref" => CSharpTokenType::Ref,
                    "return" => CSharpTokenType::Return,
                    "sbyte" => CSharpTokenType::Sbyte,
                    "sealed" => CSharpTokenType::Sealed,
                    "short" => CSharpTokenType::Short,
                    "sizeof" => CSharpTokenType::Sizeof,
                    "stackalloc" => CSharpTokenType::Stackalloc,
                    "static" => CSharpTokenType::Static,
                    "string" => CSharpTokenType::String,
                    "struct" => CSharpTokenType::Struct,
                    "switch" => CSharpTokenType::Switch,
                    "this" => CSharpTokenType::This,
                    "throw" => CSharpTokenType::Throw,
                    "true" => CSharpTokenType::True,
                    "try" => CSharpTokenType::Try,
                    "typeof" => CSharpTokenType::Typeof,
                    "uint" => CSharpTokenType::Uint,
                    "ulong" => CSharpTokenType::Ulong,
                    "unchecked" => CSharpTokenType::Unchecked,
                    "unsafe" => CSharpTokenType::Unsafe,
                    "ushort" => CSharpTokenType::Ushort,
                    "using" => CSharpTokenType::Using,
                    "virtual" => CSharpTokenType::Virtual,
                    "void" => CSharpTokenType::Void,
                    "volatile" => CSharpTokenType::Volatile,
                    "while" => CSharpTokenType::While,
                    _ => CSharpTokenType::Identifier,
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
    ///
    /// Handles single-character and multi-character operators, including:
    /// - Arithmetic: `+`, `-`, `*`, `/`, `%`
    /// - Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`
    /// - Increment/Decrement: `++`, `--`
    /// - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
    /// - Logical: `&&`, `||`, `!`
    /// - Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Increment
                    }
                    else {
                        CSharpTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Decrement
                    }
                    else {
                        CSharpTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::StarAssign
                    }
                    else {
                        CSharpTokenType::Star
                    }
                }
                '/' => {
                    // Comments are handled in lex_comment
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::SlashAssign
                    }
                    else {
                        CSharpTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::PercentAssign
                    }
                    else {
                        CSharpTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::Equal
                    }
                    else {
                        CSharpTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::NotEqual
                    }
                    else {
                        CSharpTokenType::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LeftShift
                    }
                    else {
                        CSharpTokenType::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::RightShift
                    }
                    else {
                        CSharpTokenType::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LogicalAnd
                    }
                    else {
                        CSharpTokenType::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        CSharpTokenType::LogicalOr
                    }
                    else {
                        CSharpTokenType::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    CSharpTokenType::Caret
                }
                '~' => {
                    state.advance(1);
                    CSharpTokenType::Tilde
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
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
    /// - Punctuation: `;`, `,`, `.`, `:`, `?`
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => CSharpTokenType::LeftParen,
                ')' => CSharpTokenType::RightParen,
                '[' => CSharpTokenType::LeftBracket,
                ']' => CSharpTokenType::RightBracket,
                '{' => CSharpTokenType::LeftBrace,
                '}' => CSharpTokenType::RightBrace,
                ';' => CSharpTokenType::Semicolon,
                ',' => CSharpTokenType::Comma,
                '.' => CSharpTokenType::Dot,
                ':' => CSharpTokenType::Colon,
                '?' => CSharpTokenType::Question,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
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
                state.add_token(CSharpTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }
        Ok(())
    }
}

impl<'config> Lexer<CSharpLanguage> for CSharpLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], mut cache: &'a mut impl LexerCache<CSharpLanguage>) -> LexOutput<CSharpLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, &mut cache)
    }
}
