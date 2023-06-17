use crate::{kind::ActionScriptSyntaxKind, language::ActionScriptLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ActionScriptLanguage>;

pub struct ActionScriptLexer<'config> {
    config: &'config ActionScriptLanguage,
}

impl<'config> ActionScriptLexer<'config> {
    pub fn new(config: &'config ActionScriptLanguage) -> Self {
        Self { config }
    }

    /// Skip whitespace characters
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
            state.add_token(ActionScriptSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ActionScriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ActionScriptSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // 单行注释
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(ActionScriptSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = state.peek() {
                // 多行注释
                state.advance(1);
                let mut found_end = false;
                while let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    if ch == '*' {
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            found_end = true;
                            break;
                        }
                    }
                }
                state.add_token(ActionScriptSyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else {
                // 不是注释，回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
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
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        // 未闭合的字符串
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 即使未闭合也当作字符串处理
                state.add_token(ActionScriptSyntaxKind::StringLiteral, start_pos, state.get_position());
                true
            }
            else if quote == '/' {
                // 简单的正则表达式检测（需要更复杂的上下文分析）
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '/' {
                        state.advance(1);
                        // 检查正则表达式标志
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_alphabetic() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(ActionScriptSyntaxKind::StringLiteral, start_pos, state.get_position());
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

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                let mut has_dot = ch == '.';

                if has_dot {
                    state.advance(1);
                    // 确保点后面有数字
                    if let Some(next_ch) = state.peek() {
                        if !next_ch.is_ascii_digit() {
                            state.set_position(start_pos);
                            return false;
                        }
                    }
                    else {
                        state.set_position(start_pos);
                        return false;
                    }
                }

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if ch == '.' && !has_dot {
                        has_dot = true;
                        state.advance(1);
                    }
                    else if ch == 'e' || ch == 'E' {
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
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
                        break;
                    }
                    else {
                        break;
                    }
                }

                state.add_token(ActionScriptSyntaxKind::NumberLiteral, start_pos, state.get_position());
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
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let kind = match text {
                    "as" => ActionScriptSyntaxKind::As,
                    "break" => ActionScriptSyntaxKind::Break,
                    "case" => ActionScriptSyntaxKind::Case,
                    "catch" => ActionScriptSyntaxKind::Catch,
                    "class" => ActionScriptSyntaxKind::Class,
                    "const" => ActionScriptSyntaxKind::Const,
                    "continue" => ActionScriptSyntaxKind::Continue,
                    "default" => ActionScriptSyntaxKind::Default,
                    "delete" => ActionScriptSyntaxKind::Delete,
                    "do" => ActionScriptSyntaxKind::Do,
                    "else" => ActionScriptSyntaxKind::Else,
                    "extends" => ActionScriptSyntaxKind::Extends,
                    "false" => ActionScriptSyntaxKind::False,
                    "finally" => ActionScriptSyntaxKind::Finally,
                    "for" => ActionScriptSyntaxKind::For,
                    "function" => ActionScriptSyntaxKind::Function,
                    "if" => ActionScriptSyntaxKind::If,
                    "implements" => ActionScriptSyntaxKind::Implements,
                    "import" => ActionScriptSyntaxKind::Import,
                    "in" => ActionScriptSyntaxKind::In,
                    "instanceof" => ActionScriptSyntaxKind::Instanceof,
                    "interface" => ActionScriptSyntaxKind::Interface,
                    "internal" => ActionScriptSyntaxKind::Internal,
                    "is" => ActionScriptSyntaxKind::Is,
                    "native" => ActionScriptSyntaxKind::Native,
                    "new" => ActionScriptSyntaxKind::New,
                    "null" => ActionScriptSyntaxKind::Null,
                    "package" => ActionScriptSyntaxKind::Package,
                    "private" => ActionScriptSyntaxKind::Private,
                    "protected" => ActionScriptSyntaxKind::Protected,
                    "public" => ActionScriptSyntaxKind::Public,
                    "return" => ActionScriptSyntaxKind::Return,
                    "static" => ActionScriptSyntaxKind::Static,
                    "super" => ActionScriptSyntaxKind::Super,
                    "switch" => ActionScriptSyntaxKind::Switch,
                    "this" => ActionScriptSyntaxKind::This,
                    "throw" => ActionScriptSyntaxKind::Throw,
                    "true" => ActionScriptSyntaxKind::True,
                    "try" => ActionScriptSyntaxKind::Try,
                    "typeof" => ActionScriptSyntaxKind::Typeof,
                    "use" => ActionScriptSyntaxKind::Use,
                    "var" => ActionScriptSyntaxKind::Var,
                    "void" => ActionScriptSyntaxKind::Void,
                    "while" => ActionScriptSyntaxKind::While,
                    "with" => ActionScriptSyntaxKind::With,

                    // Contextual keywords
                    "each" => ActionScriptSyntaxKind::Each,
                    "get" => ActionScriptSyntaxKind::Get,
                    "set" => ActionScriptSyntaxKind::Set,
                    "namespace" => ActionScriptSyntaxKind::Namespace,
                    "include" => ActionScriptSyntaxKind::Include,
                    "dynamic" => ActionScriptSyntaxKind::Dynamic,
                    "final" => ActionScriptSyntaxKind::Final,
                    "override" => ActionScriptSyntaxKind::Override,

                    // Type keywords
                    "Array" => ActionScriptSyntaxKind::Array,
                    "Boolean" => ActionScriptSyntaxKind::Boolean,
                    "Date" => ActionScriptSyntaxKind::Date,
                    "Error" => ActionScriptSyntaxKind::Error,
                    "Function" => ActionScriptSyntaxKind::Function_,
                    "Number" => ActionScriptSyntaxKind::Number,
                    "Object" => ActionScriptSyntaxKind::Object,
                    "RegExp" => ActionScriptSyntaxKind::RegExp,
                    "String" => ActionScriptSyntaxKind::String_,
                    "uint" => ActionScriptSyntaxKind::Uint,
                    "Vector" => ActionScriptSyntaxKind::Vector,
                    "XML" => ActionScriptSyntaxKind::Xml,
                    "XMLList" => ActionScriptSyntaxKind::XmlList,

                    _ => ActionScriptSyntaxKind::Identifier,
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

    /// 处理操作符和分隔符
    fn lex_operator_or_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::Increment
                    }
                    else {
                        ActionScriptSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::Decrement
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::Arrow
                    }
                    else {
                        ActionScriptSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::StarAssign
                    }
                    else {
                        ActionScriptSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::SlashAssign
                    }
                    else {
                        ActionScriptSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::PercentAssign
                    }
                    else {
                        ActionScriptSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ActionScriptSyntaxKind::EqualEqualEqual
                        }
                        else {
                            ActionScriptSyntaxKind::EqualEqual
                        }
                    }
                    else {
                        ActionScriptSyntaxKind::Equal
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ActionScriptSyntaxKind::NotEqualEqual
                        }
                        else {
                            ActionScriptSyntaxKind::NotEqual
                        }
                    }
                    else {
                        ActionScriptSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ActionScriptSyntaxKind::LeftShiftAssign
                        }
                        else {
                            ActionScriptSyntaxKind::LeftShift
                        }
                    }
                    else {
                        ActionScriptSyntaxKind::LessThan
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                ActionScriptSyntaxKind::UnsignedRightShiftAssign
                            }
                            else {
                                ActionScriptSyntaxKind::UnsignedRightShift
                            }
                        }
                        else if let Some('=') = state.peek() {
                            state.advance(1);
                            ActionScriptSyntaxKind::RightShiftAssign
                        }
                        else {
                            ActionScriptSyntaxKind::RightShift
                        }
                    }
                    else {
                        ActionScriptSyntaxKind::GreaterThan
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::BitwiseAndAssign
                    }
                    else {
                        ActionScriptSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::BitwiseOrAssign
                    }
                    else {
                        ActionScriptSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ActionScriptSyntaxKind::BitwiseXorAssign
                    }
                    else {
                        ActionScriptSyntaxKind::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::BitwiseNot
                }
                '?' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Question
                }
                ':' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Colon
                }
                '(' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Dot
                }
                '@' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Dollar
                }
                '\\' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Backslash
                }
                '\'' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Quote
                }
                '"' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::DoubleQuote
                }
                '`' => {
                    state.advance(1);
                    ActionScriptSyntaxKind::Backtick
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

impl<'config> Lexer<ActionScriptLanguage> for ActionScriptLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ActionScriptSyntaxKind> {
        let mut state = LexerState::new(source);

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

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator_or_delimiter(&mut state) {
                continue;
            }

            // 如果没有匹配到任何模式，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ActionScriptSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(ActionScriptSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
