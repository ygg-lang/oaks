use crate::{kind::PhpSyntaxKind, language::PhpLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, OakError, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, PhpLanguage>;

#[derive(Clone)]
pub struct PhpLexer<'config> {
    config: &'config PhpLanguage,
}

impl<'config> PhpLexer<'config> {
    pub fn new(config: &'config PhpLanguage) -> Self {
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
                state.add_token(PhpSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                // 如果已到达文件末尾，退出循环
                break;
            }
        }

        // Add EOF token
        let pos = state.get_position();
        state.add_token(PhpSyntaxKind::Eof, pos, pos);

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
            state.add_token(PhpSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(PhpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(PhpSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);
                // 单行注释
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
            else if let Some('*') = state.peek() {
                state.advance(1);
                // 多行注释
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
                state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                return false;
            }
        }
        else if let Some('#') = state.peek() {
            state.advance(1);
            // PHP 风格的单行注释
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
            true
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
                        // 字符串不能跨行（除非转义）
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(PhpSyntaxKind::StringLiteral, start_pos, state.get_position());
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

                state.add_token(PhpSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                let start_pos = state.get_position();
                let mut text = String::new();

                // 读取标识符
                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        text.push(ch);
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键字
                let kind = match text.as_str() {
                    "abstract" => PhpSyntaxKind::Abstract,
                    "and" => PhpSyntaxKind::And,
                    "array" => PhpSyntaxKind::Array,
                    "as" => PhpSyntaxKind::As,
                    "break" => PhpSyntaxKind::Break,
                    "callable" => PhpSyntaxKind::Callable,
                    "case" => PhpSyntaxKind::Case,
                    "catch" => PhpSyntaxKind::Catch,
                    "class" => PhpSyntaxKind::Class,
                    "clone" => PhpSyntaxKind::Clone,
                    "const" => PhpSyntaxKind::Const,
                    "continue" => PhpSyntaxKind::Continue,
                    "declare" => PhpSyntaxKind::Declare,
                    "default" => PhpSyntaxKind::Default,
                    "die" => PhpSyntaxKind::Exit,
                    "do" => PhpSyntaxKind::Do,
                    "echo" => PhpSyntaxKind::Echo,
                    "else" => PhpSyntaxKind::Else,
                    "elseif" => PhpSyntaxKind::Elseif,
                    "empty" => PhpSyntaxKind::Empty,
                    "enddeclare" => PhpSyntaxKind::Enddeclare,
                    "endfor" => PhpSyntaxKind::Endfor,
                    "endforeach" => PhpSyntaxKind::Endforeach,
                    "endif" => PhpSyntaxKind::Endif,
                    "endswitch" => PhpSyntaxKind::Endswitch,
                    "endwhile" => PhpSyntaxKind::Endwhile,
                    "eval" => PhpSyntaxKind::Eval,
                    "exit" => PhpSyntaxKind::Exit,
                    "extends" => PhpSyntaxKind::Extends,
                    "final" => PhpSyntaxKind::Final,
                    "finally" => PhpSyntaxKind::Finally,
                    "for" => PhpSyntaxKind::For,
                    "foreach" => PhpSyntaxKind::Foreach,
                    "function" => PhpSyntaxKind::Function,
                    "global" => PhpSyntaxKind::Global,
                    "goto" => PhpSyntaxKind::Goto,
                    "if" => PhpSyntaxKind::If,
                    "implements" => PhpSyntaxKind::Implements,
                    "include" => PhpSyntaxKind::Include,
                    "include_once" => PhpSyntaxKind::IncludeOnce,
                    "instanceof" => PhpSyntaxKind::Instanceof,
                    "insteadof" => PhpSyntaxKind::Insteadof,
                    "interface" => PhpSyntaxKind::Interface,
                    "isset" => PhpSyntaxKind::Isset,
                    "list" => PhpSyntaxKind::List,
                    "namespace" => PhpSyntaxKind::Namespace,
                    "new" => PhpSyntaxKind::New,
                    "or" => PhpSyntaxKind::Or,
                    "print" => PhpSyntaxKind::Print,
                    "private" => PhpSyntaxKind::Private,
                    "protected" => PhpSyntaxKind::Protected,
                    "public" => PhpSyntaxKind::Public,
                    "require" => PhpSyntaxKind::Require,
                    "require_once" => PhpSyntaxKind::RequireOnce,
                    "return" => PhpSyntaxKind::Return,
                    "static" => PhpSyntaxKind::Static,
                    "switch" => PhpSyntaxKind::Switch,
                    "throw" => PhpSyntaxKind::Throw,
                    "trait" => PhpSyntaxKind::Trait,
                    "try" => PhpSyntaxKind::Try,
                    "unset" => PhpSyntaxKind::Unset,
                    "use" => PhpSyntaxKind::Use,
                    "var" => PhpSyntaxKind::Var,
                    "while" => PhpSyntaxKind::While,
                    "xor" => PhpSyntaxKind::Xor,
                    "yield" => PhpSyntaxKind::Yield,
                    "true" => PhpSyntaxKind::BooleanLiteral,
                    "false" => PhpSyntaxKind::BooleanLiteral,
                    "null" => PhpSyntaxKind::NullLiteral,
                    _ => PhpSyntaxKind::Identifier,
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
                        PhpSyntaxKind::Increment
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::PlusAssign
                    }
                    else {
                        PhpSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::Decrement
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::MinusAssign
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::Arrow
                    }
                    else {
                        PhpSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::Power
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::MultiplyAssign
                    }
                    else {
                        PhpSyntaxKind::Multiply
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::DivideAssign
                    }
                    else {
                        PhpSyntaxKind::Divide
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::ModuloAssign
                    }
                    else {
                        PhpSyntaxKind::Modulo
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::Identical
                        }
                        else {
                            PhpSyntaxKind::Equal
                        }
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::DoubleArrow
                    }
                    else {
                        PhpSyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::NotIdentical
                        }
                        else {
                            PhpSyntaxKind::NotEqual
                        }
                    }
                    else {
                        PhpSyntaxKind::LogicalNot
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::LessEqual
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::LeftShiftAssign
                        }
                        else {
                            PhpSyntaxKind::LeftShift
                        }
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::Spaceship
                    }
                    else {
                        PhpSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::GreaterEqual
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::RightShiftAssign
                        }
                        else {
                            PhpSyntaxKind::RightShift
                        }
                    }
                    else {
                        PhpSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::LogicalAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::BitwiseAndAssign
                    }
                    else {
                        PhpSyntaxKind::BitwiseAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::LogicalOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::BitwiseOrAssign
                    }
                    else {
                        PhpSyntaxKind::BitwiseOr
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::BitwiseXorAssign
                    }
                    else {
                        PhpSyntaxKind::BitwiseXor
                    }
                }
                '~' => {
                    state.advance(1);
                    PhpSyntaxKind::BitwiseNot
                }
                '?' => {
                    state.advance(1);
                    if let Some('?') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::NullCoalesce
                    }
                    else {
                        PhpSyntaxKind::Question
                    }
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::DoubleColon
                    }
                    else {
                        PhpSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    PhpSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    PhpSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::ConcatAssign
                    }
                    else {
                        PhpSyntaxKind::Dot
                    }
                }
                '(' => {
                    state.advance(1);
                    PhpSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    PhpSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    PhpSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    PhpSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    PhpSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    PhpSyntaxKind::RightBrace
                }
                '$' => {
                    state.advance(1);
                    PhpSyntaxKind::Dollar
                }
                '@' => {
                    state.advance(1);
                    PhpSyntaxKind::At
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

impl<'config> Lexer<PhpLanguage> for PhpLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<PhpLanguage>,
    ) -> LexOutput<PhpLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
