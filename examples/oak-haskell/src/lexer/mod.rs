use crate::{kind::HaskellSyntaxKind, language::HaskellLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, SourceText, lexer::LexOutput, source::Source};

#[derive(Clone)]
pub struct HaskellLexer<'config> {
    config: &'config HaskellLanguage,
}

impl<'config> HaskellLexer<'config> {
    pub fn new(config: &'config HaskellLanguage) -> Self {
        Self { config }
    }

    fn skip_whitespace<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(1);
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

    fn lex_newline<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(HaskellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_single_line_comment<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            if let Some('-') = state.peek_next_n(1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(1);
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

    fn lex_multi_line_comment<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
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
                        state.advance(1);
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

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(1);
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

    fn lex_number<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
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

    fn lex_string<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(HaskellSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }
            }

            state.add_token(HaskellSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_char<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' {
                    state.advance(1);
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
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

    fn lex_operators<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Append
                    }
                    else {
                        HaskellSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Arrow
                    }
                    else {
                        HaskellSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    HaskellSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    HaskellSyntaxKind::Slash
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::Equal
                    }
                    else {
                        HaskellSyntaxKind::Assign
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::LessEqual
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::LeftArrow
                    }
                    else {
                        HaskellSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::GreaterEqual
                    }
                    else {
                        HaskellSyntaxKind::Greater
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::DoubleColon
                    }
                    else {
                        HaskellSyntaxKind::Colon
                    }
                }
                '|' => {
                    state.advance(1);
                    HaskellSyntaxKind::Pipe
                }
                '&' => {
                    state.advance(1);
                    HaskellSyntaxKind::Ampersand
                }
                '!' => {
                    state.advance(1);
                    HaskellSyntaxKind::Bang
                }
                '?' => {
                    state.advance(1);
                    HaskellSyntaxKind::Question
                }
                ';' => {
                    state.advance(1);
                    HaskellSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    HaskellSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        HaskellSyntaxKind::DoubleDot
                    }
                    else {
                        HaskellSyntaxKind::Dot
                    }
                }
                '$' => {
                    state.advance(1);
                    HaskellSyntaxKind::Dollar
                }
                '@' => {
                    state.advance(1);
                    HaskellSyntaxKind::At
                }
                '~' => {
                    state.advance(1);
                    HaskellSyntaxKind::Tilde
                }
                '\\' => {
                    state.advance(1);
                    HaskellSyntaxKind::Backslash
                }
                '`' => {
                    state.advance(1);
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

    fn lex_delimiters<S: Source>(&self, state: &mut LexerState<S, HaskellLanguage>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    HaskellSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    HaskellSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    HaskellSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    HaskellSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    HaskellSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
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
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<HaskellLanguage>,
    ) -> LexOutput<HaskellLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);

        while state.not_at_end() {
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
            if let Some(ch) = state.peek() {
                state.advance(1);
                state.add_token(HaskellSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let pos = state.get_position();
        state.add_token(HaskellSyntaxKind::Eof, pos, pos);

        state.finish(Ok(()))
    }
}
