use crate::{kind::FortranSyntaxKind, language::FortranLanguage};
use oak_core::{Lexer, LexerCache, LexerState, OakError, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, FortranLanguage>;

#[derive(Clone)]
pub struct FortranLexer<'config> {
    _config: &'config FortranLanguage,
}

impl<'config> Lexer<FortranLanguage> for FortranLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<FortranLanguage>) -> LexOutput<FortranLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> FortranLexer<'config> {
    pub fn new(config: &'config FortranLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.lex_newline(state) {
                continue;
            }

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

            if self.lex_operator_or_single_char(state) {
                continue;
            }

            // If no lexer matched, advance by one character to avoid infinite loop
            if let Some(c) = state.current() {
                state.advance(c.len_utf8());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            if ch == '\r' {
                state.advance(1);
                if state.current() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(FortranSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let mut advanced = false;

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
                advanced = true;
            }
            else {
                break;
            }
        }

        advanced
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        // Check for Fortran comment indicators
        if let Some(ch) = state.current() {
            if ch == '!' {
                // Skip to end of line
                while let Some(c) = state.current() {
                    if c == '\n' || c == '\r' {
                        break;
                    }
                    state.advance(c.len_utf8());
                }
                return true;
            }
        }
        false
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('"') {
            return false;
        }

        state.advance(1); // consume opening quote

        while let Some(ch) = state.current() {
            if ch == '"' {
                state.advance(1); // consume closing quote
                break;
            }
            if ch == '\n' || ch == '\r' {
                break; // Fortran strings don't span lines
            }
            state.advance(ch.len_utf8());
        }

        state.add_token(FortranSyntaxKind::StringLiteral, start, state.get_position());
        true
    }

    fn lex_char_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('\'') {
            return false;
        }

        state.advance(1); // consume opening quote

        // Consume exactly one character (or none for empty char literal)
        if let Some(ch) = state.current() {
            if ch != '\'' && ch != '\n' && ch != '\r' {
                state.advance(ch.len_utf8());
            }
        }

        // Consume closing quote if present
        if state.current() == Some('\'') {
            state.advance(1);
        }

