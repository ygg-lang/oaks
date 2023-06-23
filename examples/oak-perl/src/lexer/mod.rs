#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::PerlLanguage, lexer::token_type::PerlTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'s, S> = LexerState<'s, S, PerlLanguage>;

static PERL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static PERL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false });

/// Lexer for the Perl language.
///
/// This lexer transforms a source string into a stream of [`PerlTokenType`] tokens.
#[derive(Clone, Debug)]
pub struct PerlLexer<'config> {
    _config: &'config PerlLanguage,
}

impl<'config> PerlLexer<'config> {
    /// Creates a new `PerlLexer` with the given language configuration.
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { _config: config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        PERL_WHITESPACE.scan(state, PerlTokenType::Whitespace)
    }

    /// Skips single-line comments.
    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        PERL_COMMENT.scan(state, PerlTokenType::Comment, PerlTokenType::Comment)
    }

    /// Lexes a string literal (single or double quoted).
    fn lex_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // Skip opening quote

                let mut escaped = false;
                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8())
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1)
                    }
                    else if ch == quote_char {
                        state.advance(1); // Skip closing quote
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // Strings cannot span lines unless escaped
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8())
                    }
                }

                state.add_token(PerlTokenType::StringLiteral, start_pos, state.get_position());
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

    /// Lexes a variable name (starting with $, @, or %).
    fn lex_variable<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '$' => {
                    state.advance(1);
                    // Read variable name
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                    }
                    state.add_token(PerlTokenType::Dollar, start_pos, state.get_position());
                    true
                }
                '@' => {
                    state.advance(1);
                    // Read array variable name
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                    }
                    state.add_token(PerlTokenType::At, start_pos, state.get_position());
                    true
                }
                '%' => {
                    state.advance(1);
                    // Read hash variable name
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' { state.advance(ch.len_utf8()) } else { break }
                    }
                    state.add_token(PerlTokenType::Percent_, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// Lexes an identifier or a keyword.
    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8())
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "if" => PerlTokenType::If,
                    "else" => PerlTokenType::Else,
                    "elsif" => PerlTokenType::Elsif,
                    "unless" => PerlTokenType::Unless,
                    "while" => PerlTokenType::While,
                    "until" => PerlTokenType::Until,
                    "for" => PerlTokenType::For,
                    "foreach" => PerlTokenType::Foreach,
                    "do" => PerlTokenType::Do,
                    "sub" => PerlTokenType::Sub,
                    "package" => PerlTokenType::Package,
                    "use" => PerlTokenType::Use,
                    "require" => PerlTokenType::Require,
                    "my" => PerlTokenType::My,
                    "our" => PerlTokenType::Our,
                    "local" => PerlTokenType::Local,
                    "return" => PerlTokenType::Return,
                    "last" => PerlTokenType::Last,
                    "next" => PerlTokenType::Next,
                    "redo" => PerlTokenType::Redo,
                    "die" => PerlTokenType::Die,
                    "warn" => PerlTokenType::Warn,
                    "eval" => PerlTokenType::Eval,
                    "print" => PerlTokenType::Print,
                    "printf" => PerlTokenType::Printf,
                    "chomp" => PerlTokenType::Chomp,
                    "chop" => PerlTokenType::Chop,
                    "split" => PerlTokenType::Split,
                    "join" => PerlTokenType::Join,
                    "push" => PerlTokenType::Push,
                    "pop" => PerlTokenType::Pop,
                    "shift" => PerlTokenType::Shift,
                    "unshift" => PerlTokenType::Unshift,
                    "keys" => PerlTokenType::Keys,
                    "values" => PerlTokenType::Values,
                    "each" => PerlTokenType::Each,
                    "exists" => PerlTokenType::Exists,
                    "delete" => PerlTokenType::Delete,
                    "defined" => PerlTokenType::Defined,
                    "undef" => PerlTokenType::Undef,
                    "ref" => PerlTokenType::Ref,
                    "bless" => PerlTokenType::Bless,
                    "new" => PerlTokenType::New,
                    "and" => PerlTokenType::And,
                    "or" => PerlTokenType::Or,
                    "not" => PerlTokenType::Not,
                    _ => PerlTokenType::Identifier,
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

    /// Lexes a number literal.
    fn lex_number<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();
                let mut has_dot = false;

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1)
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1)
                    }
                    else {
                        break;
                    }
                }

                let kind = PerlTokenType::NumberLiteral;

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

    /// Lexes operators and punctuation characters.
    fn lex_operators_and_punctuation<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Increment
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::PlusAssign
                    }
                    else {
                        PerlTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Decrement
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Arrow
                    }
                    else {
                        PerlTokenType::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Power
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::MultiplyAssign
                    }
                    else {
                        PerlTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::DivideAssign
                    }
                    else {
                        PerlTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::ModuloAssign
                    }
                    else {
                        PerlTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            PerlTokenType::FatArrow
                        }
                        else {
                            PerlTokenType::Equal
                        }
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Match
                    }
                    else {
                        PerlTokenType::Assign
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('<') = state.peek() {
                        state.advance(1);
                        PerlTokenType::LeftShift
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            PerlTokenType::Spaceship
                        }
                        else {
                            PerlTokenType::LessEqual
                        }
                    }
                    else {
                        PerlTokenType::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        PerlTokenType::RightShift
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::GreaterEqual
                    }
                    else {
                        PerlTokenType::GreaterThan
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PerlTokenType::NotEqual
                    }
                    else if let Some('~') = state.peek() {
                        state.advance(1);
                        PerlTokenType::NotMatch
                    }
                    else {
                        PerlTokenType::LogicalNot
                    }
                }
                '&' => {
                    state.advance(1);
                    PerlTokenType::BitwiseAnd
                }
                '|' => {
                    state.advance(1);
                    PerlTokenType::BitwiseOr
                }
                '^' => {
                    state.advance(1);
                    PerlTokenType::BitwiseXor
                }
                '~' => {
                    state.advance(1);
                    PerlTokenType::BitwiseNot
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PerlTokenType::Range
                    }
                    else {
                        PerlTokenType::Concat
                    }
                }
                '?' => {
                    state.advance(1);
                    PerlTokenType::Question
                }
                ':' => {
                    state.advance(1);
                    PerlTokenType::Colon
                }
                ';' => {
                    state.advance(1);
                    PerlTokenType::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PerlTokenType::Comma
                }
                '(' => {
                    state.advance(1);
                    PerlTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PerlTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    PerlTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PerlTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PerlTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PerlTokenType::RightBrace
                }
                '\n' => {
                    state.advance(1);
                    PerlTokenType::Newline
                }
                _ => {
                    state.advance(ch.len_utf8());
                    PerlTokenType::Error
                }
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PerlLanguage> for PerlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<PerlLanguage>) -> LexOutput<PerlLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PerlLexer<'config> {
    /// Runs the lexer on the given state until the end of the source is reached.
    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Skip whitespace characters
            if self.skip_whitespace(state) {
                continue;
            }

            // Handle comments
            if self.skip_comment(state) {
                continue;
            }

            // Handle strings
            if self.lex_string(state) {
                continue;
            }

            // Handle variables
            if self.lex_variable(state) {
                continue;
            }

            // Handle identifiers and keywords
            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            // Handle numbers
            if self.lex_number(state) {
                continue;
            }

            // Handle operators and punctuation
            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // If no pattern matches, create an error token
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PerlTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }
}
