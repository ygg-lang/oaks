use crate::{kind::PurescriptSyntaxKind, language::PurescriptLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, PurescriptLanguage>;

#[derive(Clone)]
pub struct PurescriptLexer<'config> {
    config: &'config PurescriptLanguage,
}

impl<'config> PurescriptLexer<'config> {
    pub fn new(config: &'config PurescriptLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_char_literal(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_delimiter(state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PurescriptSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(PurescriptSyntaxKind::Eof, eof_pos, eof_pos);

        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(PurescriptSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PurescriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PurescriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(PurescriptSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('{') = state.peek() {
            state.advance(1);
            if let Some('-') = state.peek() {
                // 多行注释
                state.advance(1);
                let mut depth = 1;
                while let Some(ch) = state.peek() {
                    if ch == '{' {
                        state.advance(1);
                        if let Some('-') = state.peek() {
                            depth += 1;
                            state.advance(1);
                        }
                    }
                    else if ch == '-' {
                        state.advance(1);
                        if let Some('}') = state.peek() {
                            depth -= 1;
                            state.advance(1);
                            if depth == 0 {
                                break;
                            }
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PurescriptSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符或关键
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否为关键字
                let text = state.get_text_in((start_pos..state.get_position()).into());

                let token_kind = match text.as_ref() {
                    "ado" => PurescriptSyntaxKind::Ado,
                    "case" => PurescriptSyntaxKind::Case,
                    "class" => PurescriptSyntaxKind::Class,
                    "data" => PurescriptSyntaxKind::Data,
                    "derive" => PurescriptSyntaxKind::Derive,
                    "do" => PurescriptSyntaxKind::Do,
                    "else" => PurescriptSyntaxKind::Else,
                    "false" => PurescriptSyntaxKind::False,
                    "forall" => PurescriptSyntaxKind::Forall,
                    "foreign" => PurescriptSyntaxKind::Foreign,
                    "if" => PurescriptSyntaxKind::If,
                    "import" => PurescriptSyntaxKind::Import,
                    "in" => PurescriptSyntaxKind::In,
                    "infix" => PurescriptSyntaxKind::Infix,
                    "infixl" => PurescriptSyntaxKind::Infixl,
                    "infixr" => PurescriptSyntaxKind::Infixr,
                    "instance" => PurescriptSyntaxKind::Instance,
                    "let" => PurescriptSyntaxKind::Let,
                    "module" => PurescriptSyntaxKind::Module,
                    "newtype" => PurescriptSyntaxKind::Newtype,
                    "of" => PurescriptSyntaxKind::Of,
                    "then" => PurescriptSyntaxKind::Then,
                    "true" => PurescriptSyntaxKind::True,
                    "type" => PurescriptSyntaxKind::Type,
                    "where" => PurescriptSyntaxKind::Where,
                    _ => PurescriptSyntaxKind::Identifier,
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

    /// 处理数字字面
    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理十六进制数字
                if ch == '0' {
                    if let Some('x') | Some('X') = state.peek() {
                        state.advance(1);
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_hexdigit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                    else {
                        // 处理普通数
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
                else {
                    // 处理十进制数
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理小数
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理指数
                if let Some('e') | Some('E') = state.peek() {
                    state.advance(1);
                    if let Some('+') | Some('-') = state.peek() {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(PurescriptSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 字符串不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(PurescriptSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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
                    state.advance(ch.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
                state.add_token(PurescriptSyntaxKind::CharLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    PurescriptSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::Arrow
                    }
                    else {
                        PurescriptSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::Caret // 使用 Caret 代替 Power
                    }
                    else {
                        PurescriptSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::NotEqual
                    }
                    else {
                        PurescriptSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    PurescriptSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            PurescriptSyntaxKind::Equal
                        }
                        Some('>') => {
                            state.advance(1);
                            PurescriptSyntaxKind::FatArrow
                        }
                        _ => PurescriptSyntaxKind::Equal,
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            PurescriptSyntaxKind::LessEqual
                        }
                        Some('-') => {
                            state.advance(1);
                            PurescriptSyntaxKind::Bind
                        }
                        _ => PurescriptSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::GreaterEqual
                    }
                    else {
                        PurescriptSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::And
                    }
                    else {
                        return false;
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PurescriptSyntaxKind::Or
                    }
                    else {
                        PurescriptSyntaxKind::Pipe
                    }
                }
                '\\' => {
                    state.advance(1);
                    PurescriptSyntaxKind::Backslash
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

    /// 处理分隔
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => PurescriptSyntaxKind::LeftParen,
                ')' => PurescriptSyntaxKind::RightParen,
                '[' => PurescriptSyntaxKind::LeftBracket,
                ']' => PurescriptSyntaxKind::RightBracket,
                '{' => PurescriptSyntaxKind::LeftBrace,
                '}' => PurescriptSyntaxKind::RightBrace,
                ',' => PurescriptSyntaxKind::Comma,
                ';' => PurescriptSyntaxKind::Semicolon,
                '.' => PurescriptSyntaxKind::Dot,
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        state.add_token(PurescriptSyntaxKind::ColonColon, start_pos, state.get_position());
                        return true;
                    }
                    else {
                        state.add_token(PurescriptSyntaxKind::Colon, start_pos, state.get_position());
                        return true;
                    }
                }
                '?' => PurescriptSyntaxKind::Question,
                '_' => PurescriptSyntaxKind::Underscore,
                '@' => PurescriptSyntaxKind::At,
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

impl<'config> Lexer<PurescriptLanguage> for PurescriptLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<PurescriptLanguage>,
    ) -> LexOutput<PurescriptLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
