use crate::{kind::HaskellSyntaxKind, language::HaskellLanguage};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, HaskellLanguage>;

#[derive(Clone)]
pub struct HaskellLexer<'config> {
    _config: &'config HaskellLanguage,
}

impl<'config> HaskellLexer<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { _config: config }
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.bump();
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(HaskellSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.bump();
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.bump();
            if let Some('\n') = state.peek() {
                state.bump();
            }
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_single_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.bump();
                }
                state.add_token(HaskellSyntaxKind::Comment, start_pos, state.get_position());
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

    fn lex_multi_line_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '{' && state.peek_next_n(1) == Some('-') {
                        depth += 1;
                        state.advance(2);
                    }
                    else if ch == '-' && state.peek_next_n(1) == Some('}') {
                        depth -= 1;
                        state.advance(2);
                        if depth == 0 {
                            break;
                        }
                    }
                    else {
                        state.bump();
                    }
                }
                state.add_token(HaskellSyntaxKind::Comment, start_pos, state.get_position());
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

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.bump();

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = state.get_text_in((start_pos..end_pos).into());
                let kind = self.keyword_or_identifier(text.as_ref());

                state.add_token(kind, start_pos, end_pos);
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

    fn keyword_or_identifier(&self, text: &str) -> HaskellSyntaxKind {
        match text {
            "case" => HaskellSyntaxKind::Case,
            "class" => HaskellSyntaxKind::Class,
            "data" => HaskellSyntaxKind::Data,
            "default" => HaskellSyntaxKind::Default,
            "deriving" => HaskellSyntaxKind::Deriving,
            "do" => HaskellSyntaxKind::Do,
            "else" => HaskellSyntaxKind::Else,
            "if" => HaskellSyntaxKind::If,
            "import" => HaskellSyntaxKind::Import,
            "in" => HaskellSyntaxKind::In,
            "infix" => HaskellSyntaxKind::Infix,
            "infixl" => HaskellSyntaxKind::Infixl,
            "infixr" => HaskellSyntaxKind::Infixr,
            "instance" => HaskellSyntaxKind::Instance,
            "let" => HaskellSyntaxKind::Let,
            "module" => HaskellSyntaxKind::Module,
            "newtype" => HaskellSyntaxKind::Newtype,
            "of" => HaskellSyntaxKind::Of,
            "then" => HaskellSyntaxKind::Then,
            "type" => HaskellSyntaxKind::Type,
            "where" => HaskellSyntaxKind::Where,
            _ => HaskellSyntaxKind::Identifier,
        }
    }

    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.bump();

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.bump();
                    }
                    else if ch == '.' {
                        state.bump();
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.bump();
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else {
                        break;
                    }
                }

                state.add_token(HaskellSyntaxKind::Number, start_pos, state.get_position());
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

    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.bump();

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.bump();
                    state.add_token(HaskellSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.bump();
                    if let Some(_) = state.peek() {
                        state.bump();
                    }
                }
                else {
                    state.bump();
                }
            }

            state.add_token(HaskellSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.bump();

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.bump();
                    if let Some(_) = state.peek() {
                        state.bump();
                    }
                }
                else if ch != '\'' {
                    state.bump();
                }
            }

            if let Some('\'') = state.peek() {
                state.bump();
                state.add_token(HaskellSyntaxKind::CharLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.add_token(HaskellSyntaxKind::CharLiteral, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.bump();
                    if let Some('+') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::Append
                    }
                    else {
                        HaskellSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.bump();
                    if let Some('>') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::Arrow
                    }
                    else {
                        HaskellSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.bump();
                    HaskellSyntaxKind::Star
                }
                '/' => {
                    state.bump();
                    HaskellSyntaxKind::Slash
                }
                '=' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::Equal
                    }
                    else {
                        HaskellSyntaxKind::Assign
                    }
                }
                '<' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::LessEqual
                    }
                    else if let Some('-') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::LeftArrow
                    }
                    else {
                        HaskellSyntaxKind::Less
                    }
                }
                '>' => {
                    state.bump();
                    if let Some('=') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::GreaterEqual
                    }
                    else {
                        HaskellSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.bump();
                    if let Some(':') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::DoubleColon
                    }
                    else {
                        HaskellSyntaxKind::Colon
                    }
                }
                '|' => {
                    state.bump();
                    HaskellSyntaxKind::Pipe
                }
                '&' => {
                    state.bump();
                    HaskellSyntaxKind::Ampersand
                }
                '!' => {
                    state.bump();
                    HaskellSyntaxKind::Bang
                }
                '?' => {
                    state.bump();
                    HaskellSyntaxKind::Question
                }
                ';' => {
                    state.bump();
                    HaskellSyntaxKind::Semicolon
                }
                ',' => {
                    state.bump();
                    HaskellSyntaxKind::Comma
                }
                '.' => {
                    state.bump();
                    if let Some('.') = state.peek() {
                        state.bump();
                        HaskellSyntaxKind::DoubleDot
                    }
                    else {
                        HaskellSyntaxKind::Dot
                    }
                }
                '$' => {
                    state.bump();
                    HaskellSyntaxKind::Dollar
                }
                '@' => {
                    state.bump();
                    HaskellSyntaxKind::At
                }
                '~' => {
                    state.bump();
                    HaskellSyntaxKind::Tilde
                }
                '\\' => {
                    state.bump();
                    HaskellSyntaxKind::Backslash
                }
                '`' => {
                    state.bump();
                    HaskellSyntaxKind::Backtick
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

    fn lex_delimiters<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.bump();
                    HaskellSyntaxKind::LeftParen
                }
                ')' => {
                    state.bump();
                    HaskellSyntaxKind::RightParen
                }
                '[' => {
                    state.bump();
                    HaskellSyntaxKind::LeftBracket
                }
                ']' => {
                    state.bump();
                    HaskellSyntaxKind::RightBracket
                }
                '{' => {
                    state.bump();
                    HaskellSyntaxKind::LeftBrace
                }
                '}' => {
                    state.bump();
                    HaskellSyntaxKind::RightBrace
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
}

impl<'config> Lexer<HaskellLanguage> for HaskellLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<HaskellLanguage>) -> LexOutput<HaskellLanguage> {
        let mut state = State::new(source);

        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_single_line_comment(&mut state) {
                continue;
            }

            if self.lex_multi_line_comment(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_operators(&mut state) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if state.peek().is_some() {
                state.advance(1);
                state.add_token(HaskellSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        let pos = state.get_position();
        state.add_token(HaskellSyntaxKind::Eof, pos, pos);

        state.finish_with_cache(Ok(()), cache)
    }
}
