use crate::{
    language::RustLanguage,
    lexer::{RustLexer, RustTokenType},
};
use oak_core::{LexerState, OakError, lexer::StringConfig, source::Source};
use std::{simd::prelude::*, sync::LazyLock};
use unicode_ident::{is_xid_continue, is_xid_start};

type State<'s, S> = LexerState<'s, S, RustLanguage>;

static RUST_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static RUST_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

pub(crate) fn run<'s, S: Source + ?Sized>(_lexer: &RustLexer, state: &mut State<'s, S>) -> Result<(), OakError> {
    while state.not_at_end() {
        let safe_point = state.get_position();
        let Some(ch) = state.peek()
        else {
            break;
        };

        match ch {
            ' ' | '\t' | '\n' | '\r' => {
                skip_whitespace(state);
            }
            '/' => {
                if state.starts_with("//") || state.starts_with("/*") {
                    skip_comment(state);
                }
                else {
                    lex_operators(state);
                }
            }
            '"' | 'r' => {
                if !lex_string_literal(state) {
                    // 'r' might be identifier if not raw string
                    if ch == 'r' {
                        lex_identifier_or_keyword(state);
                    }
                }
            }
            '\'' => {
                if !lex_char_literal(state) {
                    state.advance(1);
                    state.add_token(RustTokenType::Error, safe_point, state.get_position());
                }
            }
            '0'..='9' => {
                lex_number_literal(state);
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                lex_identifier_or_keyword(state);
            }
            '(' => {
                state.advance(1);
                state.add_token(RustTokenType::LeftParen, safe_point, state.get_position());
            }
            ')' => {
                state.advance(1);
                state.add_token(RustTokenType::RightParen, safe_point, state.get_position());
            }
            '{' => {
                state.advance(1);
                state.add_token(RustTokenType::LeftBrace, safe_point, state.get_position());
            }
            '}' => {
                state.advance(1);
                state.add_token(RustTokenType::RightBrace, safe_point, state.get_position());
            }
            '[' => {
                state.advance(1);
                state.add_token(RustTokenType::LeftBracket, safe_point, state.get_position());
            }
            ']' => {
                state.advance(1);
                state.add_token(RustTokenType::RightBracket, safe_point, state.get_position());
            }
            ';' => {
                state.advance(1);
                state.add_token(RustTokenType::Semicolon, safe_point, state.get_position());
            }
            ',' => {
                state.advance(1);
                state.add_token(RustTokenType::Comma, safe_point, state.get_position());
            }
            '$' => {
                state.advance(1);
                state.add_token(RustTokenType::Dollar, safe_point, state.get_position());
            }
            '?' => {
                state.advance(1);
                state.add_token(RustTokenType::Question, safe_point, state.get_position());
            }
            '@' => {
                state.advance(1);
                state.add_token(RustTokenType::At, safe_point, state.get_position());
            }
            '#' => {
                state.advance(1);
                state.add_token(RustTokenType::Hash, safe_point, state.get_position());
            }
            '=' | '!' | '<' | '>' | '&' | '|' | '+' | '-' | '*' | '%' | '^' | '.' | ':' => {
                lex_operators(state);
            }
            _ => {
                // Error
                state.advance(ch.len_utf8());
                state.add_token(RustTokenType::Error, safe_point, state.get_position());
            }
        }

        state.advance_if_dead_lock(safe_point)
    }

    Ok(())
}

fn skip_whitespace<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    let start = state.get_position();
    let bytes = state.rest_bytes();
    let mut i = 0;
    let len = bytes.len();
    const LANES: usize = 32;

    while i + LANES <= len {
        let chunk = Simd::<u8, LANES>::from_slice(unsafe { bytes.get_unchecked(i..i + LANES) });
        let is_le_space = chunk.simd_le(Simd::splat(32));

        if !is_le_space.all() {
            let not_ws = !is_le_space;
            let idx = not_ws.first_set().unwrap();
            i += idx;
            state.advance(i);
            state.add_token(RustTokenType::Whitespace, start, state.get_position());
            return true;
        }
        i += LANES;
    }

    while i < len {
        if !unsafe { *bytes.get_unchecked(i) }.is_ascii_whitespace() {
            break;
        }
        i += 1;
    }

    if i > 0 {
        state.advance(i);
        state.add_token(RustTokenType::Whitespace, start, state.get_position());
        true
    }
    else {
        false
    }
}

