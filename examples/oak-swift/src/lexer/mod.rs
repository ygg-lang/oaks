#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::language::SwiftLanguage;
pub use crate::lexer::token_type::SwiftTokenType;
use oak_core::{Lexer, LexerCache, LexerState, OakError, TextEdit, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, SwiftLanguage>;

#[derive(Clone, Debug)]
pub struct SwiftLexer<'config> {
    config: &'config SwiftLanguage,
}

impl<'config> Lexer<SwiftLanguage> for SwiftLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<SwiftLanguage>) -> LexOutput<SwiftLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SwiftLexer<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { config }
    }

    /// Skip whitespace
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
            state.add_token(SwiftTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SwiftTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SwiftTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments
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
                    state.advance(ch.len_utf8());
                }
                state.add_token(SwiftTokenType::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek_next_n(1) {
                // Multi-line comment
                state.advance(2);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '/'
                        && let Some('*') = state.peek_next_n(1)
                    {
                        state.advance(2);
                        depth += 1;
                    }
                    else if ch == '*'
                        && let Some('/') = state.peek_next_n(1)
                    {
                        state.advance(2);
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(SwiftTokenType::Comment, start_pos, state.get_position());
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

    /// Handles identifiers or keywords
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Handle backtick identifier `identifier`
        let is_escaped = if let Some('`') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // If it's an escaped identifier, need to match the closing backtick
                if is_escaped {
                    if let Some('`') = state.peek() {
                        state.advance(1);
                    }
                    state.add_token(SwiftTokenType::Identifier, start_pos, state.get_position());
                    return true;
                }

                // Check if it's a keyword
                let text = state.get_text_in(core::range::Range { start: start_pos, end: state.get_position() });

                let token_kind = match text.as_ref() {
                    "class" => SwiftTokenType::Class,
                    "struct" => SwiftTokenType::Struct,
                    "enum" => SwiftTokenType::Enum,
                    "protocol" => SwiftTokenType::Protocol,
                    "extension" => SwiftTokenType::Extension,
                    "func" => SwiftTokenType::Func,
                    "var" => SwiftTokenType::Var,
                    "let" => SwiftTokenType::Let,
                    "init" => SwiftTokenType::Init,
                    "deinit" => SwiftTokenType::Deinit,
                    "subscript" => SwiftTokenType::Subscript,
                    "typealias" => SwiftTokenType::Typealias,
                    "import" => SwiftTokenType::Import,
                    "if" => SwiftTokenType::If,
                    "else" => SwiftTokenType::Else,
                    "switch" => SwiftTokenType::Switch,
                    "case" => SwiftTokenType::Case,
                    "default" => SwiftTokenType::Default,
                    "for" => SwiftTokenType::For,
                    "while" => SwiftTokenType::While,
                    "repeat" => SwiftTokenType::Repeat,
                    "do" => SwiftTokenType::Do,
                    "break" => SwiftTokenType::Break,
                    "continue" => SwiftTokenType::Continue,
                    "fallthrough" => SwiftTokenType::Fallthrough,
                    "return" => SwiftTokenType::Return,
                    "throw" => SwiftTokenType::Throw,
                    "try" => SwiftTokenType::Try,
                    "catch" => SwiftTokenType::Catch,
                    "finally" => SwiftTokenType::Finally,
                    "guard" => SwiftTokenType::Guard,
                    "defer" => SwiftTokenType::Defer,
                    "public" => SwiftTokenType::Public,
                    "private" => SwiftTokenType::Private,
                    "internal" => SwiftTokenType::Internal,
                    "fileprivate" => SwiftTokenType::Fileprivate,
                    "open" => SwiftTokenType::Open,
                    "static" => SwiftTokenType::Static,
                    "final" => SwiftTokenType::Final,
                    "override" => SwiftTokenType::Override,
                    "mutating" => SwiftTokenType::Mutating,
                    "nonmutating" => SwiftTokenType::Nonmutating,
                    "lazy" => SwiftTokenType::Lazy,
                    "weak" => SwiftTokenType::Weak,
                    "unowned" => SwiftTokenType::Unowned,
                    "optional" => SwiftTokenType::Optional,
                    "required" => SwiftTokenType::Required,
                    "convenience" => SwiftTokenType::Convenience,
                    "dynamic" => SwiftTokenType::Dynamic,
                    "infix" => SwiftTokenType::Infix,
                    "prefix" => SwiftTokenType::Prefix,
                    "postfix" => SwiftTokenType::Postfix,
                    "Any" => SwiftTokenType::Any,
                    "AnyObject" => SwiftTokenType::AnyObject,
                    "Self" => SwiftTokenType::Self_,
                    "Type" => SwiftTokenType::Type,
                    "Protocol" => SwiftTokenType::Protocol_,
                    "true" => SwiftTokenType::True,
                    "false" => SwiftTokenType::False,
                    "nil" => SwiftTokenType::Nil,
                    "as" => SwiftTokenType::As,
                    "is" => SwiftTokenType::Is,
                    "in" => SwiftTokenType::In,
                    "where" => SwiftTokenType::Where,
                    "associatedtype" => SwiftTokenType::Associatedtype,
                    "operator" => SwiftTokenType::Operator,
                    "precedencegroup" => SwiftTokenType::Precedencegroup,
                    "indirect" => SwiftTokenType::Indirect,
                    "rethrows" => SwiftTokenType::Rethrows,
                    "throws" => SwiftTokenType::Throws,
                    "inout" => SwiftTokenType::Inout,
                    _ => SwiftTokenType::Identifier,
                };
                state.add_token(token_kind, start_pos, state.get_position());
                true
            }
            else {
                if is_escaped {
                    // Backtrack backtick
                    state.set_position(start_pos);
                }
                false
            }
        }
        else {
            if is_escaped {
                // Backtrack backtick
                state.set_position(start_pos);
            }
            false
        }
    }

    /// Handles number literals
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // Handle binary, octal, hexadecimal
                if ch == '0' {
                    if let Some('b') | Some('B') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch == '0' || ch == '1' || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if let Some('o') | Some('O') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() && ch < '8' || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // Regular decimal numbers
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                else {
                    // Decimal number
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // Handle decimals
                if let Some('.') = state.peek() {
                    // If followed immediately by another dot, it's part of a range operator, not a decimal point
                    if let Some(next) = state.peek_next_n(1) {
                        if next != '.' {
                            state.advance(1);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                    else {
                        // No characters left, 1. can also be a floating point
                        state.advance(1);
                    }
                }

                // Handle exponents
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(SwiftTokenType::NumberLiteral, start_pos, state.get_position());
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

    /// Handles string literals
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Handles multi-line strings """..."""
        if let Some('"') = state.peek() {
            if let Some('"') = state.peek_next_n(1) {
                if let Some('"') = state.peek_next_n(2) {
                    // Multi-line string
                    state.advance(3);
                    while let Some(ch) = state.peek() {
                        if ch == '"' {
                            if let Some('"') = state.peek_next_n(1) {
                                if let Some('"') = state.peek_next_n(2) {
                                    state.advance(3);
                                    break;
                                }
                            }
                        }
                        state.advance(ch.len_utf8());
                    }
                    state.add_token(SwiftTokenType::StringLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // Normal string
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // Normal strings cannot span multiple lines
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(SwiftTokenType::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles operators
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::PlusAssign
                    }
                    else {
                        SwiftTokenType::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftTokenType::MinusAssign
                        }
                        Some('>') => {
                            state.advance(1);
                            SwiftTokenType::Arrow
                        }
                        _ => SwiftTokenType::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::StarAssign
                    }
                    else {
                        SwiftTokenType::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::SlashAssign
                    }
                    else {
                        SwiftTokenType::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::PercentAssign
                    }
                    else {
                        SwiftTokenType::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::Equal
                    }
                    else {
                        SwiftTokenType::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::NotEqual
                    }
                    else {
                        SwiftTokenType::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftTokenType::LessEqual
                        }
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                SwiftTokenType::LeftShiftAssign
                            }
                            else {
                                SwiftTokenType::LeftShift
                            }
                        }
                        _ => SwiftTokenType::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            SwiftTokenType::GreaterEqual
                        }
                        Some('>') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                SwiftTokenType::RightShiftAssign
                            }
                            else {
                                SwiftTokenType::RightShift
                            }
                        }
                        _ => SwiftTokenType::Greater,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            SwiftTokenType::LogicalAnd
                        }
                        Some('=') => {
                            state.advance(1);
                            SwiftTokenType::AndAssign
                        }
                        _ => SwiftTokenType::BitAnd,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            SwiftTokenType::LogicalOr
                        }
                        Some('=') => {
                            state.advance(1);
                            SwiftTokenType::OrAssign
                        }
                        _ => SwiftTokenType::BitOr,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::XorAssign
                    }
                    else {
                        SwiftTokenType::BitXor
                    }
                }
                '~' => {
                    state.advance(1);
                    SwiftTokenType::BitNot
                }
                '?' => {
                    state.advance(1);
                    if let Some('?') = state.peek() {
                        state.advance(1);
                        SwiftTokenType::QuestionQuestion
                    }
                    else {
                        SwiftTokenType::Question
                    }
                }
                '.' => {
                    state.advance(1);
                    match state.peek() {
                        Some('.') => {
                            state.advance(1);
                            match state.peek() {
                                Some('.') => {
                                    state.advance(1);
                                    SwiftTokenType::ClosedRange
                                }
                                Some('<') => {
                                    state.advance(1);
                                    SwiftTokenType::Range
                                }
                                _ => SwiftTokenType::Dot, // Or error
                            }
                        }
                        _ => SwiftTokenType::Dot,
                    }
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

    /// Handles delimiters
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => SwiftTokenType::LeftParen,
                ')' => SwiftTokenType::RightParen,
                '[' => SwiftTokenType::LeftBracket,
                ']' => SwiftTokenType::RightBracket,
                '{' => SwiftTokenType::LeftBrace,
                '}' => SwiftTokenType::RightBrace,
                ',' => SwiftTokenType::Comma,
                ';' => SwiftTokenType::Semicolon,
                ':' => SwiftTokenType::Colon,
                '@' => SwiftTokenType::At,
                '#' => SwiftTokenType::Hash,
                '$' => SwiftTokenType::Dollar,
                '_' => SwiftTokenType::Underscore,
                '\\' => SwiftTokenType::Backslash,
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
}

impl<'config> SwiftLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            // Try various lexical rules
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // If no rules match, skip current character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SwiftTokenType::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }
}