        state.add_token(FortranSyntaxKind::CharLiteral, start, state.get_position());
        true
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        // Read integer part
        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_digit() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // Check for decimal point
        if state.current() == Some('.') {
            let n1 = state.peek_next_n(1);
            if n1.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                state.advance(1); // consume '.'
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // Check for exponent (e, E, d, D for Fortran)
        if let Some(c) = state.current() {
            if c == 'e' || c == 'E' || c == 'd' || c == 'D' {
                let n1 = state.peek_next_n(1);
                if n1 == Some('+') || n1 == Some('-') || n1.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    state.advance(1);
                    if let Some(sign) = state.current() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.current() {
                        if d.is_ascii_digit() || d == '_' {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        let end = state.get_position();
        state.add_token(FortranSyntaxKind::NumberLiteral, start, end);
        true
    }

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_alphabetic() && first != '_' {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());

        let kind = match text.to_lowercase().as_str() {
            "program" => FortranSyntaxKind::Program,
            "end" => FortranSyntaxKind::End,
            "subroutine" => FortranSyntaxKind::Subroutine,
            "function" => FortranSyntaxKind::Function,
            "integer" => FortranSyntaxKind::Integer,
            "real" => FortranSyntaxKind::Real,
            "double" => FortranSyntaxKind::Double,
            "precision" => FortranSyntaxKind::Precision,
            "character" => FortranSyntaxKind::Character,
            "logical" => FortranSyntaxKind::Logical,
            "complex" => FortranSyntaxKind::Complex,
            "if" => FortranSyntaxKind::If,
            "then" => FortranSyntaxKind::Then,
            "else" => FortranSyntaxKind::Else,
            "elseif" => FortranSyntaxKind::ElseIf,
            "endif" => FortranSyntaxKind::EndIf,
            "do" => FortranSyntaxKind::Do,
            "enddo" => FortranSyntaxKind::EndDo,
            "while" => FortranSyntaxKind::While,
            "call" => FortranSyntaxKind::Call,
            "return" => FortranSyntaxKind::Return,
            "stop" => FortranSyntaxKind::Stop,
            "continue" => FortranSyntaxKind::Continue,
            "goto" => FortranSyntaxKind::Goto,
            "implicit" => FortranSyntaxKind::Implicit,
            "none" => FortranSyntaxKind::None,
            "parameter" => FortranSyntaxKind::Parameter,
            "dimension" => FortranSyntaxKind::Dimension,
            "common" => FortranSyntaxKind::Common,
            "equivalence" => FortranSyntaxKind::Equivalence,
            "external" => FortranSyntaxKind::External,
            "intrinsic" => FortranSyntaxKind::Intrinsic,
            "save" => FortranSyntaxKind::Save,
            "data" => FortranSyntaxKind::Data,
            "format" => FortranSyntaxKind::Format,
            "read" => FortranSyntaxKind::Read,
            "write" => FortranSyntaxKind::Write,
            "print" => FortranSyntaxKind::Print,
            "open" => FortranSyntaxKind::Open,
            "close" => FortranSyntaxKind::Close,
            "inquire" => FortranSyntaxKind::Inquire,
            "rewind" => FortranSyntaxKind::Rewind,
            "backspace" => FortranSyntaxKind::Backspace,
            "endfile" => FortranSyntaxKind::EndFile,
            "true" => FortranSyntaxKind::True,
            "false" => FortranSyntaxKind::False,
            "and" => FortranSyntaxKind::And,
            "or" => FortranSyntaxKind::Or,
            "not" => FortranSyntaxKind::Not,
            "eq" => FortranSyntaxKind::Eq,
            "ne" => FortranSyntaxKind::Ne,
            "lt" => FortranSyntaxKind::Lt,
            "le" => FortranSyntaxKind::Le,
            "gt" => FortranSyntaxKind::Gt,
            "ge" => FortranSyntaxKind::Ge,
            _ => FortranSyntaxKind::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operator_or_single_char<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();
        let c = match state.current() {
            Some(c) => c,
            None => return false,
        };

        match c {
            '\n' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Newline, start, state.get_position());
            }
            '(' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::LeftParen, start, state.get_position());
            }
            ')' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::RightParen, start, state.get_position());
            }
            ',' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Comma, start, state.get_position());
            }
            '=' => {
                state.advance(1);
                if state.current() == Some('=') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::EqualEqual, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Equal, start, state.get_position());
                }
            }
            '+' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Plus, start, state.get_position());
            }
            '-' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Minus, start, state.get_position());
            }
            '*' => {
                state.advance(1);
                if state.current() == Some('*') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::StarStar, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Star, start, state.get_position());
                }
            }
            '/' => {
                state.advance(1);
                if state.current() == Some('=') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::SlashEqual, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Slash, start, state.get_position());
                }
            }
            '<' => {
                state.advance(1);
                if state.current() == Some('=') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::LessEqual, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Less, start, state.get_position());
                }
            }
            '>' => {
                state.advance(1);
                if state.current() == Some('=') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::GreaterEqual, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Greater, start, state.get_position());
                }
            }
            '.' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Dot, start, state.get_position());
            }
            ':' => {
                state.advance(1);
                if state.current() == Some(':') {
                    state.advance(1);
                    state.add_token(FortranSyntaxKind::ColonColon, start, state.get_position());
                }
                else {
                    state.add_token(FortranSyntaxKind::Colon, start, state.get_position());
                }
            }
            ';' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Semicolon, start, state.get_position());
            }
            '&' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Ampersand, start, state.get_position());
            }
            '%' => {
                state.advance(1);
                state.add_token(FortranSyntaxKind::Percent, start, state.get_position());
            }
            _ => {
                return false;
            }
        }
        true
    }
}
