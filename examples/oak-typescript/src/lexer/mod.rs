use crate::{kind::TypeScriptSyntaxKind, language::TypeScriptLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, TypeScriptLanguage>;

pub struct TypeScriptLexer<'config> {
    config: &'config TypeScriptLanguage,
}

impl<'config> TypeScriptLexer<'config> {
    pub fn new(config: &'config TypeScriptLanguage) -> Self {
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
            state.add_token(TypeScriptSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(TypeScriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(TypeScriptSyntaxKind::Newline, start_pos, state.get_position());
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

                // 读取到行

                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(TypeScriptSyntaxKind::LineComment, start_pos, state.get_position());
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
                    // 未终止的注释，添加错

                    state.add_error(source.syntax_error("Unterminated comment", start_pos));
                }

                state.add_token(TypeScriptSyntaxKind::BlockComment, start_pos, state.get_position());
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
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if !found_end {
                    state.add_error(source.syntax_error("Unterminated string literal", start_pos));
                }

                state.add_token(TypeScriptSyntaxKind::StringLiteral, start_pos, state.get_position());
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

            state.add_token(TypeScriptSyntaxKind::TemplateString, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理正则表达式字面量
    fn lex_regex_literal(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            // 检查是否是注释
            if let Some(next) = source.get_char_at(start_pos + 1) {
                if next == '/' || next == '*' {
                    return false; // 让注释处理函数处
                }
            }

            // 简单的正则表达式检测（实际实现需要更复杂的上下文分析

            state.advance(1);

            let mut found_end = false;
            while let Some(ch) = state.peek() {
                if ch == '/' {
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
                    break; // 正则表达式不能跨
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 处理正则表达式标

            if found_end {
                while let Some(flag) = state.peek() {
                    if flag.is_alphabetic() {
                        state.advance(flag.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }

            if !found_end {
                state.add_error(source.syntax_error("Unterminated regex literal", start_pos));
            }

            state.add_token(TypeScriptSyntaxKind::RegexLiteral, start_pos, state.get_position());
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
                    state.add_token(TypeScriptSyntaxKind::BigIntLiteral, start_pos, state.get_position());
                }
                else {
                    state.add_token(TypeScriptSyntaxKind::NumericLiteral, start_pos, state.get_position());
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

    fn keyword_or_identifier(&self, text: &str) -> TypeScriptSyntaxKind {
        match text {
            "abstract" => TypeScriptSyntaxKind::Abstract,
            "any" => TypeScriptSyntaxKind::Any,
            "as" => TypeScriptSyntaxKind::As,
            "asserts" => TypeScriptSyntaxKind::Asserts,
            "async" => TypeScriptSyntaxKind::Async,
            "await" => TypeScriptSyntaxKind::Await,
            "boolean" => TypeScriptSyntaxKind::Boolean,
            "break" => TypeScriptSyntaxKind::Break,
            "case" => TypeScriptSyntaxKind::Case,
            "catch" => TypeScriptSyntaxKind::Catch,
            "class" => TypeScriptSyntaxKind::Class,
            "const" => TypeScriptSyntaxKind::Const,
            "constructor" => TypeScriptSyntaxKind::Constructor,
            "continue" => TypeScriptSyntaxKind::Continue,
            "debugger" => TypeScriptSyntaxKind::Debugger,
            "declare" => TypeScriptSyntaxKind::Declare,
            "default" => TypeScriptSyntaxKind::Default,
            "delete" => TypeScriptSyntaxKind::Delete,
            "do" => TypeScriptSyntaxKind::Do,
            "else" => TypeScriptSyntaxKind::Else,
            "enum" => TypeScriptSyntaxKind::Enum,
            "export" => TypeScriptSyntaxKind::Export,
            "extends" => TypeScriptSyntaxKind::Extends,
            "false" => TypeScriptSyntaxKind::False,
            "finally" => TypeScriptSyntaxKind::Finally,
            "for" => TypeScriptSyntaxKind::For,
            "from" => TypeScriptSyntaxKind::From,
            "function" => TypeScriptSyntaxKind::Function,
            "get" => TypeScriptSyntaxKind::Get,
            "global" => TypeScriptSyntaxKind::Global,
            "if" => TypeScriptSyntaxKind::If,
            "implements" => TypeScriptSyntaxKind::Implements,
            "import" => TypeScriptSyntaxKind::Import,
            "in" => TypeScriptSyntaxKind::In,
            "infer" => TypeScriptSyntaxKind::Infer,
            "instanceof" => TypeScriptSyntaxKind::Instanceof,
            "interface" => TypeScriptSyntaxKind::Interface,
            "is" => TypeScriptSyntaxKind::Is,
            "keyof" => TypeScriptSyntaxKind::Keyof,
            "let" => TypeScriptSyntaxKind::Let,
            "module" => TypeScriptSyntaxKind::Module,
            "namespace" => TypeScriptSyntaxKind::Namespace,
            "never" => TypeScriptSyntaxKind::Never,
            "new" => TypeScriptSyntaxKind::New,
            "null" => TypeScriptSyntaxKind::Null,
            "number" => TypeScriptSyntaxKind::Number,
            "object" => TypeScriptSyntaxKind::Object,
            "of" => TypeScriptSyntaxKind::Of,
            "package" => TypeScriptSyntaxKind::Package,
            "private" => TypeScriptSyntaxKind::Private,
            "protected" => TypeScriptSyntaxKind::Protected,
            "public" => TypeScriptSyntaxKind::Public,
            "readonly" => TypeScriptSyntaxKind::Readonly,
            "require" => TypeScriptSyntaxKind::Require,
            "return" => TypeScriptSyntaxKind::Return,
            "set" => TypeScriptSyntaxKind::Set,
            "static" => TypeScriptSyntaxKind::Static,
            "string" => TypeScriptSyntaxKind::String,
            "super" => TypeScriptSyntaxKind::Super,
            "switch" => TypeScriptSyntaxKind::Switch,
            "symbol" => TypeScriptSyntaxKind::Symbol,
            "this" => TypeScriptSyntaxKind::This,
            "throw" => TypeScriptSyntaxKind::Throw,
            "true" => TypeScriptSyntaxKind::True,
            "try" => TypeScriptSyntaxKind::Try,
            "type" => TypeScriptSyntaxKind::Type,
            "typeof" => TypeScriptSyntaxKind::Typeof,
            "undefined" => TypeScriptSyntaxKind::Undefined,
            "unique" => TypeScriptSyntaxKind::Unique,
            "unknown" => TypeScriptSyntaxKind::Unknown,
            "var" => TypeScriptSyntaxKind::Var,
            "void" => TypeScriptSyntaxKind::Void,
            "while" => TypeScriptSyntaxKind::While,
            "with" => TypeScriptSyntaxKind::With,
            "yield" => TypeScriptSyntaxKind::Yield,
            _ => TypeScriptSyntaxKind::IdentifierName,
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
                            TypeScriptSyntaxKind::PlusPlus
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::PlusEqual
                        }
                        _ => TypeScriptSyntaxKind::Plus,
                    }
                }
                '-' => {
                    state.advance(1);
                    match state.peek() {
                        Some('-') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::MinusMinus
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::MinusEqual
                        }
                        _ => TypeScriptSyntaxKind::Minus,
                    }
                }
                '*' => {
                    state.advance(1);
                    match state.peek() {
                        Some('*') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::StarStarEqual
                            }
                            else {
                                TypeScriptSyntaxKind::StarStar
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::StarEqual
                        }
                        _ => TypeScriptSyntaxKind::Star,
                    }
                }
                '/' => {
                    // 检查是否是注释或正则表达式
                    if let Some(next) = source.get_char_at(start_pos + 1) {
                        if next == '/' || next == '*' {
                            return false; // 让注释处理函数处
                        }
                    }
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        TypeScriptSyntaxKind::SlashEqual
                    }
                    else {
                        TypeScriptSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        TypeScriptSyntaxKind::PercentEqual
                    }
                    else {
                        TypeScriptSyntaxKind::Percent
                    }
                }
                '<' => {
                    state.advance(1);
                    match state.peek() {
                        Some('<') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::LeftShiftEqual
                            }
                            else {
                                TypeScriptSyntaxKind::LeftShift
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::LessEqual
                        }
                        _ => TypeScriptSyntaxKind::Less,
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
                                        TypeScriptSyntaxKind::UnsignedRightShiftEqual
                                    }
                                    else {
                                        TypeScriptSyntaxKind::UnsignedRightShift
                                    }
                                }
                                Some('=') => {
                                    state.advance(1);
                                    TypeScriptSyntaxKind::RightShiftEqual
                                }
                                _ => TypeScriptSyntaxKind::RightShift,
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::GreaterEqual
                        }
                        _ => TypeScriptSyntaxKind::Greater,
                    }
                }
                '=' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::EqualEqualEqual
                            }
                            else {
                                TypeScriptSyntaxKind::EqualEqual
                            }
                        }
                        Some('>') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::Arrow
                        }
                        _ => TypeScriptSyntaxKind::Equal,
                    }
                }
                '!' => {
                    state.advance(1);
                    match state.peek() {
                        Some('=') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::NotEqualEqual
                            }
                            else {
                                TypeScriptSyntaxKind::NotEqual
                            }
                        }
                        _ => TypeScriptSyntaxKind::Exclamation,
                    }
                }
                '&' => {
                    state.advance(1);
                    match state.peek() {
                        Some('&') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::AmpersandAmpersandEqual
                            }
                            else {
                                TypeScriptSyntaxKind::AmpersandAmpersand
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::AmpersandEqual
                        }
                        _ => TypeScriptSyntaxKind::Ampersand,
                    }
                }
                '|' => {
                    state.advance(1);
                    match state.peek() {
                        Some('|') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::PipePipeEqual
                            }
                            else {
                                TypeScriptSyntaxKind::PipePipe
                            }
                        }
                        Some('=') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::PipeEqual
                        }
                        _ => TypeScriptSyntaxKind::Pipe,
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        TypeScriptSyntaxKind::CaretEqual
                    }
                    else {
                        TypeScriptSyntaxKind::Caret
                    }
                }
                '~' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::Tilde
                }
                '?' => {
                    state.advance(1);
                    match state.peek() {
                        Some('?') => {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                TypeScriptSyntaxKind::QuestionQuestionEqual
                            }
                            else {
                                TypeScriptSyntaxKind::QuestionQuestion
                            }
                        }
                        Some('.') => {
                            state.advance(1);
                            TypeScriptSyntaxKind::QuestionDot
                        }
                        _ => TypeScriptSyntaxKind::Question,
                    }
                }
                '(' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        if let Some('.') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            TypeScriptSyntaxKind::DotDotDot
                        }
                        else {
                            TypeScriptSyntaxKind::Dot
                        }
                    }
                    else {
                        TypeScriptSyntaxKind::Dot
                    }
                }
                ':' => {
                    state.advance(1);
                    TypeScriptSyntaxKind::Colon
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

impl<'config> Lexer<TypeScriptLanguage> for TypeScriptLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<TypeScriptSyntaxKind> {
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
                state.add_token(TypeScriptSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(TypeScriptSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
