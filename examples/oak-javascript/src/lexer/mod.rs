use crate::{kind::JavaScriptSyntaxKind, language::JavaScriptLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S> = LexerState<S, JavaScriptLanguage>;

static JS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static JS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: Some('\\') });

#[cfg(feature = "lexer_debug")]
macro_rules! lex_debug { ($($arg:tt)*) => { println!($($arg)*); } }
#[cfg(not(feature = "lexer_debug"))]
macro_rules! lex_debug {
    ($($arg:tt)*) => {};
}

#[derive(Clone)]
pub struct JavaScriptLexer<'config> {
    config: &'config JavaScriptLanguage,
}

impl<'config> JavaScriptLexer<'config> {
    pub fn new(config: &'config JavaScriptLanguage) -> Self {
        Self { config }
    }

    fn safe_check<S: Source>(&self, state: &State<S>) -> Result<(), OakError> {
        if state.get_position() <= state.length() {
            Ok(())
        }
        else {
            Err(OakError::custom_error(format!("Lexer out-of-bounds: pos={}, len={}", state.get_position(), state.length())))
        }
    }

    /// 主要的词法分析运行方法
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        lex_debug!("Starting lexer run, source length: {}", state.length());
        while state.not_at_end() {
            let current_pos = state.get_position();
            let current_char = state.peek();
            lex_debug!("At position {}, char: {:?}", current_pos, current_char);

            self.safe_check(state)?;

            if self.skip_whitespace(state) {
                lex_debug!("Skipped whitespace");
                continue;
            }

            if self.lex_newline(state) {
                lex_debug!("Lexed newline");
                continue;
            }

            if self.lex_comment(state) {
                lex_debug!("Lexed comment");
                continue;
            }

            if self.lex_string_literal(state) {
                lex_debug!("Lexed string literal");
                continue;
            }

            if self.lex_template_literal(state) {
                lex_debug!("Lexed template literal");
                continue;
            }

            if self.lex_numeric_literal(state) {
                lex_debug!("Lexed numeric literal");
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                lex_debug!("Lexed identifier or keyword");
                continue;
            }

            if self.lex_operator_or_punctuation(state) {
                lex_debug!("Lexed operator or punctuation");
                continue;
            }

            let start = state.get_position();
            if let Some(ch) = state.peek() {
                lex_debug!("No pattern matched for char {:?}, adding error token", ch);
                state.advance(ch.len_utf8());
                state.add_token(JavaScriptSyntaxKind::Error, start, state.get_position());
            }
            else {
                lex_debug!("Reached end of file");
                break;
            }
        }
        lex_debug!("Finished lexer run");

        let eof_pos = state.get_position();
        state.add_token(JavaScriptSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match JS_WHITESPACE.scan(state.rest(), state.get_position(), JavaScriptSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
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

    /// 处理注释（行注释和块注释）
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 行注释: // ... 直到换行
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(JavaScriptSyntaxKind::LineComment, start, state.get_position());
            return true;
        }

        // 块注释: /* ... */
        if rest.starts_with("/*") {
            state.advance(2);
            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some('/') {
                    state.advance(2);
                    found_end = true;
                    break;
                }
                state.advance(ch.len_utf8());
            }

            if !found_end {
                let error = state.syntax_error("Unterminated comment", start);
                state.add_error(error);
            }

            state.add_token(JavaScriptSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char == '"' || first_char == '\'' {
                let quote = first_char;
                state.advance(1);
                let mut found_end = false;

                while let Some(ch) = state.peek() {
                    if ch == quote {
                        state.advance(1);
                        found_end = true;
                        break;
                    }
                    else if ch == '\\' {
                        // Skip escaped character
                        state.advance(1);
                        if let Some(_escaped) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        // Strings cannot span multiple lines in JavaScript
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if !found_end {
                    let error = state.syntax_error("Unterminated string literal", start_pos);
                    state.add_error(error);
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
    fn lex_template_literal<S: Source>(&self, state: &mut State<S>) -> bool {
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
                    if let Some('{') = state.peek_next_n(1) {
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
                let error = state.syntax_error("Unterminated template literal", start_pos);
                state.add_error(error);
            }

            state.add_token(JavaScriptSyntaxKind::TemplateString, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面量
    fn lex_numeric_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            // 十六进制数字 (0x 或 0X)
            if ch == '0' {
                if let Some(next) = state.peek_next_n(1) {
                    if next == 'x' || next == 'X' {
                        state.advance(2); // 跳过 '0x'
                        let mut has_digits = false;
                        while let Some(hex_ch) = state.peek() {
                            if hex_ch.is_ascii_hexdigit() {
                                state.advance(1);
                                has_digits = true;
                            }
                            else {
                                break;
                            }
                        }

                        if !has_digits {
                            let error = state.syntax_error("Invalid hexadecimal number", start_pos);
                            state.add_error(error);
                        }

                        // 检查 BigInt 后缀
                        if let Some('n') = state.peek() {
                            state.advance(1);
                            state.add_token(JavaScriptSyntaxKind::BigIntLiteral, start_pos, state.get_position());
                        }
                        else {
                            state.add_token(JavaScriptSyntaxKind::NumericLiteral, start_pos, state.get_position());
                        }
                        return true;
                    }
                }
            }

            // 普通数字或小数
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

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 必须有数字
                        let mut has_exp_digits = false;
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                                has_exp_digits = true;
                            }
                            else {
                                break;
                            }
                        }

                        if !has_exp_digits {
                            let error = state.syntax_error("Invalid number exponent", start_pos);
                            state.add_error(error);
                        }
                    }
                }

                // 检查 BigInt 后缀
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
    fn is_next_digit<S: Source>(&self, state: &State<S>) -> bool {
        if let Some(next_ch) = state.peek_next_n(1) { next_ch.is_ascii_digit() } else { false }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
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

                let text = state.get_text_in((start_pos..state.get_position()).into());
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
    fn lex_operator_or_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
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
                    if let Some(next) = state.peek_next_n(1) {
                        if next == '/' || next == '*' {
                            return false; // 让注释处理函数处理                        
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
                        if let Some('.') = state.peek_next_n(1) {
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
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<JavaScriptLanguage>,
    ) -> LexOutput<JavaScriptLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
