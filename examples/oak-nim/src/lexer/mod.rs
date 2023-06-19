use crate::{kind::NimSyntaxKind, language::NimLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, NimLanguage>;

#[derive(Clone, Debug)]
pub struct NimLexer<'config> {
    config: &'config NimLanguage,
}

impl<'config> NimLexer<'config> {
    pub fn new(config: &'config NimLanguage) -> Self {
        Self { config }
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
            state.add_token(NimSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(NimSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NimSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 检查是否是文档注释 ##
            if let Some('#') = state.peek() {
                state.advance(1);
            }

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            let kind = NimSyntaxKind::CommentToken;

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
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
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(NimSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if let Some('\'') = state.peek() {
                state.advance(1);
            }

            state.add_token(NimSyntaxKind::CharLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();
        let mut is_float = false;

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 读取数字
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !is_float {
                        // 检查是否是浮点
                        let current_pos = state.get_position();
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch.is_ascii_digit() {
                                is_float = true;
                            }
                            else {
                                state.set_position(current_pos);
                                break;
                            }
                        }
                        else {
                            state.set_position(current_pos);
                            break;
                        }
                    }
                    else if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch.is_ascii_digit() {
                                is_float = true;
                            }
                            else if next_ch == '+' || next_ch == '-' {
                                state.advance(1);
                                if let Some(digit_ch) = state.peek() {
                                    if digit_ch.is_ascii_digit() {
                                        is_float = true;
                                    }
                                    else {
                                        state.set_position(state.get_position() - 2);
                                        break;
                                    }
                                }
                                else {
                                    state.set_position(state.get_position() - 2);
                                    break;
                                }
                            }
                            else {
                                state.set_position(state.get_position() - 1);
                                break;
                            }
                        }
                        else {
                            state.set_position(state.get_position() - 1);
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }

                let kind = if is_float { NimSyntaxKind::FloatLiteral } else { NimSyntaxKind::IntLiteral };

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

    /// 处理标识符和关键字
    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start_pos..state.get_position()).into());
                let kind = match text {
                    "and" => NimSyntaxKind::AndKeyword,
                    "or" => NimSyntaxKind::OrKeyword,
                    "not" => NimSyntaxKind::NotKeyword,
                    "if" => NimSyntaxKind::IfKeyword,
                    "else" => NimSyntaxKind::ElseKeyword,
                    "elif" => NimSyntaxKind::ElifKeyword,
                    "while" => NimSyntaxKind::WhileKeyword,
                    "for" => NimSyntaxKind::ForKeyword,
                    "proc" => NimSyntaxKind::ProcKeyword,
                    "func" => NimSyntaxKind::FuncKeyword,
                    "var" => NimSyntaxKind::VarKeyword,
                    "let" => NimSyntaxKind::LetKeyword,
                    "const" => NimSyntaxKind::ConstKeyword,
                    "type" => NimSyntaxKind::TypeKeyword,
                    "import" => NimSyntaxKind::ImportKeyword,
                    "from" => NimSyntaxKind::FromKeyword,
                    "include" => NimSyntaxKind::IncludeKeyword,
                    "return" => NimSyntaxKind::ReturnKeyword,
                    "yield" => NimSyntaxKind::YieldKeyword,
                    "break" => NimSyntaxKind::BreakKeyword,
                    "continue" => NimSyntaxKind::ContinueKeyword,
                    "try" => NimSyntaxKind::TryKeyword,
                    "except" => NimSyntaxKind::ExceptKeyword,
                    "finally" => NimSyntaxKind::FinallyKeyword,
                    "raise" => NimSyntaxKind::RaiseKeyword,
                    "case" => NimSyntaxKind::CaseKeyword,
                    "of" => NimSyntaxKind::OfKeyword,
                    "when" => NimSyntaxKind::WhenKeyword,
                    "is" => NimSyntaxKind::IsKeyword,
                    "in" => NimSyntaxKind::InKeyword,
                    "nil" => NimSyntaxKind::NilKeyword,
                    _ => NimSyntaxKind::Identifier,
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

    /// 处理操作符
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Plus, start_pos, state.get_position());
                    true
                }
                '-' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Minus, start_pos, state.get_position());
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Star, start_pos, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Slash, start_pos, state.get_position());
                    true
                }
                '=' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::EqualEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Equal, start_pos, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::NotEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Exclamation, start_pos, state.get_position());
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::LessEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Less, start_pos, state.get_position());
                    }
                    true
                }
                '>' => {
                    state.advance(1);
                    if state.peek() == Some('=') {
                        state.advance(1);
                        state.add_token(NimSyntaxKind::GreaterEqual, start_pos, state.get_position());
                    }
                    else {
                        state.add_token(NimSyntaxKind::Greater, start_pos, state.get_position());
                    }
                    true
                }
                '(' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftParen, start_pos, state.get_position());
                    true
                }
                ')' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightParen, start_pos, state.get_position());
                    true
                }
                '[' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftBracket, start_pos, state.get_position());
                    true
                }
                ']' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightBracket, start_pos, state.get_position());
                    true
                }
                '{' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::LeftBrace, start_pos, state.get_position());
                    true
                }
                '}' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::RightBrace, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Comma, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Semicolon, start_pos, state.get_position());
                    true
                }
                ':' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Colon, start_pos, state.get_position());
                    true
                }
                '.' => {
                    state.advance(1);
                    state.add_token(NimSyntaxKind::Dot, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<NimLanguage> for NimLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<NimLanguage>,
    ) -> LexOutput<NimLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);

        while state.not_at_end() {
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

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，添加错误 kind
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(NimSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(NimSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}
