use crate::{kind::PythonSyntaxKind, language::PythonLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::LexOutput,
    source::{Source, TextEdit},
};

type State<'a, S> = LexerState<'a, S, PythonLanguage>;

#[derive(Clone)]
pub struct PythonLexer<'config> {
    _config: &'config PythonLanguage,
}

impl<'config> PythonLexer<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { _config: config }
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(PythonSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.current() {
            state.advance(1);
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.current() {
            state.advance(1);
            if let Some('\n') = state.current() {
                state.advance(1);
            }
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some('#') = state.current() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行尾
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(PythonSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开始
        let quote_char = match state.current() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => return false,
        };

        state.advance(1); // 跳过开始引号

        // 检查是否是三引号字符串 - 简化实现，不支持三引号
        let mut escaped = false;
        while let Some(ch) = state.current() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8());
                continue;
            }

            if ch == '\\' {
                escaped = true;
                state.advance(1);
                continue;
            }

            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                break;
            }
            else if ch == '\n' || ch == '\r' {
                // 单行字符串不能包含换行符
                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(PythonSyntaxKind::String, start_pos, state.get_position());
        true
    }

    /// 处理数字字面量
    fn lex_number<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        // 简化实现：只处理基本的十进制数字
        while let Some(ch) = state.current() {
            if ch.is_ascii_digit() || ch == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(PythonSyntaxKind::Number, start_pos, state.get_position());
        true
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 检查第一个字符
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // 读取标识符
        let mut text = String::new();
        while let Some(ch) = state.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                text.push(ch);
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        // 检查是否是关键字
        let kind = match text.as_str() {
            "and" => PythonSyntaxKind::AndKeyword,
            "as" => PythonSyntaxKind::AsKeyword,
            "assert" => PythonSyntaxKind::AssertKeyword,
            "async" => PythonSyntaxKind::AsyncKeyword,
            "await" => PythonSyntaxKind::AwaitKeyword,
            "break" => PythonSyntaxKind::BreakKeyword,
            "class" => PythonSyntaxKind::ClassKeyword,
            "continue" => PythonSyntaxKind::ContinueKeyword,
            "def" => PythonSyntaxKind::DefKeyword,
            "del" => PythonSyntaxKind::DelKeyword,
            "elif" => PythonSyntaxKind::ElifKeyword,
            "else" => PythonSyntaxKind::ElseKeyword,
            "except" => PythonSyntaxKind::ExceptKeyword,
            "False" => PythonSyntaxKind::FalseKeyword,
            "finally" => PythonSyntaxKind::FinallyKeyword,
            "for" => PythonSyntaxKind::ForKeyword,
            "from" => PythonSyntaxKind::FromKeyword,
            "global" => PythonSyntaxKind::GlobalKeyword,
            "if" => PythonSyntaxKind::IfKeyword,
            "import" => PythonSyntaxKind::ImportKeyword,
            "in" => PythonSyntaxKind::InKeyword,
            "is" => PythonSyntaxKind::IsKeyword,
            "lambda" => PythonSyntaxKind::LambdaKeyword,
            "None" => PythonSyntaxKind::NoneKeyword,
            "nonlocal" => PythonSyntaxKind::NonlocalKeyword,
            "not" => PythonSyntaxKind::NotKeyword,
            "or" => PythonSyntaxKind::OrKeyword,
            "pass" => PythonSyntaxKind::PassKeyword,
            "raise" => PythonSyntaxKind::RaiseKeyword,
            "return" => PythonSyntaxKind::ReturnKeyword,
            "True" => PythonSyntaxKind::TrueKeyword,
            "try" => PythonSyntaxKind::TryKeyword,
            "while" => PythonSyntaxKind::WhileKeyword,
            "with" => PythonSyntaxKind::WithKeyword,
            "yield" => PythonSyntaxKind::YieldKeyword,
            _ => PythonSyntaxKind::Identifier,
        };

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作符
    fn lex_operator<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        // 简化实现：只处理单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    PythonSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    PythonSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    PythonSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    PythonSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    PythonSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    PythonSyntaxKind::Assign
                }
                '<' => {
                    state.advance(1);
                    PythonSyntaxKind::Less
                }
                '>' => {
                    state.advance(1);
                    PythonSyntaxKind::Greater
                }
                '&' => {
                    state.advance(1);
                    PythonSyntaxKind::Ampersand
                }
                '|' => {
                    state.advance(1);
                    PythonSyntaxKind::Pipe
                }
                '^' => {
                    state.advance(1);
                    PythonSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    PythonSyntaxKind::Tilde
                }
                '@' => {
                    state.advance(1);
                    PythonSyntaxKind::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理分隔符
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => PythonSyntaxKind::LeftParen,
                ')' => PythonSyntaxKind::RightParen,
                '[' => PythonSyntaxKind::LeftBracket,
                ']' => PythonSyntaxKind::RightBracket,
                '{' => PythonSyntaxKind::LeftBrace,
                '}' => PythonSyntaxKind::RightBrace,
                ',' => PythonSyntaxKind::Comma,
                ':' => PythonSyntaxKind::Colon,
                ';' => PythonSyntaxKind::Semicolon,
                '.' => PythonSyntaxKind::Dot, // 简化处理，不支持省略号
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }
}

