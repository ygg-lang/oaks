use crate::{kind::PhpSyntaxKind, language::PhpLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, PhpLanguage>;

pub struct PhpLexer<'config> {
    config: &'config PhpLanguage,
}

impl<'config> PhpLexer<'config> {
    pub fn new(config: &'config PhpLanguage) -> Self {
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
            state.add_token(PhpSyntaxKind::Whitespace, start_pos, state.get_position());
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

    /// 处理 PHP 标签
    fn lex_php_tags(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // <?php 开始标
        if let Some('<') = state.peek() {
            if let Some('?') = source.get_char_at(start_pos + 1) {
                if let Some('p') = source.get_char_at(start_pos + 2) {
                    if let Some('h') = source.get_char_at(start_pos + 3) {
                        if let Some('p') = source.get_char_at(start_pos + 4) {
                            state.advance(5);
                            state.add_token(PhpSyntaxKind::OpenTag, start_pos, state.get_position());
                            return true;
                        }
                    }
                }
                // <? 短标
                state.advance(2);
                state.add_token(PhpSyntaxKind::OpenTag, start_pos, state.get_position());
                return true;
            }
            // <?= echo 标签
            else if let Some('=') = source.get_char_at(start_pos + 1) {
                state.advance(3);
                state.add_token(PhpSyntaxKind::EchoTag, start_pos, state.get_position());
                return true;
            }
        }

        // ?> 结束标签
        if let Some('?') = state.peek() {
            if let Some('>') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(PhpSyntaxKind::CloseTag, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // // 单行注释
        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        // # 单行注释
        if let Some('#') = state.peek() {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }
            state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
            return true;
        }

        // /* */ 多行注释
        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            break;
                        }
                        else {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(PhpSyntaxKind::Comment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string_literal(&self, state: &mut State) -> bool {
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
                        break; // 字符串不能跨行（除非转义
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                if found_end {
                    state.add_token(PhpSyntaxKind::StringLiteral, start_pos, state.get_position());
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
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number_literal(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = state.source.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1);

                            // 小数部分
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
                }

                // 检查科学计数法
                if let Some(e_char) = state.peek() {
                    if e_char == 'e' || e_char == 'E' {
                        let saved_pos = state.get_position();
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数部分
                        let exp_start = state.get_position();
                        while let Some(digit) = state.peek() {
                            if digit.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }

                        if state.get_position() == exp_start {
                            // 没有有效的指数，回退
                            state.set_position(saved_pos);
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

    /// 处理变量
    fn lex_variable(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('$') = state.peek() {
            state.advance(1);

            // 变量名必须以字母或下划线开
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    state.advance(ch.len_utf8());

                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(PhpSyntaxKind::Variable, start_pos, state.get_position());
                    true
                }
                else {
                    // 只有 $ 符号，回退
                    state.set_position(start_pos);
                    false
                }
            }
            else {
                // 只有 $ 符号，回退
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.source.get_text(start_pos, state.get_position());
                let token_kind = match text.to_lowercase().as_str() {
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
                    "true" | "false" => PhpSyntaxKind::BooleanLiteral,
                    "null" => PhpSyntaxKind::NullLiteral,
                    _ => PhpSyntaxKind::Identifier,
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

    /// 处理运算
    fn lex_operators(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
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
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::PowerAssign
                        }
                        else {
                            PhpSyntaxKind::Power
                        }
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
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::Spaceship
                        }
                        else {
                            PhpSyntaxKind::LessEqual
                        }
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
                '.' => {
                    state.advance(1);
                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some('.') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::Ellipsis
                        }
                        else {
                            // 回退一个字符，这不是省略号
                            state.set_position(state.get_position() - 1);
                            PhpSyntaxKind::Concat
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        PhpSyntaxKind::ConcatAssign
                    }
                    else {
                        PhpSyntaxKind::Concat
                    }
                }
                '?' => {
                    state.advance(1);
                    if let Some('?') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            PhpSyntaxKind::NullCoalesceAssign
                        }
                        else {
                            PhpSyntaxKind::NullCoalesce
                        }
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
    fn lex_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => PhpSyntaxKind::LeftParen,
                ')' => PhpSyntaxKind::RightParen,
                '[' => PhpSyntaxKind::LeftBracket,
                ']' => PhpSyntaxKind::RightBracket,
                '{' => PhpSyntaxKind::LeftBrace,
                '}' => PhpSyntaxKind::RightBrace,
                ';' => PhpSyntaxKind::Semicolon,
                ',' => PhpSyntaxKind::Comma,
                '\\' => PhpSyntaxKind::Backslash,
                '@' => PhpSyntaxKind::At,
                '$' => PhpSyntaxKind::Dollar,
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

impl<'config> Lexer<PhpLanguage> for PhpLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<PhpSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_php_tags(&mut state, source) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
                continue;
            }

            if self.lex_string_literal(&mut state) {
                continue;
            }

            if self.lex_number_literal(&mut state) {
                continue;
            }

            if self.lex_variable(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operators(&mut state, source) {
                continue;
            }

            if self.lex_delimiters(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(PhpSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(PhpSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