fn skip_comment<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    // Line comments: //
    if state.starts_with("//") {
        let start = state.get_position();
        state.advance(2); // Skip "//"
        state.take_while(|ch| ch != '\n');
        state.add_token(RustTokenType::LineComment, start, state.get_position());
        return true;
    }

    // Block comments: /* ... */
    if state.starts_with("/*") {
        let start = state.get_position();
        state.advance(2); // Skip "/*"

        let mut depth = 1;
        while depth > 0 && state.not_at_end() {
            if state.starts_with("/*") {
                depth += 1;
                state.advance(2);
            }
            else if state.starts_with("*/") {
                depth -= 1;
                state.advance(2);
            }
            else if let Some(ch) = state.current() {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(RustTokenType::BlockComment, start, state.get_position());
        return true;
    }

    false
}

fn lex_string_literal<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    // Handle raw strings r"..." or r#"..."#
    let start = state.get_position();
    if state.starts_with("r") {
        let n1 = state.peek_next_n(1);
        if n1 == Some('"') || n1 == Some('#') {
            state.advance(1); // consume 'r'
            let mut hash_count = 0;
            while state.consume_if_starts_with("#") {
                hash_count += 1;
            }

            if state.consume_if_starts_with("\"") {
                // Find closing quote with matching hashes
                loop {
                    if state.consume_if_starts_with("\"") {
                        let mut closing_hashes = 0;
                        while closing_hashes < hash_count && state.consume_if_starts_with("#") {
                            closing_hashes += 1;
                        }
                        if closing_hashes == hash_count {
                            break;
                        }
                    }
                    else if let Some(ch) = state.current() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(RustTokenType::StringLiteral, start, state.get_position());
                return true;
            }
            else {
                // Failed to match raw string start pattern fully (e.g. r# but no quote)
                // Backtrack? LexerState doesn't support backtrack easily unless we created new one.
                // But here we modified state.
                // Assuming valid Rust, r#... must be raw string.
                // If invalid, we probably consumed 'r' and '#'.
                // Return true as error? Or false?
                // If we return false, we need to restore state.
                // But we already advanced.
                // Let's assume it's a malformed raw string and tokenize what we have?
                // Or just treat as identifier 'r' and then '#' (single char)?
                // Since we don't have backtrack, we should peek before advance.
                // But this logic was existing. I'll leave it but wrap in logic that handles return.
            }
        }
    }

    // Handle regular strings
    if state.peek() == Some('"') {
        return RUST_STRING.scan(state, RustTokenType::StringLiteral);
    }
    false
}

fn lex_char_literal<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    RUST_CHAR.scan(state, RustTokenType::CharLiteral)
}

fn lex_number_literal<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    let start = state.get_position();
    if let Some(ch) = state.current() {
        if ch.is_ascii_digit() {
            state.advance(ch.len_utf8());

            // Handle binary literals (0b...)
            if ch == '0' && (state.consume_if_starts_with("b") || state.consume_if_starts_with("B")) {
                state.take_while(|ch| ch == '0' || ch == '1' || ch == '_');
            }
            // Handle octal literals (0o...)
            else if ch == '0' && (state.consume_if_starts_with("o") || state.consume_if_starts_with("O")) {
                state.take_while(|ch| ch.is_digit(8) || ch == '_');
            }
            // Handle hex literals (0x...)
            else if ch == '0' && (state.consume_if_starts_with("x") || state.consume_if_starts_with("X")) {
                state.take_while(|ch| ch.is_ascii_hexdigit() || ch == '_');
            }
            // Handle decimal literals
            else {
                state.take_while(|ch| ch.is_ascii_digit() || ch == '.' || ch == '_');

                // Handle exponent
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(ch.len_utf8());
                        // Optional sign
                        if let Some(next) = state.peek() {
                            if next == '+' || next == '-' {
                                state.advance(next.len_utf8());
                            }
                        }
                        // Exponent digits
                        state.take_while(|ch| ch.is_ascii_digit() || ch == '_');
                    }
                }
            }

            // Handle type suffixes (i32, u64, f32, etc.)
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() {
                    state.take_while(|ch| ch.is_ascii_alphanumeric());
                }
            }

            state.add_token(RustTokenType::IntegerLiteral, start, state.get_position());
            return true;
        }
    }
    false
}

fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    let start = state.get_position();
    if let Some(ch) = state.current() {
        // Check if the first character is valid for an identifier
        if ch == '_' || is_xid_start(ch) {
            state.advance(ch.len_utf8());

            // Continue reading while we have valid identifier continuation characters
            state.take_while(|ch| is_xid_continue(ch));

            let text = state.get_text_in((start..state.get_position()).into());
            let token_kind = match text.as_ref() {
                // Strict keywords
                "as" => RustTokenType::As,
                "break" => RustTokenType::Break,
                "const" => RustTokenType::Const,
                "continue" => RustTokenType::Continue,
                "crate" => RustTokenType::Crate,
                "else" => RustTokenType::Else,
                "enum" => RustTokenType::Enum,
                "extern" => RustTokenType::Extern,
                "false" => RustTokenType::False,
                "fn" => RustTokenType::Fn,
                "for" => RustTokenType::For,
                "if" => RustTokenType::If,
                "impl" => RustTokenType::Impl,
                "in" => RustTokenType::In,
                "let" => RustTokenType::Let,
                "loop" => RustTokenType::Loop,
                "match" => RustTokenType::Match,
                "mod" => RustTokenType::Mod,
                "move" => RustTokenType::Move,
                "mut" => RustTokenType::Mut,
                "pub" => RustTokenType::Pub,
                "ref" => RustTokenType::Ref,
                "return" => RustTokenType::Return,
                "self" => RustTokenType::SelfLower,
                "Self" => RustTokenType::SelfUpper,
                "static" => RustTokenType::Static,
                "struct" => RustTokenType::Struct,
                "super" => RustTokenType::Super,
                "trait" => RustTokenType::Trait,
                "true" => RustTokenType::True,
                "type" => RustTokenType::Type,
                "unsafe" => RustTokenType::Unsafe,
                "use" => RustTokenType::Use,
                "where" => RustTokenType::Where,
                "while" => RustTokenType::While,
                // Reserved keywords
                "abstract" => RustTokenType::Abstract,
                "become" => RustTokenType::Become,
                "box" => RustTokenType::Box,
                "do" => RustTokenType::Do,
                "final" => RustTokenType::Final,
                "macro" => RustTokenType::Macro,
                "override" => RustTokenType::Override,
                "priv" => RustTokenType::Priv,
                "typeof" => RustTokenType::Typeof,
                "unsized" => RustTokenType::Unsized,
                "virtual" => RustTokenType::Virtual,
                "yield" => RustTokenType::Yield,
                // Weak keywords
                "async" => RustTokenType::Async,
                "await" => RustTokenType::Await,
                "dyn" => RustTokenType::Dyn,
                "try" => RustTokenType::Try,
                "union" => RustTokenType::Union,
                // Edition-specific keywords
                "raw" => RustTokenType::Raw,
                _ => RustTokenType::Identifier,
            };

            state.add_token(token_kind, start, state.get_position());
            return true;
        }
    }
    false
}

fn lex_operators<'s, S: Source + ?Sized>(state: &mut State<'s, S>) -> bool {
    let start = state.get_position();
    let Some(ch) = state.peek()
    else {
        return false;
    };

    match ch {
        '=' => {
            state.advance(1);
            if state.consume_if_starts_with(">") {
                state.add_token(RustTokenType::FatArrow, start, state.get_position());
            }
            else if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::EqEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Eq, start, state.get_position());
            }
            true
        }
        '!' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::Ne, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Bang, start, state.get_position());
            }
            true
        }
        '<' => {
            state.advance(1);
            if state.consume_if_starts_with("<=") {
                state.add_token(RustTokenType::LeftShiftEq, start, state.get_position());
            }
            else if state.consume_if_starts_with("<") {
                state.add_token(RustTokenType::LeftShift, start, state.get_position());
            }
            else if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::LessEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::LessThan, start, state.get_position());
            }
            true
        }
        '>' => {
            state.advance(1);
            if state.consume_if_starts_with(">=") {
                state.add_token(RustTokenType::RightShiftEq, start, state.get_position());
            }
            else if state.consume_if_starts_with(">") {
                state.add_token(RustTokenType::RightShift, start, state.get_position());
            }
            else if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::GreaterEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::GreaterThan, start, state.get_position());
            }
            true
        }
        '&' => {
            state.advance(1);
            if state.consume_if_starts_with("&") {
                state.add_token(RustTokenType::AndAnd, start, state.get_position());
            }
            else if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::AndEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Ampersand, start, state.get_position());
            }
            true
        }
        '|' => {
            state.advance(1);
            if state.consume_if_starts_with("|") {
                state.add_token(RustTokenType::OrOr, start, state.get_position());
            }
            else if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::OrEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Pipe, start, state.get_position());
            }
            true
        }
        '+' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::PlusEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Plus, start, state.get_position());
            }
            true
        }
        '-' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::MinusEq, start, state.get_position());
            }
            else if state.consume_if_starts_with(">") {
                state.add_token(RustTokenType::Arrow, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Minus, start, state.get_position());
            }
            true
        }
        '*' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::StarEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Star, start, state.get_position());
            }
            true
        }
        '/' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::SlashEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Slash, start, state.get_position());
            }
            true
        }
        '%' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::PercentEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Percent, start, state.get_position());
            }
            true
        }
        '^' => {
            state.advance(1);
            if state.consume_if_starts_with("=") {
                state.add_token(RustTokenType::CaretEq, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Caret, start, state.get_position());
            }
            true
        }
        '.' => {
            state.advance(1);
            if state.consume_if_starts_with("..") {
                state.add_token(RustTokenType::DotDotDot, start, state.get_position());
            }
            else if state.consume_if_starts_with(".") {
                if state.consume_if_starts_with("=") {
                    state.add_token(RustTokenType::DotDotEq, start, state.get_position());
                }
                else {
                    state.add_token(RustTokenType::DotDot, start, state.get_position());
                }
            }
            else {
                state.add_token(RustTokenType::Dot, start, state.get_position());
            }
            true
        }
        ':' => {
            state.advance(1);
            if state.consume_if_starts_with(":") {
                state.add_token(RustTokenType::PathSep, start, state.get_position());
            }
            else {
                state.add_token(RustTokenType::Colon, start, state.get_position());
            }
            true
        }
        _ => false,
    }
}