impl<'config> Lexer<PythonLanguage> for PythonLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<PythonLanguage>) -> LexOutput<PythonLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> PythonLexer<'config> {
    pub(crate) fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let mut indent_stack = vec![0];
        let mut bracket_level: usize = 0;
        let mut at_line_start = true;

        while state.not_at_end() {
            let safe_point = state.get_position();

            if at_line_start && bracket_level == 0 {
                self.handle_indentation(state, &mut indent_stack);
                at_line_start = false;
                continue;
            }

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                        at_line_start = true;
                    }
                    '#' => {
                        self.lex_comment(state);
                    }
                    '"' | '\'' => {
                        self.lex_string(state);
                    }
                    '0'..='9' => {
                        self.lex_number(state);
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier_or_keyword(state);
                    }
                    '(' | '[' | '{' => {
                        bracket_level += 1;
                        self.lex_delimiter(state);
                    }
                    ')' | ']' | '}' => {
                        bracket_level = bracket_level.saturating_sub(1);
                        self.lex_delimiter(state);
                    }
                    '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '&' | '|' | '^' | '~' | '@' => {
                        self.lex_operator(state);
                    }
                    ',' | ':' | ';' | '.' => {
                        self.lex_delimiter(state);
                    }
                    _ => {
                        // Fallback to error
                        state.advance(ch.len_utf8());
                        state.add_token(PythonSyntaxKind::Error, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }

        // Emit remaining dedents
        while indent_stack.len() > 1 {
            indent_stack.pop();
            let pos = state.get_position();
            state.add_token(PythonSyntaxKind::Dedent, pos, pos);
        }

        Ok(())
    }

    fn handle_indentation<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, stack: &mut Vec<usize>) {
        let start_pos = state.get_position();
        let current_indent;

        // Skip comments and empty lines at start of line
        let mut temp_state = state.get_position();
        loop {
            let mut indent = 0;
            while let Some(ch) = state.get_char_at(temp_state) {
                if ch == ' ' {
                    indent += 1;
                }
                else if ch == '\t' {
                    indent += 8;
                }
                // Standard Python tab width
                else {
                    break;
                }
                temp_state += 1;
            }

            match state.get_char_at(temp_state) {
                Some('\n') | Some('\r') | Some('#') => {
                    // This is an empty line or comment-only line, ignore indentation change
                    return;
                }
                None => return, // EOF
                _ => {
                    current_indent = indent;
                    break;
                }
            }
        }

        // Advance state to skip the indentation we just measured
        if current_indent > 0 {
            let end_pos = state.get_position() + (temp_state - state.get_position());
            state.add_token(PythonSyntaxKind::Whitespace, start_pos, end_pos);
            state.set_position(end_pos);
        }

        let last_indent = *stack.last().unwrap();
        if current_indent > last_indent {
            stack.push(current_indent);
            state.add_token(PythonSyntaxKind::Indent, state.get_position(), state.get_position());
        }
        else {
            while current_indent < *stack.last().unwrap() {
                stack.pop();
                state.add_token(PythonSyntaxKind::Dedent, state.get_position(), state.get_position());
            }
            // If current_indent doesn't match any previous level, it's an indentation error,
            // but for now we just stop at the closest level.
        }
    }
}
