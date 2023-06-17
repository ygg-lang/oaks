use crate::{kind::JavaScriptSyntaxKind, language::JavaScriptLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, JavaScriptLanguage>;

pub struct JavaScriptLexer<'config> {
    config: &'config JavaScriptLanguage,
}

impl<'config> JavaScriptLexer<'config> {
    pub fn new(config: &'config JavaScriptLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
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
            state.add_token(JavaScriptSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(JavaScriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(JavaScriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注
    fn lex_line_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2); // 跳过 //

                // 读取到行结束符之前的所有字符
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(JavaScriptSyntaxKind::LineComment, start_pos, state.get_position());
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

    /// 处理块注
    fn lex_block_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2); // 跳过 /*

                let mut found_end = false;
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2); // 跳过 */
                            found_end = true;
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                if !found_end {
                    // 未终止的注释，添加错误
                    state.add_error(source.syntax_error("Unterminated comment", start_pos));
                }

                state.add_token(JavaScriptSyntaxKind::BlockComment, start_pos, state.get_position());
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
    fn lex_string_literal(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);

                let mut found_end = false;
                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // 处理转义字符
                        state.advance(1);
                        if let Some(escaped) = state.peek() {
                            state.advance(escaped.len_utf8());
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨多行
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if !found_end {
                    state.add_error(source.syntax_error("Unterminated string literal", start_pos));
                }

                state.add_token(JavaScriptSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理模板字符
    fn lex_template_literal(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);

            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\\' {
                    // 处理转义字符
                    state.advance(1);
                    if let Some(escaped) = state.peek() {
                        state.advance(escaped.len_utf8());
                    }
                }
                else if ch == '$' {
                    if let Some('{') = source.get_char_at(state.get_position() + 1) {
                        // 模板表达式，暂时跳过
                        state.advance(2);
                        let mut brace_count = 1;
                        while let Some(inner_ch) = state.peek() {
                            if inner_ch == '{' {
                                brace_count += 1;
                            }
                            else if inner_ch == '}' {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    state.advance(1);
                                    break;
                                }
                            }
                            state.advance(inner_ch.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if !found_end {
                state.add_error(source.syntax_error("Unterminated template literal", start_pos));
            }

            state.add_token(JavaScriptSyntaxKind::TemplateString, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_numeric_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '.' && self.is_next_digit(state)) {
                // 处理整数部分
                if ch != '.' {
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理小数部分
                if let Some('.') = state.peek() {
                    state.advance(1);
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }

                // 处理指数部分
                if let Some(exp) = state.peek() {
                    if exp == 'e' || exp == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
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

                // 检BigInt 后缀
                if let Some('n') = state.peek() {
                    state.advance(1);
                    state.add_token(JavaScriptSyntaxKind::BigIntLiteral, start_pos, state.get_position());
                }
                else {
                    state.add_token(JavaScriptSyntaxKind::NumericLiteral, start_pos, state.get_position());
                }
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

    /// 检查下一个字符是否是数字
    fn is_next_digit(&self, state: &State) -> bool {
        if let Some(next_ch) = state.peek_next_n(1) { next_ch.is_ascii_digit() } else { false }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(next_ch) = state.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' || next_ch == '$' {
                        state.advance(next_ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = self.keyword_or_identifier(&text);
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

    /// 判断是关键字还是标识
    fn keyword_or_identifier(&self, text: &str) -> JavaScriptSyntaxKind {
        match text {
            "abstract" => JavaScriptSyntaxKind::Abstract,
            "as" => JavaScriptSyntaxKind::As,
            "async" => JavaScriptSyntaxKind::Async,
            "await" => JavaScriptSyntaxKind::Await,
            "break" => JavaScriptSyntaxKind::Break,
            "case" => JavaScriptSyntaxKind::Case,
            "catch" => JavaScriptSyntaxKind::Catch,
            "class" => JavaScriptSyntaxKind::Class,
            "const" => JavaScriptSyntaxKind::Const,
            "continue" => JavaScriptSyntaxKind::Continue,
            "debugger" => JavaScriptSyntaxKind::Debugger,
            "default" => JavaScriptSyntaxKind::Default,
            "delete" => JavaScriptSyntaxKind::Delete,
            "do" => JavaScriptSyntaxKind::Do,
            "else" => JavaScriptSyntaxKind::Else,
            "enum" => JavaScriptSyntaxKind::Enum,
            "export" => JavaScriptSyntaxKind::Export,
            "extends" => JavaScriptSyntaxKind::Extends,
            "false" => JavaScriptSyntaxKind::False,
            "finally" => JavaScriptSyntaxKind::Finally,
            "for" => JavaScriptSyntaxKind::For,
            "function" => JavaScriptSyntaxKind::Function,
            "if" => JavaScriptSyntaxKind::If,
            "implements" => JavaScriptSyntaxKind::Implements,
            "import" => JavaScriptSyntaxKind::Import,
            "in" => JavaScriptSyntaxKind::In,
            "instanceof" => JavaScriptSyntaxKind::Instanceof,
            "interface" => JavaScriptSyntaxKind::Interface,
            "let" => JavaScriptSyntaxKind::Let,
            "new" => JavaScriptSyntaxKind::New,
            "null" => JavaScriptSyntaxKind::Null,
            "package" => JavaScriptSyntaxKind::Package,
            "private" => JavaScriptSyntaxKind::Private,
            "protected" => JavaScriptSyntaxKind::Protected,
            "public" => JavaScriptSyntaxKind::Public,
            "return" => JavaScriptSyntaxKind::Return,
            "static" => JavaScriptSyntaxKind::Static,
            "super" => JavaScriptSyntaxKind::Super,
            "switch" => JavaScriptSyntaxKind::Switch,
            "this" => JavaScriptSyntaxKind::This,
            "throw" => JavaScriptSyntaxKind::Throw,
            "true" => JavaScriptSyntaxKind::True,
            "try" => JavaScriptSyntaxKind::Try,
            "typeof" => JavaScriptSyntaxKind::Typeof,
            "undefined" => JavaScriptSyntaxKind::Undefined,
            "var" => JavaScriptSyntaxKind::Var,
            "void" => JavaScriptSyntaxKind::Void,
            "while" => JavaScriptSyntaxKind::While,
            "with" => JavaScriptSyntaxKind::With,
            "yield" => JavaScriptSyntaxKind::Yield,
            _ => JavaScriptSyntaxKind::IdentifierName,
        }
    }

    /// 处理操作符和标点符号
    fn lex_operator_or_punctuation(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    match state.peek() {
                        Some('+') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::PlusPlus
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::PlusEqual
                        }
                        _ => JavaScriptSyntaxKind::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::MinusMinus
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::MinusEqual
                        }
                        _ => JavaScriptSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    match state.peek() {
                        Some('*') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::StarStarEqual
                            }
                            else {
                                JavaScriptSyntaxKind::StarStar
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::StarEqual
                        }
                        _ => JavaScriptSyntaxKind::Star,
                    }
                }
                '/' => {
                    // 检查是否是注释
                    if let Some(next) = source.get_char_at(start_pos + 1) {
                        if next == '/' || next == '*' {
                            return false; // 让注释处理函数处                        
                        }
                    }
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptSyntaxKind::SlashEqual
                    }
                    else {
                        JavaScriptSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptSyntaxKind::PercentEqual
                    }
                    else {
                        JavaScriptSyntaxKind::Percent
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::LeftShiftEqual
                            }
                            else {
                                JavaScriptSyntaxKind::LeftShift
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::LessEqual
                        }
                        _ => JavaScriptSyntaxKind::Less,
                    }
                }
                '>' => {
                    state.advance(1);
                    match state.peek() {
                        Some('>') => {
                            state.advance(1);
                            match state.peek() {
                                Some('>') => {
                                    state.advance(1);
                                    if let Some('=') = state.peek() {
                                        state.advance(1);
                                        JavaScriptSyntaxKind::UnsignedRightShiftEqual
                                    }
                                    else {
                                        JavaScriptSyntaxKind::UnsignedRightShift
                                    }
                                }
                                Some('=') => {
                                    state.advance(1);
                                    JavaScriptSyntaxKind::RightShiftEqual
                                }
                                _ => JavaScriptSyntaxKind::RightShift,
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::GreaterEqual
                        }
                        _ => JavaScriptSyntaxKind::Greater,
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::EqualEqualEqual
                            }
                            else {
                                JavaScriptSyntaxKind::EqualEqual
                            }
                        }
                        Some('>') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::Arrow
                        }
                        _ => JavaScriptSyntaxKind::Equal,
                    }
                }
                '!' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::NotEqualEqual
                            }
                            else {
                                JavaScriptSyntaxKind::NotEqual
                            }
                        }
                        _ => JavaScriptSyntaxKind::Exclamation,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::AmpersandAmpersandEqual
                            }
                            else {
                                JavaScriptSyntaxKind::AmpersandAmpersand
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::AmpersandEqual
                        }
                        _ => JavaScriptSyntaxKind::Ampersand,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::PipePipeEqual
                            }
                            else {
                                JavaScriptSyntaxKind::PipePipe
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::PipeEqual
                        }
                        _ => JavaScriptSyntaxKind::Pipe,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        JavaScriptSyntaxKind::CaretEqual
                    }
                    else {
                        JavaScriptSyntaxKind::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::Tilde
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                JavaScriptSyntaxKind::QuestionQuestionEqual
                            }
                            else {
                                JavaScriptSyntaxKind::QuestionQuestion
                            }
                        }
                        Some('.') => {
                            state.advance(1);
                            JavaScriptSyntaxKind::QuestionDot
                        }
                        _ => JavaScriptSyntaxKind::Question,
                    }
                }
                '(' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        if let Some('.') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            JavaScriptSyntaxKind::DotDotDot
                        }
                        else {
                            JavaScriptSyntaxKind::Dot
                        }
                    }
                    else {
                        JavaScriptSyntaxKind::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    JavaScriptSyntaxKind::Colon
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

impl<'config> Lexer<JavaScriptLanguage> for JavaScriptLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<JavaScriptSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state, source) {
                continue;
            }

            if self.lex_block_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state, source) {
                continue;
            }

            if self.lex_template_literal(&mut state, source) {
                continue;
            }

            if self.lex_numeric_literal(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_punctuation(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(JavaScriptSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(JavaScriptSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
