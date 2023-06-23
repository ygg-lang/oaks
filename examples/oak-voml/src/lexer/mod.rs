#![doc = include_str!("readme.md")]
use crate::{language::VomlLanguage, lexer::token_type::VomlTokenType};
pub mod token_type;
use oak_core::{
    Lexer, LexerState, Source,
    lexer::{LexOutput, LexerCache},
};

type State<'a, S> = LexerState<'a, S, VomlLanguage>;

#[derive(Clone, Debug)]
pub struct VomlLexer<'config> {
    _config: &'config VomlLanguage,
}

impl<'config> VomlLexer<'config> {
    pub fn new(config: &'config VomlLanguage) -> Self {
        Self { _config: config }
    }

    /// Skips whitespace characters.
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(VomlTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a newline.
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(VomlTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(VomlTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Lexes a comment.
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // Line comment //
        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                // Read until end of line
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(VomlTokenType::Comment, start_pos, state.get_position());
                return true;
            }
            // Block comment /* */
            else if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = state.peek_next_n(1) {
                            state.advance(2);
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(VomlTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Lexes a string literal.
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;

                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote {
                        state.advance(1);
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // Strings cannot span lines
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                let token_kind = if quote == '"' { VomlTokenType::StringLiteral } else { VomlTokenType::CharLiteral };
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

    /// Lexes a number literal.
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // Integer part
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // Check for decimal point
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // Skip decimal point

                            // Fractional part
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // Check for exponent
                if let Some(e) = state.peek() {
                    if e == 'e' || e == 'E' {
                        let exp_start = state.get_position();
                        state.advance(1);

                        // Optional sign
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // Exponent digits
                        let mut has_exp_digits = false;
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                has_exp_digits = true;
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        if has_exp_digits {
                            is_float = true;
                        }
                        else {
                            // Backtrack to exponent start position
                            state.set_position(exp_start);
                        }
                    }
                }

                let token_kind = if is_float { VomlTokenType::FloatLiteral } else { VomlTokenType::IntegerLiteral };
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

    /// Lexes identifiers and keywords.
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text.as_ref() {
                    "module" => VomlTokenType::ModuleKw,
                    "import" => VomlTokenType::ImportKw,
                    "pub" => VomlTokenType::PubKw,
                    "fn" => VomlTokenType::FnKw,
                    "struct" => VomlTokenType::StructKw,
                    "interface" => VomlTokenType::InterfaceKw,
                    "enum" => VomlTokenType::EnumKw,
                    "type" => VomlTokenType::TypeKw,
                    "const" => VomlTokenType::ConstKw,
                    "mut" => VomlTokenType::MutKw,
                    "shared" => VomlTokenType::SharedKw,
                    "volatile" => VomlTokenType::VolatileKw,
                    "unsafe" => VomlTokenType::UnsafeKw,
                    "if" => VomlTokenType::IfKw,
                    "else" => VomlTokenType::ElseKw,
                    "for" => VomlTokenType::ForKw,
                    "in" => VomlTokenType::InKw,
                    "match" => VomlTokenType::MatchKw,
                    "or" => VomlTokenType::OrKw,
                    "return" => VomlTokenType::ReturnKw,
                    "break" => VomlTokenType::BreakKw,
                    "continue" => VomlTokenType::ContinueKw,
                    "goto" => VomlTokenType::GotoKw,
                    "defer" => VomlTokenType::DeferKw,
                    "go" => VomlTokenType::GoKw,
                    "select" => VomlTokenType::SelectKw,
                    "lock" => VomlTokenType::LockKw,
                    "rlock" => VomlTokenType::RlockKw,
                    "as" => VomlTokenType::AsKw,
                    "is" => VomlTokenType::IsKw,
                    "sizeof" => VomlTokenType::SizeofKw,
                    "typeof" => VomlTokenType::TypeofKw,
                    "offsetof" => VomlTokenType::OffsetofKw,
                    "assert" => VomlTokenType::AssertKw,
                    "panic" => VomlTokenType::PanicKw,
                    "eprintln" => VomlTokenType::EprintlnKw,
                    "println" => VomlTokenType::PrintlnKw,
                    "print" => VomlTokenType::PrintKw,
                    "eprint" => VomlTokenType::EprintKw,
                    "bool" => VomlTokenType::BoolKw,
                    "i8" => VomlTokenType::I8Kw,
                    "i16" => VomlTokenType::I16Kw,
                    "i32" => VomlTokenType::I32Kw,
                    "i64" => VomlTokenType::I64Kw,
                    "u8" => VomlTokenType::U8Kw,
                    "u16" => VomlTokenType::U16Kw,
                    "u32" => VomlTokenType::U32Kw,
                    "u64" => VomlTokenType::U64Kw,
                    "int" => VomlTokenType::IntKw,
                    "uint" => VomlTokenType::UintKw,
                    "f32" => VomlTokenType::F32Kw,
                    "f64" => VomlTokenType::F64Kw,
                    "string" => VomlTokenType::StringKw,
                    "rune" => VomlTokenType::RuneKw,
                    "byte" => VomlTokenType::ByteKw,
                    "voidptr" => VomlTokenType::VoidptrKw,
                    "char" => VomlTokenType::CharKw,
                    "true" | "false" => VomlTokenType::BoolLiteral,
                    _ => VomlTokenType::Identifier,
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

    /// Lexes operators and punctuation.
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::PlusEq
                    }
                    else if let Some('+') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::PlusPlus
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Plus
                    }
                }
                '-' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::MinusEq
                    }
                    else if let Some('-') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::MinusMinus
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::Arrow
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Minus
                    }
                }
                '*' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::StarEq
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Star
                    }
                }
                '/' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::SlashEq
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Slash
                    }
                }
                '%' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::PercentEq
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Percent
                    }
                }
                '&' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::AmpersandEq
                    }
                    else if let Some('&') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::AndAnd
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Ampersand
                    }
                }
                '|' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::PipeEq
                    }
                    else if let Some('|') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::OrOr
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::CaretEq
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Caret
                    }
                }
                '=' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::EqEq
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::FatArrow
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Eq
                    }
                }
                '!' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::Ne
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Bang
                    }
                }
                '<' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::Le
                    }
                    else if let Some('<') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VomlTokenType::LeftShiftEq
                        }
                        else {
                            state.advance(2);
                            VomlTokenType::LeftShift
                        }
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::LessThan
                    }
                }
                '>' => {
                    if let Some('=') = state.peek_next_n(1) {
                        state.advance(2);
                        VomlTokenType::Ge
                    }
                    else if let Some('>') = state.peek_next_n(1) {
                        if let Some('=') = state.peek_next_n(2) {
                            state.advance(3);
                            VomlTokenType::RightShiftEq
                        }
                        else {
                            state.advance(2);
                            VomlTokenType::RightShift
                        }
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::GreaterThan
                    }
                }
                '.' => {
                    if let Some('.') = state.peek_next_n(1) {
                        if let Some('.') = state.peek_next_n(2) {
                            state.advance(3);
                            VomlTokenType::DotDotDot
                        }
                        else {
                            state.advance(2);
                            VomlTokenType::DotDot
                        }
                    }
                    else {
                        state.advance(1);
                        VomlTokenType::Dot
                    }
                }
                ',' => {
                    state.advance(1);
                    VomlTokenType::Comma
                }
                ':' => {
                    state.advance(1);
                    VomlTokenType::Colon
                }
                ';' => {
                    state.advance(1);
                    VomlTokenType::Semicolon
                }
                '(' => {
                    state.advance(1);
                    VomlTokenType::LeftParen
                }
                ')' => {
                    state.advance(1);
                    VomlTokenType::RightParen
                }
                '[' => {
                    state.advance(1);
                    VomlTokenType::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    VomlTokenType::RightBracket
                }
                '{' => {
                    state.advance(1);
                    VomlTokenType::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    VomlTokenType::RightBrace
                }
                '?' => {
                    state.advance(1);
                    VomlTokenType::Question
                }
                '~' => {
                    state.advance(1);
                    VomlTokenType::Tilde
                }
                _ => {
                    state.advance(ch.len_utf8());
                    VomlTokenType::Error
                }
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<VomlLanguage> for VomlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<VomlLanguage>) -> LexOutput<VomlLanguage> {
        let mut state = State::new_with_cache(source, 0, cache);

        while let Some(_ch) = state.peek() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            // 如果都没有匹配，则跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(VomlTokenType::Error, start_pos, state.get_position())
            }
        }

        state.add_eof();
        state.finish_with_cache(Ok(()), cache)
    }
}
