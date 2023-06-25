#![doc = include_str!("readme.md")]
use crate::{language::ZigLanguage, lexer::token_type::ZigTokenType};
pub mod token_type;
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Source,
    lexer::{LexOutput, WhitespaceConfig},
};
use std::sync::LazyLock;

pub(crate) type State<'a, S> = LexerState<'a, S, ZigLanguage>;

static ZIG_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

/// Lexer for the Zig language.
#[derive(Clone)]
pub struct ZigLexer<'config> {
    config: &'config ZigLanguage,
}

impl<'config> Lexer<ZigLanguage> for ZigLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::source::TextEdit], cache: &'a mut impl LexerCache<ZigLanguage>) -> LexOutput<ZigLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ZigLexer<'config> {
    /// Creates a new Zig lexer with the given configuration.
    pub fn new(config: &'config ZigLanguage) -> Self {
        Self { config }
    }

    /// Main lexical analysis loop
    fn run<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_builtin(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // If no rules match, advance one character and mark as error
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ZigTokenType::Error, start_pos, state.get_position())
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Skips whitespace characters
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        ZIG_WHITESPACE.scan(state, ZigTokenType::Whitespace)
    }

    /// Skips comments
    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<'_, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);

            // Check if it's a doc comment ///
            let is_doc_comment = if state.peek() == Some('/') {
                state.advance(1);
                true
            }
            else {
                false
            };

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8())
            }

            let kind = if is_doc_comment { ZigTokenType::DocComment } else { ZigTokenType::Comment };
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    /// Parses string literals
    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Multiline string: \\...
        if state.rest().starts_with("\\\\") {
            state.advance(2);

            // Skip to end of line
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8())
            }

            // Read multiline string content
            while state.not_at_end() {
                let _line_start = state.get_position();

                // Check if it's a continuation line
                if !state.rest().starts_with("\\\\") {
                    break;
                }

                state.advance(2);

                // Read to end of line
                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        state.advance(1);
                        break;
                    }
                    state.advance(ch.len_utf8())
                }
            }

            state.add_token(ZigTokenType::StringLiteral, start, state.get_position());
            return true;
        }

        // Normal string: "..."
        if state.current() == Some('"') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(next) = state.peek() {
                        state.advance(next.len_utf8())
                    }
                    continue;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(ZigTokenType::StringLiteral, start, state.get_position());
            return true;
        }

        false
    }

    /// Parses character literals
    fn lex_char_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('\'') {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\'' {
                    state.advance(1);
                    break;
                }
                if ch == '\\' {
                    state.advance(1);
                    if let Some(next) = state.peek() {
                        state.advance(next.len_utf8())
                    }
                    continue;
                }
                state.advance(ch.len_utf8())
            }
            state.add_token(ZigTokenType::CharLiteral, start, state.get_position());
            return true;
        }
        false
    }

    /// Parses number literals
    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = state.current();
        let mut is_float = false;

        if let Some(ch) = ch {
            if ch.is_ascii_digit() {
                state.advance(1);
                // Handle hexadecimal, binary, octal
                if ch == '0' {
                    if let Some(next) = state.peek() {
                        match next {
                            'x' | 'X' => {
                                state.advance(1);
                                state.take_while(|c| c.is_ascii_hexdigit() || c == '_');
                            }
                            'b' | 'B' => {
                                state.advance(1);
                                state.take_while(|c| c == '0' || c == '1' || c == '_');
                            }
                            'o' | 'O' => {
                                state.advance(1);
                                state.take_while(|c| ('0'..='7').contains(&c) || c == '_');
                            }
                            _ => {
                                state.take_while(|c| c.is_ascii_digit() || c == '_');
                            }
                        }
                    }
                }
                else {
                    state.take_while(|c| c.is_ascii_digit() || c == '_');
                }

                // Handle decimal point
                if state.current() == Some('.') {
                    if let Some(next) = state.peek() {
                        if next.is_ascii_digit() {
                            is_float = true;
                            state.advance(1);
                            state.take_while(|c| c.is_ascii_digit() || c == '_');
                        }
                    }
                }

                // Handle exponent
                if let Some(c) = state.current() {
                    if c == 'e' || c == 'E' || c == 'p' || c == 'P' {
                        is_float = true;
                        state.advance(1);
                        if let Some(next) = state.peek() {
                            if next == '+' || next == '-' {
                                state.advance(1);
                            }
                        }
                        state.take_while(|c| c.is_ascii_digit() || c == '_');
                    }
                }

                let kind = if is_float { ZigTokenType::FloatLiteral } else { ZigTokenType::IntegerLiteral };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Parses identifiers or keywords
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());
                state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');

                let end = state.get_position();
                let text = state.get_text_in((start..end).into());
                let kind = self.get_keyword_or_identifier(&text);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Gets keyword or identifier type
    fn get_keyword_or_identifier(&self, text: &str) -> ZigTokenType {
        match text {
            // Basic structures
            "const" => ZigTokenType::Const,
            "var" => ZigTokenType::Var,
            "fn" => ZigTokenType::Fn,
            "struct" => ZigTokenType::Struct,
            "union" => ZigTokenType::Union,
            "enum" => ZigTokenType::Enum,
            "opaque" => ZigTokenType::Opaque,
            "type" => ZigTokenType::Type,
            "comptime" => ZigTokenType::Comptime,
            "inline" => ZigTokenType::Inline,
            "noinline" => ZigTokenType::NoInline,
            "pub" => ZigTokenType::Pub,
            "export" => ZigTokenType::Export,
            "extern" => ZigTokenType::Extern,
            "packed" => ZigTokenType::Packed,
            "align" => ZigTokenType::Align,
            "callconv" => ZigTokenType::CallConv,
            "linksection" => ZigTokenType::LinkSection,

            // Control flow
            "if" => ZigTokenType::If,
            "else" => ZigTokenType::Else,
            "switch" => ZigTokenType::Switch,
            "while" => ZigTokenType::While,
            "for" => ZigTokenType::For,
            "break" => ZigTokenType::Break,
            "continue" => ZigTokenType::Continue,
            "return" => ZigTokenType::Return,
            "defer" => ZigTokenType::Defer,
            "errdefer" => ZigTokenType::ErrDefer,
            "unreachable" => ZigTokenType::Unreachable,
            "noreturn" => ZigTokenType::NoReturn,

            // Error handling
            "try" => ZigTokenType::TryKeyword,
            "catch" => ZigTokenType::CatchKeyword,
            "orelse" => ZigTokenType::OrElse,
            "error" => ZigTokenType::ErrorKeyword,

            // Test and async
            "test" => ZigTokenType::Test,
            "async" => ZigTokenType::Async,
            "await" => ZigTokenType::AwaitKeyword,
            "suspend" => ZigTokenType::Suspend,
            "resume" => ZigTokenType::Resume,
            "cancel" => ZigTokenType::Cancel,

            // Memory management
            "undefined" => ZigTokenType::Undefined,
            "null" => ZigTokenType::Null,
            "volatile" => ZigTokenType::Volatile,
            "allowzero" => ZigTokenType::AllowZero,
            "noalias" => ZigTokenType::NoAlias,

            // Logical operations
            "and" => ZigTokenType::And,
            "or" => ZigTokenType::Or,

            // Others
            "anyframe" => ZigTokenType::AnyFrame,
            "anytype" => ZigTokenType::AnyType,
            "threadlocal" => ZigTokenType::ThreadLocal,

            // Basic types
            "bool" => ZigTokenType::Bool,
            "i8" => ZigTokenType::I8,
            "i16" => ZigTokenType::I16,
            "i32" => ZigTokenType::I32,
            "i64" => ZigTokenType::I64,
            "i128" => ZigTokenType::I128,
            "isize" => ZigTokenType::Isize,
            "u8" => ZigTokenType::U8,
            "u16" => ZigTokenType::U16,
            "u32" => ZigTokenType::U32,
            "u64" => ZigTokenType::U64,
            "u128" => ZigTokenType::U128,
            "usize" => ZigTokenType::Usize,
            "f16" => ZigTokenType::F16,
            "f32" => ZigTokenType::F32,
            "f64" => ZigTokenType::F64,
            "f80" => ZigTokenType::F80,
            "f128" => ZigTokenType::F128,
            "c_short" => ZigTokenType::CShort,
            "c_ushort" => ZigTokenType::CUshort,
            "c_int" => ZigTokenType::CInt,
            "c_uint" => ZigTokenType::CUint,
            "c_long" => ZigTokenType::CLong,
            "c_ulong" => ZigTokenType::CUlong,
            "c_longlong" => ZigTokenType::CLongLong,
            "c_ulonglong" => ZigTokenType::CUlongLong,
            "c_longdouble" => ZigTokenType::CLongDouble,
            "c_void" => ZigTokenType::CVoid,
            "void" => ZigTokenType::Void,
            "comptime_int" => ZigTokenType::ComptimeInt,
            "comptime_float" => ZigTokenType::ComptimeFloat,

            // Boolean literals
            "true" | "false" => ZigTokenType::BooleanLiteral,

            _ => ZigTokenType::Identifier,
        }
    }

    /// Parses builtin identifiers (e.g., @import)
    fn lex_builtin<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.current() == Some('@') {
            state.advance(1);
            if let Some(ch) = state.current() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());
                    state.take_while(|c| c.is_ascii_alphanumeric() || c == '_');
                    state.add_token(ZigTokenType::BuiltinIdentifier, start, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// Parses operators
    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Try to match the longest operator
        let ops = [
            ("<<=", ZigTokenType::LessLessAssign),
            (">>=", ZigTokenType::GreaterGreaterAssign),
            ("...", ZigTokenType::DotDotDot),
            ("==", ZigTokenType::Equal),
            ("!=", ZigTokenType::NotEqual),
            ("<=", ZigTokenType::LessEqual),
            (">=", ZigTokenType::GreaterEqual),
            ("&&", ZigTokenType::AndAnd),
            ("||", ZigTokenType::OrOr),
            ("+=", ZigTokenType::PlusAssign),
            ("-=", ZigTokenType::MinusAssign),
            ("*=", ZigTokenType::StarAssign),
            ("/=", ZigTokenType::SlashAssign),
            ("%=", ZigTokenType::PercentAssign),
            ("&=", ZigTokenType::AmpersandAssign),
            ("|=", ZigTokenType::PipeAssign),
            ("^=", ZigTokenType::CaretAssign),
            ("++", ZigTokenType::PlusPlus),
            ("--", ZigTokenType::MinusMinus),
            ("**", ZigTokenType::StarStar),
            ("->", ZigTokenType::Arrow),
            ("=>", ZigTokenType::FatArrow),
            ("<<", ZigTokenType::LessLess),
            (">>", ZigTokenType::GreaterGreater),
            (".?", ZigTokenType::DotQuestion),
            (".*", ZigTokenType::DotStar),
        ];

        for (op, kind) in ops {
            if rest.starts_with(op) {
                state.advance(op.len());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Parses single-character tokens
    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => ZigTokenType::LeftParen,
                ')' => ZigTokenType::RightParen,
                '{' => ZigTokenType::LeftBrace,
                '}' => ZigTokenType::RightBrace,
                '[' => ZigTokenType::LeftBracket,
                ']' => ZigTokenType::RightBracket,
                ',' => ZigTokenType::Comma,
                '.' => ZigTokenType::Dot,
                ':' => ZigTokenType::Colon,
                ';' => ZigTokenType::Semicolon,
                '+' => ZigTokenType::Plus,
                '-' => ZigTokenType::Minus,
                '*' => ZigTokenType::Star,
                '/' => ZigTokenType::Slash,
                '%' => ZigTokenType::Percent,
                '&' => ZigTokenType::Ampersand,
                '|' => ZigTokenType::Pipe,
                '^' => ZigTokenType::Caret,
                '~' => ZigTokenType::Tilde,
                '!' => ZigTokenType::Exclamation,
                '?' => ZigTokenType::Question,
                '<' => ZigTokenType::Less,
                '>' => ZigTokenType::Greater,
                '=' => ZigTokenType::Assign,
                _ => return false,
            };
            state.advance(1);
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }
}
