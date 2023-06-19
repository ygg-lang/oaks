use crate::{kind::PowerShellSyntaxKind, language::PowerShellLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, PowerShellLanguage>;

#[derive(Clone)]
pub struct PowerShellLexer<'config> {
    config: &'config PowerShellLanguage,
}

impl<'config> PowerShellLexer<'config> {
    pub fn new(config: &'config PowerShellLanguage) -> Self {
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

            if self.lex_variable(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators_and_punctuation(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PowerShellSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }
        }

        // Add EOF token
        let pos = state.get_position();
        state.add_token(PowerShellSyntaxKind::Eof, pos, pos);

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
            state.add_token(PowerShellSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(PowerShellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PowerShellSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);
            // 单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PowerShellSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else if let Some('<') = state.peek() {
            state.advance(1);
            if let Some('#') = state.peek() {
                state.advance(1);
                // 多行注释 <# ... #>
                let mut depth = 1;
                while let Some(ch) = state.peek()
                    && depth > 0
                {
                    if ch == '<' {
                        state.advance(1);
                        if let Some('#') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if ch == '#' {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PowerShellSyntaxKind::Comment, start_pos, state.get_position());
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
                    else if ch == '`' {
                        // PowerShell 使用反引号作为转义字符
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        break;
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 字符串可以跨行
                        state.advance(ch.len_utf8());
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PowerShellSyntaxKind::StringLiteral, start_pos, state.get_position());
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

                state.add_token(PowerShellSyntaxKind::NumberLiteral, start_pos, state.get_position());
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
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 变量名必须以字母或下划线开头
            if let Some(ch) = state.peek() {
                if ch.is_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());

                    // 后续字符可以是字母、数字或下划线
                    while let Some(ch) = state.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(PowerShellSyntaxKind::Variable, start_pos, state.get_position());
                    true
                }
                else {
                    // 只有 $ 符号，作为操作符处理
                    state.add_token(PowerShellSyntaxKind::Dollar, start_pos, state.get_position());
                    true
                }
            }
            else {
                state.add_token(PowerShellSyntaxKind::Dollar, start_pos, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "begin" => PowerShellSyntaxKind::Begin,
                    "break" => PowerShellSyntaxKind::Break,
                    "catch" => PowerShellSyntaxKind::Catch,
                    "class" => PowerShellSyntaxKind::Class,
                    "continue" => PowerShellSyntaxKind::Continue,
                    "data" => PowerShellSyntaxKind::Data,
                    "define" => PowerShellSyntaxKind::Define,
                    "do" => PowerShellSyntaxKind::Do,
                    "dynamicparam" => PowerShellSyntaxKind::DynamicParam,
                    "else" => PowerShellSyntaxKind::Else,
                    "elseif" => PowerShellSyntaxKind::ElseIf,
                    "end" => PowerShellSyntaxKind::End,
                    "exit" => PowerShellSyntaxKind::Exit,
                    "filter" => PowerShellSyntaxKind::Filter,
                    "finally" => PowerShellSyntaxKind::Finally,
                    "for" => PowerShellSyntaxKind::For,
                    "foreach" => PowerShellSyntaxKind::ForEach,
                    "from" => PowerShellSyntaxKind::From,
                    "function" => PowerShellSyntaxKind::Function,
                    "if" => PowerShellSyntaxKind::If,
                    "in" => PowerShellSyntaxKind::In,
                    "param" => PowerShellSyntaxKind::Param,
                    "process" => PowerShellSyntaxKind::Process,
                    "return" => PowerShellSyntaxKind::Return,
                    "switch" => PowerShellSyntaxKind::Switch,
                    "throw" => PowerShellSyntaxKind::Throw,
                    "trap" => PowerShellSyntaxKind::Trap,
                    "try" => PowerShellSyntaxKind::Try,
                    "until" => PowerShellSyntaxKind::Until,
                    "using" => PowerShellSyntaxKind::Using,
                    "var" => PowerShellSyntaxKind::Var,
                    "while" => PowerShellSyntaxKind::While,
                    "workflow" => PowerShellSyntaxKind::Workflow,
                    "true" => PowerShellSyntaxKind::BooleanLiteral,
                    "false" => PowerShellSyntaxKind::BooleanLiteral,
                    "null" => PowerShellSyntaxKind::NullLiteral,
                    _ => PowerShellSyntaxKind::Identifier,
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

    fn lex_operators_and_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Plus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Minus
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Equal
                    }
                    else {
                        PowerShellSyntaxKind::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::NotEqual
                    }
                    else {
                        PowerShellSyntaxKind::Exclamation
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::LessEqual
                    }
                    else {
                        PowerShellSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::GreaterEqual
                    }
                    else {
                        PowerShellSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::And
                    }
                    else {
                        PowerShellSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::Or
                    }
                    else {
                        PowerShellSyntaxKind::Pipe
                    }
                }
                '^' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Xor
                }
                '~' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Not
                }
                '?' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::DoubleColon
                    }
                    else {
                        PowerShellSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        PowerShellSyntaxKind::DotDot
                    }
                    else {
                        PowerShellSyntaxKind::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PowerShellSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PowerShellSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    PowerShellSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PowerShellSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PowerShellSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PowerShellSyntaxKind::RightBrace
                }
                '@' => {
                    state.advance(1);
                    PowerShellSyntaxKind::At
                }
                '`' => {
                    state.advance(1);
                    PowerShellSyntaxKind::Backtick
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

impl<'config> Lexer<PowerShellLanguage> for PowerShellLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<PowerShellLanguage>,
    ) -> LexOutput<PowerShellLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
