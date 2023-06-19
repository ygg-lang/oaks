use crate::{kind::PrologSyntaxKind, language::PrologLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, PrologLanguage>;

#[derive(Clone)]
pub struct PrologLexer<'config> {
    config: &'config PrologLanguage,
}

impl<'config> PrologLexer<'config> {
    pub fn new(config: &'config PrologLanguage) -> Self {
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

            if self.lex_string(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_atom_or_keyword(state) {
                continue;
            }

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PrologSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }
        }

        // Add EOF token
        let pos = state.get_position();
        state.add_token(PrologSyntaxKind::Eof, pos, pos);

        Ok(())
    }

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
            state.add_token(PrologSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(PrologSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PrologSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);
            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PrologSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);
                // 多行注释 /* ... */
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            break;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PrologSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引号

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
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串不能跨行
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PrologSyntaxKind::String, start_pos, state.get_position());
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

    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                let start_pos = state.get_position();

                // 读取整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    state.advance(1);
                    // 读取小数部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 检查科学记数法
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
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
                }

                state.add_token(PrologSyntaxKind::Integer, start_pos, state.get_position());
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

    fn lex_atom_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_lowercase() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取原子
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "is" => PrologSyntaxKind::Is,
                    "mod" => PrologSyntaxKind::Modulo,
                    _ => PrologSyntaxKind::Atom,
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

    fn lex_variable<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_ascii_uppercase() || ch == '_' {
                let start_pos = state.get_position();

                // 读取变量名
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(PrologSyntaxKind::Variable, start_pos, state.get_position());
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

    fn lex_operators_and_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    PrologSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    PrologSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::Power
                    }
                    else {
                        PrologSyntaxKind::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::IntDivide
                    }
                    else {
                        PrologSyntaxKind::Divide
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::Equal
                    }
                    else if let Some(':') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologSyntaxKind::ArithEqual
                        }
                        else {
                            // 回退
                            state.set_position(start_pos + 1);
                            PrologSyntaxKind::Unify
                        }
                    }
                    else if let Some('\\') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologSyntaxKind::NotUnify
                        }
                        else {
                            // 回退
                            state.set_position(start_pos + 1);
                            PrologSyntaxKind::Unify
                        }
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::ArithNotEqual
                    }
                    else {
                        PrologSyntaxKind::Unify
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::LessEqual
                    }
                    else {
                        PrologSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::GreaterEqual
                    }
                    else {
                        PrologSyntaxKind::Greater
                    }
                }
                '\\' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PrologSyntaxKind::NotEqual
                        }
                        else {
                            PrologSyntaxKind::NotUnify
                        }
                    }
                    else {
                        PrologSyntaxKind::BitwiseNot
                    }
                }
                '!' => {
                    state.advance(1);
                    PrologSyntaxKind::Cut
                }
                '?' => {
                    state.advance(1);
                    PrologSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PrologSyntaxKind::ColonMinus
                    }
                    else {
                        PrologSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PrologSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PrologSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    PrologSyntaxKind::Dot
                }
                '(' => {
                    state.advance(1);
                    PrologSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PrologSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    PrologSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PrologSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PrologSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PrologSyntaxKind::RightBrace
                }
                '|' => {
                    state.advance(1);
                    PrologSyntaxKind::Pipe
                }
                '^' => {
                    state.advance(1);
                    PrologSyntaxKind::BitwiseXor
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PrologLanguage> for PrologLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<PrologLanguage>,
    ) -> LexOutput<PrologLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
